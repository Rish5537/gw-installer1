// 🧩 Installer module index
// Centralizes and re-exports installer-related functionality

// === Submodules ===
pub mod nodejs;
pub mod n8n;
pub mod ollama;
pub mod runner;

// === Re-exports for easier access in lib.rs ===
pub use nodejs::check_nodejs_installed;
pub use n8n::check_n8n_installed;
pub use ollama::check_ollama_installed;
pub use runner::run_installation;
