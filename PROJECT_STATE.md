# PROJECT STATE

Updated: 2026-09-23
Repository: `KaspaPulse/whatsapp-video-preparer`
Default branch: `main`
Active task: `WVP_RUST_STRICT_HARDENING_DELTA`

## Baseline and current local identity
- Remote main observed before hardening: `5edda96a0a23b486ffbaa73b9c55ff46b6510418`
- Rust v2 implementation merge: `feeb43230e688496888a843809333c41a119d35f`
- Active local branch: `feat/rust-strict-hardening-20260923`
- Qualified technical checkpoint: `db6685287424f9c27f434473609356523687c1d8`
- Qualified technical tree: `7b168eb1fa2d5ec4848516bcea5c3ca55969ae00`
- Application version: `2.0.0`

Rust v2 remains the product foundation. This task is a bounded hardening delta,
not a reimplementation.

## Strict Rust-owned definition
The adopted standard is:
- application logic: Rust;
- automated test logic: Rust;
- repository-owned build/validation logic: Rust;
- workflow YAML: declarative orchestration only;
- inline PowerShell/Bash business logic: forbidden;
- simple `cargo`, `rustup`, and `rustc` invocations: allowed;
- external Actions: full 40-character commit SHA required.

The Rust `xtask verify` gate enforces the workflow restrictions and exact pins
for the approved attestation and cargo-deny actions.

## Hardening delta
Windows' former inline PowerShell environment/path setup for the real media test
was moved into `xtask verify-size-budget-package`. The Rust command verifies
packaged helper SHA-256 identities and launches the real size regression with
structured process arguments and Rust-set environment variables.

Attestation steps now use full-SHA-pinned `actions/attest` v4.2.2 and are
separated into main-only least-privilege jobs. Build/test jobs retain only
`contents: read`; attestation jobs add `id-token: write`, `attestations: write`,
and `artifact-metadata: write`.

A dedicated Rust Policy workflow runs the strict repository gate and full-SHA
pinned `cargo-deny-action` v2.1.1, which bundles cargo-deny 0.20.2.

## Dependency security remediation
The first strict cargo-deny qualification surfaced `RUSTSEC-2026-0253` in
`lru 0.16.4` through `cryoglyph -> iced_wgpu`. The advisory was not ignored
and `unsound = "all"` remains enforced.

A scratch graph proved that stable iced 0.14.0 supports a safe same-version
feature remediation. The repository now uses:
`default-features = false` with `tokio`, `image`, and `tiny-skia`.

Current locked all-target graph:
- `lru`: absent;
- `cryoglyph`: absent;
- `iced_wgpu`: absent;
- `iced_tiny_skia 0.14.1`: present.

Cargo.lock identity comparison against baseline:
- baseline identities: 579;
- current identities: 474;
- new identities: 0;
- removed identities: 105.

This is pruning-only; no new crate/version/source identity was introduced.

## Dependency policy
`deny.toml` checks all actual Windows/macOS targets and enforces:
- advisories: yanked deny, unsound all, unmaintained workspace, no advisory ignores;
- explicit permissive license allow-list derived from the actual graph;
- wildcard dependency requirements denied;
- duplicate versions retained as warnings for visibility;
- unknown registries and unknown Git sources denied.

Final cargo-deny 0.20.2 result:
`advisories ok, bans ok, licenses ok, sources ok`.
There are 12 duplicate-version warnings and zero errors.

## Local Windows qualification
On technical checkpoint source bytes:
- strict `xtask verify`: PASS;
- locked offline all-target check: PASS;
- Clippy with `-D warnings`: PASS;
- workspace tests: 11 passed / 0 failed;
- locked release build: PASS;
- release EXE SHA-256: `a8bc8aec5be3b872016b05c3f9891bc629873e67f54b2cd170e679eb1d7b87c3`;
- packaged helper SHA-256 identities: PASS;
- independent package `SHA256SUMS`: 4/4 PASS;
- real high-motion size-budget test through the new Rust package command: 1 passed / 0 failed;
- packaged GUI startup smoke: PASS after 6 seconds.

The product `src/*.rs`, `tests/`, and `build.rs` were not changed by this
hardening task. Existing media-algorithm evidence therefore remains reusable;
renderer/binary evidence was correctly requalified on Windows because the iced
feature graph changed.

## KSSS
Pinned KSSS v1.2.0 remains the authority. Local strengthening now binds:
- strict workflow orchestration;
- full-SHA Action pins;
- cargo-deny version/check set and advisory posture;
- actions/attest v4.2.2 exact SHA and fail-closed private-repository handling.

## Remote qualification state
Hardening push/PR/merge: NOT_STARTED.
New exact-head Windows CI: NOT_RUN.
New exact-head macOS arm64/x64 CI: NOT_RUN.
Main-only provenance/SBOM attestations: NOT_RUN.
Private-repository attestation plan eligibility: NOT_VERIFIED.
Release/tag/deployment: NOT_STARTED.

The old merged-main Actions budget failures are historical evidence from before
this hardening branch. Re-observe current GitHub state before publication rather
than assuming the old blocker still applies.

## Continuity
Canonical operation journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Do not repeat completed local qualification while its validity predicates hold.

NEXT ACTION: if publication/integration is within the current owner authorization,
re-observe remote main/open PRs and publish the existing branch. Require exact
new-head Windows/macOS/Rust Policy CI before integration. Do not release, tag, or
deploy as part of this state.
