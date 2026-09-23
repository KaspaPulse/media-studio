# ACTIVE TASK

## Task
WVP_RUST_STRICT_HARDENING_DELTA

## Status
LOCAL HARDENING QUALIFIED â€” DURABLE STATE COMMITTED

## Objective
Strengthen the already-merged Rust v2 repository so repository-owned application,
test, build, and validation logic remains Rust while GitHub workflow YAML is
declarative orchestration only.

Do not redo the Rust v2 transformation or restore retired Python/PowerShell/shell
source files.

## Current identities
- Remote main observed before hardening: `5edda96a0a23b486ffbaa73b9c55ff46b6510418`
- Rust v2 implementation merge: `feeb43230e688496888a843809333c41a119d35f`
- Hardening branch: `feat/rust-strict-hardening-20260923`
- Technical checkpoint: `db6685287424f9c27f434473609356523687c1d8`
- Technical checkpoint tree: `7b168eb1fa2d5ec4848516bcea5c3ca55969ae00`
- Application version: `2.0.0`

## Completed and verified locally
- Strict Rust-owned workflow validator: PASS.
- Workflow inline PowerShell environment/path logic removed.
- External GitHub Actions remain full-40-character-SHA pinned.
- `actions/attest` v4.2.2 is pinned to `1e69f48acb82d1966a394da916b4c1698aa569d6`.
- `cargo-deny` 0.20.2 policy covers advisories, licenses, bans, and sources.
- No advisory IDs are ignored.
- `RUSTSEC-2026-0253` dependency path was removed by selecting iced 0.14.0 tiny-skia instead of the default wgpu renderer.
- Locked graph contains no `lru`, `cryoglyph`, or `iced_wgpu`; `iced_tiny_skia 0.14.1` is present.
- Cargo.lock remediation is pruning-only: 0 new package identities, 105 removed.
- Windows locked check, Clippy, workspace tests, release build, package hashes, real size-budget test, and native GUI smoke all PASS.
- Product `src/*.rs`, tests, and `build.rs` were not changed by this delta.

## Not yet verified
- Hardening branch publication: NOT_STARTED.
- Hardening PR: NOT_STARTED.
- New exact-head Windows CI: NOT_RUN.
- New exact-head macOS arm64/x64 CI: NOT_RUN.
- Main-only v4 provenance/SBOM attestations: NOT_RUN.
- Private-repository attestation plan eligibility: NOT_VERIFIED.
- Release/tag/deployment: NOT_STARTED.

The earlier GitHub Actions budget failures belong to the pre-hardening merged-main
state. Do not assume they still block the new hardening head; re-observe GitHub at
publication time.

## DO NOT REPEAT
Do not rerun the Rust v2 migration or unaffected algorithm qualification.
Do not restore wgpu/cryoglyph/lru merely to preserve the old dependency graph.
Do not add an advisory ignore for RUSTSEC-2026-0253.
Do not weaken attestation or Rust-only controls to obtain a green result.

## NEXT SAFE ACTION
If GitHub publication/integration is within the current owner authorization, re-observe
remote main/open PRs, then publish the existing hardening branch without replaying
valid local qualification. Exact new-head CI must qualify Windows and native macOS.
