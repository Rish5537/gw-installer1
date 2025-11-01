use std::{fs, net::TcpListener};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager}; // ✅ Required for app_handle.path()

#[derive(Debug, Serialize, Deserialize)]
pub struct PortConfig {
    pub n8n_port: u16,
    pub ollama_port: u16,
}

/// ✅ Helper function to check if a port is free
fn check_port_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

/// ✅ Finds first available port in range
fn find_available_port(start: u16, end: u16) -> Option<u16> {
    (start..=end).find(|&port| check_port_available(port))
}

/// ✅ Allocates ports and saves configuration file in app data dir
#[tauri::command]
pub fn allocate_ports(app_handle: AppHandle) -> Result<PortConfig, String> {
    let base_n8n = 5678;
    let base_ollama = 11434;

    let n8n_port = find_available_port(base_n8n, base_n8n + 20).unwrap_or(base_n8n);
    let ollama_port = find_available_port(base_ollama, base_ollama + 20).unwrap_or(base_ollama);

    let config = PortConfig {
        n8n_port,
        ollama_port,
    };

    // ✅ Access path API via Manager trait
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    // ✅ Ensure the directory exists
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    }

    // ✅ Save configuration file
    let config_path = app_data_dir.join(".gwconfig");
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(config)
}
