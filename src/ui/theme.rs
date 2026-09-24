use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedTheme {
    Light,
    Dark,
}

impl ThemePreference {
    #[must_use]
    pub const fn resolve(self, system_theme: ResolvedTheme) -> ResolvedTheme {
        match self {
            Self::System => system_theme,
            Self::Light => ResolvedTheme::Light,
            Self::Dark => ResolvedTheme::Dark,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_is_default_and_tracks_the_supplied_system_theme() {
        assert_eq!(ThemePreference::default(), ThemePreference::System);
        assert_eq!(
            ThemePreference::System.resolve(ResolvedTheme::Dark),
            ResolvedTheme::Dark
        );
        assert_eq!(
            ThemePreference::System.resolve(ResolvedTheme::Light),
            ResolvedTheme::Light
        );
    }

    #[test]
    fn explicit_preferences_override_system_theme() {
        assert_eq!(
            ThemePreference::Light.resolve(ResolvedTheme::Dark),
            ResolvedTheme::Light
        );
        assert_eq!(
            ThemePreference::Dark.resolve(ResolvedTheme::Light),
            ResolvedTheme::Dark
        );
    }
}
