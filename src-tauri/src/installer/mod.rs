// 🧩 Installer module index

// Submodules
pub mod nodejs;

// Re-exports for easier access in lib.rs
pub use crate::installer::nodejs::check_nodejs_installed;

// ✅ Your installer logic moved here (from installer.rs)
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct InstallProgress {
    step: String,
    message: String,
    progress: u8,
}

#[tauri::command]
pub fn run_installation(app_handle: AppHandle) -> Result<(), String> {
    let steps = vec![
        ("System Setup", "echo Preparing system..."),
        ("Node Setup", "node -v"),
        ("Git Setup", "git --version"),
        ("Python Setup", "python --version"),
    ];

    for (i, (step_name, cmd)) in steps.iter().enumerate() {
        let progress = ((i + 1) as f32 / steps.len() as f32 * 100.0) as u8;

        app_handle
            .emit(
                "install-progress",
                InstallProgress {
                    step: step_name.to_string(),
                    message: format!("Running `{}`...", cmd),
                    progress,
                },
            )
            .map_err(|e| e.to_string())?;

        let mut output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", cmd])
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|e| e.to_string())?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|e| e.to_string())?
        };

        let reader = BufReader::new(output.stdout.take().ok_or("Failed to read output")?);

        for line in reader.lines() {
            let msg = line.unwrap_or_default();
            app_handle
                .emit("install-log", format!("[{}] {}", step_name, msg))
                .map_err(|e| e.to_string())?;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    app_handle
        .emit(
            "install-complete",
            InstallProgress {
                step: "Complete".to_string(),
                message: "Installation successful!".to_string(),
                progress: 100,
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}
