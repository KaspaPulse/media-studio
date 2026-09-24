use crate::export_profile::{
    AudioCodec, ExportProfile, OutputContainer, QualityIntent, RemuxPolicy, VideoCodec,
};
use crate::media::{SizeConstrainedSettings, Toolchain, create_size_constrained_segments};
use crate::media_probe::{MediaMetadata, MediaProbe};
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingMode {
    Remux,
    Transcode,
    SizeConstrainedTranscode,
}

#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub outputs: Vec<PathBuf>,
    pub mode: ProcessingMode,
    pub effective_segment_seconds: f64,
    pub max_output_bytes: u64,
}

pub struct ProcessingEngine;

impl ProcessingEngine {
    /// Processes an acquired media path according to an export profile.
    ///
    /// Acquisition provenance is intentionally absent from this API. The same source path is
    /// processed identically whether it came from a local file or remote acquisition.
    ///
    /// # Errors
    /// Returns an error when probing, profile validation, remux/transcode execution, or output
    /// verification fails.
    pub fn process<F>(
        source: &Path,
        job: &Path,
        profile: &ExportProfile,
        tools: &Toolchain,
        mut progress: F,
    ) -> Result<ProcessingResult>
    where
        F: FnMut(u8),
    {
        profile.validate()?;
        let metadata = MediaProbe::probe(source, &tools.ffprobe)?;
        let clips_dir = job.join("clips");
        fs::create_dir_all(&clips_dir).with_context(|| {
            format!("failed to create output directory {}", clips_dir.display())
        })?;

        if let Some(target_bytes) = profile.target_file_bytes {
            let requested_segment_seconds = profile
                .max_segment_seconds
                .unwrap_or(metadata.duration_seconds);
            let result = create_size_constrained_segments(
                source,
                job,
                metadata.duration_seconds,
                requested_segment_seconds,
                SizeConstrainedSettings {
                    target_bytes,
                    hard_limit_bytes: profile.hard_file_bytes,
                    max_long_edge: profile.max_long_edge,
                    pixel_format: profile.pixel_format.as_deref(),
                },
                tools,
                progress,
            )?;
            return Ok(ProcessingResult {
                outputs: result.clips,
                mode: ProcessingMode::SizeConstrainedTranscode,
                effective_segment_seconds: result.effective_segment_seconds,
                max_output_bytes: result.max_clip_bytes,
            });
        }

        let source_size = fs::metadata(source)
            .with_context(|| format!("failed to inspect media source {}", source.display()))?
            .len();

        if can_remux(&metadata, profile, source_size) {
            let output = clips_dir.join("status_000.mp4");
            progress(0);
            remux_clip(source, &output, profile, tools)?;
            let size = verify_output(&output, profile.hard_file_bytes)?;
            progress(100);
            return Ok(ProcessingResult {
                outputs: vec![output],
                mode: ProcessingMode::Remux,
                effective_segment_seconds: metadata.duration_seconds,
                max_output_bytes: size,
            });
        }

        let segments = segment_plan(metadata.duration_seconds, profile.max_segment_seconds)?;
        let mut outputs = Vec::with_capacity(segments.len());
        let mut max_output_bytes = 0_u64;
        progress(0);

        for (index, (start_seconds, duration_seconds)) in segments.iter().copied().enumerate() {
            let output = clips_dir.join(format!("status_{index:03}.mp4"));
            transcode_quality_clip(
                source,
                &output,
                start_seconds,
                duration_seconds,
                profile,
                tools,
            )?;
            let size = verify_output(&output, profile.hard_file_bytes)?;
            max_output_bytes = max_output_bytes.max(size);
            outputs.push(output);
            let percent =
                u8::try_from((((index + 1) * 100) / segments.len()).min(100)).unwrap_or(100);
            progress(percent);
        }

        if outputs.is_empty() {
            bail!("processing produced no output files");
        }

        Ok(ProcessingResult {
            outputs,
            mode: ProcessingMode::Transcode,
            effective_segment_seconds: profile
                .max_segment_seconds
                .unwrap_or(metadata.duration_seconds)
                .min(metadata.duration_seconds),
            max_output_bytes,
        })
    }
}

