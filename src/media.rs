use anyhow::{Context, Result, bail};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use url::Url;

pub const DEFAULT_SEGMENT_SECONDS: f64 = 29.0;
pub const WHATSAPP_LIMIT_BYTES: u64 = 10_000_000;
pub const DEFAULT_TARGET_BYTES: u64 = 9_500_000;
const _: () = assert!(DEFAULT_TARGET_BYTES < WHATSAPP_LIMIT_BYTES);
const AUDIO_KBPS: u32 = 128;
const MIN_VIDEO_KBPS: u32 = 180;
const MAX_VIDEO_KBPS: u32 = 8_000;
const CONTAINER_KBPS_RESERVE: u32 = 32;
const BUDGET_RATIO: f64 = 0.92;
const MAX_ENCODE_ATTEMPTS: usize = 4;

#[derive(Debug, Clone)]
pub struct Toolchain {
    pub ffmpeg: PathBuf,
    pub ffprobe: PathBuf,
    pub ytdlp: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SegmentationResult {
    pub clips: Vec<PathBuf>,
    pub effective_segment_seconds: f64,
    pub max_clip_bytes: u64,
}

/// Converts a positive duration value to seconds.
///
/// # Errors
/// Returns an error for unknown units, non-finite values, or values not greater than zero.
pub fn duration_to_seconds(value: f64, unit: &str) -> Result<f64> {
    let factor = match unit {
        "seconds" => 1.0,
        "minutes" => 60.0,
        "hours" => 3_600.0,
        other => bail!("unsupported duration unit: {other}"),
    };
    let seconds = value * factor;
    if !seconds.is_finite() || seconds <= 0.0 {
        bail!("segment duration must be greater than zero");
    }
    Ok(seconds)
}

#[must_use]
pub fn is_valid_url(value: &str) -> bool {
    Url::parse(value.trim())
        .is_ok_and(|parsed| matches!(parsed.scheme(), "http" | "https") && parsed.host().is_some())
}

impl Toolchain {
    /// Resolves and validates `FFmpeg`, `FFprobe`, and `yt-dlp`.
    ///
    /// # Errors
    /// Returns an error when any required helper cannot be executed successfully.
    pub fn discover() -> Result<Self> {
        let ffmpeg = resolve_tool(executable_name("ffmpeg"));
        let ffprobe = resolve_tool(executable_name("ffprobe"));
        let ytdlp = resolve_tool(executable_name("yt-dlp"));
        verify_tool(&ffmpeg, "-version")?;
        verify_tool(&ffprobe, "-version")?;
        verify_tool(&ytdlp, "--version")?;
        Ok(Self {
            ffmpeg,
            ffprobe,
            ytdlp,
        })
    }
}

fn executable_name(stem: &str) -> String {
    if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_owned()
    }
}

fn resolve_tool(name: String) -> PathBuf {
    if let Ok(exe) = std::env::current_exe()
        && let Some(parent) = exe.parent()
    {
        let mut candidates = vec![parent.join("resources").join(&name), parent.join(&name)];
        if let Some(contents) = parent.parent() {
            candidates.push(contents.join("Resources").join(&name));
        }
        for candidate in candidates {
            if candidate.is_file() {
                return candidate;
            }
        }
    }
    PathBuf::from(name)
}

fn verify_tool(path: &Path, version_arg: &str) -> Result<()> {
    let mut command = Command::new(path);
    command
        .arg(version_arg)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    hide_window(&mut command);
    let status = command
        .status()
        .with_context(|| format!("required tool not found: {}", path.display()))?;
    if !status.success() {
        bail!("required tool failed its version check: {}", path.display());
    }
    Ok(())
}

#[cfg(windows)]
fn hide_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_window(_command: &mut Command) {}

/// Creates an isolated job directory with original and clips subdirectories.
///
/// # Errors
/// Returns an error when the output directories cannot be created.
pub fn make_job_dir(root: &Path) -> Result<PathBuf> {
    fs::create_dir_all(root)
        .with_context(|| format!("failed to create output root {}", root.display()))?;
    let stamp = Utc::now().format("%Y%m%d_%H%M%S_%3f");
    let job = root.join(format!("WhatsApp_Video_{stamp}"));
    fs::create_dir(&job)
        .with_context(|| format!("failed to create job directory {}", job.display()))?;
    fs::create_dir(job.join("original"))?;
    fs::create_dir(job.join("clips"))?;
    Ok(job)
}

