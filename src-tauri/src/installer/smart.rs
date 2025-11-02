use tauri::{AppHandle, Emitter};
use crate::installer::{
    check_nodejs_installed,
    check_n8n_installed,
    check_ollama_installed,
    install_n8n,
    install_ollama,
    run_installation,
};
use std::{thread, time::Duration};
use serde::Serialize;

#[derive(Serialize, Clone)] // ✅ added Clone
pub struct SmartProgress {
    step: String,
    message: String,
    progress: u8,
}

#[tauri::command]
pub async fn smart_installer(app: AppHandle) -> Result<(), String> {
    app.emit("install-log", "🚀 Starting Smart Installer...").ok();

    // === Step 1: Check Node.js ===
    let node_check = check_nodejs_installed(); // ✅ no .map_err

    if !node_check.installed {
        app.emit(
            "install-log",
            "❌ Node.js not found. Please install it first using the official link.",
        )
        .ok();

        #[cfg(target_os = "windows")]
        let url = "https://nodejs.org/dist/latest-v18.x/node-v18.x-x64.msi";
        #[cfg(target_os = "macos")]
        let url = "https://nodejs.org/en/download/";
        #[cfg(target_os = "linux")]
        let url = "https://nodejs.org/en/download/package-manager/";

        app.emit("node-missing", url).ok();
        return Err("Node.js not installed".into());
    }

    app.emit("install-log", "✅ Node.js detected.").ok();

    // === Step 2: Agentic Platform (n8n) ===
    let n8n_status = check_n8n_installed().unwrap_or_else(|_| {
        panic!("n8n check failed");
    });

    if !n8n_status.installed {
        app.emit("install-log", "⬇ Installing Agentic Platform...").ok();
        install_n8n(app.clone())?;
        app.emit("install-log", "✅ Agentic Platform installed.").ok();
    } else {
        app.emit("install-log", "✅ Agentic Platform already installed.").ok();
    }

    // === Step 3: AI Brain (Ollama) ===
    let ollama_status = check_ollama_installed().unwrap_or_else(|_| {
        panic!("Ollama check failed");
    });

    if !ollama_status.installed {
        app.emit("install-log", "⬇ Installing AI Brain...").ok();
        install_ollama(app.clone())?;
        app.emit("install-log", "✅ AI Brain installed.").ok();
    } else {
        app.emit("install-log", "✅ AI Brain already installed.").ok();
    }

    // === Step 4: Final installation tasks ===
    app.emit("install-log", "⚙ Finalizing setup...").ok();
    run_installation(app.clone())?;

    thread::sleep(Duration::from_secs(1));

    app.emit(
        "install-complete",
        SmartProgress {
            step: "Complete".into(),
            message: "🎉 All systems ready. Launch Gignaati Workbench.".into(),
            progress: 100,
        },
    )
    .ok();

    Ok(())
}

#[tauri::command]
pub fn launch_platform(app: AppHandle) -> Result<(), String> {
    app.emit("install-log", "🚀 Launching Gignaati Workbench...").ok();

    // Try to run n8n through npm if global binary is missing
    let commands = if cfg!(target_os = "windows") {
        vec![
            ("cmd", vec!["/C", "n8n"]),
            ("cmd", vec!["/C", "npx", "n8n"]),
            ("cmd", vec!["/C", "npm", "exec", "n8n"]),
        ]
    } else {
        vec![
            ("sh", vec!["-c", "n8n"]),
            ("sh", vec!["-c", "npx n8n"]),
            ("sh", vec!["-c", "npm exec n8n"]),
        ]
    };

    for (exe, args) in commands {
        if std::process::Command::new(exe)
            .args(args)
            .spawn()
            .is_ok()
        {
            app.emit("install-log", "✅ Gignaati Workbench launched!").ok();
            return Ok(());
        }
    }

    Err("❌ Could not launch Gignaati Workbench. Please verify installation.".into())
}

