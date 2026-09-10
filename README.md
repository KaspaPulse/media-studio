# WhatsApp Video Preparer

Windows and macOS desktop app for downloading a user-provided video URL at the best available quality and preparing WhatsApp-ready clips.

## Features
- Native Windows `.exe` GUI with no console window.
- Branded application icon embedded in Windows EXE, macOS app, and the running window.
- Native macOS `.app` packaged in `.dmg` for Apple Silicon and Intel Macs.
- Arabic RTL and English LTR interface.
- URL and filesystem path fields remain LTR in both languages.
- Best-quality download via `yt-dlp`.
- Bundled FFmpeg processing.
- Keeps the original downloaded source.
- Creates MP4 clips using H.264 + AAC + yuv420p + faststart.
- Defaults to 29-second segments and lets the user choose a custom duration in seconds, minutes, or hours.
- Shows numeric download and conversion/splitting progress, including download speed and ETA when available.
- Provides post-completion actions to open the original file, first prepared clip, or results folder.
- Uses forced keyframes at the selected segment boundary.
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

## macOS build
Run `./build_macos.sh arm64` on Apple Silicon or `./build_macos.sh x64` on Intel. GitHub Actions builds both architectures and produces `.dmg` packages.

The macOS build is ad-hoc signed for package integrity but is not Apple Developer ID signed or notarized, so Gatekeeper may require the user to explicitly allow the app on first launch.

Use only with media you own or are authorized to download and share.
