// 🔧 Phase 3.1 — System Detection & Validation Integration

// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import modules
mod system;       // ✅ New module (replaces old `system_check`)
mod installer;

use system::detector::validate_requirements; // Import function

// Example command — still available for testing
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // ✅ Register all callable commands
        .invoke_handler(tauri::generate_handler![
            greet,
            validate_requirements,
            installer::run_installation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}