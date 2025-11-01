// 🧩 Gignaati Workbench Installer Backend
// 🔧 Phase 3.4 — Node.js + n8n + Ollama Detection Integration

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
    run_installation,
};

// === Example command (still useful for testing) ===
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// === Main Tauri Application Entry ===
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            validate_requirements,
            allocate_ports,
            check_nodejs_installed,
            check_n8n_installed,
            check_ollama_installed,
            run_installation
        ])
        .run(tauri::generate_context!())
        .expect("❌ Failed to run Gignaati Workbench Installer");
}
