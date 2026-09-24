use crate::media::Toolchain;
use anyhow::{Context, Result, anyhow, bail};
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

const FINAL_PATH_REPORT: &str = ".media-studio-yt-dlp-final-path.txt";

/// Downloads the best available source into the job's original directory.
///
/// External yt-dlp configuration and plugins are disabled so repository behavior cannot be
/// extended by untrusted machine-local configuration. The final path is reported by yt-dlp
/// after post-processing and then validated to remain inside the job's original directory.
///
/// # Errors
/// Returns an error when yt-dlp cannot start, exits unsuccessfully, does not report exactly
/// one final path, or reports a path outside the expected job directory.
pub fn download_source<F>(
    url: &str,
    job: &Path,
    tools: &Toolchain,
    mut progress: F,
) -> Result<PathBuf>
where
    F: FnMut(u8, String),
{
    let original_dir = job.join("original");
    fs::create_dir_all(&original_dir).with_context(|| {
        format!(
            "failed to create remote source directory {}",
            original_dir.display()
        )
    })?;

    let report_path = original_dir.join(FINAL_PATH_REPORT);
    if report_path.exists() {
        fs::remove_file(&report_path).with_context(|| {
            format!(
                "failed to remove stale yt-dlp path report {}",
                report_path.display()
            )
        })?;
    }

    let mut command = build_ytdlp_command(
        &tools.ytdlp,
        &tools.ffmpeg,
        url,
        &original_dir,
        FINAL_PATH_REPORT,
    );
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    hide_window(&mut command);

    let mut child = command
        .spawn()
        .with_context(|| format!("failed to start {}", tools.ytdlp.display()))?;
    let stdout = child.stdout.take().context("yt-dlp stdout unavailable")?;
    let mut stderr = child.stderr.take().context("yt-dlp stderr unavailable")?;
    let stderr_reader = thread::spawn(move || -> std::io::Result<String> {
        let mut text = String::new();
        stderr.read_to_string(&mut text)?;
        Ok(text)
    });

    progress(0, String::new());
    for line in BufReader::new(stdout).lines() {
        let line = line.context("failed to read yt-dlp output")?;
        if let Some((percent, detail)) = parse_progress_line(&line) {
            progress(percent, detail);
        }
    }

    let status = child.wait().context("failed waiting for yt-dlp")?;
    let stderr_text = stderr_reader
        .join()
        .map_err(|_| anyhow!("yt-dlp stderr reader panicked"))?
        .context("failed to read yt-dlp stderr")?;

    if !status.success() {
        bail!("yt-dlp failed: {}", stderr_text.trim());
    }

    let reported = read_final_path_report(&report_path)?;
    let resolved = validate_reported_source(&reported, &original_dir)?;
    let _ = fs::remove_file(&report_path);
    progress(100, String::new());
    Ok(resolved)
}

fn build_ytdlp_command(
    ytdlp: &Path,
    ffmpeg: &Path,
    url: &str,
    original_dir: &Path,
    report_file: &str,
) -> Command {
    let mut command = Command::new(ytdlp);
    command
        .current_dir(original_dir)
        .env("YTDLP_NO_PLUGINS", "1")
        .args([
            "--ignore-config",
            "--no-plugin-dirs",
            "--no-playlist",
            "--windows-filenames",
            "--newline",
            "--no-warnings",
            "--progress",
            "--progress-template",
            "download:%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s",
            "--print-to-file",
            "after_move:%(filepath)s",
            report_file,
            "-f",
            "bestvideo*+bestaudio/best",
            "--merge-output-format",
            "mkv",
            "-o",
            "source.%(ext)s",
        ]);

    if let Some(parent) = packaged_tool_parent(ffmpeg) {
        command.arg("--ffmpeg-location").arg(parent);
    }
    command.arg(url);
    command
}

fn read_final_path_report(report_path: &Path) -> Result<PathBuf> {
    let text = fs::read_to_string(report_path).with_context(|| {
        format!(
            "yt-dlp completed without a readable final-path report: {}",
            report_path.display()
        )
    })?;
    let paths: Vec<_> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if paths.len() != 1 {
        bail!(
            "yt-dlp final-path report must contain exactly one path, found {}",
            paths.len()
        );
    }
    Ok(PathBuf::from(paths[0]))
}

