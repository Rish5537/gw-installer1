use std::process::Command;
use serde::Serialize;
use sysinfo::{System, SystemExt, DiskExt};

#[derive(Serialize)]
pub struct SystemInfo {
    node: Option<String>,
    git: Option<String>,
    python: Option<String>,
    os: String,
    ram_gb: u64,
    disk_gb: u64,
}

#[tauri::command]
pub fn detect_system() -> SystemInfo {
    let node = Command::new("node")
        .arg("-v")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok());

    let git = Command::new("git")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok());

    let python = Command::new("python")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok());

    let os = std::env::consts::OS.to_string();

    let mut sys = System::new_all();
    sys.refresh_all();
    let ram_gb = sys.total_memory() / 1024 / 1024;
    let disk_gb = sys
        .disks()
        .iter()
        .map(|d| d.total_space() / 1024 / 1024 / 1024)
        .sum();

    SystemInfo {
        node,
        git,
        python,
        os,
        ram_gb,
        disk_gb,
    }
}
