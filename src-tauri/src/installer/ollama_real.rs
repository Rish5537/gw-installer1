use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::process::{Command, Stdio};

#[derive(Serialize, Clone)]
struct ComponentLog {
    component: String,
    message: String,
}

#[tauri::command]
pub fn install_ollama_real(app: AppHandle) -> Result<(), String> {
    app.emit("component-log", ComponentLog {
        component: "AI Brain".into(),
        message: "🧠 Checking Ollama installation...".into(),
    }).ok();

    // --- Check if Ollama is already installed ---
    let check = Command::new("ollama")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    match check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            app.emit("component-log", ComponentLog {
                component: "AI Brain".into(),
                message: format!("✅ Ollama detected ({})", version),
            }).ok();
            return Ok(());
        }
        _ => {
            // --- Not found — ask user to install manually ---
            app.emit("component-log", ComponentLog {
                component: "AI Brain".into(),
                message: "⚠ Ollama not found on this system.".into(),
            }).ok();
            app.emit("component-log", ComponentLog {
                component: "AI Brain".into(),
                message: "💡 Please download Ollama manually from https://ollama.com/download/windows".into(),
            }).ok();
            app.emit("component-log", ComponentLog {
                component: "AI Brain".into(),
                message: "⬆ Once installed, click 'Check Again' in the installer to continue.".into(),
            }).ok();
            return Err("Ollama not found — waiting for manual installation".into());
        }
    }
}
