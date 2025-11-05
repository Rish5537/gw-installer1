// 🧩 Gignaati Workbench Installer Backend
// 🔧 Phase 4.4 — Port Binding, n8n Launch Integration, and Ollama Bridge
//
// This file connects all installer, environment, and runtime systems
// for the Gignaati Workbench backend (n8n + Ollama + SmartInstaller).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// === Core Modules ===
mod system;
mod config;            // ✅ Global configuration manager
mod ports;             // ✅ Port allocation and detection logic
mod installer;         // ✅ Installation orchestration (Node, n8n, Ollama)
mod ollama_server;     // ✅ Ollama runtime manager (serve, stop, models)
mod n8n_manager;       // ✅ Agentic Platform controller (n8n + Ollama bridge)

// === Imports ===
use tauri::AppHandle;
use system::detector::validate_requirements;
use crate::ports::manager::{allocate_ports, PortConfig};
use ollama_server::*;
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
    format!("Hello, {}! Welcome to the Gignaati Workbench Installer 🚀", name)
}

// === Config & Port Commands ===

/// ✅ Allocate ports dynamically and return the assigned configuration.
#[tauri::command]
fn allocate_ports_command(app: AppHandle) -> Result<PortConfig, String> {
    allocate_ports(app)
}

/// ✅ Load and return global configuration for the Workbench.
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

            // --- Config & Port Layer ---
            allocate_ports_command,   // returns PortConfig
            get_config_command,       // returns AppConfig

            // --- Core Installers (Node.js + n8n + Ollama) ---
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

            // --- Ollama Runtime Control ---
            start_ollama_server,      // ✅ Start local Ollama service
            stop_ollama_server,       // ✅ Stop it safely
            list_ollama_models,       // ✅ List available local models
            pull_ollama_model,        // ✅ Download new LLM models
            repair_ollama_model,
            remove_ollama_model,
            cancel_ollama_download,

            // --- Agentic Platform / n8n Integration ---
            n8n_manager::launch_n8n_with_ollama,   // 🚀 Launch n8n bound to Ollama port
            n8n_manager::stop_n8n,                 // 🛑 Stop n8n process
            n8n_manager::check_n8n_health,         // 🔎 Check n8n health
            n8n_manager::launch_agentic_platform,  // 🌐 Open Agentic Platform UI in main webview

            // --- Internal Launch (n8n UI) ---
            launch_n8n_internally,
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("❌ Failed to run Gignaati Workbench Installer");
}
