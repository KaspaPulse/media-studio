# PROJECT STATE

Updated: 2026-09-23
Repository: `KaspaPulse/whatsapp-video-preparer`
Default branch: `main`
Active local branch: `wvp-rust-ksss-20260923`
Baseline main observed: `a605b3d30d6aed7304f9d9b617a31f86c00fb3da`
Candidate version: `2.0.0`

## Current candidate

The repository-owned implementation has been rebuilt in Rust. The former
Python/PySide6 application, Python tests, `pyproject.toml`, PowerShell build
script, and Bash build script are retired from the candidate tree. External
FFmpeg, FFprobe, and yt-dlp binaries remain explicit pinned helper boundaries.

The WhatsApp defect root cause was duration-only segmentation without an
enforced post-encode byte limit. v2 targets 9,500,000 bytes under a
10,000,000-byte hard limit, measures each encoded file, and retries only an
oversized clip with a lower bitrate.
## Locally verified evidence

- Rust toolchain: 1.98.1.
- Repository/KSSS Rust gate: PASS.
- `cargo fmt --all -- --check`: PASS on final source after the last Rust edit.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- Workspace tests: 11 passed, 0 failed; real helper-dependent test ignored in
  the ordinary suite by design.
- Real high-motion size-budget integration test: PASS, 1 passed / 0 failed,
  using the pinned packaged FFmpeg and FFprobe.
- `cargo build --release --locked -p whatsapp-video-preparer`: PASS.
- Windows final package: PASS; all four entries in `SHA256SUMS` independently
  matched.
- Packaged Windows GUI bounded startup smoke: PASS; the spawned GUI remained
  alive for 6 seconds and only that captured PID was terminated.
- Final Windows EXE SHA-256:
  `5be021c19a3efb69045b0d04a56fb8e0584bd05996c2be32dabe9a8fa8cca6a1`.
## Helper supply-chain identities

Windows package helper SHA-256 values:

- FFmpeg: `4044b3924c977ad31229d504c5d5b8685f9553124fbaff6e9c99048b42830341`
- FFprobe: `fc37ca23d31ee08bb8f7e108edf3822f6ef3efc1a8d306bbe0b779190230710b`
- yt-dlp: `66674953fe251b89f4d08c5f0e35e0728679bd67ab3d7d05c0562af101dd3e7a`

The Rust `xtask` downloads to a process-specific partial path, syncs bytes,
verifies SHA-256 before publication into the package, rejects mismatched
existing helpers, sets executable permissions where required, and performs a
version self-check.
## KSSS binding

Pinned KSSS identity:

- Release: `v1.2.0`
- Source SHA: `967ed5068947a39961d5d5cc483ef65d25a61059`
- Policy bundle:
  `3c1c8b449d736aba5fec5cffd688496b06d81f287f55fff4c3f42e36c3b58ec6`
- Consumer Runtime SHA-256:
  `38309d2ab8fa30096d99940f855e88173faa182e60db33f2a96b2d3408507430`
- Runtime sequence: 2
- Trust root: `ksss-trust-root-1`

The repository validates pinned adoption/risk/applicability identities locally
with Rust. This is not a claim that KSSS cryptographic acquisition acceptance,
GitHub CI, release publication, or production adoption has occurred.
## Remote and platform status

- GitHub candidate push: NOT_STARTED.
- Pull request: NOT_STARTED.
- Merge: NOT_STARTED.
- Rust v2 GitHub Actions: NOT_RUN.
- macOS v2 package/runtime qualification: NOT_VERIFIED; its workflow is defined
  for native arm64 and x64 GitHub-hosted runners.
- Rust v2 release: NOT_PUBLISHED.
- Historical published release v1.4.1 remains unchanged.

## Continuity

Canonical operation journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Current Git HEAD must always be verified dynamically. The journal records
operation receipts and the eventual local checkpoint commit identity.

NEXT ACTION: complete staged-diff validation and create the local checkpoint
commit. Any GitHub push/PR/merge/release remains a distinct external state and
must follow the active authorization boundary and repository protections.
