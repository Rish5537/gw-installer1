use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{Read},
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub node_version: Option<String>,
    pub npm_version: Option<String>,
    pub n8n_installed: bool,
    pub n8n_path: Option<String>,
    pub n8n_port: Option<u16>,
    pub ollama_installed: bool,
    pub ollama_path: Option<String>,
    pub ollama_version: Option<String>,
    pub ollama_port: Option<u16>,
    pub ollama_default_model: Option<String>,
}

impl AppConfig {
    /// Load existing configuration or create a new default file
    pub fn load() -> Self {
        let path = config_path();
        if let Ok(mut file) = File::open(&path) {
            let mut data = String::new();
            if file.read_to_string(&mut data).is_ok() {
                if let Ok(parsed) = serde_json::from_str::<AppConfig>(&data) {
                    return parsed;
                }
            }
        }
        // create new default file
        let cfg = AppConfig::default();
        cfg.save();
        cfg
    }

    /// Save configuration back to disk
    pub fn save(&self) {
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    /// Update fields from a partial config and persist
    #[allow(dead_code)]
    pub fn update(&mut self, partial: AppConfig) {
        if partial.node_version.is_some() {
            self.node_version = partial.node_version;
        }
        if partial.npm_version.is_some() {
            self.npm_version = partial.npm_version;
        }
        if partial.n8n_path.is_some() {
            self.n8n_path = partial.n8n_path;
        }
        if partial.n8n_port.is_some() {
            self.n8n_port = partial.n8n_port;
        }
        if partial.ollama_path.is_some() {
            self.ollama_path = partial.ollama_path;
        }
        if partial.ollama_port.is_some() {
            self.ollama_port = partial.ollama_port;
        }
        if partial.ollama_version.is_some() {
            self.ollama_version = partial.ollama_version;
        }
        if partial.ollama_default_model.is_some() {
            self.ollama_default_model = partial.ollama_default_model;
        }
        self.n8n_installed |= partial.n8n_installed;
        self.ollama_installed |= partial.ollama_installed;
        self.save();
    }
}

/// Determine cross-platform config file path
pub fn config_path() -> PathBuf {
    let base = if cfg!(target_os = "windows") {
        env::var("APPDATA").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."))
    } else {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("gignaati");
        path
    };
    base.join("config.json")
}
