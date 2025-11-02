// 🧩 Gignaati Workbench Installer Backend
// 🔧 Phase 3.7 — Smart Progress Tracking & System Validation

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
    install_n8n,
    install_ollama,
    run_installation,
    smart_installer,
    launch_platform,
    start_progress_tracking,
};

// === Example command (useful for testing basic Tauri bridge) ===
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// === Main Tauri Application Entry Point ===
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
            install_ollama,
            run_installation,
            smart_installer,
            launch_platform,
            start_progress_tracking
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("❌ Failed to run Gignaati Workbench Installer");
}
