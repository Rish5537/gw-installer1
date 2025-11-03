use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

#[derive(Serialize, Clone)]
struct ComponentLog {
    component: String,
    message: String,
}

#[tauri::command]
pub fn install_n8n_real(app: AppHandle) -> Result<(), String> {
    let component_name = "Agentic Platform";

    app.emit(
        "component-log",
        ComponentLog {
            component: component_name.into(),
            message: "⬇ Installing Agentic Platform (real install via npm)...".into(),
        },
    )
    .ok();

    // --- Step 1: Try to locate npm automatically ---
    let npm_cmd = detect_npm_path().ok_or_else(|| {
        app.emit(
            "component-log",
            ComponentLog {
                component: component_name.into(),
                message:
                    "⚠ npm not found. Ensure Node.js is installed and added to PATH.".into(),
            },
        )
        .ok();
        "npm not found in PATH or standard locations.".to_string()
    })?;

    app.emit(
        "component-log",
        ComponentLog {
            component: component_name.into(),
            message: format!("🧠 Using npm from '{}'", npm_cmd),
        },
    )
    .ok();

    // --- Step 2: Launch npm install ---
    let mut cmd = Command::new(&npm_cmd)
        .args(["install", "-g", "n8n@latest", "--legacy-peer-deps"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start npm: {}", e))?;

    let stdout = cmd
        .stdout
        .take()
        .ok_or("Failed to capture stdout from npm")?;
    let stderr = cmd
        .stderr
        .take()
        .ok_or("Failed to capture stderr from npm")?;

    // --- Stream stdout ---
    let app_out = app.clone();
    let comp_out = component_name.to_string();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            if let Some(filtered) = filter_log_line(&line, false) {
                app_out
                    .emit(
                        "component-log",
                        ComponentLog {
                            component: comp_out.clone(),
                            message: filtered,
                        },
                    )
                    .ok();
            }
        }
    });

    // --- Stream stderr ---
    let app_err = app.clone();
    let comp_err = component_name.to_string();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            if let Some(filtered) = filter_log_line(&line, false) {
                app_err
                    .emit(
                        "component-log",
                        ComponentLog {
                            component: comp_err.clone(),
                            message: filtered,
                        },
                    )
                    .ok();
            }
        }
    });

    // --- Wait for process ---
    let status = cmd
        .wait()
        .map_err(|e| format!("Failed to wait on npm: {}", e))?;

    if status.success() {
        app.emit(
            "component-log",
            ComponentLog {
                component: component_name.into(),
                message: "✅ n8n successfully installed globally!".into(),
            },
        )
        .ok();
        Ok(())
    } else {
        Err(format!("❌ n8n installation failed with status {:?}", status))
    }
}

/// 🔍 Auto-detect npm binary path across common locations
fn detect_npm_path() -> Option<String> {
    // Try default first
    if Command::new("npm").arg("-v").output().is_ok() {
        return Some("npm".to_string());
    }

    // Windows common installs
    let candidates = [
        r"C:\Program Files\nodejs\npm.cmd",
        r"C:\Program Files (x86)\nodejs\npm.cmd",
        r"C:\Users\%USERNAME%\AppData\Roaming\nvm\v18.17.0\npm.cmd",
        r"C:\Users\%USERNAME%\AppData\Roaming\npm\npm.cmd",
    ];

    for c in candidates {
        if std::path::Path::new(c).exists() {
            return Some(c.to_string());
        }
    }

    // Unix-like systems
    let unix_candidates = ["/usr/local/bin/npm", "/opt/homebrew/bin/npm", "/usr/bin/npm"];
    for c in unix_candidates {
        if std::path::Path::new(c).exists() {
            return Some(c.to_string());
        }
    }

    None
}

/// 🧹 Filter out noisy npm logs
fn filter_log_line(line: &str, developer_mode: bool) -> Option<String> {
    if developer_mode {
        return Some(line.to_string());
    }

    let l = line.trim();

    // Ignore noise
    if l.contains("deprecated")
        || l.contains("npm WARN deprecated")
        || l.contains("npm WARN ERESOLVE")
        || l.contains("conflicting peer dependency")
        || l.contains("peerOptional")
        || l.starts_with("npm WARN")
    {
        return None;
    }

    // Highlight useful info
    if l.contains("added ") && l.contains("packages") {
        return Some(format!("📦 {}", l));
    }
    if l.contains("audited ") {
        return Some(format!("🔍 {}", l));
    }
    if l.contains("up to date") {
        return Some(format!("✅ {}", l));
    }
    if l.contains("ERR!") {
        return Some(format!("❌ {}", l));
    }

    if !l.is_empty() {
        Some(l.to_string())
    } else {
        None
    }
}
