# Repository adoption â€” interruption-safe continuity

Policy ID: UNIVERSAL_CONTINUITY_V2_20260913
Repository: KaspaPulse/whatsapp-video-preparer
Current task: WVP_RUST_STRICT_HARDENING_DELTA

## Binding and canonical state
The Rust v2 product transformation is already merged. The active lane is the
bounded strict-Rust hardening delta requested by the owner on 2026-09-23.

Canonical local operation journal:
C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md

Read AGENTS.md, PROJECT_STATE.md, ACTIVE_TASK.md, CURRENT_STATE.md, this file,
DURABLE_LOCAL_CHECKPOINTING.md, RESUME.md, and the latest relevant journal entry
before material work. Bind the real host/repository/worktree/branch/HEAD/dirty
state first.

## Recovery and evidence
Observable Git/filesystem/process/CI/runtime evidence wins over stale prose.
Recover undocumented progress before retrying. Preserve partial work and active
jobs. Reuse verified evidence only while its source/environment/artifact/policy
predicates hold.

The current hardening technical checkpoint is recorded in PROJECT_STATE.md.
Product-source evidence from Rust v2 remains reusable where product source did
not change. Renderer/binary evidence is separate because the iced feature graph
changed during security remediation.

## KSSS binding
KSSS authority remains the pinned v1.2.0 identity under .security/ksss/.
Local strengthening adds strict workflow ownership, dependency-policy, and
attestation controls without changing KSSS authority.

## Authorization boundary
The owner request authorizes this hardening task and its required local
qualification. GitHub push/PR/merge/release or other external mutation remains
subject to the actual current task authorization and repository protections;
this continuity document does not independently expand authority.

No destructive cleanup, production infrastructure mutation, credential change,
release, tag, deployment, or unrelated repository work is granted by this policy.
