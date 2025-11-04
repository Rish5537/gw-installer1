// 🧩 Installer Module Index
// Central export hub for all installer logic components

// === Modules ===
pub mod nodejs;
pub mod n8n;
pub mod ollama;
pub mod runner;
pub mod smart;
pub mod progress;
pub mod cleanup;
pub mod install_n8n_real;  // ✅ real npm-based n8n installer
pub mod ollama_real;       // ✅ guided Ollama installer
pub mod environment;

// === Re-exports for lib.rs ===
pub use nodejs::check_nodejs_installed;
pub use n8n::{check_n8n_installed, install_n8n};
pub use ollama::{check_ollama_installed, install_ollama};
pub use runner::run_installation;
pub use smart::{smart_installer, launch_platform};
pub use progress::start_progress_tracking;
pub use cleanup::cleanup_installation;
pub use install_n8n_real::install_n8n_real;
pub use ollama_real::install_ollama_real;
pub use environment::{validate_environment, launch_n8n_internally};