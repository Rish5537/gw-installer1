use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::{thread, time::Duration};

/// Progress payload for cleanup progress bar
#[derive(Serialize, Clone)]
pub struct CleanupProgress {
    pub component: String,
    pub percent: u8,
    pub status: String,
    pub message: String,
}

/// Unified log payload to match frontend ComponentLog type
#[derive(Serialize, Clone)]
pub struct ComponentLog {
    pub component: String,
    pub message: String,
}

#[tauri::command]
pub async fn cleanup_installation(app: AppHandle) -> Result<(), String> {
    let components = vec!["Node.js", "Agentic Platform", "AI Brain"];

    // Start cleanup header message
    app.emit(
        "component-log",
        ComponentLog {
            component: "Cleanup".into(),
            message: "🧹 Starting Cleanup Process...".into(),
        },
    )
    .ok();

    // Simulate cleanup process for each component
    for name in components {
        simulate_cleanup(&app, name);
    }

    // Finish message
    app.emit(
        "component-log",
        ComponentLog {
            component: "Cleanup".into(),
            message: "✅ Cleanup Complete. System is ready for a fresh installation.".into(),
        },
    )
    .ok();

    Ok(())
}

/// Simulates removing a component (with fake progress)
fn simulate_cleanup(app: &AppHandle, name: &str) {
    app.emit(
        "component-log",
        ComponentLog {
            component: name.to_string(),
            message: format!("🧼 Removing {}...", name),
        },
    )
    .ok();

    for i in 0..=100 {
        thread::sleep(Duration::from_millis(35));
        app.emit(
            "component-progress",
            CleanupProgress {
                component: name.to_string(),
                percent: i,
                status: if i < 100 {
                    "running".into()
                } else {
                    "done".into()
                },
                message: format!("Cleaning {}... {}%", name, i),
            },
        )
        .ok();
    }

    app.emit(
        "component-log",
        ComponentLog {
            component: name.to_string(),
            message: format!("🗑 {} removed successfully.", name),
        },
    )
    .ok();
}
