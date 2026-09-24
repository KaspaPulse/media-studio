# ACTIVE TASK

## Task

KASPAPULSE_MEDIA_STUDIO_V3

## Phase

G0 — KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION_FREEZE

## Status

IN_PROGRESS

## Authorization

The owner has granted continuous authorization for the v3 task through G0 → G9, including
branches, source/tests/docs/governance edits, required Cargo changes, commits, push, PRs,
CI, merges after gates, repository rename to `KaspaPulse/media-studio`, v3.0.0 tag and
GitHub Release, release assets, checksums, SBOM/provenance verification, and durable
state/checkpoints.

Out of scope:
- force push/history rewrite;
- deletion or rewriting of v2 tags/releases/assets;
- credential rotation;
- DNS changes;
- unrelated production infrastructure;
- DRM circumvention;
- unrelated repositories.

## Source baseline

SOURCE_BASE_SHA:
`2440690ecc1845b79de47550e672d767edbb9393`

SOURCE_BASE_TREE:
`b712b1da64b5de3739cc087292d1cb427621a931`

CURRENT_PRODUCT:
WhatsApp Video Preparer 2.0.0

TARGET_PRODUCT:
KaspaPulse Media Studio 3.0.0

TARGET_REPOSITORY:
`KaspaPulse/media-studio`

## G0 objective

Freeze a versioned documentation-only Foundation before any v3 implementation.

Canonical document:
`docs/foundation/KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION.md`

Required G0 evidence:
- Foundation document complete/versioned;
- nine pillars defined;
- architectural invariants defined;
- required-vs-verified semantics defined;
- v2 history preserved;
- implementation source diff = 0;
- dependency diff = 0;
- workflow diff = 0;
- v3 implementation remains NOT_STARTED.

## Do not repeat

Do not rerun v2 Rust migration, build, packaging, GUI smoke, SBOM, provenance, or release
qualification merely because v3 G0 exists. Reuse v2 evidence as historical evidence while
its relevant predicates remain unchanged.

## Next safe action

Verify the G0 documentation diff, commit only the four allowed paths, push and open the G0
PR, verify exact-head required checks, merge without drift, verify exact-main, then continue
to G1 automatically.
