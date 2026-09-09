# PROJECT_STATE.md

Updated: 2026-09-09
Repository: `KaspaPulse/whatsapp-video-preparer` (private)
Default branch: `main`

## Reconciled actual state
Current main HEAD: `6e7f588a7b6cecd0b4f25769ee0b84a8196583e8` — `ci: smoke-test Windows EXE before upload`.
Working tree is clean and local `main` matches `origin/main` after fetch/fast-forward.
Windows frozen entrypoint defect was fixed by changing `__main__.py` to an absolute package import.
Windows workflow run `34387730286` completed successfully, including build/tests and an EXE startup smoke test.
Current Windows artifact: `WhatsAppVideoPreparer-Windows`, artifact ID `10118474858`, artifact ZIP digest `sha256:22ee3ff41e03e60d2d8046d9f9740388febc4fd3204930cf65b56dd9b74de80f`.
The published `v1.3.0` release is stale and contains the pre-fix EXE; do not direct users to that EXE.

## macOS feature state
Feature branch: `feature/macos-build` at `0a7c0a1f9164abea89db2d4eb7d058a8211ca218`.
PR #1 `Add native macOS builds` is open and mergeable into `main`.
macOS workflow run `34387365119` completed successfully for Apple Silicon arm64 and Intel x64.
Both macOS jobs passed tests, package build, code-sign verification, GUI startup smoke test, and artifact upload.
macOS arm64 artifact ID: `10118272984`.
macOS x64 artifact ID: `10118292117`.
The branch bumps the application version to `1.3.1` and adds native `.app`/`.dmg` packaging.

## Current confidence
The corrected Windows EXE has a successful automated startup smoke test on Windows GitHub Actions.
Both macOS architecture packages have successful native macOS startup smoke tests on GitHub Actions.
Interactive end-user workflow validation is still separate from CI smoke verification.

## NEXT ACTION
User explicitly approved merging PR #1, then verifying fresh Windows and macOS builds on merged `main`, and publishing release `v1.3.1` with exactly these final assets: `WhatsAppVideoPreparer-Windows-x64.exe`, `WhatsAppVideoPreparer-macOS-arm64.dmg`, and `WhatsAppVideoPreparer-macOS-x64.dmg`. Do not publish until all merged-main verification runs succeed.