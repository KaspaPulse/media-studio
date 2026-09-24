# ACTIVE TASK

## Task
KASPAPULSE_MEDIA_STUDIO_V3

## Status
IN_PROGRESS

## Current phase
G5 — PROCESSING ENGINE DECOUPLING

## Completed
G0_FOUNDATION_FREEZE=PASS
G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G4_EXPORT_PROFILES=VERIFIED_SUCCESS_LOCAL_CHECKPOINT

G4 checkpoint:
`9a4acb9740829b64ea4b032e9995292a40b59786`

## G5 objective
Create a source-independent ProcessingEngine that accepts an acquired media path and ExportProfile.

Required decisions:
- use MediaProbe metadata before processing;
- remux compatible media when constraints already hold;
- transcode otherwise;
- use profile limits for size, segment duration, long edge, pixel format, codec/container;
- preserve aspect ratio and avoid upscaling;
- verify actual outputs after processing;
- keep the current v2 UI behavior by constructing a WhatsApp profile from its existing controls.

No G6/G7 final UI implementation in this phase.

## Do not repeat
Do not replay G0-G4 qualification while relevant predicates remain unchanged.

## Next safe action
Implement the bounded ProcessingEngine refactor and focused unit tests; then affected-surface qualification.
