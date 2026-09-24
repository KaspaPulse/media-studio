use anyhow::{Result, bail};

pub const WHATSAPP_HARD_LIMIT_BYTES: u64 = 10_000_000;
pub const WHATSAPP_TARGET_BYTES: u64 = 9_500_000;
pub const WHATSAPP_MAX_SEGMENT_SECONDS: f64 = 29.0;
pub const WHATSAPP_MAX_LONG_EDGE: u32 = 1_280;

const _: () = assert!(WHATSAPP_TARGET_BYTES < WHATSAPP_HARD_LIMIT_BYTES);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinExportProfile {
    UniversalMp4,
    WhatsApp,
    HighQuality,
    WebCompatible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputContainer {
    Mp4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodec {
    H264,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCodec {
    Aac,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityIntent {
    Balanced,
    SizeConstrained,
    HighQuality,
    Compatibility,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemuxPolicy {
    WhenCompatible,
    Never,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportProfile {
    pub name: String,
    pub container: OutputContainer,
    pub video_codec: VideoCodec,
    pub audio_codec: AudioCodec,
    pub quality_intent: QualityIntent,
    pub target_file_bytes: Option<u64>,
    pub hard_file_bytes: Option<u64>,
    pub max_segment_seconds: Option<f64>,
    pub max_long_edge: Option<u32>,
    pub pixel_format: Option<String>,
    pub faststart: bool,
    pub remux_policy: RemuxPolicy,
}

impl ExportProfile {
    #[must_use]
    pub fn builtin(profile: BuiltinExportProfile) -> Self {
        match profile {
            BuiltinExportProfile::UniversalMp4 => Self {
                name: "Universal MP4".to_owned(),
                container: OutputContainer::Mp4,
                video_codec: VideoCodec::H264,
                audio_codec: AudioCodec::Aac,
                quality_intent: QualityIntent::Balanced,
                target_file_bytes: None,
                hard_file_bytes: None,
                max_segment_seconds: None,
                max_long_edge: None,
                pixel_format: Some("yuv420p".to_owned()),
                faststart: true,
                remux_policy: RemuxPolicy::WhenCompatible,
            },
            BuiltinExportProfile::WhatsApp => Self {
                name: "WhatsApp".to_owned(),
                container: OutputContainer::Mp4,
                video_codec: VideoCodec::H264,
                audio_codec: AudioCodec::Aac,
                quality_intent: QualityIntent::SizeConstrained,
                target_file_bytes: Some(WHATSAPP_TARGET_BYTES),
                hard_file_bytes: Some(WHATSAPP_HARD_LIMIT_BYTES),
                max_segment_seconds: Some(WHATSAPP_MAX_SEGMENT_SECONDS),
                max_long_edge: Some(WHATSAPP_MAX_LONG_EDGE),
                pixel_format: Some("yuv420p".to_owned()),
                faststart: true,
                remux_policy: RemuxPolicy::WhenCompatible,
            },
            BuiltinExportProfile::HighQuality => Self {
                name: "High Quality".to_owned(),
                container: OutputContainer::Mp4,
                video_codec: VideoCodec::H264,
                audio_codec: AudioCodec::Aac,
                quality_intent: QualityIntent::HighQuality,
                target_file_bytes: None,
                hard_file_bytes: None,
                max_segment_seconds: None,
                max_long_edge: None,
                pixel_format: None,
                faststart: true,
                remux_policy: RemuxPolicy::WhenCompatible,
            },
            BuiltinExportProfile::WebCompatible => Self {
                name: "Web Compatible".to_owned(),
                container: OutputContainer::Mp4,
                video_codec: VideoCodec::H264,
                audio_codec: AudioCodec::Aac,
                quality_intent: QualityIntent::Compatibility,
                target_file_bytes: None,
                hard_file_bytes: None,
                max_segment_seconds: None,
                max_long_edge: None,
                pixel_format: Some("yuv420p".to_owned()),
                faststart: true,
                remux_policy: RemuxPolicy::WhenCompatible,
            },
        }
    }

    /// Builds the `WhatsApp` profile with a validated per-job duration and target size.
    ///
    /// # Errors
    /// Returns an error when the supplied duration or target conflicts with the `WhatsApp` hard limit.
    pub fn whatsapp(max_segment_seconds: f64, target_file_bytes: u64) -> Result<Self> {
        let mut profile = Self::builtin(BuiltinExportProfile::WhatsApp);
        profile.max_segment_seconds = Some(max_segment_seconds);
        profile.target_file_bytes = Some(target_file_bytes);
        profile.validate()?;
        Ok(profile)
    }

    /// Builds a caller-defined profile while preserving the same validation contract as built-ins.
    ///
    /// # Errors
    /// Returns an error when the profile name is empty, byte constraints are invalid, or optional
    /// duration/resolution/pixel-format values are unusable.
    pub fn custom(mut profile: Self) -> Result<Self> {
        profile.quality_intent = QualityIntent::Custom;
        profile.validate()?;
        Ok(profile)
    }

    /// Validates profile invariants without applying processing behavior.
    ///
    /// # Errors
    /// Returns an error when the profile contains contradictory or non-positive limits.
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            bail!("export profile name must not be empty");
        }

        if self.target_file_bytes == Some(0) {
            bail!("target file size must be greater than zero");
        }
        if self.hard_file_bytes == Some(0) {
            bail!("hard file size must be greater than zero");
        }
        if let (Some(target), Some(hard)) = (self.target_file_bytes, self.hard_file_bytes)
            && target >= hard
        {
            bail!("target file size must remain below the hard file size");
        }

        if let Some(seconds) = self.max_segment_seconds
            && (!seconds.is_finite() || seconds <= 0.0)
        {
            bail!("maximum segment duration must be a positive finite value");
        }

        if let Some(edge) = self.max_long_edge
            && edge < 2
        {
            bail!("maximum long edge must be at least 2 pixels");
        }

        if self
            .pixel_format
            .as_deref()
            .is_some_and(|format| format.trim().is_empty())
        {
            bail!("pixel format must not be empty");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whatsapp_profile_preserves_v2_size_and_duration_policy() {
        let profile = ExportProfile::builtin(BuiltinExportProfile::WhatsApp);

        assert_eq!(profile.target_file_bytes, Some(9_500_000));
        assert_eq!(profile.hard_file_bytes, Some(10_000_000));
        assert_eq!(profile.max_segment_seconds, Some(29.0));
        assert_eq!(profile.max_long_edge, Some(1_280));
        assert_eq!(profile.pixel_format.as_deref(), Some("yuv420p"));
        assert_eq!(profile.quality_intent, QualityIntent::SizeConstrained);
        profile.validate().unwrap();
    }

    #[test]
    fn whatsapp_job_profile_accepts_valid_runtime_overrides() {
        let profile = ExportProfile::whatsapp(45.0, 9_000_000).unwrap();
        assert_eq!(profile.max_segment_seconds, Some(45.0));
        assert_eq!(profile.target_file_bytes, Some(9_000_000));
        assert_eq!(profile.hard_file_bytes, Some(WHATSAPP_HARD_LIMIT_BYTES));

        assert!(ExportProfile::whatsapp(0.0, 9_000_000).is_err());
        assert!(ExportProfile::whatsapp(29.0, WHATSAPP_HARD_LIMIT_BYTES).is_err());
    }

    #[test]
    fn builtins_exist_without_inventing_size_limits_for_general_profiles() {
        for builtin in [
            BuiltinExportProfile::UniversalMp4,
            BuiltinExportProfile::HighQuality,
            BuiltinExportProfile::WebCompatible,
        ] {
            let profile = ExportProfile::builtin(builtin);
            assert_eq!(profile.target_file_bytes, None);
            assert_eq!(profile.hard_file_bytes, None);
            assert_eq!(profile.max_segment_seconds, None);
            profile.validate().unwrap();
        }
    }

    #[test]
    fn custom_profile_is_supported_and_validated() {
        let profile = ExportProfile::custom(ExportProfile {
            name: "My profile".to_owned(),
            container: OutputContainer::Mp4,
            video_codec: VideoCodec::H264,
            audio_codec: AudioCodec::Aac,
            quality_intent: QualityIntent::Balanced,
            target_file_bytes: Some(50_000_000),
            hard_file_bytes: Some(60_000_000),
            max_segment_seconds: Some(120.0),
            max_long_edge: Some(1_920),
            pixel_format: Some("yuv420p".to_owned()),
            faststart: true,
            remux_policy: RemuxPolicy::WhenCompatible,
        })
        .unwrap();

        assert_eq!(profile.quality_intent, QualityIntent::Custom);
    }

    #[test]
    fn invalid_custom_profile_fails_closed() {
        let invalid = ExportProfile {
            name: "Invalid".to_owned(),
            container: OutputContainer::Mp4,
            video_codec: VideoCodec::H264,
            audio_codec: AudioCodec::Aac,
            quality_intent: QualityIntent::Custom,
            target_file_bytes: Some(10_000_000),
            hard_file_bytes: Some(10_000_000),
            max_segment_seconds: Some(0.0),
            max_long_edge: Some(1),
            pixel_format: Some(String::new()),
            faststart: true,
            remux_policy: RemuxPolicy::Never,
        };

        assert!(invalid.validate().is_err());
    }
}
