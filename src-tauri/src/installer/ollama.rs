use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct OllamaStatus {
    installed: bool,
    version: Option<String>,
}

#[tauri::command]
pub fn check_ollama_installed() -> Result<OllamaStatus, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "ollama --version"])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("ollama --version")
            .output()
    };

    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Ok(OllamaStatus {
                installed: true,
                version: Some(version),
            })
        }
        _ => Ok(OllamaStatus {
            installed: false,
            version: None,
        }),
    }
}
