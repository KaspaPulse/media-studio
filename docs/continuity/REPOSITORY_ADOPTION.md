# Repository adoption — interruption-safe continuity

Policy ID: UNIVERSAL_CONTINUITY_V2_20260913
Repository: KaspaPulse/whatsapp-video-preparer
Current task: WVP-RUST-KSSS-20260923

## Binding and canonical state

The active lane is the Rust v2 migration, WhatsApp byte-budget fix, and KSSS v1.2.0
adoption requested by the repository owner on 2026-09-23.

Canonical local operation journal for this lane:

C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md

Read AGENTS.md, PROJECT_STATE.md, ACTIVE_TASK.md, CURRENT_STATE.md, this file,
DURABLE_LOCAL_CHECKPOINTING.md, RESUME.md, and the latest relevant journal entry before
material work. Bind the real host/repository/worktree/branch/HEAD/dirty state first.

## Recovery and evidence

Observable Git/filesystem/process/CI/runtime evidence wins over stale prose.
Recover undocumented progress before retrying. Preserve partial work and active jobs.
Reuse verified evidence only while its source/environment/artifact/policy predicates hold.
Do not repeat product tests solely because a session or message ended.

## KSSS binding

KSSS authority is the pinned v1.2.0 identity in .security/ksss/.
The repository does not vendor the Python Consumer Runtime; its owned execution/tooling
surface remains Rust-only. Adoption metadata does not certify application or release health.

## Authorization boundary

The current owner request authorizes this repository transformation and its required local
qualification. GitHub push/PR/merge/release or other external mutation must remain within
the owner's actual current task scope and repository protections; this continuity document
does not independently expand authority.

No destructive cleanup, production infrastructure mutation, or unrelated repository work
is granted by this policy.
