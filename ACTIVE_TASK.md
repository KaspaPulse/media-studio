# ACTIVE TASK

## Task
KASPAPULSE_MEDIA_STUDIO_V3

## Status
IN_PROGRESS

## Current phase
G7 — V3 MAIN SCREEN

## Completed
G0_FOUNDATION_FREEZE=PASS
G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G4_EXPORT_PROFILES=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G5_PROCESSING_ENGINE_DECOUPLING=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G6_UI_DESIGN_SYSTEM=VERIFIED_SUCCESS_LOCAL_CHECKPOINT

G6 checkpoint:
`6ab0f026a270f26e361b3b649b1321b00e74db53`

## G7 objective
Build the general KaspaPulse Media Studio v3 main screen on the completed domain/acquisition/probe/profile/processing/UI foundations.

Required decisions:
- LocalFile and RemoteUrl source UX;
- non-drag file picker plus optional drag/drop;
- source metadata presentation from MediaProbe;
- ExportProfile picker and output controls;
- AppViewState-driven progress/results/error UX;
- responsive Compact/Standard/Wide composition;
- application-owned RTL/LTR ordering with technical values LTR;
- System/Light/Dark theme wiring;
- semantic keyboard/focus behavior and deterministic UI identities where supported.

No repository rename, v2 history rewrite, release tag, or publication in this phase.

## Do not repeat
Do not replay G0-G6 qualification while relevant predicates remain unchanged.

## Next safe action
Inspect the exact iced 0.14.0 APIs, then implement G7 in bounded local slices and qualify only the affected UI/integration surfaces before any remote push.
