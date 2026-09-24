use crate::i18n::Language;
use crate::ui::theme::ThemePreference;
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const CURRENT_SETTINGS_APP_ID: &str = "KaspaPulseMediaStudio";
const LEGACY_SETTINGS_APP_ID: &str = "WhatsAppVideoPreparer";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub language: Language,
    pub output_dir: PathBuf,
    pub theme: ThemePreference,
}

impl Default for AppSettings {
    fn default() -> Self {
        let output_dir = UserDirs::new()
            .and_then(|dirs| dirs.download_dir().map(Path::to_path_buf))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        Self {
            language: Language::Arabic,
            output_dir,
            theme: ThemePreference::System,
        }
    }
}

impl AppSettings {
    fn settings_path(app_id: &str) -> Option<PathBuf> {
        ProjectDirs::from("com", "KaspaPulse", app_id)
            .map(|dirs| dirs.config_dir().join("settings.json"))
    }

    fn path() -> Option<PathBuf> {
        Self::settings_path(CURRENT_SETTINGS_APP_ID)
    }

    fn legacy_path() -> Option<PathBuf> {
        Self::settings_path(LEGACY_SETTINGS_APP_ID)
    }

    fn load_from(path: Option<PathBuf>) -> Option<Self> {
        let data = fs::read(path?).ok()?;
        serde_json::from_slice(&data).ok()
    }

    #[must_use]
    pub fn load() -> Self {
        Self::load_from(Self::path())
            .or_else(|| Self::load_from(Self::legacy_path()))
            .unwrap_or_default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_settings_without_theme_preserve_existing_values() {
        let legacy = br#"{"language":"english","output_dir":"C:\\Media"}"#;
        let settings: AppSettings = serde_json::from_slice(legacy).unwrap();

        assert_eq!(settings.language, Language::English);
        assert_eq!(settings.output_dir, PathBuf::from(r"C:\Media"));
        assert_eq!(settings.theme, ThemePreference::System);
    }

    #[test]
    fn v3_settings_identity_preserves_v2_fallback_identity() {
        assert_eq!(CURRENT_SETTINGS_APP_ID, "KaspaPulseMediaStudio");
        assert_eq!(LEGACY_SETTINGS_APP_ID, "WhatsAppVideoPreparer");
        assert_ne!(CURRENT_SETTINGS_APP_ID, LEGACY_SETTINGS_APP_ID);
    }
}
