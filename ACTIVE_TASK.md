# ACTIVE TASK

## Task
KASPAPULSE_MEDIA_STUDIO_V3

## Status
IN_PROGRESS

## Current phase
G8 — NATIVE QUALIFICATION

## Completed
G0_FOUNDATION_FREEZE=PASS
G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G4_EXPORT_PROFILES=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G5_PROCESSING_ENGINE_DECOUPLING=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G6_UI_DESIGN_SYSTEM=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G7_V3_MAIN_SCREEN=VERIFIED_SUCCESS_LOCAL_CHECKPOINTS
G7_NATIVE_INTERACTION_QUALIFICATION=VERIFIED_SUCCESS_WINDOWS_NATIVE

G7C checkpoint:
`40a835142e0786c485b7926e5fa57ef0f52beba8`

## G7R objective
Migrate identity from WhatsApp Video Preparer / KaspaPulse/whatsapp-video-preparer to KaspaPulse Media Studio / KaspaPulse/media-studio without rewriting v2 history.

Required decisions:
- inventory all repository/product/package/bundle/settings/workflow/release identity surfaces;
- preserve v2.0.0 tag/release/assets unchanged;
- provide settings identity migration so existing users retain preferences;
- update future v3 artifact and workflow metadata only;
- locally qualify the identity delta before GitHub repository rename;
- after authorized rename, verify redirects and historical release access.

G7R-A local identity migration is qualified at `dbcab7807540feb6a70fd6106dca261d3d237bfc` / tree `7b8ea8fcd5aa0127a9c026f04ec7adc94656b2bf`.

G7R_B_GITHUB_REPOSITORY_RENAME=VERIFIED_SUCCESS
Repository ID `1362998218` is preserved at `KaspaPulse/media-studio`; old repository URLs redirect and v2.0.0 history remains intact.

## G8 objective
Qualify the complete Media Studio v3 candidate locally wherever reliable, then use one qualified push for required GitHub/macOS validation.

Required:
- Windows native candidate and package identity;
- local/remote acquisition integration;
- representative media formats;
- remux/transcode/profile/WhatsApp budget evidence;
- responsive RTL/LTR/theme/keyboard interaction evidence;
- exact-head Windows/macOS/Rust Policy remote CI after local qualification;
- exact-main SBOM/provenance after merge.

No v3 release tag/publication until G8 closes.

## G8 local qualification
LOCAL_SUCCESS=YES
LOCAL_CANDIDATE=QUALIFIED
KNOWN_LOCAL_BLOCKERS=NONE
LOCAL_REPAIR_HEAD=`679a2a3a5ba046e6cbc734a3e6caf828f11ae0ae`
LOCAL_REPAIR_TREE=`ee432c82c0c1512974e673709d7829319319e7ad`

Verified/reused locally on Windows: Rust/KSSS/cargo-deny/Clippy/fmt gates; exact-code release/package/GUI smoke; LocalFile read-only; packaged yt-dlp RemoteUrl loopback; MP4/MKV/MOV/WebM; remux/transcode; WhatsApp size-budget evidence; Compact/Standard/Wide RTL UI; System/Light/Dark themes; Tab/ShiftTab/Space/Enter/Escape semantic focus. The failed GitHub RemoteUrl loopback harness was repaired in test-only code and the targeted native test, targeted Clippy, repository gate, fmt, and diff-check now PASS locally. Existing package evidence remains valid because application/package bytes did not change.

PR #10 first exact-head attempt: Rust Policy PASS; macOS arm64 PASS; Windows FAILED only in RemoteUrl loopback harness; macOS x64 FAILED only at DMG creation with `hdiutil: Resource busy`.

REMOTE_VALIDATION_REQUIRED=YES
Required remotely on the repaired exact head: automatic Rust Policy, Windows workflow, macOS x64/arm64 build/package/native harness; after merge, exact-main provenance and SPDX SBOM attestations.

## Do not repeat
Do not replay G0-G7 qualification while relevant predicates remain unchanged.

## Next safe action
Push the single repaired and locally qualified commit to the existing PR #10 branch once. Observe only the automatically triggered exact-head Windows/macOS/Rust Policy validation; do not manually rerun the old failed workflows and do not merge until every required exact-head gate passes.
