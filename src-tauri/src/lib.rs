// 🧩 Gignaati Workbench Installer Backend
// 🔧 Phase 3.4 — Node.js Detection + Installer Integration

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// === Modules ===
mod system;      // ✅ System detection
mod ports;       // ✅ Port management
mod installer;   // ✅ Installer + Node.js detection

// === Imports ===
use system::detector::validate_requirements;
use ports::manager::allocate_ports;
use installer::{run_installation, check_nodejs_installed};

// === Example command ===
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// === Entry point ===
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            validate_requirements,
            allocate_ports,
            run_installation,
            check_nodejs_installed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
