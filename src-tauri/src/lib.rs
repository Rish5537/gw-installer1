// 🧩 Gignaati Workbench Installer Backend
// 🔧 Phase 3.3 — Port Management System + System Validation Integration

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// === Modules ===
mod system;      // ✅ System detection and validation (Phase 3.1)
mod installer;   // ✅ Installation handler
mod ports;       // ✅ New: Port management system (Phase 3.3)

use system::detector::validate_requirements;
use ports::manager::allocate_ports;

// === Example test command ===
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// === Tauri entrypoint ===
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,                   // sample
            validate_requirements,   // from system/detector.rs
            allocate_ports,          // from ports/manager.rs
            installer::run_installation // from installer.rs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
