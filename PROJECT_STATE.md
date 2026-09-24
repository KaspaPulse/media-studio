# PROJECT STATE

Updated: 2026-09-24
Repository: `KaspaPulse/whatsapp-video-preparer`
Visibility: PUBLIC
Default branch: `main`
Active task: `KASPAPULSE_MEDIA_STUDIO_V3`
Current phase: `G2 — SOURCE ACQUISITION`

## Historical product boundary

CURRENT_PRODUCT=WhatsApp Video Preparer
CURRENT_VERSION=2.0.0
TARGET_PRODUCT=KaspaPulse Media Studio
TARGET_VERSION=3.0.0
TARGET_REPOSITORY=KaspaPulse/media-studio
REPOSITORY_RENAME_STATUS=DEFERRED

V2_HISTORICAL_BOUNDARY=IMMUTABLE
V2_0_0_RELEASE=PRESERVE
V2_TAGS=DO_NOT_REWRITE
V2_RELEASE_ASSETS=DO_NOT_RENAME_OR_REPLACE

## Foundation freeze

Canonical Foundation:
`docs/foundation/KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION.md`

G0 final main:
- SHA: `27c68ff509477d53e454a4f6f80b7eeffb8e7d15`
- tree: `2428660026f840f73631b8d0b31da32157038014`
- PR #9: MERGED_SQUASH

Exact-main G0 evidence:
- Rust Policy `35995254915`: SUCCESS
- Windows `35995254993`: SUCCESS including provenance and SPDX SBOM attestation
- macOS `35995254996`: SUCCESS on x64 and arm64 including provenance and SPDX SBOM attestation

FOUNDATION_FREEZE=PASS

## G1 — Product / Domain Model

Implementation lane:
`feat/media-studio-v3-g1-domain-model-20260924`

Qualified local checkpoint:
- commit: `6270f011ef293482063573bc5f97a2574b502f2a`
- tree: `8d2c2652fe8c79ef7e3f62e82b85690e8bb2c62b`

Implemented:
- ProductIdentity
- InputSource with LocalFile(PathBuf) and RemoteUrl(Url)
- Locale
- UiDirection
- AppViewState
- MirrorPolicy
- FocusPolicy / semantic logical traversal foundation

Qualification:
- cargo fmt: PASS
- cargo fmt --check: PASS
- cargo test --locked --lib: PASS
- cargo clippy --locked --lib -- -D warnings: PASS
- cargo run -p xtask --locked -- verify: PASS
- Rust-only repository gate: PASS
- KSSS gate: PASS
- cargo-deny policy: PASS

G1_STATUS=VERIFIED_SUCCESS_LOCAL_CHECKPOINT

## Current phase — G2 Source Acquisition

Required:
- local-file source acquisition without mutating/moving/deleting the original;
- remote URL acquisition through pinned yt-dlp;
- `--ignore-config`;
- plugin directories disabled;
- `YTDLP_NO_PLUGINS=1`;
- exact final-path reporting instead of newest-file guessing where supported;
- result path containment validation inside the job directory;
- no DRM circumvention;
- no universal-platform support claim.

UI file picker / drag-and-drop presentation remains a later UI stage; G2 provides the Rust acquisition capability consumed by that UI.

## Preserved baselines

GUI_FRAMEWORK=iced
GUI_FRAMEWORK_BASELINE=0.14.0
ICED_DEFAULT_FEATURES=DISABLED
ICED_RENDERER_BASELINE=tiny-skia
RUST_BASELINE=1.98.1
CARGO_LOCK_REQUIRED=YES
BUILD_WITH_LOCKED=YES
OWNED_IMPLEMENTATION=100_PERCENT_RUST

UPSTREAM_RTL_LAYOUT_REFERENCE=iced-rs/iced#3303
BIDIRECTIONAL_LAYOUT=APPLICATION_OWNED_UNTIL_UPSTREAM_CAPABILITY_IS_RELEASED_AND_QUALIFIED

SCREEN_READER_ACCESSIBILITY=NOT_CLAIMED_UNTIL_TOOLKIT_SUPPORT_AND_NATIVE_VERIFICATION
WCAG_2_2=DESIGN_REFERENCE
WCAG2ICT_2_2=NON_WEB_SOFTWARE_GUIDANCE_REFERENCE
FORMAL_ACCESSIBILITY_CONFORMANCE_CLAIM=NO

## Continuity

Canonical operation journal:
`C:\WVP-Rust-Migration-20260923\OPERATION_JOURNAL.md`

Execution:
`RECOVER → RECONCILE → CLASSIFY → CONTINUE`

After material transitions:
`PERSIST → VERIFY → CHECKPOINT → CONTINUE`

Evidence:
- TEST_THE_AFFECTED_SURFACE
- REUSE_VALID_EVIDENCE
- RERUN_ONLY_WHEN_INVALIDATED
- NO_FAKE_PASS

DO_NOT_REPEAT:
- G0 exact-head/main qualification while its predicates remain unchanged;
- G1 local fmt/tests/clippy/xtask verification while its source remains unchanged.

NEXT ACTION:
Implement and qualify G2 source acquisition on top of G1 checkpoint, then create the next durable local checkpoint.
