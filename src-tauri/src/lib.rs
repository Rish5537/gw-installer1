// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Import the new Rust system check module
mod system_check;
mod installer;

// Simple example command (default)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Register both greet and detect_system commands
        .invoke_handler(tauri::generate_handler![
            greet,
            system_check::detect_system,
            installer::run_installation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

