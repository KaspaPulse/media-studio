# Repository adoption — Media Studio v3 transition

Policy ID: UNIVERSAL_CONTINUITY_V2_20260913

Current GitHub repository:
`KaspaPulse/media-studio`

Historical repository URL (redirect verified):
`KaspaPulse/whatsapp-video-preparer`

Current task:
`KASPAPULSE_MEDIA_STUDIO_V3`

## Binding and canonical state

The published WhatsApp Video Preparer v2.0.0 history is immutable and remains valid historical
release evidence. The active lane is the authorized Media Studio v3 migration.

Canonical local operation journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Active implementation worktree:
`C:\WVP-Media-Studio-V3-G1-20260924`

Always reconcile AGENTS.md, PROJECT_STATE.md, ACTIVE_TASK.md, CURRENT_STATE.md, the continuity
policy, this file, and the latest journal receipt with actual Git/filesystem/process/CI state
before material work.

## Recovery and evidence

Observable Git/filesystem/process/CI/runtime evidence wins over stale prose. Recover undocumented
progress before retrying. Preserve partial work and active jobs. Reuse verified evidence only
while its source/environment/artifact/policy validity predicates hold.

The v3 implementation is developed and qualified locally first. The GitHub repository rename to
`KaspaPulse/media-studio` completed during G7R-B with repository ID/history preserved. GitHub is
used for candidate integration only after a coherent local candidate is qualified.

## KSSS binding

KSSS authority remains the pinned v1.2.0 identity under `.security/ksss/`. Repository identity
fields track the v3 target `KaspaPulse/media-studio`; KSSS release, trust root, runtime identity,
control IDs, and helper supply-chain identities remain unchanged unless separately invalidated.

## Historical boundary

Do not rewrite, delete, rename, or replace the published v2.0.0 tag, release assets, provenance,
SBOMs, or historical design/handoff records.

## Authorization boundary

The owner has authorized the complete KaspaPulse Media Studio 3.0 task, including qualified
branches, pushes, PRs, merges, the GitHub repository rename to `KaspaPulse/media-studio`, and the
v3.0.0 tag/release after their required gates pass.

Force pushes, destructive history rewrites, credential/secret changes, DNS/infrastructure
mutations, DRM circumvention, and unrelated repositories remain outside authorization.
