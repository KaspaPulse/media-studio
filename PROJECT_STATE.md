# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `KASPAPULSE_MEDIA_STUDIO_V3`
Current phase: `G0 — KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION_FREEZE`

## Actual source baseline

- Source base SHA: `2440690ecc1845b79de47550e672d767edbb9393`
- Source base tree: `b712b1da64b5de3739cc087292d1cb427621a931`
- Current product: WhatsApp Video Preparer
- Current version: 2.0.0
- Target product: KaspaPulse Media Studio
- Target version: 3.0.0
- Target repository: `KaspaPulse/media-studio`
- Repository rename status: DEFERRED
- V3 implementation: NOT_STARTED

## Historical v2 boundary

v2 is immutable historical evidence:
- v2.0.0 tag: PRESERVE;
- v2.0.0 GitHub Release: PUBLISHED / PRESERVE;
- v2 release assets: DO_NOT_RENAME_OR_REPLACE;
- v2 provenance/SBOM: HISTORICAL_EVIDENCE.

Do not rewrite, delete, retag, or replace v2 history while implementing v3.

## v3 Foundation

Canonical Foundation:
`docs/foundation/KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION.md`

Nine pillars:
1. PRODUCT_IDENTITY
2. INPUT_SOURCE_ARCHITECTURE
3. MEDIA_PROBE
4. PROCESSING_ENGINE
5. EXPORT_PROFILE
6. UI_FOUNDATION
7. INTERNATIONALIZATION
8. INTERACTION_AND_ACCESSIBILITY
9. UI_TESTABILITY

Foundation baseline:
- GUI framework: iced 0.14.0;
- iced default features: disabled;
- renderer: tiny-skia;
- Rust: 1.98.1;
- Cargo.lock required;
- qualified builds use --locked;
- repository-owned implementation remains 100% Rust.

RTL layout is application-owned until an upstream capability is released and independently
qualified. Reference: iced-rs/iced#3303. The application must not depend on that unmerged PR.

Screen-reader accessibility is not claimed until toolkit support and native verification.
WCAG 2.2 is a design reference; WCAG2ICT 2.2 is non-web-software guidance. No formal
accessibility conformance claim is made by the Foundation.

## G0 scope

G0 is documentation-only.

Allowed:
- `docs/foundation/KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION.md`
- `PROJECT_STATE.md`
- `ACTIVE_TASK.md`
- `CURRENT_STATE.md`

Forbidden in G0:
- `src/**`;
- `tests/**`;
- Cargo.toml/Cargo.lock mutation;
- workflow implementation mutation;
- repository/binary/bundle rename;
- InputSource implementation;
- UI implementation.

G0 success requires zero implementation, dependency, and workflow diff from the source base.

## Continuity

Canonical operation journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Execution policy:
`RECOVER → RECONCILE → CLASSIFY → CONTINUE`

After meaningful state transitions:
`PERSIST → VERIFY → CHECKPOINT → CONTINUE`

Evidence policy:
- TEST_THE_AFFECTED_SURFACE
- REUSE_VALID_EVIDENCE
- RERUN_ONLY_WHEN_INVALIDATED
- NO_FAKE_PASS

## Current classification

G0 Foundation documentation: IN_PROGRESS
V3 implementation: NOT_STARTED
Repository rename: DEFERRED
v3.0.0 release: NOT_STARTED

NEXT ACTION:
Finish the G0 documentation-only delta, verify the exact allowed-path diff, commit/push/PR,
wait for required exact-head checks, merge with no drift, verify exact-main, then proceed
automatically to G1 under the owner authorization.
