# AGENTS.md

## Startup protocol
Before making any change, read `AGENTS.md`, `AGENTS.override.md`, and `PROJECT_STATE.md` in full.
Then verify the real repository state: current branch, HEAD, working tree, upstream, and latest commit.
If recorded state differs from reality, reality wins and the discrepancy must be reconciled before continuing.

## Source of truth order
1. Actual repository state.
2. `PROJECT_STATE.md` after reconciliation with reality.
3. Relevant technical documentation, ADRs, and runbooks.
4. Prior conversation context only as supporting context.

## Continuity rules
Continue from the `NEXT ACTION` in `PROJECT_STATE.md` unless the user explicitly changes priority.
Do not repeat completed work, broad audits, builds, or tests unless evidence shows they are incomplete, failed, regressed, or stale.
Prefer targeted verification over general rescanning when the next action is clear.
Read an ADR or runbook only when it is materially relevant.
Read `PLANS.md` only when it contains a multi-stage plan marked `ACTIVE`.

## Project invariants
The Windows application must launch as a GUI without a visible console window.
Arabic UI uses RTL layout; English UI uses LTR layout.
URL and filesystem path inputs remain LTR in both languages.
The media flow keeps the original source, prepares MP4 H.264/AAC output, defaults to 29-second clips, and supports user-selected segment duration.
Do not claim interactive Windows GUI validation unless the executable was actually run on Windows.

## Interruption-safe continuity — owner baseline v2

Read and apply [the interruption-safe execution supplement](docs/continuity/INTERRUPTION_SAFE_EXECUTION.md), subordinate to this repository contract.
Use [repository adoption](docs/continuity/REPOSITORY_ADOPTION.md), [resume procedure](docs/continuity/RESUME.md) and [checkpoint template](docs/continuity/CHECKPOINT_TEMPLATE.md) with the existing canonical ledger.
Persist material-operation intent before execution; verify and record outcomes immediately afterward. Recover unknown outcomes before retrying.
Preserve local work, active task ownership, project-specific hosts, authorization limits and valid source-bound evidence.
Validate NEXT ACTION, then continue authorized work rather than merely summarize. This policy grants no push, release or production authority.

## Rust-only repository-owned implementation

All repository-owned executable application code, automated tests, and build/governance
tooling must be Rust. Shell and Python source files are forbidden. YAML, JSON, TOML,
Markdown, and binary assets may be used only as configuration, policy, metadata, or assets.

FFmpeg, FFprobe, and yt-dlp are external helper boundaries, not repository-owned code.
Invoke them only with structured process APIs and argument arrays; never construct shell
command strings from user input. Pin and verify helper identities for distributable artifacts.

## Strict workflow ownership

Workflow YAML is declarative orchestration, not a second implementation language.
Repository-owned build, packaging, validation, policy, and test logic belongs in Rust
(primarily `xtask`). Simple `cargo`, `rustup`, and `rustc` invocations are
allowed in workflow `run` steps; inline PowerShell/Bash business logic, explicit script
shells, and repository-owned .ps1/.sh/.py execution are forbidden. External Actions must
use full 40-character commit SHAs. Dependency-policy and attestation identities are pinned
and validated by the repository/KSSS gate.

## WhatsApp byte-budget invariant

The user-selected duration is a maximum, not permission to exceed the byte limit.
Every generated MP4 must be measured after encoding. The default target is 9,500,000 bytes,
strictly below the 10,000,000-byte hard limit. Oversized output must be re-encoded within
a bounded retry policy or fail closed; never return an oversized clip as success.

## KSSS v1.2.0 consumer boundary

Read .security/ksss/README.md and use the pinned KSSS v1.2.0 identity there.
Risk classification and audit profile are separate; a profile cannot weaken the risk floor.
AI suspicion alone never blocks. Evidence reuse is allowed only while its validity predicates
hold. KSSS adoption PASS is not application, artifact, release, or deployment PASS.

## Durable local checkpointing

Read docs/continuity/DURABLE_LOCAL_CHECKPOINTING.md for every new/resumed engineering session.
After each meaningful state transition, persist and verify durable state before continuing.
Do not depend on conversation memory or terminal scrollback for recovery.
