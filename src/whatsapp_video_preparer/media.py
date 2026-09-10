from __future__ import annotations

import os
import re
import subprocess
import sys
import time
from pathlib import Path
from urllib.parse import urlparse

import imageio_ffmpeg
import yt_dlp

SEGMENT_SECONDS = 29


def duration_to_seconds(value: float, unit: str) -> float:
    factors = {"seconds": 1.0, "minutes": 60.0, "hours": 3600.0}
    if unit not in factors:
        raise ValueError(f"Unsupported duration unit: {unit}")
    seconds = float(value) * factors[unit]
    if seconds <= 0:
        raise ValueError("Segment duration must be greater than zero")
    return seconds


def is_valid_url(value: str) -> bool:
    try:
        parsed = urlparse(value.strip())
        return parsed.scheme in {"http", "https"} and bool(parsed.netloc)
    except Exception:
        return False


def parse_ffmpeg_progress_seconds(line: str) -> float | None:
    key, sep, value = line.strip().partition("=")
    if not sep:
        return None
    if key in {"out_time_us", "out_time_ms"}:
        try:
            return float(value) / 1_000_000.0
        except ValueError:
            return None
    if key == "out_time":
        try:
            hours, minutes, seconds = value.split(":", 2)
            return int(hours) * 3600 + int(minutes) * 60 + float(seconds)
        except (TypeError, ValueError):
            return None
    return None


def ffmpeg_path() -> Path:
    if getattr(sys, "frozen", False):
        binary_name = "ffmpeg.exe" if sys.platform == "win32" else "ffmpeg"
        return Path(sys._MEIPASS) / "resources" / binary_name
    return Path(imageio_ffmpeg.get_ffmpeg_exe())


def hidden_flags() -> dict:
    if os.name == "nt":
        return {"creationflags": subprocess.CREATE_NO_WINDOW}
    return {}


def probe_duration(source: Path) -> float:
    result = subprocess.run(
        [str(ffmpeg_path()), "-hide_banner", "-i", str(source)],
        capture_output=True, text=True, **hidden_flags(),
    )
    text = str(result.stdout) + "\n" + str(result.stderr)
    match = re.search(r"Duration:\s*(\d+):(\d+):(\d+(?:\.\d+)?)", text)
    if not match:
        raise RuntimeError("Unable to determine media duration")
    hours, minutes, seconds = match.groups()
    return int(hours) * 3600 + int(minutes) * 60 + float(seconds)


def make_job_dir(root: Path) -> Path:
    stamp = time.strftime("%Y%m%d_%H%M%S")
    job = root / f"WhatsApp_Video_{stamp}"
    job.mkdir(parents=True, exist_ok=False)
    (job / "original").mkdir()
    (job / "clips").mkdir()
    return job


def download_source(url: str, job: Path, progress=None) -> Path:
    target = str(job / "original" / "source.%(ext)s")
    ffmpeg = ffmpeg_path()
    options = {
        "format": "bestvideo*+bestaudio/best",
        "outtmpl": target,
        "merge_output_format": "mkv",
        "ffmpeg_location": str(ffmpeg.parent),
        "noplaylist": True,
        "quiet": True,
        "no_warnings": True,
        "windowsfilenames": True,
    }
    if progress:
        options["progress_hooks"] = [progress]
    with yt_dlp.YoutubeDL(options) as ydl:
        ydl.download([url])
    files = [p for p in (job / "original").iterdir() if p.is_file()]
    if not files:
        raise RuntimeError("Download completed without an output file")
    return max(files, key=lambda p: p.stat().st_mtime)


def segment_command(source: Path, clips_dir: Path, segment_seconds: float = SEGMENT_SECONDS) -> list[str]:
    output = clips_dir / "status_%03d.mp4"
    seconds_text = f"{segment_seconds:g}"
    scale = (
        "scale="
        "'if(gte(iw,ih),min(1920,iw),-2)':"
        "'if(gte(iw,ih),-2,min(1920,ih))'"
    )
    return [
        str(ffmpeg_path()), "-hide_banner", "-loglevel", "error", "-y",
        "-i", str(source),
        "-map", "0:v:0", "-map", "0:a:0?",
        "-vf", scale,
        "-c:v", "libx264", "-preset", "medium", "-crf", "20",
        "-pix_fmt", "yuv420p", "-profile:v", "high", "-level", "4.1",
        "-force_key_frames", f"expr:gte(t,n_forced*{seconds_text})",
        "-c:a", "aac", "-b:a", "160k", "-ar", "48000",
        "-f", "segment", "-segment_time", seconds_text,
        "-reset_timestamps", "1", "-segment_format", "mp4",
        "-segment_format_options", "movflags=+faststart",
        str(output),
    ]


def create_segments(
    source: Path,
    job: Path,
    segment_seconds: float = SEGMENT_SECONDS,
    progress=None,
) -> list[Path]:
    duration = probe_duration(source)
    command = segment_command(source, job / "clips", segment_seconds)
    command[1:1] = ["-progress", "pipe:1", "-nostats"]
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,
        **hidden_flags(),
    )
    last_percent = -1
    if progress:
        progress(0)
        last_percent = 0
    assert process.stdout is not None
    for line in process.stdout:
        elapsed = parse_ffmpeg_progress_seconds(line)
        if elapsed is None or duration <= 0:
            continue
        percent = max(0, min(99, int((elapsed / duration) * 100)))
        if progress and percent != last_percent:
            progress(percent)
            last_percent = percent
    returncode = process.wait()
    stderr = process.stderr.read() if process.stderr is not None else ""
    if returncode != 0:
        detail = (stderr or "FFmpeg failed").strip()
        raise RuntimeError(detail[-2000:])
    clips = sorted((job / "clips").glob("status_*.mp4"))
    if not clips:
        raise RuntimeError("FFmpeg produced no clips")
    if progress:
        progress(100)
    return clips
