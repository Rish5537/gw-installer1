// 🧩 Gignaati Workbench Installer Backend
// 🔧 Phase 4.1 — Real Installation (n8n + Ollama Safe Mode)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// === Modules ===
mod system;
mod ports;
mod installer;

// === Imports ===
use system::detector::validate_requirements;
use ports::manager::allocate_ports;
use installer::{
    check_nodejs_installed,
    check_n8n_installed,
    check_ollama_installed,
    install_n8n,          // legacy simulated install (kept for fallback)
    install_n8n_real,     // ✅ new real npm-based installer
    install_ollama,       // simulated fallback
    install_ollama_real,  // ✅ new safe guided installer
    run_installation,
    smart_installer,
    launch_platform,
    start_progress_tracking,
    cleanup_installation,
};

// === Example Command ===
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// === Main Entry Point ===
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            validate_requirements,
            allocate_ports,
            check_nodejs_installed,
            check_n8n_installed,
            check_ollama_installed,
            install_n8n,
            install_n8n_real,
            install_ollama,
            install_ollama_real,
            run_installation,
            smart_installer,
            launch_platform,
            start_progress_tracking,
            cleanup_installation
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("❌ Failed to run Gignaati Workbench Installer");
}
