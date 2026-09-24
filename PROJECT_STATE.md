# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `KASPAPULSE_MEDIA_STUDIO_V3`
Current phase: `G4 — EXPORT PROFILES`

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
G0_FOUNDATION_FREEZE=PASS
G0_FINAL_MAIN_SHA=`27c68ff509477d53e454a4f6f80b7eeffb8e7d15`
G0_FINAL_MAIN_TREE=`2428660026f840f73631b8d0b31da32157038014`

G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G1_CHECKPOINT_SHA=`6270f011ef293482063573bc5f97a2574b502f2a`

G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_CHECKPOINT_SHA=`59540f352f1f66f5dd07a79d52c0ab5a74015888`

G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_CHECKPOINT_SHA=`dcc5006f3feff0e431dac271ddd4fcab3d7e1f64`
G3_CHECKPOINT_TREE=`2f844c068cb586d82a748a5f33aaadeeb8bf4e95`
G3 evidence: MediaProbe tests 5/5 PASS, lib check PASS, clippy PASS, xtask verify PASS.

## Current phase — G4 Export Profiles

Goal: make WhatsApp a profile rather than generic engine policy.

Required architecture:
- Universal MP4;
- WhatsApp;
- High Quality;
- Web Compatible;
- Custom profile construction.

WhatsApp policy source of truth moves to the export-profile module while existing v2 public
constant names remain temporary compatibility aliases until G5 processing-engine decoupling.

Current validator caveat:
`xtask::verify_size_budget` currently parses WhatsApp constants from `src/media.rs`.
G4 must update the Rust validator to verify the new export-profile source of truth instead
of duplicating policy values.

No final processing-engine behavior change is required until G5.

## Baselines
GUI_FRAMEWORK=iced
GUI_FRAMEWORK_BASELINE=0.14.0
ICED_DEFAULT_FEATURES=DISABLED
ICED_RENDERER_BASELINE=tiny-skia
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

DO_NOT_REPEAT:
G0 exact-main CI and G1/G2/G3 local qualification while relevant predicates remain unchanged.

NEXT ACTION:
Implement and qualify G4 ExportProfile architecture and validator migration, then checkpoint.
