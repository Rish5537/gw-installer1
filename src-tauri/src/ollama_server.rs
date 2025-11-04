use tauri::{AppHandle, Emitter};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::path::Path;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::env;
use once_cell::sync::Lazy;

use crate::config::AppConfig;

#[derive(Serialize, Clone)]
struct ComponentLog {
    component: String,
    message: String,
}

// === Global Handles ===
static OLLAMA_PROCESS: Lazy<Arc<Mutex<Option<std::process::Child>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

static OLLAMA_DOWNLOAD: Lazy<Arc<Mutex<Option<std::process::Child>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// 🚀 Start Ollama server
#[tauri::command]
pub fn start_ollama_server(app: AppHandle) -> Result<(), String> {
    let component = "Ollama Server";
    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: "🚀 Attempting to start Ollama server...".into(),
        },
    )
    .ok();

    let config = AppConfig::load();
    let ollama_port = config.n8n_port.unwrap_or(11434);
    let ollama_path =
        detect_ollama_path().ok_or("❌ Ollama binary not found on this system.")?;

    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: format!("📂 Ollama binary located at '{}'", ollama_path),
        },
    )
    .ok();

    if is_ollama_running(ollama_port) {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: format!("✅ Ollama already running on port {}", ollama_port),
            },
        )
        .ok();
        return Ok(());
    }

    let mut cmd = Command::new(&ollama_path)
        .arg("serve")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("❌ Failed to start Ollama server: {}", e))?;

    let stdout = cmd.stdout.take().unwrap();
    let stderr = cmd.stderr.take().unwrap();

    let app_out = app.clone();
    let component_out = component.to_string();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            app_out
                .emit(
                    "component-log",
                    ComponentLog {
                        component: component_out.clone(),
                        message: line,
                    },
                )
                .ok();
        }
    });

    let app_err = app.clone();
    let component_err = component.to_string();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            app_err
                .emit(
                    "component-log",
                    ComponentLog {
                        component: component_err.clone(),
                        message: format!("⚠ {}", line),
                    },
                )
                .ok();
        }
    });

    {
        let mut handle = OLLAMA_PROCESS.lock().unwrap();
        *handle = Some(cmd);
    }

    thread::sleep(Duration::from_secs(3));
    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: format!(
                "✅ Ollama server started successfully on port {}",
                ollama_port
            ),
        },
    )
    .ok();

    Ok(())
}

/// 🛑 Stop Ollama server
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
        )
        .ok();
    } else {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: "ℹ Ollama server was not running.".into(),
            },
        )
        .ok();
    }
    Ok(())
}

/// 📦 List available models
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

/// ⬇ Pull model from Ollama registry (real-time JSON progress)
#[tauri::command]
pub fn pull_ollama_model(app: AppHandle, model_name: String) -> Result<(), String> {
    let component = "Ollama Model Pull";
    let ollama_path = detect_ollama_path().ok_or("❌ Ollama binary not found.")?;

    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: format!("⬇ Starting download for model '{}'...", model_name),
        },
    )
    .ok();

    thread::spawn(move || {
        // Choose correct mode
        let cmd_result = if is_ollama_running(11434) {
            Command::new("curl")
                .args([
                    "-N",
                    "-s",
                    "-X",
                    "POST",
                    "http://localhost:11434/api/pull",
                    "-H",
                    "Content-Type: application/json",
                    "-d",
                    &format!("{{\"name\":\"{}\"}}", model_name),
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
        } else {
            Command::new(&ollama_path)
                .args(["pull", &model_name])
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
        };

        let mut cmd = match cmd_result {
            Ok(c) => c,
            Err(e) => {
                let _ = app.emit(
                    "component-log",
                    ComponentLog {
                        component: component.into(),
                        message: format!("❌ Failed to start pull: {}", e),
                    },
                );
                return;
            }
        };

        let stdout = cmd.stdout.take();
        {
            let mut dl = OLLAMA_DOWNLOAD.lock().unwrap();
            *dl = Some(cmd);
        }

        // Stream and parse JSON output
        if let Some(stdout) = stdout {
            let app_progress = app.clone();
            let comp = component.to_string();
            let model = model_name.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                let mut last_emit = Instant::now();
                let mut progress_seen = false;

                for line in reader.lines().flatten() {
                    if let Some(msg) = parse_ollama_json_line(&line) {
                        progress_seen = true;
                        if last_emit.elapsed() > Duration::from_secs(1) {
                            let _ = app_progress.emit(
                                "component-log",
                                ComponentLog {
                                    component: comp.clone(),
                                    message: msg,
                                },
                            );
                            last_emit = Instant::now();
                        }
                    }
                }

                if progress_seen {
                    let _ = app_progress.emit(
                        "component-log",
                        ComponentLog {
                            component: comp.clone(),
                            message: format!("✅ Finished pulling '{}'", model),
                        },
                    );
                }
            });
        }

        // Wait until completion or cancellation
        let status = {
            let mut handle = OLLAMA_DOWNLOAD.lock().unwrap();
            if let Some(mut child) = handle.take() {
                child.wait()
            } else {
                return;
            }
        };

        match status {
            Ok(s) if s.success() => {
                let _ = app.emit(
                    "component-log",
                    ComponentLog {
                        component: component.into(),
                        message: format!("✅ Model '{}' pulled successfully.", model_name),
                    },
                );
            }
            Ok(_) => {
                let _ = app.emit(
                    "component-log",
                    ComponentLog {
                        component: component.into(),
                        message:
                            "❌ Model pull failed. 💡 Try the Repair Model Pull option.".into(),
                    },
                );
            }
            Err(e) => {
                let _ = app.emit(
                    "component-log",
                    ComponentLog {
                        component: component.into(),
                        message: format!("❌ Pull interrupted: {}", e),
                    },
                );
            }
        }
    });

    Ok(())
}

