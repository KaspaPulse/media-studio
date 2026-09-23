# PROJECT STATE

Updated: 2026-09-23
Repository: `KaspaPulse/whatsapp-video-preparer`
Default branch: `main`
Rust v2 PR: #5 — MERGED_SQUASH

## Merged main identity

- Main SHA: `feeb43230e688496888a843809333c41a119d35f`
- Main tree: `9f00816972fb68ebb68971b4aeb863916b4fb21d`
- PR exact head: `0bc8c1b6956eeef7bea1987f4bd6fddaa09186f9`
- PR exact-head tree: `9f00816972fb68ebb68971b4aeb863916b4fb21d`
- Merge method: squash
- Candidate/application version: `2.0.0`

The squash merge preserved the exact qualified candidate tree. The PR head and
merged main therefore differ by commit identity only, not repository tree bytes.

## Rust-only repository-owned implementation

The former Python application/tests, `pyproject.toml`, PowerShell build script,
and Bash build script are absent from merged `main`.

GitHub language accounting after merge reports only:

`Rust: 64492`

A recursive tracked-tree scan found no repository-owned executable code files
with Python, PowerShell, Bash/shell, JavaScript, or TypeScript extensions.

FFmpeg, FFprobe, and yt-dlp remain explicit external helper boundaries whose
release URLs and SHA-256 identities are pinned under `.security/ksss/`.

## WhatsApp byte-budget fix

The original defect was duration-only segmentation: a 29-second clip could still
exceed WhatsApp's 10 MB acceptance limit.

Rust v2 enforces:
- hard limit: 10,000,000 bytes;
- default target: 9,500,000 bytes;
- actual post-encode file measurement;
- bounded per-clip bitrate retry;
- fail-closed behavior if a clip cannot satisfy the target.

The real high-motion regression test exercises the production segmentation path.

## Verified evidence

Local Windows qualification before publication:
- repository/KSSS gate PASS;
- format PASS;
- workspace Clippy with `-D warnings` PASS;
- workspace unit tests: 11 passed, 0 failed;
- real high-motion size-budget test: 1 passed, 0 failed;
- locked release build PASS;
- packaged Windows `SHA256SUMS`: 4/4 PASS;
- packaged Windows GUI startup smoke PASS.

PR exact-head GitHub CI on `0bc8c1b...`:
- Rust Windows run `35896904411`: SUCCESS;
- Rust macOS run `35896904064`: SUCCESS;
- macOS x64: SUCCESS;
- macOS arm64: SUCCESS.

Those PR runs validated KSSS, format, Clippy, tests, release builds, pinned-helper
packaging, real size-budget regression, native GUI startup smoke, SPDX SBOM, and
artifact upload on their applicable platforms.

## KSSS binding

Pinned KSSS identity:
- release: `v1.2.0`;
- source SHA: `967ed5068947a39961d5d5cc483ef65d25a61059`;
- policy bundle: `3c1c8b449d736aba5fec5cffd688496b06d81f287f55fff4c3f42e36c3b58ec6`;
- Consumer Runtime SHA-256: `38309d2ab8fa30096d99940f855e88173faa182e60db33f2a96b2d3408507430`;
- runtime sequence: 2;
- trust root: `ksss-trust-root-1`.

## Exact-main Actions blocker

Automatic push-to-main runs were created for the merged SHA but no runner ever
started:

- Windows run `35900279813`: failure before job start;
- macOS run `35900280102`: both jobs failed before job start.

For all three jobs GitHub reports:
- `runner_id=0`;
- `steps=[]`;
- annotation: `The job was not started because an Actions budget is preventing further use.`

Classification:
- merged Rust v2 repository state: VERIFIED_SUCCESS;
- exact-main product rerun: BLOCKED_BY_GITHUB_ACTIONS_BUDGET before execution;
- main-only provenance/SBOM attestations: NOT_RUN / BLOCKED_BY_GITHUB_ACTIONS_BUDGET;
- release/tag/deployment: NOT_STARTED.

This blocker is external to repository code. Do not weaken workflows or remove
attestation gates to hide it.

## Continuity

Canonical local operation journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Evidence reuse remains valid because the merged main tree exactly equals the
successful PR exact-head tree. A commit-ID change alone does not invalidate
tree-bound product evidence.

NEXT ACTION: after the GitHub Actions budget is resolved, rerun the existing
exact-main Windows and macOS workflows once for the then-current documentation-only
main state, verify main-only provenance/SBOM attestations, and close the blocker.
Do not rebuild or retest locally unless a relevant validity predicate changes.
