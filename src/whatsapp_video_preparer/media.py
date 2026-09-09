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


def is_valid_url(value: str) -> bool:
    try:
        parsed = urlparse(value.strip())
        return parsed.scheme in {"http", "https"} and bool(parsed.netloc)
    except Exception:
        return False


def ffmpeg_path() -> Path:
    if getattr(sys, "frozen", False):
        return Path(sys._MEIPASS) / "resources" / "ffmpeg.exe"
    return Path(imageio_ffmpeg.get_ffmpeg_exe())


def hidden_flags() -> dict:
    if os.name == "nt":
        return {"creationflags": subprocess.CREATE_NO_WINDOW}
    return {}


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


def segment_command(source: Path, clips_dir: Path) -> list[str]:
    output = clips_dir / "status_%03d.mp4"
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
        "-force_key_frames", f"expr:gte(t,n_forced*{SEGMENT_SECONDS})",
        "-c:a", "aac", "-b:a", "160k", "-ar", "48000",
        "-f", "segment", "-segment_time", str(SEGMENT_SECONDS),
        "-reset_timestamps", "1", "-segment_format", "mp4",
        "-segment_format_options", "movflags=+faststart",
        str(output),
    ]


def create_segments(source: Path, job: Path) -> list[Path]:
    command = segment_command(source, job / "clips")
    result = subprocess.run(command, capture_output=True, text=True, **hidden_flags())
    if result.returncode != 0:
        detail = (result.stderr or result.stdout or "FFmpeg failed").strip()
        raise RuntimeError(detail[-2000:])
    clips = sorted((job / "clips").glob("status_*.mp4"))
    if not clips:
        raise RuntimeError("FFmpeg produced no clips")
    return clips
