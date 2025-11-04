use serde::Serialize;
use std::process::Command;
use std::time::Duration;
use tauri::{AppHandle, Emitter, WebviewWindowBuilder};
use tauri::WebviewUrl;

#[derive(Serialize, Debug)]
pub struct EnvironmentStatus {
    pub node_installed: bool,
    pub node_version: Option<String>,
    pub n8n_installed: bool,
    pub n8n_version: Option<String>,
    pub ollama_installed: bool,
    pub ollama_version: Option<String>,
}

#[tauri::command]
pub fn validate_environment() -> EnvironmentStatus {
    EnvironmentStatus {
        node_installed: check_exists("node"),
        node_version: get_version("node", "-v"),
        n8n_installed: check_exists("n8n"),
        n8n_version: get_version("n8n", "--version"),
        ollama_installed: check_exists("ollama"),
        ollama_version: get_version("ollama", "--version"),
    }
}

/// ✅ Quick check: does a binary exist in PATH?
fn check_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// 🧾 Capture version output
fn get_version(cmd: &str, arg: &str) -> Option<String> {
    Command::new(cmd)
        .arg(arg)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// 🧠 Launch n8n inside an embedded WebView (Tauri 2 syntax)
#[tauri::command]
pub async fn launch_n8n_internally(app: AppHandle) -> Result<(), String> {
    // Check if n8n is installed
    if !check_exists("n8n") {
        return Err("n8n is not installed.".into());
    }

    // Start n8n background process
    tauri::async_runtime::spawn(async move {
        let _ = Command::new("n8n")
            .arg("start")
            .spawn()
            .expect("Failed to start n8n background process");
    });

    // Wait for server to boot
    std::thread::sleep(Duration::from_secs(5));

    // Notify frontend
    app.emit(
        "component-log",
        "🌐 Launching n8n inside Gignaati Workbench...",
    )
    .ok();

    // ✅ Create a new Webview window pointing to the external URL
    let url = WebviewUrl::External("http://localhost:5678".parse().unwrap());

    WebviewWindowBuilder::new(&app, "n8n_webview", url)
        .title("Gignaati Workbench — Agentic Platform")
        .resizable(true)
        .fullscreen(false)
        .build()
        .map_err(|e| format!("Failed to open n8n window: {}", e))?;

    Ok(())
}
