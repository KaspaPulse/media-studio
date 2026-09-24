# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `KASPAPULSE_MEDIA_STUDIO_V3`
Current phase: `G5 — PROCESSING ENGINE DECOUPLING`

## Product boundary
CURRENT_PRODUCT=WhatsApp Video Preparer
CURRENT_VERSION=2.0.0
TARGET_PRODUCT=KaspaPulse Media Studio
TARGET_VERSION=3.0.0
TARGET_REPOSITORY=KaspaPulse/media-studio
REPOSITORY_RENAME_STATUS=DEFERRED
V2_HISTORICAL_BOUNDARY=IMMUTABLE

## Completed and verified
G0_FOUNDATION_FREEZE=PASS
G0_FINAL_MAIN_SHA=`27c68ff509477d53e454a4f6f80b7eeffb8e7d15`

G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G1_CHECKPOINT_SHA=`6270f011ef293482063573bc5f97a2574b502f2a`

G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_CHECKPOINT_SHA=`59540f352f1f66f5dd07a79d52c0ab5a74015888`

G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_CHECKPOINT_SHA=`dcc5006f3feff0e431dac271ddd4fcab3d7e1f64`

G4_EXPORT_PROFILES=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G4_CHECKPOINT_SHA=`9a4acb9740829b64ea4b032e9995292a40b59786`
WhatsApp policy source of truth is now `src/export_profile.rs`; v2 constant names are compatibility aliases.
Built-ins: Universal MP4, WhatsApp, High Quality, Web Compatible. Custom profile architecture is validated.
Affected profile/media/xtask qualification PASS.

## Current phase — G5 Processing Engine Decoupling
Goal: processing consumes a prepared source path + ExportProfile, not acquisition provenance.

Required:
- ProcessingEngine is source-independent;
- MediaProbe informs remux/transcode choice;
- remux when safe and sufficient;
- transcode when required;
- preserve aspect ratio;
- no accidental upscale;
- size/duration/resolution constraints come from ExportProfile;
- bounded output verification and deterministic failure;
- current v2 UI continues to use a WhatsApp profile until G7.

G5 may refactor worker request to carry ExportProfile.
G5 must not implement the final v3 screen.

## Baselines
GUI_FRAMEWORK=iced
GUI_FRAMEWORK_BASELINE=0.14.0
RUST_BASELINE=1.98.1
CARGO_LOCK_REQUIRED=YES
BUILD_WITH_LOCKED=YES
OWNED_IMPLEMENTATION=100_PERCENT_RUST

## Continuity
Canonical journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Policies:
`RECOVER → RECONCILE → CLASSIFY → CONTINUE`
`PERSIST → VERIFY → CHECKPOINT → CONTINUE`
`TEST_THE_AFFECTED_SURFACE / REUSE_VALID_EVIDENCE / RERUN_ONLY_WHEN_INVALIDATED / NO_FAKE_PASS`

NEXT ACTION:
Implement and qualify G5 ProcessingEngine while preserving current v2 WhatsApp behavior.
