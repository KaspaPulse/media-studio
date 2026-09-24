# ACTIVE TASK

## Task
KASPAPULSE_MEDIA_STUDIO_V3

## Status
IN_PROGRESS

## Current phase
G6 — UI DESIGN SYSTEM

## Completed
G0_FOUNDATION_FREEZE=PASS
G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G4_EXPORT_PROFILES=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G5_PROCESSING_ENGINE_DECOUPLING=VERIFIED_SUCCESS_LOCAL_CHECKPOINT

G5 checkpoint:
`33e3d93c594081f0050496bcaff02de5f881af28`

## G6 objective
Create the reusable Rust-owned UI design-system foundation required by the frozen v3 contract while preserving the current v2 main screen until G7.

Required decisions:
- logical locale-aware layout primitives;
- UiDirection and MirrorPolicy foundations;
- semantic focus-order foundations;
- BiDi isolation helpers for mixed Arabic/technical content;
- design tokens and semantic component styles;
- System / Light / Dark theme preference architecture;
- testability hooks where supported by iced 0.14.0;
- no screen-reader support claim without native toolkit support and verification.

No G7 final main-screen implementation, repository rename, or release mutation in this phase.

## Do not repeat
Do not replay G0-G5 qualification while relevant predicates remain unchanged.

## Next safe action
Read the frozen UI contracts, implement the bounded G6 design-system foundation, and run affected-surface Rust/UI qualification only.
