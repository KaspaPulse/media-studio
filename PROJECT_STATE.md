# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `KASPAPULSE_MEDIA_STUDIO_V3`
Current phase: `G7 — V3 MAIN SCREEN`

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

G5_PROCESSING_ENGINE_DECOUPLING=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G5_CHECKPOINT_SHA=`33e3d93c594081f0050496bcaff02de5f881af28`
Processing now consumes an acquired media path plus ExportProfile and is independent of acquisition provenance.
MediaProbe drives remux/transcode decisions; compatible general media can remux, incompatible media transcodes, and size-constrained profiles keep bounded post-encode verification.
Current v2 UI behavior is preserved by constructing a validated WhatsApp profile.
Affected processing/media integration qualification PASS under OP-0099.

G6_UI_DESIGN_SYSTEM=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G6_CHECKPOINT_SHA=`6ab0f026a270f26e361b3b649b1321b00e74db53`
Rust-owned UI foundations now provide logical direction/mirroring, BiDi isolation, semantic focus order, caller-tuned responsive layout classes, System/Light/Dark theme preference, and semantic design tokens.
UI foundation tests: 13/13 PASS; lib check/Clippy and repository policy gate PASS under OP-0103/OP-0104.

## Current phase — G7 V3 Main Screen
Goal: replace the WhatsApp-only v2 screen with the general KaspaPulse Media Studio v3 UX while consuming the completed G1-G6 foundations.

Required:
- LocalFile and RemoteUrl source selection with non-drag file picker always available;
- optional drag-and-drop source selection;
- structured MediaProbe information surfaced before processing;
- ExportProfile selection and output controls;
- responsive Compact/Standard/Wide layout;
- true application-owned RTL/LTR visual ordering with technical fields kept LTR;
- System/Light/Dark theme wiring;
- semantic focus/keyboard behavior and stable testable identities where supported;
- progress/results/error states driven by AppViewState semantics;
- no broad screen-reader accessibility claim.

G7 must not rename the repository, rewrite v2 history, or publish v3.

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
Inspect iced 0.14.0 APIs used by the frozen contract, then implement G7 in bounded local slices: source/profile/state wiring, responsive RTL/theme layout, and keyboard/testability qualification.
