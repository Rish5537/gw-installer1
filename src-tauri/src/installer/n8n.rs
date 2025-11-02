use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Serialize)]
pub struct N8nStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub message: String,
}

#[tauri::command]
pub fn check_n8n_installed() -> Result<N8nStatus, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "n8n --version"]).output()
    } else {
        Command::new("sh").arg("-c").arg("n8n --version").output()
    };

    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Ok(N8nStatus {
                installed: true,
                version: Some(version),
                message: "✅ Agentic Platform detected.".into(),
            })
        }
        _ => Ok(N8nStatus {
            installed: false,
            version: None,
            message: "⚠ Agentic Platform not found.".into(),
        }),
    }
}

#[tauri::command]
pub fn install_n8n(app: AppHandle) -> Result<(), String> {
    app.emit("install-log", "⬇ Installing Agentic Platform...").ok();

    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "npm install -g n8n"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start install: {}", e))?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("npm install -g n8n")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start install: {}", e))?
    };

    let stdout = cmd.stdout.take().ok_or("No stdout from install")?;
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.unwrap_or_default();
        app.emit("install-log", format!("[Agentic] {}", line)).ok();
    }

    let output = cmd.wait_with_output().map_err(|e| e.to_string())?;
    if output.status.success() {
        app.emit("install-log", "✅ Agentic Platform installation completed.").ok();
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("Agentic Platform install failed: {}", err))
    }
}
