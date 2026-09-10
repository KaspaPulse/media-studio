# PROJECT_STATE.md

Updated: 2026-09-10
Repository: `KaspaPulse/whatsapp-video-preparer` (private)
Default branch: `main`

## Reconciled actual state
Current implementation commit: `90f1d7ebbf9470e9e1c8fcd72f37cc4126e70c42` — merge of PR #3 `Add branded application icon`.
Application version is `1.4.1`.
Windows workflow run `34453989542` completed successfully, including 21 tests, EXE build, icon embedding, GUI startup smoke test, and artifact upload.
macOS workflow run `34453989477` completed successfully for Apple Silicon arm64 and Intel x64, including tests, package build, signing verification, GUI startup smoke tests, and artifact uploads.
The feature branch and local release staging directory were removed after verification.

## Published release
Release `v1.4.1` is published, is not a draft or prerelease, and targets implementation commit `90f1d7ebbf9470e9e1c8fcd72f37cc4126e70c42`.
Release URL: `https://github.com/KaspaPulse/whatsapp-video-preparer/releases/tag/v1.4.1`.
`WhatsAppVideoPreparer-Windows-x64.exe`: 123714623 bytes, SHA-256 `eae1734772d4658cc1086643bb332e737d472fbf8bbfe6b4b2bd65b31f63dab0`.
`WhatsAppVideoPreparer-macOS-arm64.dmg`: 107020281 bytes, SHA-256 `c23280a99bbacb36fbaf70ea5bfed91550ce601272450a1e1b128f5d835ecfbe`.
`WhatsAppVideoPreparer-macOS-x64.dmg`: 122970326 bytes, SHA-256 `af8a0b7705f52496c117b17475429ba137e7b7312757d4fd05ffa52ff58826a2`.
GitHub reports exactly these three release assets in `uploaded` state with matching sizes and digests.

## v1.4.1 icon verification
The approved green video-preparation mark is stored as `assets/app_icon.png`, `assets/app_icon.ico`, and `assets/app_icon.icns`.
Windows PyInstaller logged `Copying icon to EXE`; the final PE contains both RT_ICON and RT_GROUP_ICON resources.
Both macOS DMGs contain `Contents/Resources/app_icon.icns` and `Contents/Resources/assets/app_icon.png`, and each Info.plist sets `CFBundleIconFile` to `app_icon.icns`.
The Qt runtime loads the bundled PNG as both the application and window icon.
All v1.4.0 progress, custom-duration, RTL/LTR, media, and result-opening behavior remains in v1.4.1.

## Current confidence and NEXT ACTION
Automated packaging and startup verification is complete for all published v1.4.1 artifacts. Interactive confirmation that the branded icon is visually displayed in Windows Explorer/taskbar/window and macOS Finder/Dock remains an end-user visual check.
NEXT ACTION: have the user run the published `v1.4.1` on their actual desktop and confirm the icon appearance. Treat any observed icon-display issue as a targeted defect without repeating successful media/build verification.
