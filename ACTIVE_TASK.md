# ACTIVE TASK

## Task
KASPAPULSE_MEDIA_STUDIO_V3

## Status
IN_PROGRESS

## Current phase
G4 — EXPORT PROFILES

## Completed
G0_FOUNDATION_FREEZE=PASS
G1_PRODUCT_DOMAIN_MODEL=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G2_SOURCE_ACQUISITION=VERIFIED_SUCCESS_LOCAL_CHECKPOINT
G3_MEDIA_PROBE=VERIFIED_SUCCESS_LOCAL_CHECKPOINT

G3 checkpoint:
`dcc5006f3feff0e431dac271ddd4fcab3d7e1f64`

## G4 objective
Introduce a Rust ExportProfile model with built-in Universal MP4, WhatsApp, High Quality,
and Web Compatible presets plus a validated Custom architecture.

Move WhatsApp size/duration/resolution source-of-truth policy out of generic media policy.
Keep the old v2 public constant names only as temporary compatibility aliases until G5.

Update repository-owned Rust validation so the WhatsApp size policy is validated at its new
source of truth. Do not weaken the 10,000,000-byte hard limit or 9,500,000-byte target.

## Constraints
No dependency/workflow mutation.
No final UI work.
No repository rename.
No G5 processing behavior change beyond compatibility aliases.

## Next safe action
Implement export_profile.rs, compatibility aliases, and the smallest xtask validator migration;
then run affected Rust/profile/validator qualification.
