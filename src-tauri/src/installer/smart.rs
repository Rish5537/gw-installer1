use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::{thread, time::Duration};

#[derive(Serialize, Clone)]
pub struct ComponentProgress {
    component: String,
    percent: u8,
    status: String,
    message: String,
    eta_seconds: Option<u32>,
}

#[derive(Serialize, Clone)]
pub struct ComponentLog {
    component: String,
    message: String,
}

#[tauri::command]
pub async fn smart_installer(app: AppHandle) -> Result<(), String> {
    app.emit(
        "component-log",
        ComponentLog {
            component: "Smart Installer".into(),
            message: "🚀 Starting Smart Installation (simulation)...".into(),
        },
    ).ok();

    let components = vec![
        ("Node.js", 25),
        ("Agentic Platform", 35),
        ("AI Brain", 30),
        ("Finalizing Setup", 10),
    ];

    for (name, weight) in components {
        simulate_component(&app, name, weight);
    }

    app.emit(
        "smart-complete",
        ComponentLog {
            component: "Smart Installer".into(),
            message: "🎉 All simulated components installed successfully! Ready to launch.".into(),
        },
    ).ok();

    Ok(())
}

/// Simulate each component installation
fn simulate_component(app: &AppHandle, name: &str, _weight: u8) {
    app.emit(
        "component-log",
        ComponentLog {
            component: name.to_string(),
            message: format!("⏳ Starting {}...", name),
        },
    ).ok();

    for i in 1..=100 {
        thread::sleep(Duration::from_millis(40));
        let eta = Some(((100 - i) / 2) as u32);

        app.emit(
            "component-progress",
            ComponentProgress {
                component: name.to_string(),
                percent: i,
                status: if i < 100 { "running".into() } else { "done".into() },
                message: format!("{} progress: {}%", name, i),
                eta_seconds: eta,
            },
        ).ok();
    }

    app.emit(
        "component-log",
        ComponentLog {
            component: name.to_string(),
            message: format!("✅ {} simulation complete.", name),
        },
    ).ok();
}

#[tauri::command]
pub fn launch_platform(app: AppHandle) -> Result<(), String> {
    app.emit(
        "component-log",
        ComponentLog {
            component: "Smart Installer".into(),
            message: "🚀 Launching simulated Gignaati Workbench...".into(),
        },
    ).ok();

    thread::sleep(Duration::from_secs(2));

    app.emit(
        "component-log",
        ComponentLog {
            component: "Smart Installer".into(),
            message: "✅ Simulation: Gignaati Workbench launched successfully!".into(),
        },
    ).ok();

    Ok(())
}
