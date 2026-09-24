# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `KASPAPULSE_MEDIA_STUDIO_V3`
Current phase: `G3 — MEDIA PROBE`

## Product boundary

CURRENT_PRODUCT=WhatsApp Video Preparer
CURRENT_VERSION=2.0.0
TARGET_PRODUCT=KaspaPulse Media Studio
TARGET_VERSION=3.0.0
TARGET_REPOSITORY=KaspaPulse/media-studio
REPOSITORY_RENAME_STATUS=DEFERRED

V2_HISTORICAL_BOUNDARY=IMMUTABLE
V2_0_0_RELEASE=PRESERVE
V2_TAGS=DO_NOT_REWRITE
V2_RELEASE_ASSETS=DO_NOT_RENAME_OR_REPLACE

## Completed and verified

### G0 — Foundation Freeze
FINAL_MAIN_SHA=`27c68ff509477d53e454a4f6f80b7eeffb8e7d15`
FINAL_MAIN_TREE=`2428660026f840f73631b8d0b31da32157038014`
PR_9=MERGED_SQUASH
Rust Policy `35995254915` PASS.
Windows `35995254993` PASS including provenance/SPDX attestation.
macOS `35995254996` PASS on x64/arm64 including provenance/SPDX attestation.
FOUNDATION_FREEZE=PASS

### G1 — Product / Domain Model
CHECKPOINT_SHA=`6270f011ef293482063573bc5f97a2574b502f2a`
CHECKPOINT_TREE=`8d2c2652fe8c79ef7e3f62e82b85690e8bb2c62b`
Implemented ProductIdentity, InputSource, Locale, UiDirection, AppViewState, MirrorPolicy,
and FocusPolicy foundation.
Affected-surface qualification PASS.

### G2 — Source Acquisition
CHECKPOINT_SHA=`59540f352f1f66f5dd07a79d52c0ab5a74015888`
CHECKPOINT_TREE=`e3f97e74cca182b78a3178369724b8a0cebedc63`

Implemented:
- LocalFile read-only acquisition;
- RemoteUrl acquisition through pinned yt-dlp;
- --ignore-config;
- --no-plugin-dirs;
- YTDLP_NO_PLUGINS=1;
- after_move final-path reporting;
- reported path containment in job/original;
- no newest-file guessing on successful remote acquisition.

G2 qualification:
- fmt/fmt-check PASS;
- lib tests 24/24 PASS;
- binary check PASS;
- all-target clippy -D warnings PASS after smallest import-scope fix;
- xtask repository/KSSS/Rust-only/cargo-deny gate PASS.

G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT

## Current phase — G3 Media Probe

Goal: add structured FFprobe-based metadata inspection without an extension allowlist.

Required metadata includes, as applicable:
- container;
- duration;
- video/audio streams;
- codec;
- dimensions;
- frame rate;
- pixel format;
- rotation/orientation;
- relevant color/HDR metadata.

SUPPORTED_INPUT=MEDIA_READABLE_BY_BUNDLED_FFMPEG_BUILD

G3 must fail closed when FFprobe fails or structured metadata cannot be interpreted.
G3 does not yet decouple the processing engine or implement export profiles.

## Preserved baselines

GUI_FRAMEWORK=iced
GUI_FRAMEWORK_BASELINE=0.14.0
ICED_DEFAULT_FEATURES=DISABLED
ICED_RENDERER_BASELINE=tiny-skia
RUST_BASELINE=1.98.1
CARGO_LOCK_REQUIRED=YES
BUILD_WITH_LOCKED=YES
OWNED_IMPLEMENTATION=100_PERCENT_RUST

SCREEN_READER_ACCESSIBILITY=NOT_CLAIMED_UNTIL_TOOLKIT_SUPPORT_AND_NATIVE_VERIFICATION
WCAG_2_2=DESIGN_REFERENCE
WCAG2ICT_2_2=NON_WEB_SOFTWARE_GUIDANCE_REFERENCE
FORMAL_ACCESSIBILITY_CONFORMANCE_CLAIM=NO

## Continuity

Canonical journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Policies:
`RECOVER → RECONCILE → CLASSIFY → CONTINUE`
`PERSIST → VERIFY → CHECKPOINT → CONTINUE`
`TEST_THE_AFFECTED_SURFACE / REUSE_VALID_EVIDENCE / RERUN_ONLY_WHEN_INVALIDATED / NO_FAKE_PASS`

DO_NOT_REPEAT:
G0 exact-head/main CI, G1 qualification, or G2 qualification while their validity predicates remain unchanged.

NEXT ACTION:
Implement and qualify G3 structured MediaProbe, then create the next durable local checkpoint.
