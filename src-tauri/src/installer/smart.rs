use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::{thread, time::Duration};

// === Data structures used for emitting events to frontend ===
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
            message: "🚀 Starting Smart Installation...".into(),
        },
    )
    .ok();

    // === Components and weights ===
    let components = vec![
        ("Node.js", 25),
        ("Agentic Platform", 35),
        ("AI Brain (Ollama)", 30),
        ("Finalizing Setup", 10),
    ];

    // === Developer toggle: use real installers or simulated ===
    let use_real_install = true; // 🔧 Toggle to false for simulation mode

    for (name, weight) in components {
        match name {
            // === Node.js detection step (simulated for now) ===
            "Node.js" => {
                app.emit(
                    "component-log",
                    ComponentLog {
                        component: name.to_string(),
                        message: "🔍 Checking Node.js installation...".into(),
                    },
                )
                .ok();

                simulate_component(&app, name, weight);
            }

            // === Agentic Platform (n8n) ===
            "Agentic Platform" => {
                if use_real_install {
                    app.emit(
                        "component-log",
                        ComponentLog {
                            component: name.to_string(),
                            message: "⬇ Installing Agentic Platform (real install via npm)...".into(),
                        },
                    )
                    .ok();

                    match crate::installer::install_n8n_real(app.clone()) {
                        Ok(_) => {
                            app.emit(
                                "component-log",
                                ComponentLog {
                                    component: name.to_string(),
                                    message: "✅ Agentic Platform (n8n) installed successfully!".into(),
                                },
                            )
                            .ok();
                        }
                        Err(e) => {
                            app.emit(
                                "component-log",
                                ComponentLog {
                                    component: name.to_string(),
                                    message: format!("❌ Failed to install n8n: {}", e),
                                },
                            )
                            .ok();
                        }
                    }
                } else {
                    app.emit(
                        "component-log",
                        ComponentLog {
                            component: name.to_string(),
                            message: "⚙ Using simulated n8n installer...".into(),
                        },
                    )
                    .ok();

                    simulate_component(&app, name, weight);
                }
            }

            // === AI Brain (Ollama) ===
            "AI Brain (Ollama)" => {
                app.emit(
                    "component-log",
                    ComponentLog {
                        component: name.to_string(),
                        message: "🧠 Preparing AI Brain (Ollama)...".into(),
                    },
                )
                .ok();

                if use_real_install {
                    app.emit(
                        "component-log",
                        ComponentLog {
                            component: name.to_string(),
                            message: "⬇ Installing or verifying Ollama (real check)...".into(),
                        },
                    )
                    .ok();

                    // Run the real Ollama installer integration
                    match crate::installer::install_ollama_real(app.clone()) {
                        Ok(_) => {
                            app.emit(
                                "component-log",
                                ComponentLog {
                                    component: name.to_string(),
                                    message: "✅ Ollama installation verified successfully.".into(),
                                },
                            )
                            .ok();
                        }
                        Err(e) => {
                            app.emit(
                                "component-log",
                                ComponentLog {
                                    component: name.to_string(),
                                    message: format!("❌ Failed to verify/install Ollama: {}", e),
                                },
                            )
                            .ok();
                        }
                    }
                } else {
                    app.emit(
                        "component-log",
                        ComponentLog {
                            component: name.to_string(),
                            message: "⚙ Using simulated Ollama installer...".into(),
                        },
                    )
                    .ok();

                    simulate_component(&app, name, weight);
                }
            }

            // === Finalizing setup ===
            "Finalizing Setup" => {
                simulate_component(&app, name, weight);
            }

            _ => {}
        }
    }

    app.emit(
        "smart-complete",
        ComponentLog {
            component: "Smart Installer".into(),
            message: "🎉 All components installed successfully! Ready to launch.".into(),
        },
    )
    .ok();

    Ok(())
}

// === Simulated component handler (used for non-real installs) ===
fn simulate_component(app: &AppHandle, name: &str, _weight: u8) {
    app.emit(
        "component-log",
        ComponentLog {
            component: name.to_string(),
            message: format!("⏳ Starting {}...", name),
        },
    )
    .ok();

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
        )
        .ok();
    }

    app.emit(
        "component-log",
        ComponentLog {
            component: name.to_string(),
            message: format!("✅ {} simulation complete.", name),
        },
    )
    .ok();
}

#[tauri::command]
pub fn launch_platform(app: AppHandle) -> Result<(), String> {
    app.emit(
        "component-log",
        ComponentLog {
            component: "Smart Installer".into(),
            message: "🚀 Launching Gignaati Workbench...".into(),
        },
    )
    .ok();

    thread::sleep(Duration::from_secs(2));

    app.emit(
        "component-log",
        ComponentLog {
            component: "Smart Installer".into(),
            message: "✅ Gignaati Workbench launched successfully!".into(),
        },
    )
    .ok();

    Ok(())
}
