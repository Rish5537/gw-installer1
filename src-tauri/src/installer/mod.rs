// 🧩 Installer module index

pub mod nodejs;
pub mod n8n;
pub mod ollama;
pub mod runner;
pub mod smart;
pub mod progress;
pub mod cleanup;

// Re-exports for lib.rs
pub use nodejs::check_nodejs_installed;
pub use n8n::{check_n8n_installed, install_n8n};
pub use ollama::{check_ollama_installed, install_ollama};
pub use runner::run_installation;
pub use smart::{smart_installer, launch_platform};
pub use progress::start_progress_tracking;
