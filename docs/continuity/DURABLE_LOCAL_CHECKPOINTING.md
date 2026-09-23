# Durable local checkpointing

Policy ID: WVP_DURABLE_LOCAL_CHECKPOINTING_V1_20260923
Status: mandatory repository operating policy.

## Core rule

Progress must remain resumable from disk at all times. Chat memory, terminal
scrollback, and an unfinished patch are not durable state. After every meaningful
state transition, persist the result, verify it, checkpoint it, and continue.

Canonical sequence:

`PERSIST -> VERIFY -> CHECKPOINT -> CONTINUE`

A new session starts with:

`RECOVER -> RECONCILE -> CLASSIFY -> CONTINUE`

Never use `RESTART -> REBUILD -> RETEST` merely because a session ended.
## Recovery before mutation

Before changing state, bind the exact repository, host, root, worktree, branch,
HEAD, tree, index, staged/unstaged/untracked work, remotes when relevant, active
jobs/processes, logs, artifacts, test receipts, runtime evidence, and CI evidence.

Read applicable AGENTS files, PROJECT_STATE.md, ACTIVE_TASK.md, CURRENT_STATE.md,
active plans, continuity policy, RESUME, the operation journal/ledger, and the
latest relevant handoff/checkpoint.

If durable prose conflicts with Git, filesystem, process, log, CI, or runtime
evidence, observable reality wins. Reconcile the durable record before relying on it.

Classify every uncertain operation as exactly one of:

`NEVER_STARTED | RUNNING | VERIFIED_SUCCESS | FAILED | PARTIAL | BLOCKED | UNKNOWN`
## Required behavior by classification

- NEVER_STARTED: start only if it remains the first correct unfinished action.
- RUNNING: reattach/read the existing job and its log; do not create a duplicate.
- VERIFIED_SUCCESS: reuse the evidence while its validity predicates still hold.
- FAILED: preserve evidence, diagnose the cause, repair the smallest affected surface.
- PARTIAL: preserve completed output and resume at the first unfinished boundary.
- BLOCKED: record the blocker and continue independent safe authorized work.
- UNKNOWN: verify side effects before retry; absence of a receipt is not failure.

An interruption alone never authorizes reset, clean, stash, discard, restore,
revert, overwrite, reclone, worktree recreation, untracked-file deletion,
automatic pull/merge/rebase, or replay of build/test/qualification/deploy/release.
## Save-progress-as-you-go

Write stable source changes to disk as soon as each coherent unit is ready.
Do not accumulate material implementation only in conversation or scratch space.

Checkpoint immediately after meaningful events such as:

- a material discovery or root cause;
- a coherent file-change set;
- a passed or failed meaningful validation;
- a completed build or artifact;
- a commit or material HEAD/branch transition;
- the start or completion of a long operation;
- an authorized external mutation;
- a blocker change or NEXT_SAFE_ACTION change.

Do not defer all documentation until task completion.
## Write-ahead operation records

Before every materially state-changing operation, append a durable record containing:

- OPERATION_ID and TASK_ID;
- UTC time, host, repository/root/worktree;
- starting branch/HEAD and relevant dirty/index fingerprints;
- intent, bounded command description, inputs, and authorization scope;
- expected result and verification method;
- prohibited actions and safest recovery after interruption.

When a long operation starts, persist its PID/job/session/log/artifact identity as
soon as known. After it returns, independently verify effects and append actual
result, classification, evidence identity/hash, post-state, validity predicates,
and NEXT_SAFE_ACTION.
## Evidence reuse

Apply all three rules:

- `TEST_THE_AFFECTED_SURFACE`
- `REUSE_VALID_EVIDENCE`
- `RERUN_ONLY_WHEN_INVALIDATED`

Do not invalidate a PASS solely because a message/session ended, documentation
changed, a new commit has the same relevant tree, or the task advanced phases.
Identify the exact changed validity predicate and rerun only the affected evidence.

Never convert NOT_VERIFIED to PASS. Absence of an error is not proof of success.
Absence of a process is not proof of failure without checking durable evidence.
## Local work protection and commits

Preserve staged, unstaged, untracked, partial, generated, failed-but-diagnostic,
receipt, artifact, test, build, runtime, and historical checkpoint data.

A dirty worktree is evidence to understand, not a cleanup request.

When a coherent change is sufficiently verified and local commits are authorized,
prefer a small local checkpoint commit rather than leaving a large verified change
uncommitted indefinitely. Do not stage unrelated work. A commit supplements rather
than replaces the operation journal and checkpoint.
## Long-running work and handoff

Long builds, tests, qualification, packaging, scans, or release operations must
write durable logs/output identities rather than depend on terminal scrollback.
After interruption, inspect the existing job before launching another.

A fresh session must be able to reconstruct from Git + filesystem + checkpoints +
operation journal + logs + receipts + artifacts:

task and authorization boundary; repository/worktree; branch/HEAD; local changes;
completed/verified work; active/partial/failed/unknown work; reusable evidence;
blockers; DO_NOT_REPEAT; and the exact NEXT_SAFE_ACTION.

Before claiming completion, reread the durable state and prove it is self-sufficient.
