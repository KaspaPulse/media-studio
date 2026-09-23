# Rust strict hardening delta â€” local qualification handoff

Date: 2026-09-23
Task: `WVP_RUST_STRICT_HARDENING_DELTA`
Repository: `KaspaPulse/whatsapp-video-preparer`
Host: `Server`

## Identity
- Baseline main: `5edda96a0a23b486ffbaa73b9c55ff46b6510418`
- Branch: `feat/rust-strict-hardening-20260923`
- Technical checkpoint: `db6685287424f9c27f434473609356523687c1d8`
- Technical tree: `7b168eb1fa2d5ec4848516bcea5c3ca55969ae00`

## Scope
The hardening delta changes workflow orchestration, Rust `xtask` validation,
dependency policy, KSSS local strengthening, and the iced renderer feature set.
It does not modify product `src/*.rs`, tests, or `build.rs`.

## Security finding and remediation
Strict cargo-deny initially discovered `RUSTSEC-2026-0253` through:
`iced_wgpu -> cryoglyph -> lru 0.16.4`.

No advisory ignore was added. A scratch all-target graph established a stable
same-version remediation: iced 0.14.0 with default features disabled and
`tokio,image,tiny-skia` enabled.

The qualified target-filtered dependency graphs for `x86_64-pc-windows-msvc`,
`aarch64-apple-darwin`, and `x86_64-apple-darwin` have no reachable `lru`,
`cryoglyph`, or `iced_wgpu`; `iced_tiny_skia 0.14.1` is present on each target.
`Cargo.lock` may retain inactive optional package records for those crates. The
lockfile has zero new package identities and 105 removed identities relative to
baseline. Therefore `RUSTSEC-2026-0253` has no active path in the qualified
Windows/macOS build graphs.

## Strict Rust workflow hardening
- workflow YAML permits declarative orchestration and simple Rust tool invocations;
- explicit shell selectors and inline script business logic are rejected;
- external Actions require full 40-character SHAs;
- the package size regression is orchestrated by Rust `xtask`, not PowerShell;
- `actions/attest` v4.2.2 is pinned to `1e69f48acb82d1966a394da916b4c1698aa569d6`;
- `cargo-deny-action` v2.1.1 is pinned to `3c6349835b2b7b196a839186cb8b78e02f7b5f25`.

## Local evidence
- `hardening-xtask-verify-post-remediation.log`: strict gate PASS.
- `hardening-cargo-deny-post-remediation.log`: advisories/licenses/bans/sources PASS; zero errors.
- `hardening-locked-check.log`: locked offline workspace/all-target check PASS.
- `hardening-clippy.log`: Clippy `-D warnings` PASS.
- `hardening-tests.log`: 11 passed / 0 failed.
- `hardening-release-build.log`: release build PASS.
- `hardening-package-windows.log`: package and pinned helpers PASS.
- `hardening-size-budget-package.log`: real high-motion size regression 1 PASS / 0 FAIL.
- `hardening-gui-smoke.log`: packaged GUI remained alive for 6 seconds; PASS.

Release/package executable SHA-256:
`a8bc8aec5be3b872016b05c3f9891bc629873e67f54b2cd170e679eb1d7b87c3`.

Independent package SHA256SUMS verification: 4/4 PASS.

## Evidence boundaries
Rust v2 product/media algorithm evidence remains reusable because product source
did not change. The iced renderer dependency graph did change, so Windows
binary/runtime evidence was requalified locally.

Native macOS hardening evidence must come from the new exact-head GitHub CI; the
previous Rust v2 macOS PASS cannot be transferred to the changed renderer graph.

## Remote state
Push: NOT_STARTED.
PR: NOT_STARTED.
Merge: NOT_STARTED.
New exact-head CI: NOT_RUN.
Main-only attestations: NOT_RUN.
Private-repository attestation plan eligibility: NOT_VERIFIED.
Release/tag/deployment: NOT_STARTED.

The previous GitHub Actions budget failure is historical until current GitHub
state is re-observed for this new head.

## Resume
Use the canonical operation journal at:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

NEXT_SAFE_ACTION: if publication is within authorization, re-observe remote
main/open PRs, then publish this existing qualified branch and require exact
new-head Windows/macOS/Rust Policy CI.
