# WhatsApp Video Preparer

Windows desktop app for downloading a user-provided video URL at the best available quality and preparing WhatsApp-ready clips.

## Features
- Native `.exe` GUI with no console window.
- Arabic RTL and English LTR interface.
- URL and filesystem path fields remain LTR in both languages.
- Best-quality download via `yt-dlp`.
- Bundled FFmpeg processing.
- Keeps the original downloaded source.
- Creates MP4 clips using H.264 + AAC + yuv420p + faststart.
- Splits into 29-second segments with forced keyframes.
- Preserves aspect ratio and avoids upscaling.
- Caps the long edge at 1920 pixels.

## Development
```bash
python -m pip install -e .
python -m pytest -q
python -m whatsapp_video_preparer
```

## Windows build
Run `build.ps1`, or push to `main` and download the GitHub Actions artifact `WhatsAppVideoPreparer-Windows`.

Use only with media you own or are authorized to download and share.
