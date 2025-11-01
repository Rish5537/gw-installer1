// 🧩 Gignaati Workbench Installer
// 🔧 Node.js Detection Module (Phase 3.4.2)

use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct NodeCheckResult {
    pub installed: bool,
    pub version: Option<String>,
    pub compatible: bool,
    pub message: String,
}

#[tauri::command]
pub fn check_nodejs_installed() -> NodeCheckResult {
    let output = Command::new("node")
        .arg("-v")
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let version_raw = String::from_utf8_lossy(&result.stdout).trim().to_string();

                // Basic version compatibility check (>= 18.0.0)
                let compatible = version_raw
                    .trim_start_matches('v')
                    .split('.')
                    .next()
                    .and_then(|v| v.parse::<u32>().ok())
                    .map(|major| major >= 18)
                    .unwrap_or(false);

                NodeCheckResult {
                    installed: true,
                    version: Some(version_raw.clone()),
                    compatible,
                    message: if compatible {
                        format!("✅ Using existing Node.js {}", version_raw)
                    } else {
                        format!("⚠ Node.js {} is outdated (v18+ required)", version_raw)
                    },
                }
            } else {
                NodeCheckResult {
                    installed: false,
                    version: None,
                    compatible: false,
                    message: "⚠ Node.js not found on system PATH.".to_string(),
                }
            }
        }
        Err(_) => NodeCheckResult {
            installed: false,
            version: None,
            compatible: false,
            message: "⚠ Node.js not found on system PATH.".to_string(),
        },
    }
}
