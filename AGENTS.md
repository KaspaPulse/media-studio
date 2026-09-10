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
