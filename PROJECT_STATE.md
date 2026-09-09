# PROJECT_STATE.md

Updated: 2026-09-09
Repository: `KaspaPulse/whatsapp-video-preparer` (private)
Default branch: `main`

## Reconciled actual state
Release implementation commit: `3209a9d124cbc60ca767eefdb3e0920cfa55ec5c` — `Merge PR #1: Add native macOS builds`.
PR #1 `Add native macOS builds` is merged into `main`.
Application version is `1.3.1`.
Merged-main Windows workflow run `34389211307` completed successfully, including build/tests, EXE startup smoke test, and artifact upload.
Merged-main macOS workflow run `34389211325` completed successfully for Apple Silicon arm64 and Intel x64, including build/tests, native app startup smoke tests, and artifact uploads.

## Published release
Release `v1.3.1` is published and targets commit `3209a9d124cbc60ca767eefdb3e0920cfa55ec5c`.
Release URL: `https://github.com/KaspaPulse/whatsapp-video-preparer/releases/tag/v1.3.1`.
Final asset `WhatsAppVideoPreparer-Windows-x64.exe`: 123701085 bytes, SHA-256 `82f731545d89198cfd3a3f9d49237d889f9905f554c9bd80dbd2570809cdf06a`.
Final asset `WhatsAppVideoPreparer-macOS-arm64.dmg`: 108367701 bytes, SHA-256 `3ab61c2edf13fa9b3621ad253900f5e2a29d762ff8abb2a1e4999dae6eba2049`.
Final asset `WhatsAppVideoPreparer-macOS-x64.dmg`: 121754327 bytes, SHA-256 `1923206ce48d608ec1e03f80009d0e917194daf61202d03c9184fba145855542`.
GitHub reports all three release assets in `uploaded` state with matching digests.
The stale `v1.3.0` Windows executable must not be used.

## Current confidence
The corrected Windows EXE has a successful automated startup smoke test on a Windows GitHub Actions runner and is a PE32+ GUI x86-64 executable.
Both macOS architecture packages have successful native macOS startup smoke tests on GitHub Actions.
Interactive end-user workflow validation remains separate from CI smoke verification.
macOS packages are ad-hoc signed but not Apple Developer ID notarized; Gatekeeper may require `Open Anyway` on first launch.

## NEXT ACTION
Have the user validate the published `v1.3.1` on their actual Windows and macOS machines: launch, Arabic RTL/English LTR switching, URL download, original-source retention, 29-second H.264/AAC segmentation, and result-folder opening. Treat any observed failure as a targeted defect and do not repeat successful broad builds or audits without evidence of regression.