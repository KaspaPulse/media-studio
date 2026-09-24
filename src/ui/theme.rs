use iced::theme::Mode;
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

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::System => Self::Light,
            Self::Light => Self::Dark,
            Self::Dark => Self::System,
        }
    }
}

#[must_use]
pub const fn resolved_system_theme(mode: Mode) -> ResolvedTheme {
    match mode {
        Mode::Dark => ResolvedTheme::Dark,
        Mode::None | Mode::Light => ResolvedTheme::Light,
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

    #[test]
    fn theme_preference_cycles_without_losing_system_mode() {
        assert_eq!(ThemePreference::System.next(), ThemePreference::Light);
        assert_eq!(ThemePreference::Light.next(), ThemePreference::Dark);
        assert_eq!(ThemePreference::Dark.next(), ThemePreference::System);
    }

    #[test]
    fn iced_system_modes_map_fail_safe_to_light_or_dark() {
        assert_eq!(resolved_system_theme(Mode::Dark), ResolvedTheme::Dark);
        assert_eq!(resolved_system_theme(Mode::Light), ResolvedTheme::Light);
        assert_eq!(resolved_system_theme(Mode::None), ResolvedTheme::Light);
    }
}
