# KSSS adoption

This repository adopts KSSS v1.2.0 as an external, immutable governance authority.
The repository does not vendor or fork the KSSS Python consumer runtime because the
owner requires repository-owned executable code, tests, and tooling to remain Rust-only.

Pinned identity:

- KSSS release: v1.2.0
- Source SHA: 967ed5068947a39961d5d5cc483ef65d25a61059
- Policy bundle digest: 3c1c8b449d736aba5fec5cffd688496b06d81f287f55fff4c3f42e36c3b58ec6
- Consumer Runtime SHA-256: 38309d2ab8fa30096d99940f855e88173faa182e60db33f2a96b2d3408507430
- Runtime sequence: 2
- Trust root: ksss-trust-root-1

`cargo run -p xtask -- verify` validates the repository-owned adoption metadata,
risk/applicability digests, Rust-only language policy, size-budget invariant, and
continuity files. Cryptographic acquisition/update verification remains a separate
KSSS trust-boundary operation and must verify the pinned release artifact before
accepting a future KSSS identity.
