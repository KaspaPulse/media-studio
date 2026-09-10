# PROJECT_STATE.md

Updated: 2026-09-10
Repository: `KaspaPulse/whatsapp-video-preparer` (private)
Default branch: `main`

## Reconciled actual state
Current implementation commit: `7424145f3add3b74d8e1536dcabbff16e40fe25d` — `Merge pull request #2 from KaspaPulse/feature/progress-duration-controls`.
PR #2 is merged into `main`; application version is `1.4.0`.
Windows workflow run `34450323217` completed successfully, including tests, EXE build, GUI startup smoke test, and artifact upload.
macOS workflow run `34450323265` completed successfully for Apple Silicon arm64 and Intel x64, including tests, package build, native startup smoke tests, and artifact uploads.
Local feature staging files were removed after release verification.

## Published release
Release `v1.4.0` is published and targets commit `7424145f3add3b74d8e1536dcabbff16e40fe25d`.
Release URL: `https://github.com/KaspaPulse/whatsapp-video-preparer/releases/tag/v1.4.0`.
`WhatsAppVideoPreparer-Windows-x64.exe`: 123705550 bytes, SHA-256 `4561cc9d3f798fee154835fb95fb5b3230431bf6ec3338e40bb51c55e1a1fa8e`.
`WhatsAppVideoPreparer-macOS-arm64.dmg`: 106939037 bytes, SHA-256 `1d5789fcc5e797457c4175a1354c3b4576ea3c529720107660b1e6860ba77788`.
`WhatsAppVideoPreparer-macOS-x64.dmg`: 122879883 bytes, SHA-256 `0c5d0365a10aac5d9924adce46eb1e9aac1e6e1ed9af3d0489044f60c69dca88`.
GitHub reports exactly these three release assets in `uploaded` state with matching sizes and digests.

## v1.4.0 behavior
The UI shows numeric 0–100% progress for download and FFmpeg conversion/splitting; download detail includes speed and ETA when yt-dlp provides them.
Segment duration is user-selectable in seconds, minutes, or hours and defaults to 29 seconds.
After success, separate actions open the original source, first prepared clip, and results folder.
Windows remains a GUI executable without a console; Arabic is RTL, English is LTR, and URL/path inputs remain LTR.
The media pipeline keeps the original source and outputs H.264/AAC MP4 clips with forced keyframes and no upscaling.

## Current confidence and NEXT ACTION
Automated and integration verification is complete for the published artifacts; interactive end-user workflow validation remains separate from CI smoke tests.
NEXT ACTION: validate `v1.4.0` on an actual Windows and macOS desktop using a real authorized video URL, including progress display, custom duration, produced clip lengths/codecs, and all open-file/open-folder actions. Any observed failure should be handled as a targeted defect without repeating successful broad builds.
