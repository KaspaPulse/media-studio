# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `WVP_RUST_STRICT_HARDENING_DELTA` — CLOSED

## Qualified technical foundation
- Rust v2 implementation merge: `feeb43230e688496888a843809333c41a119d35f`
- Strict hardening PR: #7 — MERGED_SQUASH
- Qualified technical main SHA: `7c1d608fbd78fed2e0fcc52c30e85a3d9546628f`
- Qualified technical main tree: `615d24aa0021919cb1c30bb06db56b6bfe36e8e3`
- Application version: `2.0.0`

## Strict Rust-owned definition
The repository standard is:
- application logic: Rust;
- automated test logic: Rust;
- repository-owned build, packaging, validation, and governance logic: Rust;
- workflow YAML: declarative orchestration only;
- inline PowerShell/Bash business logic: forbidden;
- simple `cargo`, `rustup`, and `rustc` workflow invocations: allowed;
- external Actions: full 40-character commit SHA required.

The Rust `xtask verify` gate enforces these restrictions.

## Dependency and supply-chain hardening
Strict cargo-deny surfaced `RUSTSEC-2026-0253` through the previous `iced_wgpu -> cryoglyph -> lru 0.16.4` path. No advisory ignore was added.

The repository remains on stable `iced 0.14.0` with default features disabled and `tokio,image,tiny-skia` enabled. On the qualified Windows x64, macOS arm64, and macOS x64 target-filtered dependency graphs, `lru`, `cryoglyph`, and `iced_wgpu` are unreachable while `iced_tiny_skia 0.14.1` is present. `Cargo.lock` may retain inactive optional package records.

Lock identity comparison against the pre-hardening baseline:
- baseline identities: 579;
- qualified identities: 474;
- new identities: 0;
- removed identities: 105.

`deny.toml` enforces advisories, licenses, bans, and sources across the actual Windows/macOS targets. Final cargo-deny result is PASS with no advisory ignores.

## Workflow and attestation hardening
The package size regression is orchestrated by Rust `xtask`, not inline PowerShell. Build jobs have least privilege. Main-only attestation jobs use full-SHA-pinned `actions/attest` v4.2.2 and only the permissions needed for provenance/SBOM persistence.

The initial attestation attempt on the qualified main failed because GitHub did not support persistence for the user-owned private repository. After explicit owner authorization, the repository was changed to PUBLIC. Re-running the same workflow path on the same qualified technical main succeeded without weakening any control.

## Verified GitHub evidence
Exact-head before merge:
- Rust Policy `35918330368`: SUCCESS;
- Windows `35918329646`: SUCCESS;
- macOS `35918329621`: SUCCESS on x64 and arm64.

Exact-main:
- Rust Policy `35921197250`: SUCCESS;
- Windows `35921197316`, attempt 2: SUCCESS, including build provenance and SPDX SBOM attestation;
- macOS `35921197346`, attempt 2: SUCCESS for x64 and arm64, including build provenance and SPDX SBOM attestation.

The qualified technical main reports Rust-only GitHub language accounting and zero tracked executable-code files with forbidden Python, PowerShell, shell, JavaScript, TypeScript, CMD, or BAT extensions.

## Public transition safety
Before changing visibility, a heuristic current-tree/history scan found no sensitive key files and no actual private-key, GitHub PAT, AWS access-key, Telegram bot-token, or generic secret-assignment matches. Broad `sk-` matches were inspected and were false positives from `xtask-...` log names.

## Boundaries
Release, tag, and deployment remain NOT_STARTED. They are not implied by this task closure.

## Continuity
Canonical operation journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Do not repeat completed qualification while its validity predicates hold.

NEXT ACTION: none for `WVP_RUST_STRICT_HARDENING_DELTA`. Treat any release, tag, deployment, or new feature as a new explicitly authorized task.