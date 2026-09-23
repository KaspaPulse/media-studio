# WhatsApp Video Preparer Design

Status: Historical v1 design, superseded by the Rust v2 architecture documented in `README.md` and `PROJECT_STATE.md`. The v1 1920-pixel/one-pass segmentation notes below are retained as history and are not the current size-budget contract.

## Goal
Build a Windows desktop `.exe` that downloads a user-supplied video URL, keeps the highest-quality source, converts it to WhatsApp-friendly MP4, and splits it into 29-second clips.

## UX
- Native graphical window only; no visible CMD or PowerShell console.
- Arabic and English are switchable at runtime.
- Arabic uses true RTL layout; English uses LTR.
- URL and filesystem path fields always remain LTR for correctness.
- Main flow: paste URL, choose output folder, click prepare, open result folder.

## Media pipeline
- `yt-dlp` downloads the best available video+audio.
- A bundled FFmpeg executable performs all conversion and splitting.
- Output codec profile: H.264 video, AAC audio, `yuv420p`, `+faststart`.
- Preserve aspect ratio and avoid synthetic upscaling.
- Cap the long edge at 1920 pixels, yielding up to 1920x1080 landscape or 1080x1920 portrait.
- Re-encode once and force keyframes every 29 seconds for accurate segment boundaries.
