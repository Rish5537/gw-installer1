use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::process::Command;
use std::path::Path;
use std::env;

#[derive(Serialize, Clone)]
struct ComponentLog {
    component: String,
    message: String,
}

#[tauri::command]
pub fn install_ollama_real(app: AppHandle) -> Result<(), String> {
    let component_name = "AI Brain (Ollama)";

    app.emit("component-log", ComponentLog {
        component: component_name.into(),
        message: "🧠 Checking Ollama installation...".into(),
    }).ok();

    if let Some(path) = detect_ollama_path() {
        if let Some(ver) = check_ollama_version(&path) {
            app.emit("component-log", ComponentLog {
                component: component_name.into(),
                message: format!("✅ Ollama detected at '{}' (version {}).", path, ver),
            }).ok();

            // ✨ Friendly summary
            app.emit("component-log", ComponentLog {
                component: component_name.into(),
                message: "✅ Already installed — no action required.".into(),
            }).ok();

            return Ok(());
        }
    }

    // ❌ If not detected
    app.emit("component-log", ComponentLog {
        component: component_name.into(),
        message: "⚠ Ollama not found on this system.".into(),
    }).ok();
    app.emit("component-log", ComponentLog {
        component: component_name.into(),
        message: "💡 Please download Ollama manually from https://ollama.com/download/windows".into(),
    }).ok();
    Err("Ollama not found — waiting for manual installation".into())
}

/// ✅ Detect Ollama binary
fn detect_ollama_path() -> Option<String> {
    if Command::new("ollama").arg("--version").output().is_ok() {
        return Some("ollama".into());
    }

    if let Ok(local) = env::var("LOCALAPPDATA") {
        let user_path = format!("{local}\\Programs\\Ollama\\ollama.exe");
        if Path::new(&user_path).exists() {
            println!("✅ Found Ollama in user-local path: {}", user_path);
            return Some(user_path);
        }
    }

    let win_candidates = [
        r"C:\Program Files\Ollama\ollama.exe",
        r"C:\Program Files (x86)\Ollama\ollama.exe",
    ];
    for path in win_candidates {
        if Path::new(path).exists() {
            println!("✅ Found Ollama at {}", path);
            return Some(path.to_string());
        }
    }

    let unix_candidates = ["/usr/local/bin/ollama", "/usr/bin/ollama"];
    for c in unix_candidates {
        if Path::new(c).exists() {
            println!("✅ Found Ollama at {}", c);
            return Some(c.to_string());
        }
    }

    println!("⚠ Ollama not detected in known paths");
    None
}

/// 🧾 Run `ollama --version`
fn check_ollama_version(path: &str) -> Option<String> {
    if let Ok(output) = Command::new(path).arg("--version").output() {
        if output.status.success() {
            return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }
    None
}
