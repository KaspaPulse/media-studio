use crate::export_profile::BuiltinExportProfile;
use crate::i18n::Strings;
use crate::media_probe::MediaMetadata;
use crate::ui::bidi::isolate_ltr;
use std::time::Duration;

pub const BUILTIN_PROFILES: [BuiltinExportProfile; 4] = [
    BuiltinExportProfile::UniversalMp4,
    BuiltinExportProfile::WhatsApp,
    BuiltinExportProfile::HighQuality,
    BuiltinExportProfile::WebCompatible,
];

#[must_use]
pub const fn profile_label(strings: &Strings, profile: BuiltinExportProfile) -> &'static str {
    match profile {
        BuiltinExportProfile::UniversalMp4 => strings.profile_universal_mp4,
        BuiltinExportProfile::WhatsApp => strings.profile_whatsapp,
        BuiltinExportProfile::HighQuality => strings.profile_high_quality,
        BuiltinExportProfile::WebCompatible => strings.profile_web_compatible,
    }
}

#[must_use]
pub fn media_summary(metadata: &MediaMetadata) -> String {
    let mut parts = Vec::new();

    if let Some(container) = metadata.container.as_deref() {
        parts.push(isolate_ltr(container));
    }

    if let Some(video) = metadata.video_streams.first() {
        if let Some(codec) = video.codec.as_deref() {
            parts.push(isolate_ltr(codec));
        }
        if let (Some(width), Some(height)) = (video.width, video.height) {
            parts.push(isolate_ltr(format!("{width}×{height}")));
        }
        if let Some(frame_rate) = video.frame_rate {
            parts.push(isolate_ltr(format!("{frame_rate:.2} fps")));
        }
    }

    if let Some(audio) = metadata.audio_streams.first()
        && let Some(codec) = audio.codec.as_deref()
    {
        parts.push(isolate_ltr(codec));
    }

    parts.push(isolate_ltr(format_duration(metadata.duration_seconds)));
    parts.join(" · ")
}

fn format_duration(seconds: f64) -> String {
    let total_seconds = Duration::try_from_secs_f64(seconds.max(0.0)).map_or(0, |duration| {
        duration
            .as_secs()
            .saturating_add(u64::from(duration.subsec_nanos() >= 500_000_000))
    });
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{AR, EN};
    use crate::media_probe::{AudioStreamMetadata, ColorMetadata, VideoStreamMetadata};

    #[test]
    fn profile_labels_are_localized_without_domain_display_strings() {
        assert_eq!(
            profile_label(&EN, BuiltinExportProfile::UniversalMp4),
            "Universal MP4"
        );
        assert_eq!(
            profile_label(&AR, BuiltinExportProfile::HighQuality),
            "جودة عالية"
        );
    }

    #[test]
    fn media_summary_keeps_technical_values_isolated() {
        let metadata = MediaMetadata {
            container: Some("matroska,webm".to_owned()),
            duration_seconds: 125.0,
            video_streams: vec![VideoStreamMetadata {
                index: 0,
                codec: Some("h264".to_owned()),
                width: Some(1920),
                height: Some(1080),
                frame_rate: Some(29.97),
                pixel_format: Some("yuv420p".to_owned()),
                rotation_degrees: None,
                color: ColorMetadata::default(),
                side_data_types: Vec::new(),
            }],
            audio_streams: vec![AudioStreamMetadata {
                index: 1,
                codec: Some("aac".to_owned()),
                sample_rate_hz: Some(48_000),
                channels: Some(2),
            }],
        };

        let summary = media_summary(&metadata);
        assert!(summary.contains("matroska,webm"));
        assert!(summary.contains("1920×1080"));
        assert!(summary.contains("29.97 fps"));
        assert!(summary.contains("2:05"));
        assert!(summary.contains('\u{2066}'));
        assert!(summary.contains('\u{2069}'));
    }
}
