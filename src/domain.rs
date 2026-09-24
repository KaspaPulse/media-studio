use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductIdentity {
    pub publisher: &'static str,
    pub product: &'static str,
    pub version: &'static str,
    pub repository: &'static str,
}

pub const LEGACY_V2_PRODUCT_IDENTITY: ProductIdentity = ProductIdentity {
    publisher: "KaspaPulse",
    product: "WhatsApp Video Preparer",
    version: "2.0.0",
    repository: "KaspaPulse/whatsapp-video-preparer",
};

pub const CURRENT_PRODUCT_IDENTITY: ProductIdentity = ProductIdentity {
    publisher: "KaspaPulse",
    product: "KaspaPulse Media Studio",
    version: "3.0.0",
    repository: "KaspaPulse/media-studio",
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSourceKind {
    LocalFile,
    RemoteUrl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputSource {
    LocalFile(PathBuf),
    RemoteUrl(Url),
}

impl InputSource {
    #[must_use]
    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::LocalFile(path.into())
    }

    #[must_use]
    pub const fn remote(url: Url) -> Self {
        Self::RemoteUrl(url)
    }

    #[must_use]
    pub const fn kind(&self) -> InputSourceKind {
        match self {
            Self::LocalFile(_) => InputSourceKind::LocalFile,
            Self::RemoteUrl(_) => InputSourceKind::RemoteUrl,
        }
    }

    #[must_use]
    pub fn local_path(&self) -> Option<&Path> {
        match self {
            Self::LocalFile(path) => Some(path.as_path()),
            Self::RemoteUrl(_) => None,
        }
    }

    #[must_use]
    pub const fn remote_url(&self) -> Option<&Url> {
        match self {
            Self::LocalFile(_) => None,
            Self::RemoteUrl(url) => Some(url),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AppViewState {
    #[default]
    Empty,
    AcquiringSource,
    Probing,
    SourceReady,
    Processing,
    Finalizing,
    Completed,
    Cancelling,
    Cancelled,
    Failed,
}

impl AppViewState {
    #[must_use]
    pub const fn is_cancellable(self) -> bool {
        matches!(
            self,
            Self::AcquiringSource | Self::Probing | Self::Processing | Self::Finalizing
        )
    }

    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::Failed)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MirrorPolicy {
    #[default]
    FollowLocale,
    Never,
    Always,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FocusTraversal {
    #[default]
    SemanticLogicalOrder,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FocusPolicy {
    traversal: FocusTraversal,
}

impl FocusPolicy {
    #[must_use]
    pub const fn semantic_logical_order() -> Self {
        Self {
            traversal: FocusTraversal::SemanticLogicalOrder,
        }
    }

    #[must_use]
    pub const fn traversal(self) -> FocusTraversal {
        self.traversal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_identity_keeps_v2_history_distinct_from_current_v3_identity() {
        assert_eq!(LEGACY_V2_PRODUCT_IDENTITY.version, "2.0.0");
        assert_eq!(CURRENT_PRODUCT_IDENTITY.version, "3.0.0");
        assert_eq!(CURRENT_PRODUCT_IDENTITY.product, "KaspaPulse Media Studio");
        assert_ne!(
            LEGACY_V2_PRODUCT_IDENTITY.repository,
            CURRENT_PRODUCT_IDENTITY.repository
        );
    }

    #[test]
    fn input_source_preserves_local_and_remote_identity() {
        let local = InputSource::local("video.mkv");
        assert_eq!(local.kind(), InputSourceKind::LocalFile);
        assert_eq!(local.local_path(), Some(Path::new("video.mkv")));
        assert!(local.remote_url().is_none());

        let url = Url::parse("https://example.com/video").unwrap();
        let remote = InputSource::remote(url.clone());
        assert_eq!(remote.kind(), InputSourceKind::RemoteUrl);
        assert_eq!(remote.remote_url(), Some(&url));
        assert!(remote.local_path().is_none());
    }

    #[test]
    fn app_view_state_models_cancellable_and_terminal_states() {
        for state in [
            AppViewState::AcquiringSource,
            AppViewState::Probing,
            AppViewState::Processing,
            AppViewState::Finalizing,
        ] {
            assert!(state.is_cancellable());
        }

        for state in [
            AppViewState::Completed,
            AppViewState::Cancelled,
            AppViewState::Failed,
        ] {
            assert!(state.is_terminal());
        }

        assert!(!AppViewState::Empty.is_cancellable());
        assert!(!AppViewState::SourceReady.is_terminal());
    }

    #[test]
    fn focus_policy_defaults_to_semantic_logical_order() {
        assert_eq!(
            FocusPolicy::default().traversal(),
            FocusTraversal::SemanticLogicalOrder
        );
        assert_eq!(
            FocusPolicy::semantic_logical_order().traversal(),
            FocusTraversal::SemanticLogicalOrder
        );
    }

    #[test]
    fn mirror_policy_default_follows_locale() {
        assert_eq!(MirrorPolicy::default(), MirrorPolicy::FollowLocale);
    }
}
