use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemInfo {
    node: Option<String>,
    git: Option<String>,
    python: Option<String>,
    os: String,
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

    SystemInfo { node, git, python, os }
}
