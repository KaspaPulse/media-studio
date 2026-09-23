use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tempfile::tempdir;
use whatsapp_video_preparer::media::{
    DEFAULT_SEGMENT_SECONDS, DEFAULT_TARGET_BYTES, Toolchain, create_segments,
};

#[test]
#[ignore = "requires pinned external FFmpeg/FFprobe helpers"]
fn real_high_motion_encode_stays_within_whatsapp_budget() {
    let ffmpeg = required_helper("WVP_FFMPEG");
    let ffprobe = required_helper("WVP_FFPROBE");
    let temp = tempdir().expect("temporary directory");
    let source = temp.path().join("synthetic-source.mp4");
    let job = temp.path().join("job");
    fs::create_dir_all(job.join("clips")).expect("clips directory");

    let status = Command::new(&ffmpeg)
        .args(["-hide_banner", "-loglevel", "error", "-y"])
        .args(["-f", "lavfi", "-i", "testsrc2=size=1280x720:rate=30"])
        .args(["-f", "lavfi", "-i", "sine=frequency=997:sample_rate=48000"])
        .args(["-t", "36"])
        .args(["-c:v", "libx264", "-preset", "ultrafast", "-b:v", "12M"])
        .args([
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "aac",
            "-b:a",
            "192k",
            "-shortest",
        ])
        .arg(&source)
        .status()
        .expect("generate synthetic source");
    assert!(
        status.success(),
        "synthetic FFmpeg source generation failed"
    );

    let tools = Toolchain {
        ffmpeg,
        ffprobe,
        ytdlp: PathBuf::from("not-used-by-segmentation-test"),
    };
    let result = create_segments(
        &source,
        &job,
        DEFAULT_SEGMENT_SECONDS,
        DEFAULT_TARGET_BYTES,
        &tools,
        |_| {},
    )
    .expect("size-safe segmentation");

    assert!(result.clips.len() >= 2, "expected multiple clips");
    assert!(result.max_clip_bytes <= DEFAULT_TARGET_BYTES);
    for clip in result.clips {
        let size = fs::metadata(&clip).expect("clip metadata").len();
        assert!(
            size <= DEFAULT_TARGET_BYTES,
            "{} exceeded budget: {size}",
            clip.display()
        );
    }
}

fn required_helper(name: &str) -> PathBuf {
    env::var_os(name).map_or_else(
        || panic!("{name} must point to a pinned helper binary"),
        PathBuf::from,
    )
}
