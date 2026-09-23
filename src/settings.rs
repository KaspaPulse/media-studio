use crate::i18n::Language;
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: Language,
    pub output_dir: PathBuf,
}

impl Default for AppSettings {
    fn default() -> Self {
        let output_dir = UserDirs::new()
            .and_then(|dirs| dirs.download_dir().map(Path::to_path_buf))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        Self {
            language: Language::Arabic,
            output_dir,
        }
    }
}

impl AppSettings {
    fn path() -> Option<PathBuf> {
        ProjectDirs::from("com", "KaspaPulse", "WhatsAppVideoPreparer")
            .map(|dirs| dirs.config_dir().join("settings.json"))
    }

    #[must_use]
    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };
        let Ok(data) = fs::read(path) else {
            return Self::default();
        };
        serde_json::from_slice(&data).unwrap_or_default()
    }

    pub fn save(&self) {
        let Some(path) = Self::path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_vec_pretty(self) {
            let _ = fs::write(path, data);
        }
    }
}
