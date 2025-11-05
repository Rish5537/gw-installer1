use tauri::{AppHandle, Emitter, Manager}; // Manager removed — not used in Tauri v2
use serde::Serialize;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use once_cell::sync::Lazy;

use crate::config::AppConfig;

#[derive(Serialize, Clone)]
struct ComponentLog {
    component: String,
    message: String,
}

// Global handle for n8n child process
static N8N_PROCESS: Lazy<Arc<Mutex<Option<std::process::Child>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

// Utility: check whether something is listening on the given port (127.0.0.1)
fn is_listening(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}

// Try to free a port. Windows uses netstat + taskkill; Unix uses lsof + kill.
fn free_port(port: u16) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let cmd = format!("netstat -ano | findstr :{}", port);
        let output = Command::new("cmd")
            .args(&["/C", &cmd])
            .output()
            .map_err(|e| format!("Failed to run netstat: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if stdout.trim().is_empty() {
            return Ok(());
        }

        for line in stdout.lines() {
            if let Some(pid_str) = line.split_whitespace().last() {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    let _ = Command::new("taskkill")
                        .args(&["/F", "/PID", &pid.to_string()])
                        .output()
                        .map_err(|e| format!("Failed taskkill: {}", e))?;
                }
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("lsof -t -i :{}", port))
            .output()
            .map_err(|e| format!("Failed to run lsof: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if stdout.trim().is_empty() {
            return Ok(());
        }

        for pid_line in stdout.lines() {
            if let Ok(pid) = pid_line.trim().parse::<i32>() {
                let _ = Command::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .output()
                    .map_err(|e| format!("Failed to kill PID {}: {}", pid, e))?;
            }
        }
        Ok(())
    }
}

// Try locating possible n8n executable paths or fallback to `npx n8n`
fn detect_n8n_command() -> (String, Vec<String>) {
    #[cfg(target_os = "windows")]
    {
        let default_path = r"C:\Users\Nilkhil\AppData\Roaming\npm\n8n.cmd";
        if std::path::Path::new(default_path).exists() {
            return (default_path.to_string(), vec!["start".to_string()]);
        }
    }

    if which::which("n8n").is_ok() {
        return ("n8n".to_string(), vec!["start".to_string()]);
    }

    if which::which("npx").is_ok() {
        return (
            "npx".to_string(),
            vec!["--yes".to_string(), "n8n".to_string(), "start".to_string()],
        );
    }

    ("npx".to_string(), vec!["n8n".to_string(), "start".to_string()])
}

/// 🚀 Launch n8n with OLLAMA_API_URL
#[tauri::command]
pub fn launch_n8n_with_ollama(app: AppHandle) -> Result<(), String> {
    let component = "Agentic Platform (n8n)";
    app.emit("component-log", ComponentLog {
        component: component.into(),
        message: "🚀 Launching n8n with Ollama binding...".into(),
    }).ok();

    let cfg = AppConfig::load();
    let ollama_port = cfg.ollama_port.unwrap_or(11434);
    let n8n_port = cfg.n8n_port.unwrap_or(5678);

    let _ = free_port(n8n_port);

    let (bin, mut base_args) = detect_n8n_command();
    base_args.push("--port".to_string());
    base_args.push(n8n_port.to_string());

    let mut cmd = Command::new(&bin);
    for a in &base_args {
        cmd.arg(a);
    }

    cmd.env("OLLAMA_API_URL", format!("http://127.0.0.1:{}", ollama_port));
    cmd.env("DB_SQLITE_POOL_SIZE", "2");
    cmd.env("N8N_RUNNERS_ENABLED", "true");
    cmd.env("N8N_BLOCK_ENV_ACCESS_IN_NODE", "false");
    cmd.env("N8N_GIT_NODE_DISABLE_BARE_REPOS", "true");
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("❌ Failed to launch n8n: {}", e))?;
    let stdout_opt = child.stdout.take();
    let stderr_opt = child.stderr.take();

    {
        let mut guard = N8N_PROCESS.lock().unwrap();
        *guard = Some(child);
    }

    // 🔁 Stream logs
    if let Some(stdout) = stdout_opt {
        let app_clone = app.clone();
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines().flatten() {
                let _ = app_clone.emit("component-log", ComponentLog {
                    component: "Agentic Platform (n8n)".into(),
                    message: line,
                });
            }
        });
    }

    if let Some(stderr) = stderr_opt {
        let app_clone = app.clone();
        thread::spawn(move || {
            for line in BufReader::new(stderr).lines().flatten() {
                let _ = app_clone.emit("component-log", ComponentLog {
                    component: "Agentic Platform (n8n)".into(),
                    message: format!("⚠ {}", line),
                });
            }
        });
    }

    // ⏳ Wait a few seconds before opening
    thread::sleep(Duration::from_secs(3));
    app.emit("component-log", ComponentLog {
        component: component.into(),
        message: format!("✅ n8n launched on port {}.", n8n_port),
    }).ok();

    // 🌐 Open in main Tauri window
    if let Some(main_window) = app.webview_windows().get("main") {
        let url = format!("http://127.0.0.1:{}", n8n_port);
        let _ = main_window.eval(&format!("window.location.href = '{}';", url));
    }

    Ok(())
}

/// 🛑 Stop n8n process
#[tauri::command]
pub fn stop_n8n(app: AppHandle) -> Result<(), String> {
    let component = "Agentic Platform (n8n)";
    let mut guard = N8N_PROCESS.lock().unwrap();

    if let Some(child) = guard.as_mut() {
        let _ = child.kill();
        *guard = None;
        app.emit("component-log", ComponentLog {
            component: component.into(),
            message: "🛑 n8n stopped.".into(),
        }).ok();
        return Ok(());
    }

    app.emit("component-log", ComponentLog {
        component: component.into(),
        message: "ℹ n8n was not running.".into(),
    }).ok();
    Ok(())
}

/// 🔍 Check n8n health
#[tauri::command]
pub fn check_n8n_health(app: AppHandle) -> Result<String, String> {
    let cfg = AppConfig::load();
    let n8n_port = cfg.n8n_port.unwrap_or(5678);
    let addr = format!("127.0.0.1:{}", n8n_port);

    if TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(2)).is_ok() {
        let msg = format!("✅ n8n is reachable at http://{}", addr);
        app.emit("component-log", ComponentLog {
            component: "Agentic Platform (n8n)".into(),
            message: msg.clone(),
        }).ok();
        Ok(msg)
    } else {
        let msg = format!("❌ n8n not responding at http://{}", addr);
        app.emit("component-log", ComponentLog {
            component: "Agentic Platform (n8n)".into(),
            message: msg.clone(),
        }).ok();
        Err(msg)
    }
}

/// 🌐 Launch Agentic Platform UI in same window
#[tauri::command]
pub fn launch_agentic_platform(app: AppHandle) -> Result<(), String> {
    let cfg = AppConfig::load();
    let n8n_port = cfg.n8n_port.unwrap_or(5678);
    let n8n_url = format!("http://127.0.0.1:{}", n8n_port);

    app.emit("component-log", ComponentLog {
        component: "Agentic Platform (n8n)".into(),
        message: format!("🌐 Launching Agentic Platform at {}", n8n_url),
    }).ok();

    // Ensure n8n is running
    if !is_listening(n8n_port) {
        let _ = launch_n8n_with_ollama(app.clone());
        thread::sleep(Duration::from_secs(3));
    }

    // Open inside main window
    if let Some(main_window) = app.webview_windows().get("main") {
        let _ = main_window.eval(&format!("window.location.href = '{}';", n8n_url));
    }

    Ok(())
}
