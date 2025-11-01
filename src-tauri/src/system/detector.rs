use serde::Serialize;
use std::path::Path;
use fs2::available_space;
use sysinfo::{System};

// ✅ Struct to hold validation data
#[derive(Serialize, Debug)]
pub struct ValidationResult {
    pub passed: bool,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub os: String,
    pub ram_gb: u64,
    pub disk_gb: u64,
}

// ✅ Detect the current OS
fn detect_os() -> String {
    std::env::consts::OS.to_string()
}

// ✅ RAM check (returns GB)
fn check_ram() -> u64 {
    let sys = System::new_all();
    let total = sys.total_memory();
    // sysinfo gives memory in KiB → convert to GB
    total / 1024 / 1024
}

// ✅ Disk space check (returns GB)
fn check_disk_space<P: AsRef<Path>>(path: P) -> u64 {
    available_space(path).unwrap_or(0) / 1024 / 1024 / 1024
}

// ✅ Main command exposed to frontend
#[tauri::command]
pub fn validate_requirements(min_ram: u64, min_disk: u64) -> ValidationResult {
    let os = detect_os();
    let ram_gb = check_ram();
    let disk_gb = check_disk_space("C:\\");

    let mut issues = vec![];
    let mut warnings = vec![];

    if ram_gb < min_ram {
        issues.push(format!(
            "Insufficient RAM: {} GB (found) < {} GB (required)",
            ram_gb, min_ram
        ));
    }

    if disk_gb < min_disk {
        issues.push(format!(
            "Low disk space: {} GB (found) < {} GB (required)",
            disk_gb, min_disk
        ));
    }

    if os != "windows" && os != "macos" && os != "linux" {
        warnings.push(format!("Unrecognized OS detected: {}", os));
    }

    ValidationResult {
        passed: issues.is_empty(),
        issues,
        warnings,
        os,
        ram_gb,
        disk_gb,
    }
}
