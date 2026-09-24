# ACTIVE TASK

## Task
KASPAPULSE_MEDIA_STUDIO_V3

## Status
IN_PROGRESS

## Current phase
G7R-B — GITHUB REPOSITORY RENAME

## Completed
G0_FOUNDATION_FREEZE=PASS
G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G4_EXPORT_PROFILES=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G5_PROCESSING_ENGINE_DECOUPLING=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G6_UI_DESIGN_SYSTEM=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G7_V3_MAIN_SCREEN=VERIFIED_SUCCESS_LOCAL_CHECKPOINTS

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

## G7R-B objective
Perform only the authorized GitHub repository rename to `KaspaPulse/media-studio`, update the local origin remote, and verify redirects plus immutable v2 history before any v3 candidate push.

No v3 release tag/publication in this phase.

## Do not repeat
Do not replay G0-G7 qualification while relevant predicates remain unchanged.

## Next safe action
Fresh-read remote main/open PRs/repository identity, then execute the authorized GitHub rename and verify local origin, redirects, and v2.0.0 history before pushing the qualified v3 candidate.
