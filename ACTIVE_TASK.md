# ACTIVE TASK

## Task
KASPAPULSE_MEDIA_STUDIO_V3

## Status
IN_PROGRESS

## Current phase
G3 — MEDIA PROBE

## Authorization
Continuous owner authorization covers G0 → G9 for this task.
Force-push/history rewrite, rewriting v2 history, credential/DNS/unrelated infrastructure
changes, DRM circumvention, and unrelated repositories remain out of scope.

## Completed
G0_FOUNDATION_FREEZE=PASS
G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT

G1 checkpoint:
`6270f011ef293482063573bc5f97a2574b502f2a`

G2 checkpoint:
`59540f352f1f66f5dd07a79d52c0ab5a74015888`

## G3 objective
Implement a structured Rust MediaProbe using FFprobe JSON.
Do not use file-extension allowlists as the capability boundary.
Capture container/duration/stream/codec/dimension/frame-rate/pixel-format/orientation and
relevant color/HDR metadata where present.

The probe must:
- invoke FFprobe through structured Command arguments;
- preserve Windows no-console behavior;
- parse unknown future fields safely;
- fail closed on FFprobe failure or unusable metadata;
- remain independent of export-profile and UI concerns.

## Do not repeat
Do not repeat G0/G1/G2 qualification unless relevant predicates change.

## Next safe action
Implement MediaProbe and focused unit tests, then run affected-surface Rust qualification only.