/// Reads media duration using `FFprobe`.
///
/// # Errors
/// Returns an error when `FFprobe` fails or reports a non-positive/invalid duration.
pub fn probe_duration(source: &Path, tools: &Toolchain) -> Result<f64> {
    let mut command = Command::new(&tools.ffprobe);
    command.args([
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
    ]);
    command.arg(source);
    hide_window(&mut command);
    let output = command
        .output()
        .with_context(|| format!("failed to run ffprobe for {}", source.display()))?;
    if !output.status.success() {
        bail!(
            "ffprobe failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let seconds: f64 = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .context("ffprobe returned an invalid duration")?;
    if !seconds.is_finite() || seconds <= 0.0 {
        bail!("media duration is not positive");
    }
    Ok(seconds)
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn effective_segment_seconds(requested: f64, target_bytes: u64) -> f64 {
    let minimum_total_kbps = f64::from(MIN_VIDEO_KBPS + AUDIO_KBPS + CONTAINER_KBPS_RESERVE);
    let max_by_size = (target_bytes as f64 * 8.0 * BUDGET_RATIO) / (minimum_total_kbps * 1_000.0);
    requested.min(max_by_size.max(1.0))
}

#[must_use]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn target_video_bitrate_kbps(duration_seconds: f64, target_bytes: u64) -> u32 {
    let total_kbps = (target_bytes as f64 * 8.0 * BUDGET_RATIO) / duration_seconds / 1_000.0;
    let video_kbps = total_kbps - f64::from(AUDIO_KBPS + CONTAINER_KBPS_RESERVE);
    video_kbps
        .floor()
        .clamp(f64::from(MIN_VIDEO_KBPS), f64::from(MAX_VIDEO_KBPS)) as u32
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn adjusted_bitrate_kbps(current: u32, actual_bytes: u64, target_bytes: u64) -> u32 {
    if actual_bytes == 0 {
        return current;
    }
    let ratio = target_bytes as f64 / actual_bytes as f64;
    (f64::from(current) * ratio * 0.90)
        .floor()
        .clamp(96.0, f64::from(MAX_VIDEO_KBPS)) as u32
}

fn encode_clip(
    source: &Path,
    output: &Path,
    start_seconds: f64,
    duration_seconds: f64,
    video_kbps: u32,
    tools: &Toolchain,
) -> Result<()> {
    let maxrate = video_kbps.saturating_mul(11) / 10;
    let bufsize = video_kbps.saturating_mul(2);
    let mut command = Command::new(&tools.ffmpeg);
    command.args(["-hide_banner", "-loglevel", "error", "-y"]);
    command.args(["-ss", &format!("{start_seconds:.3}")]);
    command.arg("-i").arg(source);
    command.args(["-t", &format!("{duration_seconds:.3}")]);
    command.args(["-map", "0:v:0", "-map", "0:a:0?"]);
    command.args([
        "-vf",
        "scale='if(gte(iw,ih),min(1280,iw),-2)':'if(gte(iw,ih),-2,min(1280,ih))'",
        "-c:v",
        "libx264",
        "-preset",
        "medium",
        "-pix_fmt",
        "yuv420p",
        "-profile:v",
        "high",
        "-level",
        "4.1",
    ]);
    command.args(["-b:v", &format!("{video_kbps}k")]);
    command.args(["-maxrate", &format!("{maxrate}k")]);
    command.args(["-bufsize", &format!("{bufsize}k")]);
    command.args(["-c:a", "aac", "-b:a", "128k", "-ar", "48000"]);
    command.args(["-movflags", "+faststart"]);
    command.arg(output);
    hide_window(&mut command);
    let output_result = command
        .output()
        .with_context(|| format!("failed to start FFmpeg for {}", output.display()))?;
    if !output_result.status.success() {
        bail!(
            "FFmpeg failed: {}",
            String::from_utf8_lossy(&output_result.stderr).trim()
        );
    }
    Ok(())
}

fn encode_size_safe_clip(
    source: &Path,
    output: &Path,
    start_seconds: f64,
    duration_seconds: f64,
    target_bytes: u64,
    tools: &Toolchain,
) -> Result<u64> {
    let mut bitrate = target_video_bitrate_kbps(duration_seconds, target_bytes);
    for attempt in 1..=MAX_ENCODE_ATTEMPTS {
        if output.exists() {
            fs::remove_file(output)?;
        }
        encode_clip(
            source,
            output,
            start_seconds,
            duration_seconds,
            bitrate,
            tools,
        )?;
        let size = fs::metadata(output)?.len();
        if size <= target_bytes {
            return Ok(size);
        }
        if attempt < MAX_ENCODE_ATTEMPTS {
            bitrate = adjusted_bitrate_kbps(bitrate, size, target_bytes);
        }
    }
    let size = fs::metadata(output).map_or(0, |meta| meta.len());
    bail!("unable to satisfy clip size budget: {size} bytes > {target_bytes} bytes")
}
/// Encodes verified H.264/AAC MP4 clips that do not exceed the configured byte target.
///
/// The requested duration is treated as a maximum. It is shortened automatically when
/// the minimum bitrate floor would otherwise make the byte budget impossible.
///
/// # Errors
/// Returns an error for invalid size targets, media probing/encoding failures, or when
/// a clip cannot satisfy the target after the bounded retry policy.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
pub fn create_segments<F>(
    source: &Path,
    job: &Path,
    requested_segment_seconds: f64,
    target_bytes: u64,
    tools: &Toolchain,
    mut progress: F,
) -> Result<SegmentationResult>
where
    F: FnMut(u8),
{
    if target_bytes == 0 || target_bytes > WHATSAPP_LIMIT_BYTES {
        bail!("target size must be between 1 byte and the WhatsApp hard limit");
    }
    let total_duration = probe_duration(source, tools)?;
    let segment_seconds = effective_segment_seconds(requested_segment_seconds, target_bytes);
    let count = (total_duration / segment_seconds).ceil().max(1.0) as usize;
    let clips_dir = job.join("clips");
    let mut clips = Vec::with_capacity(count);
    let mut max_clip_bytes = 0_u64;

    progress(0);
    for index in 0..count {
        let start = index as f64 * segment_seconds;
        let remaining = (total_duration - start).max(0.0);
        if remaining <= 0.001 {
            break;
        }
        let duration = remaining.min(segment_seconds);
        let output = clips_dir.join(format!("status_{index:03}.mp4"));
        let size = encode_size_safe_clip(source, &output, start, duration, target_bytes, tools)?;
        max_clip_bytes = max_clip_bytes.max(size);
        clips.push(output);
        let percent = u8::try_from((((index + 1) * 100) / count).min(100)).unwrap_or(100);
        progress(percent);
    }

    if clips.is_empty() {
        bail!("FFmpeg produced no clips");
    }
    for clip in &clips {
        let size = fs::metadata(clip)
            .with_context(|| format!("encoded clip is missing or unreadable: {}", clip.display()))?
            .len();
        if size > target_bytes {
            bail!(
                "post-encode size verification failed: {} is {size} bytes",
                clip.display()
            );
        }
    }

    Ok(SegmentationResult {
        clips,
        effective_segment_seconds: segment_seconds,
        max_clip_bytes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_validation_accepts_only_http_and_https_hosts() {
        assert!(is_valid_url("https://example.com/video"));
        assert!(is_valid_url("http://example.com/video"));
        assert!(!is_valid_url("example.com/video"));
        assert!(!is_valid_url("file:///tmp/video.mp4"));
    }

    #[test]
    fn duration_units_are_supported() {
        assert!((duration_to_seconds(29.0, "seconds").unwrap() - 29.0).abs() < f64::EPSILON);
        assert!((duration_to_seconds(2.0, "minutes").unwrap() - 120.0).abs() < f64::EPSILON);
        assert!((duration_to_seconds(1.0, "hours").unwrap() - 3_600.0).abs() < f64::EPSILON);
    }
    #[test]
    fn normal_29_second_request_is_not_shortened() {
        let effective = effective_segment_seconds(DEFAULT_SEGMENT_SECONDS, DEFAULT_TARGET_BYTES);
        assert!((effective - DEFAULT_SEGMENT_SECONDS).abs() < f64::EPSILON);
    }

    #[test]
    fn very_long_request_is_shortened_to_fit_size_floor() {
        let effective = effective_segment_seconds(3_600.0, DEFAULT_TARGET_BYTES);
        assert!(effective < 3_600.0);
        assert!(effective > DEFAULT_SEGMENT_SECONDS);
    }

    #[test]
    fn bitrate_budget_for_29_seconds_is_bounded() {
        let bitrate = target_video_bitrate_kbps(DEFAULT_SEGMENT_SECONDS, DEFAULT_TARGET_BYTES);
        assert!((MIN_VIDEO_KBPS..=MAX_VIDEO_KBPS).contains(&bitrate));
        assert!(bitrate > 1_000);
    }

    #[test]
    fn oversized_retry_reduces_bitrate() {
        let reduced = adjusted_bitrate_kbps(2_000, 11_000_000, DEFAULT_TARGET_BYTES);
        assert!(reduced < 2_000);
    }
}
