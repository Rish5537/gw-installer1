use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Serialize)]
pub struct OllamaStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub message: String,
}

#[tauri::command]
pub fn check_ollama_installed() -> Result<OllamaStatus, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "ollama --version"]).output()
    } else {
        Command::new("sh").arg("-c").arg("ollama --version").output()
    };

    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Ok(OllamaStatus {
                installed: true,
                version: Some(version),
                message: "✅ AI Brain detected.".into(),
            })
        }
        _ => Ok(OllamaStatus {
            installed: false,
            version: None,
            message: "⚠ AI Brain not found.".into(),
        }),
    }
}

#[tauri::command]
pub fn install_ollama(app: AppHandle) -> Result<(), String> {
    app.emit("install-log", "⬇ Installing AI Brain...").ok();

    let cmd_str = if cfg!(target_os = "windows") {
        "powershell -Command \"Invoke-WebRequest https://ollama.ai/download/OllamaSetup.exe -OutFile $env:TEMP\\OllamaSetup.exe; Start-Process $env:TEMP\\OllamaSetup.exe -Wait\""
    } else {
        "curl -fsSL https://ollama.ai/install.sh | sh"
    };

    let mut process = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd_str])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start Ollama install: {}", e))?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start Ollama install: {}", e))?
    };

    let stdout = process.stdout.take().ok_or("No stdout available")?;
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let msg = line.unwrap_or_default();
        app.emit("install-log", format!("[AI Brain] {}", msg)).ok();
    }

    let output = process.wait_with_output().map_err(|e| e.to_string())?;
    if output.status.success() {
        app.emit("install-log", "✅ AI Brain installation completed.").ok();
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("AI Brain install failed: {}", err))
    }
}
