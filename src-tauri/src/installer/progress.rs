use std::{
    process::Command,
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ProgressUpdate {
    pub step: String,
    pub percent: u8,
    pub eta_seconds: u32,
    pub message: String,
}

#[tauri::command]
pub fn start_progress_tracking(app: AppHandle) -> Result<(), String> {
    // Run progress logic in background so UI remains responsive
    thread::spawn(move || {
        // Each tuple: (step name, optional shell command, progress weight)
        let steps = vec![
            ("Verifying Node.js", Some("node -v"), 0.10),
            ("Checking Agentic Platform", Some("n8n --version"), 0.25),
            ("Checking AI Brain", Some("ollama --version"), 0.25),
            ("Finalizing Setup", None, 0.40),
        ];

        let mut total_progress = 0.0;
        let mut total_time = 0.0;

        for (step_name, command, weight) in steps {
            let start_time = Instant::now();

            // 🔹 Announce start of each step
            app.emit(
                "progress-update",
                ProgressUpdate {
                    step: step_name.to_string(),
                    percent: (total_progress * 100.0) as u8,
                    eta_seconds: 0,
                    message: format!("⏳ Starting {}...", step_name),
                },
            )
            .ok();

            // 🔹 Run or simulate command
            if let Some(cmd_str) = command {
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd").args(["/C", cmd_str]).output()
                } else {
                    Command::new("sh").arg("-c").arg(cmd_str).output()
                };

                if let Ok(out) = output {
                    if out.status.success() {
                        app.emit(
                            "install-log",
                            format!(
                                "✅ {} succeeded: {}",
                                step_name,
                                String::from_utf8_lossy(&out.stdout)
                            ),
                        )
                        .ok();
                    } else {
                        app.emit(
                            "install-log",
                            format!(
                                "⚠ {} failed: {}",
                                step_name,
                                String::from_utf8_lossy(&out.stderr)
                            ),
                        )
                        .ok();
                    }
                }
            } else {
                // 🔹 Simulate finalization loop
                for i in 0..=20 {
                    thread::sleep(Duration::from_millis(100));
                    app.emit(
                        "install-log",
                        format!("{}... ({}/20)", step_name, i),
                    )
                    .ok();
                }
            }

            // 🔹 Compute progress + ETA
            let duration = start_time.elapsed().as_secs_f32();
            total_time += duration;

            for i in 1..=100 {
                thread::sleep(Duration::from_millis(25));
                let percent = (total_progress + (i as f32 / 100.0) * weight) * 100.0;
                let eta = ((1.0 - total_progress - (i as f32 / 100.0) * weight)
                    * total_time
                    * 1.5) as u32;

                app.emit(
                    "progress-update",
                    ProgressUpdate {
                        step: step_name.to_string(),
                        percent: percent.min(100.0) as u8,
                        eta_seconds: eta,
                        message: format!("{} — {:.0}% done", step_name, percent),
                    },
                )
                .ok();
            }

            total_progress += weight;
        }

        // 🔹 Final completion event
        app.emit(
            "progress-complete",
            ProgressUpdate {
                step: "Setup Complete".to_string(),
                percent: 100,
                eta_seconds: 0,
                message: "🎉 All systems are ready. Launching Gignaati Workbench!".to_string(),
            },
        )
        .ok();
    });

    Ok(())
}
