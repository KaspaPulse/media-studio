# Rust v2 local qualification checkpoint

Date: 2026-09-23
Task: WVP-RUST-KSSS-20260923
Status: LOCAL QUALIFICATION VERIFIED

## Qualified implementation identity

- Commit: `28fa0cb980df6468a8bfe32c5b42994bdffffde5`
- Tree: `462730225a5138732f533aa26265ddb37399b2aa`
- Branch: `wvp-rust-ksss-20260923`
- Base main observed before migration:
  `a605b3d30d6aed7304f9d9b617a31f86c00fb3da`
- Candidate version: 2.0.0
## Verified evidence

- Rust/KSSS repository gate: PASS.
- Final format check: PASS.
- Workspace Clippy with warnings denied: PASS.
- Workspace ordinary tests: 11 passed, 0 failed.
- Real high-motion byte-budget integration test: 1 passed, 0 failed.
- Locked release build: PASS.
- Windows final package SHA256SUMS: 4/4 PASS.
- Packaged Windows GUI smoke: PASS after 6 seconds.
- Final EXE SHA-256:
  `5be021c19a3efb69045b0d04a56fb8e0584bd05996c2be32dabe9a8fa8cca6a1`.

Durable command output is under `C:\WVP-Rust-Migration-20260923\`
and operation state is in `OPERATION_JOURNAL.md`.

## Post-merge reconciliation — 2026-09-23

PR #5 was squash-merged.

- Rust v2 implementation merge SHA: `feeb43230e688496888a843809333c41a119d35f`
- Rust v2 implementation merge tree: `9f00816972fb68ebb68971b4aeb863916b4fb21d`
- Current main SHA: verify dynamically after documentation-only reconciliation
- PR exact head: `0bc8c1b6956eeef7bea1987f4bd6fddaa09186f9`
- PR exact-head tree: `9f00816972fb68ebb68971b4aeb863916b4fb21d`
- Windows PR CI `35896904411`: SUCCESS
- macOS PR CI `35896904064`: SUCCESS on x64 and arm64
- GitHub languages after merge: Rust only
- forbidden tracked executable-code extensions: none

Exact-main push runs were created but their jobs never acquired a runner.
GitHub check annotations state exactly:
`The job was not started because an Actions budget is preventing further use.`

Therefore exact-main Actions and main-only attestations remain BLOCKED/NOT_RUN.
The code transformation itself is merged and the merged tree is identical to
the fully qualified PR tree. Do not weaken controls or replay valid local tests.
