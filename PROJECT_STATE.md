# PROJECT_STATE.md

Updated: 2026-09-09
Repository: `KaspaPulse/whatsapp-video-preparer` (private)
Default branch: `main`

## Reconciled actual state
Implementation baseline: `0763356c24b5590eacd389736b88ebe6070cc798` — `feat: add bilingual WhatsApp video preparer`.
Governance continuity commit: `bea2528` — `docs: establish project continuity state [skip ci]`.
The application version is `1.3.0`.
GitHub Actions run `34383220566` completed successfully on Windows.
The build/test step reported `5 passed`.
PyInstaller used the Windows GUI bootloader `runw.exe`, so the built EXE is configured without a console subsystem.
Artifact `WhatsAppVideoPreparer-Windows` was uploaded as artifact ID `10116822307`.
Artifact ZIP SHA-256: `7cab3dca965dd0d10e6e7c83c98192e8de526c2de156e7b620dd69914efd3284`.
Built EXE SHA-256: `94c358ff0b08ef70c0992cd72c68a073561b5431cd188f841d0913e78185f301`.
Local PE inspection confirmed `PE32+ executable (GUI) x86-64` and `Subsystem 00000002 (Windows GUI)`.
Private GitHub release `v1.3.0` is published with `WhatsAppVideoPreparer.exe` and `SHA256SUMS.txt`.
Release URL: `https://github.com/KaspaPulse/whatsapp-video-preparer/releases/tag/v1.3.0`.

## Current confidence
Build and automated tests are verified on a Windows GitHub Actions runner.
Artifact ZIP and EXE hashes were independently rechecked after download.
The PE GUI subsystem was independently verified.
Interactive execution of the EXE on the user's Windows desktop has not yet been verified.
Do not repeat the successful build unless a code/build change requires it.

## NEXT ACTION
Have the user run `WhatsAppVideoPreparer.exe` on Windows and verify the real GUI flow: launch without console, Arabic RTL/English LTR switching, download, 29-second segmentation, and result-folder opening. Treat any failure as a targeted defect. macOS support is not yet built; if the user prioritizes macOS, design and add a native macOS `.app`/`.dmg` build separately rather than attempting to run the Windows EXE on macOS.
