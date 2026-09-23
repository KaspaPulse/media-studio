# WhatsApp Video Preparer

WhatsApp Video Preparer 2.0 is a Rust desktop application for downloading a
user-authorized video source and preparing WhatsApp-ready MP4 clips on Windows
and macOS.

## Why v2 exists

The former implementation split video primarily by time. A 29-second H.264
clip can still exceed WhatsApp's 10 MB limit when the source has enough motion
or visual complexity.

v2 treats file size as an enforced output invariant. The default target is
9,500,000 bytes, below the 10,000,000-byte hard limit. Every encoded MP4 is
measured after encoding. If a clip is too large, only that clip is re-encoded
with a lower bitrate under a bounded retry policy. Failure is explicit if the
budget cannot be satisfied.
## Rust-owned implementation

Repository-owned executable code, automated tests, build orchestration, and
governance tooling are Rust. YAML, JSON, TOML, Markdown, and application assets
are configuration/data, not alternate implementation languages.

External process boundaries are pinned rather than reimplemented:

- FFmpeg and FFprobe: static release `n8.1.2-1`.
- yt-dlp: release `2026.08.19`.
- Exact URLs and SHA-256 digests: `.security/ksss/helper-supply-chain.json`.
- Acquisition, digest verification, executable self-check, packaging, and GUI
  smoke orchestration: Rust `xtask`.

The application resolves helpers from the packaged `resources` directory,
including the macOS `Contents/Resources` layout.
## Media behavior

- Default requested clip duration: 29 seconds.
- User-selected duration is a maximum; it can be shortened when the byte budget
  would otherwise be impossible at the minimum encoding floor.
- Video: H.264, `yuv420p`, High profile, fast-start MP4.
- Audio: AAC at 128 kbps.
- Aspect ratio is preserved and synthetic upscaling is avoided.
- The long edge is capped at 1280 pixels.
- Original downloaded source is retained in the job directory.
- Arabic and English UI modes are preserved.

The real regression test `tests/size_budget.rs` generates high-motion media
and exercises the production segmentation path. It must be run with the pinned
FFmpeg/FFprobe helpers.
## Development and qualification

The repository pins Rust 1.98.1.

```text
cargo run -p xtask --locked -- verify
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build --release --locked -p whatsapp-video-preparer
```

Fetch verified helpers on the current platform through `xtask fetch-helpers`.
Official Windows/macOS workflows build the release binary, package the pinned
helpers, execute the real size-budget regression test, run a bounded GUI smoke,
generate SPDX 2.3 SBOM output, and use full-SHA-pinned GitHub Actions. Build
provenance and SBOM attestations are emitted on pushes to `main`.
## KSSS and continuity

The repository is bound to KSSS v1.2.0 with source, policy-bundle, trust-root,
runtime, risk, and applicability identities under `.security/ksss/`.
Repository adoption does not by itself assert application, CI, release, or
production health.

Durable execution rules live under `docs/continuity/`. Material operations
use write-ahead intent and post-operation receipts; valid evidence is reused
until a real validity predicate changes.

The published v1.4.1 release remains historical production evidence. The Rust
v2 candidate is a separate migration and must not be described as published,
merged, or released until those states are independently verified.