fn can_remux(metadata: &MediaMetadata, profile: &ExportProfile, source_size: u64) -> bool {
    if profile.remux_policy == RemuxPolicy::Never || profile.target_file_bytes.is_some() {
        return false;
    }

    if let Some(hard_limit) = profile.hard_file_bytes
        && source_size > hard_limit
    {
        return false;
    }

    if let Some(max_seconds) = profile.max_segment_seconds
        && metadata.duration_seconds > max_seconds
    {
        return false;
    }

    let Some(video) = metadata.video_streams.first() else {
        return false;
    };
    let video_codec_matches = match profile.video_codec {
        VideoCodec::H264 => video
            .codec
            .as_deref()
            .is_some_and(|codec| codec.eq_ignore_ascii_case("h264")),
    };
    if !video_codec_matches {
        return false;
    }

    let audio_codec_matches =
        metadata
            .audio_streams
            .first()
            .is_none_or(|audio| match profile.audio_codec {
                AudioCodec::Aac => audio
                    .codec
                    .as_deref()
                    .is_some_and(|codec| codec.eq_ignore_ascii_case("aac")),
            });
    if !audio_codec_matches {
        return false;
    }

    if let Some(max_long_edge) = profile.max_long_edge {
        let (Some(width), Some(height)) = (video.width, video.height) else {
            return false;
        };
        if width.max(height) > max_long_edge {
            return false;
        }
    }

    if let Some(required_pixel_format) = profile.pixel_format.as_deref()
        && video.pixel_format.as_deref() != Some(required_pixel_format)
    {
        return false;
    }

    true
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn segment_plan(total_duration: f64, max_segment_seconds: Option<f64>) -> Result<Vec<(f64, f64)>> {
    if !total_duration.is_finite() || total_duration <= 0.0 {
        bail!("media duration must be a positive finite value");
    }
    let segment_seconds = max_segment_seconds.unwrap_or(total_duration);
    if !segment_seconds.is_finite() || segment_seconds <= 0.0 {
        bail!("segment duration must be a positive finite value");
    }

    let count = (total_duration / segment_seconds).ceil().max(1.0) as usize;
    let mut segments = Vec::with_capacity(count);
    for index in 0..count {
        let start = index as f64 * segment_seconds;
        let remaining = (total_duration - start).max(0.0);
        if remaining <= 0.001 {
            break;
        }
        segments.push((start, remaining.min(segment_seconds)));
    }
    Ok(segments)
}

fn remux_clip(
    source: &Path,
    output: &Path,
    profile: &ExportProfile,
    tools: &Toolchain,
) -> Result<()> {
    let mut command = build_remux_command(source, output, profile, tools);
    hide_window(&mut command);
    run_ffmpeg(command, output, "remux")
}

fn build_remux_command(
    source: &Path,
    output: &Path,
    profile: &ExportProfile,
    tools: &Toolchain,
) -> Command {
    let mut command = Command::new(&tools.ffmpeg);
    command.args(["-hide_banner", "-loglevel", "error", "-y"]);
    command.arg("-i").arg(source);
    command.args(["-map", "0:v:0", "-map", "0:a:0?", "-c", "copy"]);
    if profile.faststart {
        command.args(["-movflags", "+faststart"]);
    }
    command.arg(output);
    command
}

fn transcode_quality_clip(
    source: &Path,
    output: &Path,
    start_seconds: f64,
    duration_seconds: f64,
    profile: &ExportProfile,
    tools: &Toolchain,
) -> Result<()> {
    let mut command = build_quality_transcode_command(
        source,
        output,
        start_seconds,
        duration_seconds,
        profile,
        tools,
    );
    hide_window(&mut command);
    run_ffmpeg(command, output, "transcode")
}

fn build_quality_transcode_command(
    source: &Path,
    output: &Path,
    start_seconds: f64,
    duration_seconds: f64,
    profile: &ExportProfile,
    tools: &Toolchain,
) -> Command {
    let (preset, crf, audio_kbps) = match profile.quality_intent {
        QualityIntent::HighQuality => ("slow", "18", "192k"),
        QualityIntent::Compatibility | QualityIntent::SizeConstrained => ("medium", "22", "128k"),
        QualityIntent::Balanced | QualityIntent::Custom => ("medium", "20", "160k"),
    };

    let mut command = Command::new(&tools.ffmpeg);
    command.args(["-hide_banner", "-loglevel", "error", "-y"]);
    command.args(["-ss", &format!("{start_seconds:.3}")]);
    command.arg("-i").arg(source);
    command.args(["-t", &format!("{duration_seconds:.3}")]);
    command.args(["-map", "0:v:0", "-map", "0:a:0?"]);

    if let Some(max_long_edge) = profile.max_long_edge {
        command.arg("-vf").arg(scale_filter(max_long_edge));
    }

    match profile.video_codec {
        VideoCodec::H264 => {
            command.args(["-c:v", "libx264", "-preset", preset, "-crf", crf]);
        }
    }
    if let Some(pixel_format) = profile.pixel_format.as_deref() {
        command.args(["-pix_fmt", pixel_format]);
    }

    match profile.audio_codec {
        AudioCodec::Aac => {
            command.args(["-c:a", "aac", "-b:a", audio_kbps, "-ar", "48000"]);
        }
    }

    match profile.container {
        OutputContainer::Mp4 => {
            if profile.faststart {
                command.args(["-movflags", "+faststart"]);
            }
        }
    }

    command.arg(output);
    command
}

fn run_ffmpeg(mut command: Command, output: &Path, operation: &str) -> Result<()> {
    let result = command.output().with_context(|| {
        format!(
            "failed to start FFmpeg {operation} for {}",
            output.display()
        )
    })?;
    if !result.status.success() {
        bail!(
            "FFmpeg {operation} failed: {}",
            String::from_utf8_lossy(&result.stderr).trim()
        );
    }
    Ok(())
}

fn verify_output(output: &Path, hard_limit_bytes: Option<u64>) -> Result<u64> {
    let size = fs::metadata(output)
        .with_context(|| {
            format!(
                "processed output is missing or unreadable: {}",
                output.display()
            )
        })?
        .len();
    if size == 0 {
        bail!("processed output is empty: {}", output.display());
    }
    if let Some(hard_limit_bytes) = hard_limit_bytes
        && size > hard_limit_bytes
    {
        bail!(
            "processed output exceeds hard limit: {} is {size} bytes > {hard_limit_bytes}",
            output.display()
        );
    }
    Ok(size)
}

fn scale_filter(max_long_edge: u32) -> String {
    format!(
        "scale='if(gte(iw,ih),min({max_long_edge},iw),-2)':'if(gte(iw,ih),-2,min({max_long_edge},ih))'"
    )
}

#[cfg(windows)]
fn hide_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
const fn hide_window(_command: &mut Command) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export_profile::{BuiltinExportProfile, ExportProfile};
    use crate::media_probe::{AudioStreamMetadata, ColorMetadata, VideoStreamMetadata};

    fn metadata() -> MediaMetadata {
        MediaMetadata {
            container: Some("matroska,webm".to_owned()),
            duration_seconds: 20.0,
            video_streams: vec![VideoStreamMetadata {
                index: 0,
                codec: Some("h264".to_owned()),
                width: Some(1280),
                height: Some(720),
                frame_rate: Some(30.0),
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
        }
    }

    #[test]
    fn compatible_general_profile_can_remux() {
        let profile = ExportProfile::builtin(BuiltinExportProfile::UniversalMp4);
        assert!(can_remux(&metadata(), &profile, 20_000_000));
    }

    #[test]
    fn pixel_or_resolution_constraint_forces_transcode() {
        let mut profile = ExportProfile::builtin(BuiltinExportProfile::UniversalMp4);
        profile.max_long_edge = Some(640);
        assert!(!can_remux(&metadata(), &profile, 20_000_000));

        profile.max_long_edge = None;
        profile.pixel_format = Some("yuv444p".to_owned());
        assert!(!can_remux(&metadata(), &profile, 20_000_000));
    }

    #[test]
    fn size_constrained_profile_never_uses_simple_remux_path() {
        let profile = ExportProfile::builtin(BuiltinExportProfile::WhatsApp);
        assert!(!can_remux(&metadata(), &profile, 1_000_000));
    }

    #[test]
    fn segment_plan_is_bounded_and_complete() {
        let plan = segment_plan(60.0, Some(29.0)).unwrap();
        assert_eq!(plan.len(), 3);
        assert_eq!(plan[0], (0.0, 29.0));
        assert_eq!(plan[1], (29.0, 29.0));
        assert_eq!(plan[2], (58.0, 2.0));
    }

    #[test]
    fn scale_filter_only_uses_minimum_source_or_limit_dimensions() {
        let filter = scale_filter(1280);
        assert!(filter.contains("min(1280,iw)"));
        assert!(filter.contains("min(1280,ih)"));
    }

    #[test]
    fn remux_command_copies_streams_and_enables_faststart() {
        let profile = ExportProfile::builtin(BuiltinExportProfile::UniversalMp4);
        let tools = Toolchain {
            ffmpeg: PathBuf::from("ffmpeg"),
            ffprobe: PathBuf::from("ffprobe"),
            ytdlp: PathBuf::from("yt-dlp"),
        };
        let command = build_remux_command(
            Path::new("source.mkv"),
            Path::new("output.mp4"),
            &profile,
            &tools,
        );
        let args: Vec<_> = command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect();
        assert!(args.windows(2).any(|pair| pair == ["-c", "copy"]));
        assert!(
            args.windows(2)
                .any(|pair| pair == ["-movflags", "+faststart"])
        );
    }
}
