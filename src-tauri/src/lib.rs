// 🧩 Gignaati Workbench Installer Backend
// 🔧 Phase 4.2 — Config + Port Management Foundation (n8n + Ollama)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// === Modules ===
mod system;
mod config;     // ✅ Global configuration manager
mod ports;      // ✅ Port allocation and detection
mod installer;

// === Imports ===
use tauri::AppHandle;
use system::detector::validate_requirements;
// import the manager's allocate_ports and PortConfig from the ports module
use crate::ports::manager::{allocate_ports, PortConfig};
use installer::{
    check_nodejs_installed,
    check_n8n_installed,
    check_ollama_installed,
    validate_environment,
    install_n8n,          // legacy simulated install (kept for fallback)
    install_n8n_real,     // ✅ real npm-based installer
    install_ollama,       // simulated fallback
    install_ollama_real,  // ✅ guided safe installer
    run_installation,
    smart_installer,
    launch_platform,
    start_progress_tracking,
    cleanup_installation,
    launch_n8n_internally,
};

// === Example Command (for Tauri) ===
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// === Config & Port Commands ===

/// ✅ Allocate ports and return the PortConfig to the frontend
#[tauri::command]
fn allocate_ports_command(app: AppHandle) -> Result<PortConfig, String> {
    // forward to ports::manager::allocate_ports which returns Result<PortConfig, String>
    allocate_ports(app)
}

/// ✅ Load and return global configuration (AppConfig)
#[tauri::command]
fn get_config_command() -> Result<crate::config::AppConfig, String> {
    Ok(crate::config::AppConfig::load())
}

// === Main Entry Point ===
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // --- Utility / System ---
            greet,
            validate_requirements,
            allocate_ports_command,  // ✅ returns PortConfig
            get_config_command,      // ✅ returns AppConfig

            // --- Core Installers ---
            check_nodejs_installed,
            check_n8n_installed,
            check_ollama_installed,
            validate_environment,
            install_n8n,
            install_n8n_real,
            install_ollama,
            install_ollama_real,

            // --- Execution Flow ---
            run_installation,
            smart_installer,
            launch_platform,
            start_progress_tracking,
            cleanup_installation,

            // --- Internal Launch ---
            launch_n8n_internally,
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("❌ Failed to run Gignaati Workbench Installer");
}
