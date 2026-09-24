use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct MediaMetadata {
    pub container: Option<String>,
    pub duration_seconds: f64,
    pub video_streams: Vec<VideoStreamMetadata>,
    pub audio_streams: Vec<AudioStreamMetadata>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoStreamMetadata {
    pub index: u32,
    pub codec: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<f64>,
    pub pixel_format: Option<String>,
    pub rotation_degrees: Option<f64>,
    pub color: ColorMetadata,
    pub side_data_types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioStreamMetadata {
    pub index: u32,
    pub codec: Option<String>,
    pub sample_rate_hz: Option<u32>,
    pub channels: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ColorMetadata {
    pub range: Option<String>,
    pub space: Option<String>,
    pub transfer: Option<String>,
    pub primaries: Option<String>,
}

pub struct MediaProbe;

impl MediaProbe {
    /// Probes media through `FFprobe` and returns structured metadata.
    ///
    /// Capability is determined by the actual bundled `FFprobe` build, not by filename extension.
    ///
    /// # Errors
    /// Returns an error when the source is not a regular file, `FFprobe` fails, its JSON is invalid,
    /// no audio/video streams are present, or no positive finite duration can be established.
    pub fn probe(source: &Path, ffprobe: &Path) -> Result<MediaMetadata> {
        if !source.is_file() {
            bail!("media source is not a regular file: {}", source.display());
        }

        let mut command = build_ffprobe_command(ffprobe, source);
        hide_window(&mut command);
        let output = command
            .output()
            .with_context(|| format!("failed to run FFprobe for {}", source.display()))?;

        if !output.status.success() {
            bail!(
                "FFprobe failed for {}: {}",
                source.display(),
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }

        parse_probe_json(&output.stdout)
    }
}

fn build_ffprobe_command(ffprobe: &Path, source: &Path) -> Command {
    let mut command = Command::new(ffprobe);
    command.args([
        "-v",
        "error",
        "-show_streams",
        "-show_format",
        "-of",
        "json",
    ]);
    command.arg(source);
    command
}

fn parse_probe_json(bytes: &[u8]) -> Result<MediaMetadata> {
    let raw: RawProbe =
        serde_json::from_slice(bytes).context("FFprobe returned invalid JSON metadata")?;

    let duration_seconds = raw
        .format
        .as_ref()
        .and_then(|format| parse_positive_f64(format.duration.as_deref()))
        .or_else(|| {
            raw.streams
                .iter()
                .filter_map(|stream| parse_positive_f64(stream.duration.as_deref()))
                .reduce(f64::max)
        })
        .context("FFprobe metadata does not contain a positive finite duration")?;

    let mut video_streams = Vec::new();
    let mut audio_streams = Vec::new();

    for stream in &raw.streams {
        match stream.codec_type.as_deref() {
            Some("video") => video_streams.push(VideoStreamMetadata {
                index: stream.index,
                codec: stream.codec_name.clone(),
                width: stream.width,
                height: stream.height,
                frame_rate: parse_frame_rate(stream.avg_frame_rate.as_deref())
                    .or_else(|| parse_frame_rate(stream.r_frame_rate.as_deref())),
                pixel_format: stream.pix_fmt.clone(),
                rotation_degrees: stream
                    .side_data_list
                    .iter()
                    .find_map(|side_data| side_data.rotation)
                    .or_else(|| parse_finite_f64(stream.tags.rotate.as_deref())),
                color: ColorMetadata {
                    range: stream.color_range.clone(),
                    space: stream.color_space.clone(),
                    transfer: stream.color_transfer.clone(),
                    primaries: stream.color_primaries.clone(),
                },
                side_data_types: stream
                    .side_data_list
                    .iter()
                    .filter_map(|side_data| side_data.side_data_type.clone())
                    .collect(),
            }),
            Some("audio") => audio_streams.push(AudioStreamMetadata {
                index: stream.index,
                codec: stream.codec_name.clone(),
                sample_rate_hz: stream
                    .sample_rate
                    .as_deref()
                    .and_then(|rate| rate.parse::<u32>().ok()),
                channels: stream.channels,
            }),
            _ => {}
        }
    }

    if video_streams.is_empty() && audio_streams.is_empty() {
        bail!("FFprobe metadata contains no audio or video streams");
    }

    let container = raw.format.and_then(|format| nonempty(format.format_name));

    Ok(MediaMetadata {
        container,
        duration_seconds,
        video_streams,
        audio_streams,
    })
}

fn parse_positive_f64(value: Option<&str>) -> Option<f64> {
    parse_finite_f64(value).filter(|number| *number > 0.0)
}

fn parse_finite_f64(value: Option<&str>) -> Option<f64> {
    value
        .and_then(|text| text.trim().parse::<f64>().ok())
        .filter(|number| number.is_finite())
}

fn parse_frame_rate(value: Option<&str>) -> Option<f64> {
    let value = value?.trim();
    if let Some((numerator, denominator)) = value.split_once('/') {
        let numerator = numerator.parse::<f64>().ok()?;
        let denominator = denominator.parse::<f64>().ok()?;
        if !numerator.is_finite()
            || !denominator.is_finite()
            || numerator <= 0.0
            || denominator <= 0.0
        {
            return None;
        }
        return Some(numerator / denominator);
    }

    parse_positive_f64(Some(value))
}

fn nonempty(value: Option<String>) -> Option<String> {
    value.filter(|text| !text.trim().is_empty())
}

#[derive(Debug, Deserialize)]
struct RawProbe {
    #[serde(default)]
    streams: Vec<RawStream>,
    format: Option<RawFormat>,
}

#[derive(Debug, Deserialize)]
struct RawFormat {
    format_name: Option<String>,
    duration: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawStream {
    index: u32,
    codec_type: Option<String>,
    codec_name: Option<String>,
    duration: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    avg_frame_rate: Option<String>,
    r_frame_rate: Option<String>,
    pix_fmt: Option<String>,
    sample_rate: Option<String>,
    channels: Option<u32>,
    color_range: Option<String>,
    color_space: Option<String>,
    color_transfer: Option<String>,
    color_primaries: Option<String>,
    #[serde(default)]
    tags: RawTags,
    #[serde(default)]
    side_data_list: Vec<RawSideData>,
}

#[derive(Debug, Default, Deserialize)]
struct RawTags {
    rotate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawSideData {
    side_data_type: Option<String>,
    rotation: Option<f64>,
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
    use std::ffi::OsStr;

    const FIXTURE: &str = r#"{
        "streams": [
            {
                "index": 0,
                "codec_type": "video",
                "codec_name": "hevc",
                "width": 3840,
                "height": 2160,
                "avg_frame_rate": "30000/1001",
                "r_frame_rate": "30000/1001",
                "pix_fmt": "yuv420p10le",
                "color_range": "tv",
                "color_space": "bt2020nc",
                "color_transfer": "smpte2084",
                "color_primaries": "bt2020",
                "tags": {"rotate": "90"},
                "side_data_list": [
                    {"side_data_type": "Mastering display metadata"},
                    {"side_data_type": "Display Matrix", "rotation": -90}
                ]
            },
            {
                "index": 1,
                "codec_type": "audio",
                "codec_name": "aac",
                "sample_rate": "48000",
                "channels": 2
            }
        ],
        "format": {
            "format_name": "mov,mp4,m4a,3gp,3g2,mj2",
            "duration": "12.500"
        }
    }"#;

    #[test]
    fn parses_structured_video_audio_color_and_orientation_metadata() {
        let metadata = parse_probe_json(FIXTURE.as_bytes()).unwrap();

        assert_eq!(
            metadata.container.as_deref(),
            Some("mov,mp4,m4a,3gp,3g2,mj2")
        );
        assert!((metadata.duration_seconds - 12.5).abs() < f64::EPSILON);
        assert_eq!(metadata.video_streams.len(), 1);
        assert_eq!(metadata.audio_streams.len(), 1);

        let video = &metadata.video_streams[0];
        assert_eq!(video.codec.as_deref(), Some("hevc"));
        assert_eq!((video.width, video.height), (Some(3840), Some(2160)));
        assert!((video.frame_rate.unwrap() - (30_000.0 / 1_001.0)).abs() < 0.000_001);
        assert_eq!(video.pixel_format.as_deref(), Some("yuv420p10le"));
        assert_eq!(video.rotation_degrees, Some(-90.0));
        assert_eq!(video.color.transfer.as_deref(), Some("smpte2084"));
        assert_eq!(video.color.primaries.as_deref(), Some("bt2020"));
        assert!(
            video
                .side_data_types
                .iter()
                .any(|kind| kind == "Mastering display metadata")
        );

        let audio = &metadata.audio_streams[0];
        assert_eq!(audio.codec.as_deref(), Some("aac"));
        assert_eq!(audio.sample_rate_hz, Some(48_000));
        assert_eq!(audio.channels, Some(2));
    }

    #[test]
    fn falls_back_to_stream_duration_when_format_duration_is_missing() {
        let fixture = br#"{
            "streams": [
                {
                    "index": 0,
                    "codec_type": "video",
                    "codec_name": "h264",
                    "duration": "5.25",
                    "width": 1280,
                    "height": 720,
                    "avg_frame_rate": "25/1"
                }
            ],
            "format": {"format_name": "matroska,webm"}
        }"#;

        let metadata = parse_probe_json(fixture).unwrap();
        assert!((metadata.duration_seconds - 5.25).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_metadata_without_positive_duration() {
        let fixture = br#"{
            "streams": [
                {"index": 0, "codec_type": "video", "codec_name": "h264", "duration": "N/A"}
            ],
            "format": {"format_name": "matroska", "duration": "0"}
        }"#;

        assert!(parse_probe_json(fixture).is_err());
    }

    #[test]
    fn frame_rate_parser_rejects_invalid_ratios() {
        assert_eq!(parse_frame_rate(Some("0/0")), None);
        assert_eq!(parse_frame_rate(Some("30/0")), None);
        assert_eq!(parse_frame_rate(Some("not-a-rate")), None);
        assert_eq!(parse_frame_rate(Some("24")), Some(24.0));
    }

    #[test]
    fn ffprobe_command_requests_structured_json() {
        let command = build_ffprobe_command(Path::new("ffprobe"), Path::new("C:/media/video.mkv"));
        let args: Vec<_> = command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect();

        assert_eq!(
            args,
            vec![
                "-v",
                "error",
                "-show_streams",
                "-show_format",
                "-of",
                "json",
                "C:/media/video.mkv"
            ]
        );
        assert_eq!(command.get_program(), OsStr::new("ffprobe"));
    }
}
