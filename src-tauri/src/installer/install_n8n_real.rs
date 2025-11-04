use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::path::Path;

#[derive(Serialize, Clone)]
struct ComponentLog {
    component: String,
    message: String,
}

#[tauri::command]
pub fn install_n8n_real(app: AppHandle) -> Result<(), String> {
    let component_name = "Agentic Platform";

    app.emit("component-log", ComponentLog {
        component: component_name.into(),
        message: "⬇ Checking Agentic Platform (n8n) installation...".into(),
    }).ok();

    // === Step 1: Detect existing n8n ===
    if let Some(existing_path) = detect_existing_n8n() {
        if let Ok(output) = Command::new(&existing_path).arg("--version").output() {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                app.emit("component-log", ComponentLog {
                    component: component_name.into(),
                    message: format!(
                        "✅ n8n already installed at '{}' (version {}). Skipping reinstall.",
                        existing_path, version
                    ),
                }).ok();

                // ✨ Friendly summary for UI
                app.emit("component-log", ComponentLog {
                    component: component_name.into(),
                    message: "✅ Already installed — no action required.".into(),
                }).ok();

                app.emit("component-progress", serde_json::json!({
                    "component": component_name,
                    "percent": 100,
                    "status": "done",
                    "message": "Agentic Platform already installed.",
                    "eta_seconds": 0
                })).ok();

                return Ok(());
            }
        }
    }

    // === Step 2: Locate npm ===
    let npm_cmd = detect_npm_path().ok_or_else(|| {
        app.emit("component-log", ComponentLog {
            component: component_name.into(),
            message: "⚠ npm not found. Ensure Node.js is installed and added to PATH.".into(),
        }).ok();
        "npm not found in PATH or standard locations.".to_string()
    })?;

    app.emit("component-log", ComponentLog {
        component: component_name.into(),
        message: format!("🧠 Using npm from '{}'", npm_cmd),
    }).ok();

    // === Step 3: Run installation ===
    let mut cmd = Command::new(&npm_cmd)
        .args(["install", "-g", "n8n@latest", "--legacy-peer-deps", "--force"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start npm: {}", e))?;

    let stdout = cmd.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = cmd.stderr.take().ok_or("Failed to capture stderr")?;

    let app_out = app.clone();
    let comp_out = component_name.to_string();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            if let Some(filtered) = filter_log_line(&line, false) {
                app_out.emit("component-log", ComponentLog {
                    component: comp_out.clone(),
                    message: filtered,
                }).ok();
            }
        }
    });

    let app_err = app.clone();
    let comp_err = component_name.to_string();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            if let Some(filtered) = filter_log_line(&line, false) {
                app_err.emit("component-log", ComponentLog {
                    component: comp_err.clone(),
                    message: filtered,
                }).ok();
            }
        }
    });

    let status = cmd.wait().map_err(|e| format!("Failed to wait on npm: {}", e))?;

    if status.success() {
        app.emit("component-log", ComponentLog {
            component: component_name.into(),
            message: "✅ n8n successfully installed globally!".into(),
        }).ok();

        app.emit("component-progress", serde_json::json!({
            "component": component_name,
            "percent": 100,
            "status": "done",
            "message": "Agentic Platform (n8n) installation complete.",
            "eta_seconds": 0
        })).ok();

        Ok(())
    } else {
        app.emit("component-progress", serde_json::json!({
            "component": component_name,
            "percent": 100,
            "status": "failed",
            "message": "n8n installation failed.",
            "eta_seconds": 0
        })).ok();
        Err(format!("❌ n8n installation failed with status {:?}", status))
    }
}

/// ✅ Detect existing n8n installation (handles user & global npm folders)
fn detect_existing_n8n() -> Option<String> {
    if Command::new("n8n").arg("--version").output().is_ok() {
        return Some("n8n".into());
    }

    if let Ok(appdata) = std::env::var("APPDATA") {
        let path = format!("{appdata}\\npm\\n8n.cmd");
        if Path::new(&path).exists() {
            println!("✅ Found n8n at {}", path);
            return Some(path);
        }
    }

    let candidates = [
        r"C:\ProgramData\npm\n8n.cmd",
        r"C:\Program Files\nodejs\n8n.cmd",
        r"C:\Program Files (x86)\nodejs\n8n.cmd",
        "/usr/local/bin/n8n",
        "/usr/bin/n8n",
    ];
    for c in candidates {
        if Path::new(c).exists() {
            println!("✅ Found n8n at {}", c);
            return Some(c.to_string());
        }
    }

    println!("⚠ n8n not found in any known paths");
    None
}

/// 🔍 Detect npm binary
fn detect_npm_path() -> Option<String> {
    if Command::new("npm").arg("-v").output().is_ok() {
        return Some("npm".to_string());
    }

    let candidates = [
        r"C:\Program Files\nodejs\npm.cmd",
        r"C:\Program Files (x86)\nodejs\npm.cmd",
        r"C:\Users\%USERNAME%\AppData\Roaming\npm\npm.cmd",
        "/usr/local/bin/npm",
        "/usr/bin/npm",
        "/opt/homebrew/bin/npm",
    ];
    for c in candidates {
        let expanded = shellexpand::full(c).unwrap_or_else(|_| c.into()).to_string();
        if Path::new(&expanded).exists() {
            return Some(expanded);
        }
    }
    None
}

/// 🧹 Filter noisy npm logs
fn filter_log_line(line: &str, dev: bool) -> Option<String> {
    if dev { return Some(line.to_string()); }
    let l = line.trim();
    if l.starts_with("npm WARN") || l.contains("deprecated") { return None; }
    if l.contains("added ") { return Some(format!("📦 {}", l)); }
    if l.contains("up to date") { return Some(format!("✅ {}", l)); }
    if l.contains("ERR!") { return Some(format!("❌ {}", l)); }
    if !l.is_empty() { Some(l.to_string()) } else { None }
}
