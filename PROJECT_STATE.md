# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `KASPAPULSE_MEDIA_STUDIO_V3`
Current phase: `G7R-B — GITHUB REPOSITORY RENAME`

## Product boundary
CURRENT_PRODUCT=WhatsApp Video Preparer
CURRENT_VERSION=2.0.0
TARGET_PRODUCT=KaspaPulse Media Studio
TARGET_VERSION=3.0.0
TARGET_REPOSITORY=KaspaPulse/media-studio
REPOSITORY_RENAME_STATUS=READY_FOR_AUTHORIZED_GITHUB_RENAME
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

G7_V3_MAIN_SCREEN=VERIFIED_SUCCESS_LOCAL_CHECKPOINTS
G7A_CHECKPOINT_SHA=`083da022e376174725b7420568d05fbdd4160358`
G7B_CHECKPOINT_SHA=`6615a5021f670eda39fdf1a193c0f685a9ae743e`
G7C_CHECKPOINT_SHA=`40a835142e0786c485b7926e5fa57ef0f52beba8`
G7 now supports LocalFile and RemoteUrl UX, optional drag/drop with required file picker fallback, MediaProbe metadata display, ExportProfile selection, AppViewState-driven progress, responsive Compact/Standard/Wide layout, application-owned RTL/LTR visual ordering, System/Light/Dark theme preference, and keyboard/focus/testability wiring.
Static and focused local qualification PASS; native keyboard/focus interaction remains NOT_VERIFIED until G8.

## Current phase — G7R Identity Migration / Repository Rename
Goal: migrate product/repository identity to KaspaPulse Media Studio / KaspaPulse/media-studio while preserving v2 history and user settings continuity.

Required:
- inventory every current v2 identity surface before mutation;
- update repository/product/package/application metadata coherently;
- preserve v2.0.0 tag/release/assets/history immutably;
- migrate settings identity without losing existing user preferences;
- update workflow/release artifact naming for future v3 only;
- perform GitHub repository rename only after local identity delta is qualified;
- verify old repository URL redirects and historical release access after rename.

G7R_A_LOCAL_IDENTITY_MIGRATION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G7R_A_CHECKPOINT_SHA=`dbcab7807540feb6a70fd6106dca261d3d237bfc`
G7R_A_CHECKPOINT_TREE=`7b8ea8fcd5aa0127a9c026f04ec7adc94656b2bf`
Windows v3 release/package identity and packaged GUI smoke PASS; macOS native package identity remains REMOTE_VALIDATION_REQUIRED.

## Current phase — G7R-B GitHub Repository Rename
Goal: rename the GitHub repository from `KaspaPulse/whatsapp-video-preparer` to `KaspaPulse/media-studio`, update the local origin URL, and verify redirects/history without changing source bytes.

Required:
- fresh-read remote main and open PRs before rename;
- rename only if remote main remains the G0-qualified baseline and no conflicting PR exists;
- update local origin to the new canonical URL after GitHub confirms rename;
- verify old repository URL redirects;
- verify v2.0.0 tag/release/assets remain reachable and unchanged;
- do not push the 17-commit v3 candidate until rename verification completes.

G7R-B must not publish v3.0.0 or rewrite historical v2 artifacts.

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
Fresh-read GitHub repository/main/open PRs, then perform the authorized repository rename to KaspaPulse/media-studio, update local origin, and verify redirects/v2 history before the first v3 push.
