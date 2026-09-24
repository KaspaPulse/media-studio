# KaspaPulse Media Studio

KaspaPulse Media Studio 3.0 is the next-generation Rust desktop application in this repository.
It accepts local media files or user-authorized remote media URLs, probes media through the
bundled FFprobe build, and processes exports through profile-driven FFmpeg workflows.

Version 3.0.0 is still a development candidate until the native qualification and release gates
complete. The published WhatsApp Video Preparer v2.0.0 tag, release, attestations, and assets
remain immutable historical evidence.

## Sources and media support

- Local media files are opened read-only; originals are never overwritten.
- Remote URLs are acquired through the pinned yt-dlp helper where its bundled extractors can
  handle the source.
- yt-dlp runs with repository-controlled arguments, external config ignored, and plugins disabled.
- Media capability is determined by FFprobe/FFmpeg at runtime instead of a filename-extension
  allowlist.
- Drag and drop is optional; the local file picker remains available as the non-drag alternative.

The project does not claim support for every website or every media format in existence. Support
is bounded by the pinned yt-dlp and FFmpeg/FFprobe builds, authentication requirements, and source
availability. DRM circumvention is outside project scope.

## Export profiles and processing

Built-in profiles currently include:

- Universal MP4
- WhatsApp
- High Quality
- Web Compatible

WhatsApp is a profile, not the processing engine. Its historical qualified safety policy remains
9,500,000 target bytes below a 10,000,000-byte hard limit with post-processing measurement and
bounded retries.

The processing engine is source-independent. It can remux compatible media when profile
constraints already hold, or transcode when required. Aspect ratio is preserved and scaling is
bounded by the selected profile.

## Desktop UI

The v3 UI is built with the locked iced 0.14 baseline using the tiny-skia renderer. The
application owns RTL/LTR layout semantics instead of depending on an unreleased upstream RTL
layout patch.

Implemented architecture includes:

- Arabic and English locale modes
- application-owned RTL/LTR visual ordering
- LTR handling and BiDi isolation for technical values
- Compact, Standard, and Wide responsive layouts
- System, Light, and Dark theme preferences
- semantic keyboard/focus wiring and stable IDs where iced 0.14 exposes them

Screen-reader accessibility is not claimed until the toolkit exposes suitable native
accessibility support and the application completes native verification.

## Rust-owned implementation

Repository-owned application logic, tests, build/packaging orchestration, validation, and
governance tooling are Rust. YAML, JSON, TOML, Markdown, and application assets are
configuration/data, not alternate implementation languages.

External helper boundaries remain pinned:

- FFmpeg and FFprobe: static release `n8.1.2-1`
- yt-dlp: release `2026.08.19`
- exact URLs and SHA-256 digests: `.security/ksss/helper-supply-chain.json`

The project remains bound to KSSS v1.2.0 and enforces the Rust-only repository gate, locked
dependencies, cargo-deny policy, full-SHA GitHub Actions, SBOM generation, and provenance
attestations.

## Development

The repository pins Rust 1.98.1.

```text
cargo run -p xtask --locked -- verify
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build --release --locked -p media-studio
```

Official Windows and macOS workflows build from the exact source identity and produce future v3
artifacts under the `KaspaPulseMediaStudio-*` naming contract.

## Continuity and release policy

Development follows:

```text
RECOVER -> RECONCILE -> CLASSIFY -> CONTINUE
PERSIST -> VERIFY -> CHECKPOINT -> CONTINUE
TEST_THE_AFFECTED_SURFACE
REUSE_VALID_EVIDENCE
RERUN_ONLY_WHEN_INVALIDATED
LOCAL_FIRST
REMOTE_LAST
```

The v3.0.0 release is published only after exact-source native qualification succeeds for the
required Windows/macOS targets and the release artifacts, checksums, SBOMs, and provenance resolve
to the same qualified source identity.
