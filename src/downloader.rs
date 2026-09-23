use crate::media::Toolchain;
use anyhow::{Context, Result, bail};
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::SystemTime;

/// Downloads the best available source into the job's original directory.
///
/// # Errors
/// Returns an error when yt-dlp cannot start, exits unsuccessfully, or produces no file.
pub fn download_source<F>(
    url: &str,
    job: &Path,
    tools: &Toolchain,
    mut progress: F,
) -> Result<PathBuf>
where
    F: FnMut(u8, String),
{
    let template = job
        .join("original")
        .join("source.%(ext)s")
        .to_string_lossy()
        .into_owned();

    let mut command = Command::new(&tools.ytdlp);
    command.args([
        "--no-playlist",
        "--windows-filenames",
        "--newline",
        "--no-warnings",
        "--progress",
        "--progress-template",
        "download:%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s",
        "-f",
        "bestvideo*+bestaudio/best",
        "--merge-output-format",
        "mkv",
        "-o",
        &template,
    ]);
    if let Some(parent) = packaged_tool_parent(&tools.ffmpeg) {
        command.arg("--ffmpeg-location").arg(parent);
    }
    command.arg(url);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    hide_window(&mut command);

    let mut child = command
        .spawn()
        .with_context(|| format!("failed to start {}", tools.ytdlp.display()))?;
    let stdout = child.stdout.take().context("yt-dlp stdout unavailable")?;
    let mut stderr = child.stderr.take().context("yt-dlp stderr unavailable")?;

    progress(0, String::new());
    for line in BufReader::new(stdout).lines() {
        let line = line.context("failed to read yt-dlp output")?;
        if let Some((percent, detail)) = parse_progress_line(&line) {
            progress(percent, detail);
        }
    }

    let status = child.wait().context("failed waiting for yt-dlp")?;
    let mut stderr_text = String::new();
    stderr.read_to_string(&mut stderr_text)?;
    if !status.success() {
        bail!("yt-dlp failed: {}", stderr_text.trim());
    }
    progress(100, String::new());

    newest_source(&job.join("original")).context("download completed without an output file")
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

fn newest_source(directory: &Path) -> Option<PathBuf> {
    fs::read_dir(directory)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) != Some("part")
        })
        .max_by_key(|path| {
            fs::metadata(path)
                .and_then(|metadata| metadata.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        })
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
}
