use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::path::Path;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;

// ✅ Import your AppConfig
use crate::config::AppConfig;

// ✅ Use once_cell instead of lazy_static (modern, lightweight)
use once_cell::sync::Lazy;

#[derive(Serialize, Clone)]
struct ComponentLog {
    component: String,
    message: String,
}

// ✅ Global mutable reference to Ollama server process
// We store the Command::Child process handle here once started
static OLLAMA_PROCESS: Lazy<Arc<Mutex<Option<std::process::Child>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// ✅ Start the Ollama server
#[tauri::command]
pub fn start_ollama_server(app: AppHandle) -> Result<(), String> {
    let component = "Ollama Server";

    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: "🚀 Attempting to start Ollama server...".into(),
        },
    ).ok();

    // Load configuration
    let config = AppConfig::load();
    let ollama_port = config.n8n_port.unwrap_or(11434); // ✅ fallback to default

    // Check for Ollama binary
    let ollama_path = detect_ollama_path()
        .ok_or("❌ Ollama binary not found on this system.")?;

    // If already running, skip
    if is_ollama_running(ollama_port) {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: format!("✅ Ollama already running on port {}", ollama_port),
            },
        ).ok();
        return Ok(());
    }

    // Spawn Ollama process
    let mut cmd = Command::new(ollama_path)
        .arg("serve")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("❌ Failed to start Ollama server: {}", e))?;

    let stdout = cmd.stdout.take().unwrap();
    let stderr = cmd.stderr.take().unwrap();

    // Stream stdout
    let app_out = app.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            app_out.emit(
                "component-log",
                ComponentLog {
                    component: component.into(),
                    message: line,
                },
            ).ok();
        }
    });

    // Stream stderr
    let app_err = app.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            app_err.emit(
                "component-log",
                ComponentLog {
                    component: component.into(),
                    message: format!("⚠ {}", line),
                },
            ).ok();
        }
    });

    // Store process handle
    {
        let mut handle = OLLAMA_PROCESS.lock().unwrap();
        *handle = Some(cmd);
    }

    // Wait a bit before confirming start
    thread::sleep(Duration::from_secs(3));

    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: format!("✅ Ollama server started successfully on port {}", ollama_port),
        },
    ).ok();

    Ok(())
}

/// ✅ Stop the Ollama server
#[tauri::command]
pub fn stop_ollama_server(app: AppHandle) -> Result<(), String> {
    let component = "Ollama Server";

    let mut handle = OLLAMA_PROCESS.lock().unwrap();
    if let Some(child) = handle.as_mut() {
        let _ = child.kill();
        *handle = None;

        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: "🛑 Ollama server stopped successfully.".into(),
            },
        ).ok();

        Ok(())
    } else {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: "ℹ Ollama server was not running.".into(),
            },
        ).ok();
        Ok(())
    }
}

/// ✅ List available models
#[tauri::command]
pub fn list_ollama_models() -> Result<Vec<String>, String> {
    let ollama_path = detect_ollama_path().ok_or("❌ Ollama binary not found.")?;

    let output = Command::new(&ollama_path)
        .arg("list")
        .output()
        .map_err(|e| format!("Failed to list models: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "❌ Failed to list models: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let result = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    Ok(result)
}

/// ✅ Pull model from registry
#[tauri::command]
pub fn pull_ollama_model(app: AppHandle, model_name: String) -> Result<(), String> {
    let component = "Ollama Model Pull";

    let ollama_path = detect_ollama_path().ok_or("❌ Ollama binary not found.")?;

    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: format!("⬇ Pulling model '{}'...", model_name),
        },
    ).ok();

    let mut cmd = Command::new(&ollama_path)
        .args(["pull", &model_name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start pull: {}", e))?;

    let stdout = cmd.stdout.take().unwrap();
    let stderr = cmd.stderr.take().unwrap();

    // Stream output
    let app_out = app.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            app_out.emit(
                "component-log",
                ComponentLog {
                    component: component.into(),
                    message: format!("📦 {}", line),
                },
            ).ok();
        }
    });

    // Stream errors
    let app_err = app.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            app_err.emit(
                "component-log",
                ComponentLog {
                    component: component.into(),
                    message: format!("⚠ {}", line),
                },
            ).ok();
        }
    });

    let status = cmd.wait().map_err(|e| format!("Error waiting for pull: {}", e))?;

    if status.success() {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: format!("✅ Model '{}' pulled successfully.", model_name),
            },
        ).ok();
        Ok(())
    } else {
        Err(format!("❌ Model pull failed: {:?}", status))
    }
}

/// 🧩 Utility — detect Ollama binary
fn detect_ollama_path() -> Option<String> {
    let candidates = [
        "ollama",
        r"C:\Program Files\Ollama\ollama.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Programs\Ollama\ollama.exe",
        "/usr/local/bin/ollama",
        "/usr/bin/ollama",
    ];
    for c in candidates {
        let expanded = shellexpand::full(c).unwrap_or_else(|_| c.into()).to_string();
        if Path::new(&expanded).exists() {
            return Some(expanded);
        }
    }
    None
}

/// 🩺 Utility — check if Ollama server is already running
fn is_ollama_running(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}