fn validate_reported_source(reported: &Path, original_dir: &Path) -> Result<PathBuf> {
    let candidate = if reported.is_absolute() {
        reported.to_path_buf()
    } else {
        original_dir.join(reported)
    };

    let root = fs::canonicalize(original_dir).with_context(|| {
        format!(
            "failed to resolve remote source directory {}",
            original_dir.display()
        )
    })?;
    let resolved = fs::canonicalize(&candidate).with_context(|| {
        format!(
            "reported remote source does not exist: {}",
            candidate.display()
        )
    })?;

    if !resolved.starts_with(&root) {
        bail!(
            "yt-dlp reported a source outside the job directory: {}",
            resolved.display()
        );
    }
    if !resolved.is_file() {
        bail!(
            "yt-dlp reported source is not a regular file: {}",
            resolved.display()
        );
    }
    Ok(resolved)
}

fn packaged_tool_parent(tool: &Path) -> Option<&Path> {
    if tool.is_absolute() || tool.components().count() > 1 {
        tool.parent()
            .filter(|parent| !parent.as_os_str().is_empty())
    } else {
        None
    }
}

fn parse_progress_line(line: &str) -> Option<(u8, String)> {
    let payload = line.strip_prefix("download:")?;
    let mut parts = payload.split('|');
    let raw_percent = parts.next()?.trim().trim_end_matches('%');
    let integer = raw_percent.split('.').next()?.trim();
    let percent = integer.parse::<u8>().ok()?.min(100);
    let speed = parts.next().unwrap_or_default().trim();
    let eta = parts.next().unwrap_or_default().trim();
    let mut detail = Vec::new();
    if !speed.is_empty() && speed != "NA" {
        detail.push(speed.to_owned());
    }
    if !eta.is_empty() && eta != "NA" {
        detail.push(format!("ETA {eta}"));
    }
    Some((percent, detail.join(" · ")))
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
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parses_ytdlp_progress() {
        let parsed = parse_progress_line("download: 42.5%|3.2MiB/s|00:10").unwrap();
        assert_eq!(parsed.0, 42);
        assert_eq!(parsed.1, "3.2MiB/s · ETA 00:10");
    }

    #[test]
    fn ignores_unrelated_output() {
        assert!(parse_progress_line("metadata: example").is_none());
    }

    #[test]
    fn ytdlp_command_ignores_external_configuration_and_plugins() {
        let temp = tempdir().unwrap();
        let command = build_ytdlp_command(
            Path::new("yt-dlp"),
            Path::new("ffmpeg"),
            "https://example.com/video",
            temp.path(),
            FINAL_PATH_REPORT,
        );
        let args: Vec<_> = command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();

        assert!(args.iter().any(|arg| arg == "--ignore-config"));
        assert!(args.iter().any(|arg| arg == "--no-plugin-dirs"));
        assert!(
            args.windows(2).any(|pair| {
                pair[0] == "--print-to-file" && pair[1] == "after_move:%(filepath)s"
            })
        );
        assert!(args.iter().any(|arg| arg == FINAL_PATH_REPORT));
        assert_eq!(command.get_current_dir(), Some(temp.path()));
        assert!(command.get_envs().any(|(key, value)| {
            key == OsStr::new("YTDLP_NO_PLUGINS") && value == Some(OsStr::new("1"))
        }));
    }

    #[test]
    fn final_path_report_requires_exactly_one_path() {
        let temp = tempdir().unwrap();
        let report = temp.path().join("report.txt");
        fs::write(&report, "source.mkv\n").unwrap();
        assert_eq!(
            read_final_path_report(&report).unwrap(),
            PathBuf::from("source.mkv")
        );

        fs::write(&report, "a.mkv\nb.mkv\n").unwrap();
        assert!(read_final_path_report(&report).is_err());
    }

    #[test]
    fn reported_source_must_remain_inside_original_directory() {
        let temp = tempdir().unwrap();
        let original = temp.path().join("original");
        fs::create_dir(&original).unwrap();
        let inside = original.join("source.mkv");
        fs::write(&inside, b"media").unwrap();

        let validated = validate_reported_source(Path::new("source.mkv"), &original).unwrap();
        assert_eq!(validated, fs::canonicalize(&inside).unwrap());

        let outside = temp.path().join("outside.mkv");
        fs::write(&outside, b"media").unwrap();
        assert!(validate_reported_source(&outside, &original).is_err());
    }
}