/// ⏹ Cancel active model download
#[tauri::command]
pub fn cancel_ollama_download(app: AppHandle) -> Result<(), String> {
    let component = "Ollama Cancel Download";
    let mut handle = OLLAMA_DOWNLOAD.lock().unwrap();

    if let Some(child) = handle.as_mut() {
        let _ = child.kill();
        *handle = None;
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: "⏹ Download cancelled by user.".into(),
            },
        )
        .ok();
    } else {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: "ℹ No active download to cancel.".into(),
            },
        )
        .ok();
    }

    Ok(())
}

/// 🗑 Remove model
#[tauri::command]
pub fn remove_ollama_model(app: AppHandle, model_name: String) -> Result<(), String> {
    let component = "Ollama Remove Model";
    let ollama_path = detect_ollama_path().ok_or("❌ Ollama binary not found.")?;

    let output = Command::new(&ollama_path)
        .args(["rm", &model_name])
        .output()
        .map_err(|e| format!("❌ Failed to remove model: {}", e))?;

    if output.status.success() {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: format!("✅ Model '{}' removed successfully.", model_name),
            },
        )
        .ok();
    } else {
        app.emit(
            "component-log",
            ComponentLog {
                component: component.into(),
                message: format!(
                    "❌ Removal failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            },
        )
        .ok();
    }
    Ok(())
}

/// ♻ Repair model pull
#[tauri::command]
pub fn repair_ollama_model(app: AppHandle, model_name: String) -> Result<(), String> {
    let component = "Ollama Repair Pull";
    app.emit(
        "component-log",
        ComponentLog {
            component: component.into(),
            message: format!("🔄 Attempting to repair pull for '{}'...", model_name),
        },
    )
    .ok();
    pull_ollama_model(app, model_name)
}

// === Helpers ===

#[derive(Deserialize)]
struct OllamaProgress {
    status: Option<String>,
    completed: Option<u64>,
    total: Option<u64>,
}

/// Cleanly interpret Ollama JSON lines
fn parse_ollama_json_line(line: &str) -> Option<String> {
    if let Ok(json) = serde_json::from_str::<OllamaProgress>(line) {
        if let (Some(c), Some(t)) = (json.completed, json.total) {
            let pct = ((c as f64 / t as f64) * 100.0) as u8;
            Some(format!("📦 Downloading model: {}% complete", pct))
        } else if let Some(status) = json.status {
            Some(format!("📦 {}", status))
        } else {
            None
        }
    } else if line.contains("pulling") {
        Some(format!("📦 {}", line))
    } else {
        None
    }
}

fn detect_ollama_path() -> Option<String> {
    let username = env::var("USERNAME").unwrap_or_default();
    let candidates = vec![
        "ollama".to_string(),
        r"C:\Program Files\Ollama\ollama.exe".to_string(),
        format!(
            r"C:\Users\{}\AppData\Local\Programs\Ollama\ollama.exe",
            username
        ),
        "/usr/local/bin/ollama".to_string(),
        "/usr/bin/ollama".to_string(),
    ];
    for c in candidates {
        if Path::new(&c).exists() {
            return Some(c);
        }
    }
    None
}

fn is_ollama_running(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}
