use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct N8nStatus {
    installed: bool,
    version: Option<String>,
}

#[tauri::command]
pub fn check_n8n_installed() -> Result<N8nStatus, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "n8n --version"])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("n8n --version")
            .output()
    };

    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Ok(N8nStatus {
                installed: true,
                version: Some(version),
            })
        }
        _ => Ok(N8nStatus {
            installed: false,
            version: None,
        }),
    }
}
