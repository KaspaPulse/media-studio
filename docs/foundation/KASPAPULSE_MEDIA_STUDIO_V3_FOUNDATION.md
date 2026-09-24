# KaspaPulse Media Studio 3.0 Foundation

Foundation ID: KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION
Foundation version: 1.0
Status: FROZEN_CANDIDATE / DOCUMENTATION_ONLY
Task: KASPAPULSE_MEDIA_STUDIO_V3
Date: 2026-09-24

## 1. Foundation identity

This document is the canonical architectural and product contract for the transition from
WhatsApp Video Preparer 2.x to KaspaPulse Media Studio 3.x.

The Foundation defines requirements, invariants, sequencing, evidence rules, and claim
boundaries. It does not by itself implement or qualify v3 functionality.

Requirement and verification status are deliberately separate:

- REQUIRED means implementation is mandatory.
- NATIVE_VERIFICATION_REQUIRED means real native qualification is mandatory.
- NOT_VERIFIED means the capability has not yet earned a PASS.
- PASS may be recorded only after implementation and applicable qualification evidence.

NO_FAKE_PASS applies throughout this document.

## 2. Source baseline

CURRENT_REPOSITORY=KaspaPulse/whatsapp-video-preparer
CURRENT_VISIBILITY=PUBLIC
SOURCE_BASE_SHA=2440690ecc1845b79de47550e672d767edbb9393
SOURCE_BASE_TREE=b712b1da64b5de3739cc087292d1cb427621a931
CURRENT_PRODUCT=WhatsApp Video Preparer
CURRENT_VERSION=2.0.0
V3_IMPLEMENTATION=NOT_STARTED

The source baseline is historical identity for G0. Later stages must bind their own exact
source SHA/tree before qualification or publication.

## 3. Historical v2 boundary

V2_HISTORICAL_BOUNDARY=IMMUTABLE
V2_0_0_RELEASE=PRESERVE
V2_TAGS=DO_NOT_REWRITE
V2_RELEASE_ASSETS=DO_NOT_RENAME_OR_REPLACE
V2_PROVENANCE=HISTORICAL_EVIDENCE
V2_SBOM=HISTORICAL_EVIDENCE

The published v2.0.0 release remains a correct historical WhatsApp-focused Rust release.
The v3 program may evolve product identity and architecture, but it must not rewrite,
retag, replace, or reinterpret v2 release assets.

## 4. Target product identity

TARGET_PRODUCT=KaspaPulse Media Studio
TARGET_VERSION=3.0.0
TARGET_REPOSITORY=KaspaPulse/media-studio
REPOSITORY_RENAME_STATUS=DEFERRED
PRODUCT_RENAME_IMPLEMENTATION_STATUS=NOT_STARTED

The target repository name is an authorized future governance transition, not a statement
that the rename has already happened.

## 5. Scope

The v3 program transforms the product from a WhatsApp-focused video preparer into a
general-purpose media preparation application while preserving the repository-owned
Rust-only implementation policy.

In scope:

- local media input;
- remote URL acquisition through the pinned yt-dlp boundary;
- FFprobe-driven capability inspection;
- processing independent of acquisition source;
- export profiles, with WhatsApp as one profile;
- remux when safe and sufficient;
- transcode when required;
- responsive modern desktop UI;
- Arabic RTL and English LTR;
- theme support;
- keyboard/focus correctness;
- testability as an architectural property;
- qualified Windows x64, macOS x64, and macOS arm64 outputs;
- v3 release publication after exact-source qualification.

## 6. Explicit non-goals

The Foundation does not authorize or promise:

- DRM circumvention;
- arbitrary plugin execution;
- a claim that every video format in existence is supported;
- a claim that every website/platform is supported;
- screen-reader accessibility before toolkit support and native verification;
- formal WCAG conformance certification;
- repository rename during G0;
- rewriting historical v2 tags/releases/assets;
- unbounded cancellation behavior without qualification;
- production infrastructure, DNS, or unrelated repository changes.

## 7. Nine pillars

KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION consists of:

1. PRODUCT_IDENTITY
2. INPUT_SOURCE_ARCHITECTURE
3. MEDIA_PROBE
4. PROCESSING_ENGINE
5. EXPORT_PROFILE
6. UI_FOUNDATION
7. INTERNATIONALIZATION
8. INTERACTION_AND_ACCESSIBILITY
9. UI_TESTABILITY

No pillar may silently weaken the Rust-only, supply-chain, evidence, or historical v2
boundaries.

## 8. Architecture contracts

### 8.1 Product identity

Current and target identities are distinct until the authorized identity migration occurs.
v3 implementation must not present the target repository identity as completed before the
repository rename is independently verified.

### 8.2 InputSource

The target domain model must support, at minimum:

```rust
enum InputSource {
    LocalFile(PathBuf),
    RemoteUrl(Url),
}
```

The exact Rust type layout may evolve, but the semantic distinction is required.

InputSource must not encode WhatsApp-specific policy.

### 8.3 Source acquisition

Local source contract:

LOCAL_SOURCE_MUTATION=FORBIDDEN
READ_ONLY_INPUT=YES
ORIGINAL_NEVER_OVERWRITTEN=YES
ORIGINAL_NEVER_MOVED=YES
ORIGINAL_NEVER_DELETED=YES

Remote source contract:

REMOTE_BACKEND=pinned yt-dlp
YTDLP_IGNORE_CONFIG=REQUIRED
YTDLP_PLUGIN_DIRS_DISABLED=REQUIRED
YTDLP_NO_PLUGINS_ENV=REQUIRED
EXACT_FINAL_PATH_REPORTING=REQUIRED_WHERE_SUPPORTED

Required hardening:

```text
--ignore-config
--no-plugin-dirs
YTDLP_NO_PLUGINS=1
```

The application must validate reported output paths as belonging to the intended job
directory before processing.

MULTI_PLATFORM_ACQUISITION=
SUPPORTED_WHERE_BUNDLED_YT_DLP_CAN_EXTRACT

DOWNLOAD_FROM_ANY_PLATFORM=NOT_CLAIMED
DRM_CIRCUMVENTION=FORBIDDEN

A separate DirectHttp backend is not required for the initial v3 architecture unless a
concrete need is established.

### 8.4 MediaProbe

Media capability is determined by the bundled FFmpeg/FFprobe build, not by a static file
extension allowlist.

SUPPORTED_INPUT=
MEDIA_READABLE_BY_BUNDLED_FFMPEG_BUILD

MediaProbe should provide structured metadata as needed, including:

- container;
- duration;
- video streams;
- audio streams;
- codecs;
- dimensions;
- frame rate;
- pixel format;
- rotation/orientation;
- relevant color/HDR metadata.

Probe failure is fail-closed with a clear user-facing error.

### 8.5 ProcessingEngine

The ProcessingEngine consumes a qualified media source plus an ExportProfile. It must not
depend on whether the source originated locally or remotely.

Required processing policy:

REMUX_WHEN_SAFE_AND_SUFFICIENT=REQUIRED
TRANSCODE_ONLY_WHEN_REQUIRED=REQUIRED
PRESERVE_ASPECT_RATIO=REQUIRED
NO_ACCIDENTAL_UPSCALE=REQUIRED
BOUNDED_OUTPUT_VERIFICATION=REQUIRED
DETERMINISTIC_FAILURES=REQUIRED

### 8.6 ExportProfile

WhatsApp-specific limits move out of generic processing policy and into an export profile.

The architecture must permit profiles such as:

- Universal MP4;
- WhatsApp;
- High Quality;
- Web Compatible;
- Custom.

Not every profile has a byte limit.

The WhatsApp profile preserves the qualified v2 size policy:

```text
target_bytes=9_500_000
hard_limit_bytes=10_000_000
post_encode_verification=REQUIRED
```

### 8.7 AppViewState

The semantic application state model is:

```text
Empty
→ AcquiringSource
→ Probing
→ SourceReady
→ Processing
→ Finalizing
→ Completed

Long-running state
→ Cancelling
→ Cancelled

Execution state
→ Failed
```

AcquiringSource means making the selected source ready for processing; it does not imply
that a local file is downloaded.

CANCELLATION_ARCHITECTURE=SUPPORTED
CANCELLATION_IMPLEMENTATION_STATUS=NOT_STARTED
CANCELLATION_IMPLEMENTATION_REQUIRED_IN_FOUNDATION=NO

No cancellation PASS may be claimed before process termination, partial-output policy,
state consistency, race handling, and restartability are qualified.

## 9. UI framework contract

GUI_FRAMEWORK=iced
GUI_FRAMEWORK_BASELINE=0.14.0
GUI_FRAMEWORK_CHANGE=NO

ICED_DEFAULT_FEATURES=DISABLED
ICED_RENDERER_BASELINE=tiny-skia
ICED_RENDERER_CHANGE=REQUIRES_REQUALIFICATION

RUST_BASELINE=1.98.1
CARGO_LOCK_REQUIRED=YES
BUILD_WITH_LOCKED=YES

The current qualified baseline uses:

```toml
iced = { version = "0.14.0", default-features = false, features = ["tokio", "image", "tiny-skia"] }
```

The toolkit's experimental status is acknowledged. The application must qualify the
behavior it relies on instead of assuming broader upstream API or behavior stability.

UPSTREAM_RTL_LAYOUT_REFERENCE=iced-rs/iced#3303
UPSTREAM_RTL_PR_STATE=OPEN
UPSTREAM_RTL_PR_MERGED=NO

Transient mergeability state is intentionally not frozen.

TEXT_DIRECTION != LAYOUT_DIRECTION
APP_OWNS_LAYOUT_DIRECTION=YES

BIDIRECTIONAL_LAYOUT=
APPLICATION_OWNED_UNTIL_UPSTREAM_CAPABILITY_IS_RELEASED_AND_QUALIFIED

The application must not depend on an unmerged upstream RTL patch.

## 10. RTL/LTR and locale contract

Target locale architecture includes:

- Language;
- UiDirection;
- localized labels and units;
- localized status templates;
- BiDi isolation helpers;
- formatting policy.

TRUE_RTL_REQUIREMENT=REQUIRED
TRUE_RTL_STATUS=NOT_VERIFIED

ARABIC_SHAPING_REQUIREMENT=NATIVE_VERIFICATION_REQUIRED
ARABIC_SHAPING_STATUS=NOT_VERIFIED

ARABIC_BIDI_REQUIREMENT=NATIVE_VERIFICATION_REQUIRED
ARABIC_BIDI_STATUS=NOT_VERIFIED

RTL_LAYOUT_REQUIREMENT=REQUIRED
RTL_LAYOUT_STATUS=NOT_VERIFIED

LTR_LAYOUT_REQUIREMENT=REQUIRED
LTR_LAYOUT_STATUS=NOT_VERIFIED

UI components must use logical start/end semantics instead of scattering ad hoc
`if rtl { ... } else { ... }` branching through screens.

Target UI architecture may use modules such as:

```text
src/ui/
├── direction.rs
├── locale.rs
├── theme.rs
├── tokens.rs
├── focus.rs
├── bidi.rs
├── components/
└── screens/
```

Exact file layout may evolve while preserving the semantic boundary.

### 10.1 Mirror policy

Directional components and icons require an explicit semantic policy concept such as:

```rust
enum MirrorPolicy {
    FollowLocale,
    Never,
    Always,
}
```

Visual mirroring must not blindly flip media-semantic controls.

### 10.2 Technical fields

Technical values remain LTR in both Arabic and English UI where direction is inherently
technical:

URL_FIELD_DIRECTION=LTR
PATH_FIELD_DIRECTION=LTR
FILENAME_FIELD_DIRECTION=LTR
HASH_FIELD_DIRECTION=LTR
VERSION_FIELD_DIRECTION=LTR
CODEC_FIELD_DIRECTION=LTR
RESOLUTION_FIELD_DIRECTION=LTR

Mixed Arabic/Latin strings must use explicit isolation where appropriate.

LOCALIZED_UNITS_REQUIREMENT=REQUIRED
LOCALIZED_UNITS_STATUS=NOT_VERIFIED

LOCALIZED_STATUS_MESSAGES_REQUIREMENT=REQUIRED
LOCALIZED_STATUS_MESSAGES_STATUS=NOT_VERIFIED

MIXED_BIDI_ISOLATION_REQUIREMENT=REQUIRED
MIXED_BIDI_ISOLATION_STATUS=NOT_VERIFIED

Domain enums must not own localized presentation strings.

## 11. Responsive UI contract

The UI must support semantic layout classes:

- Compact;
- Standard;
- Wide.

Exact breakpoint values are implementation tuning, not Foundation constants.

Required qualification:

COMPACT_LAYOUT_REQUIREMENT=REQUIRED
STANDARD_LAYOUT_REQUIREMENT=REQUIRED
WIDE_LAYOUT_REQUIREMENT=REQUIRED

ARABIC_COMPACT_REQUIREMENT=REQUIRED
ARABIC_STANDARD_REQUIREMENT=REQUIRED
ARABIC_WIDE_REQUIREMENT=REQUIRED

ENGLISH_COMPACT_REQUIREMENT=REQUIRED
ENGLISH_STANDARD_REQUIREMENT=REQUIRED
ENGLISH_WIDE_REQUIREMENT=REQUIRED

All statuses remain NOT_VERIFIED until native implementation qualification.

## 12. Theme and design-system contract

ThemePreference must support:

- System;
- Light;
- Dark.

System is the target default.

SYSTEM_THEME_REQUIREMENT=REQUIRED
SYSTEM_THEME_STATUS=NOT_VERIFIED
LIGHT_THEME_REQUIREMENT=REQUIRED
LIGHT_THEME_STATUS=NOT_VERIFIED
DARK_THEME_REQUIREMENT=REQUIRED
DARK_THEME_STATUS=NOT_VERIFIED

The design system must use semantic tokens for:

- spacing;
- radius;
- typography;
- control sizes;
- surfaces;
- borders;
- elevation;
- motion where used;
- semantic color roles.

Components must consume semantic roles rather than scattering raw visual constants through
screens.

## 13. Focus and keyboard contract

KEYBOARD_ACCESSIBILITY=IN_SCOPE
FOCUS_ACCESSIBILITY=IN_SCOPE

FOCUS_TRAVERSAL=SEMANTIC_LOGICAL_ORDER
VISUAL_ORDER=LOCALE_AWARE

Visual RTL mirroring must not be treated as a requirement to mechanically reverse all focus
traversal.

Required qualification includes:

KEYBOARD_NAVIGATION_REQUIREMENT=REQUIRED
KEYBOARD_NAVIGATION_STATUS=NOT_VERIFIED
TAB_FORWARD_REQUIREMENT=REQUIRED
TAB_FORWARD_STATUS=NOT_VERIFIED
TAB_REVERSE_REQUIREMENT=REQUIRED
TAB_REVERSE_STATUS=NOT_VERIFIED
ENTER_SPACE_ACTIONS_REQUIREMENT=REQUIRED
ENTER_SPACE_ACTIONS_STATUS=NOT_VERIFIED
ESCAPE_BEHAVIOR_REQUIREMENT=REQUIRED
ESCAPE_BEHAVIOR_STATUS=NOT_VERIFIED
VISIBLE_FOCUS_REQUIREMENT=REQUIRED
VISIBLE_FOCUS_STATUS=NOT_VERIFIED
FOCUS_NOT_OBSCURED_REQUIREMENT=REQUIRED
FOCUS_NOT_OBSCURED_STATUS=NOT_VERIFIED

Stable widget/focus identities are required where needed for deterministic interaction
testing.

## 14. Accessibility claim boundary

SCREEN_READER_ACCESSIBILITY=
NOT_CLAIMED_UNTIL_TOOLKIT_SUPPORT_AND_NATIVE_VERIFICATION

No broad ACCESSIBILITY=PASS claim is permitted merely because keyboard/focus requirements
pass.

WCAG_2_2=DESIGN_REFERENCE
WCAG2ICT_2_2=NON_WEB_SOFTWARE_GUIDANCE_REFERENCE
FORMAL_ACCESSIBILITY_CONFORMANCE_CLAIM=NO

WCAG/WCAG2ICT references guide design and testing; they are not by themselves a formal
conformance certification for this native desktop application.

DRAG_DROP=OPTIONAL_INPUT_METHOD
NON_DRAG_FILE_PICKER=REQUIRED

Any drag-and-drop interaction must have a non-drag alternative.

## 15. UI testability contract

UI_TESTABILITY=FOUNDATIONAL
UI_TESTABILITY_STATUS=NOT_VERIFIED

The architecture must make room for:

- stable widget IDs where applicable;
- headless semantic tests;
- E2E interaction tests;
- RTL/LTR fixtures;
- Compact/Standard/Wide fixtures;
- focus-order tests;
- state-transition tests;
- native GUI smoke tests.

Screenshot/visual fixtures may supplement, but not replace, semantic interaction tests.

## 16. Rust-only invariants

OWNED_IMPLEMENTATION=100_PERCENT_RUST

APPLICATION_LOGIC=100_PERCENT_RUST
TEST_LOGIC=100_PERCENT_RUST
OWNED_BUILD_LOGIC=100_PERCENT_RUST
OWNED_VALIDATION_LOGIC=100_PERCENT_RUST

WORKFLOW_YAML=DECLARATIVE_ORCHESTRATION_ONLY
INLINE_POWERSHELL_BASH_BUSINESS_LOGIC=FORBIDDEN
EXTERNAL_ACTIONS=FULL_40_CHAR_SHA_REQUIRED

YAML/TOML/JSON/Markdown/assets are configuration, policy, metadata, or assets rather than
alternate repository-owned implementation languages.

FFmpeg, FFprobe, and yt-dlp remain external pinned helper boundaries.

## 17. Dependency and supply-chain invariants

- Cargo.lock is required.
- Qualified builds use --locked.
- Helper identities remain pinned and verified.
- Repository/KSSS gates remain active.
- cargo-deny remains active.
- No advisory ignore may be added merely to obtain a passing build.
- Changing the iced renderer or feature graph requires affected-surface requalification.
- External GitHub Actions remain full-SHA pinned.
- Supply-chain controls may be strengthened but not silently weakened.

## 18. Required vs verified semantics

Foundation documentation may declare requirements but may not pre-claim implementation PASS.

Pattern:

```text
<FEATURE>_REQUIREMENT=REQUIRED
<FEATURE>_STATUS=NOT_VERIFIED
```

After implementation and qualification only:

```text
<FEATURE>=PASS
```

NOT_VERIFIED, PARTIAL, UNKNOWN, FAILED, or BLOCKED must never be promoted to PASS without
evidence.

## 19. Implementation sequence

The authorized sequence is:

G0 — FOUNDATION FREEZE

G1 — PRODUCT / DOMAIN MODEL
- ProductIdentity
- InputSource
- Locale
- UiDirection
- AppViewState
- MirrorPolicy
- FocusPolicy foundations

G2 — SOURCE ACQUISITION
- LocalFile
- RemoteUrl
- yt-dlp hardening
- exact final-path handling

G3 — MEDIA PROBE
- structured FFprobe metadata

G4 — EXPORT PROFILES
- Universal MP4
- WhatsApp
- High Quality
- Web Compatible
- Custom architecture

G5 — PROCESSING ENGINE DECOUPLING
- source-independent processing
- remux/transcode policy

G6 — UI DESIGN SYSTEM
- tokens
- themes
- components
- focus
- BiDi/direction

G7 — V3 MAIN SCREEN
- responsive UI
- true RTL/LTR
- local/remote source UX
- profile/output/progress/results UX

G7R — IDENTITY MIGRATION / REPOSITORY RENAME
- governance transition to KaspaPulse/media-studio
- metadata/settings/bundle/artifact migration
- preserve v2 history

G8 — NATIVE QUALIFICATION
- Windows x64
- macOS x64
- macOS arm64
- source paths/profiles/remux/transcode
- RTL/LTR/themes/keyboard
- headless/E2E/native GUI
- Rust Policy/cargo-deny/SBOM/provenance

G9 — v3.0.0 RELEASE
- exact qualified source freeze
- v3.0.0 tag
- GitHub Release
- platform assets/checksums/SBOM/provenance verification

A stage may be split into smaller implementation slices if that reduces risk while preserving
the stage contract.

## 20. Qualification gates

Qualification follows:

TEST_THE_AFFECTED_SURFACE
REUSE_VALID_EVIDENCE
RERUN_ONLY_WHEN_INVALIDATED
NO_FAKE_PASS

Failures follow:

PRESERVE_EVIDENCE
→ ROOT_CAUSE
→ SMALLEST_FIX
→ AFFECTED_SURFACE_ONLY_REQUALIFICATION

No security gate may be weakened merely to obtain green CI.

Before release:

FINAL_MAIN_QUALIFIED=YES
OPEN_PRS=0
RELEASE_BLOCKERS=NONE
TAG_v3.0.0=ABSENT
RELEASE_v3.0.0=ABSENT

Release source, tag, artifacts, checksums, SBOM, and provenance must resolve to the same
qualified source identity.

## 21. Evidence invalidation rules

Evidence may be reused only while its relevant predicates remain valid.

Examples of invalidating changes include, as applicable:

- source bytes;
- dependency graph;
- Cargo.lock;
- Rust toolchain;
- iced renderer/features;
- helper identities;
- workflow/security policy;
- target platform;
- packaging path;
- artifact content;
- UI behavior under qualification.

Documentation-only changes do not automatically invalidate unrelated binary evidence.

## 22. Change control

### G0 allowed paths

Only these paths may change in the Foundation freeze delta:

```text
docs/foundation/KASPAPULSE_MEDIA_STUDIO_V3_FOUNDATION.md
PROJECT_STATE.md
ACTIVE_TASK.md
CURRENT_STATE.md
```

### G0 forbidden mutations

```text
src/**
tests/**
Cargo.toml
Cargo.lock
.github/workflows/**
repository rename
binary rename
bundle identifier mutation
InputSource implementation
UI implementation
```

### G0 acceptance

FOUNDATION_DOCUMENT=COMPLETE
FOUNDATION_VERSIONED=YES
SOURCE_BASE_SHA=2440690ecc1845b79de47550e672d767edbb9393
SOURCE_BASE_TREE=b712b1da64b5de3739cc087292d1cb427621a931
NINE_PILLARS_DEFINED=YES
ARCHITECTURAL_INVARIANTS_DEFINED=YES
REQUIRED_VS_VERIFIED_SEMANTICS=DEFINED
V2_HISTORY_PRESERVED=YES
IMPLEMENTATION_SOURCE_DIFF=0
DEPENDENCY_DIFF=0
WORKFLOW_DIFF=0
V3_IMPLEMENTATION=NOT_STARTED

G0 success freezes the contract; it does not claim any v3 runtime capability as implemented.

## 23. Repository identity migration

Repository rename is deferred until G7R.

Authorized target:

```text
KaspaPulse/whatsapp-video-preparer
→ KaspaPulse/media-studio
```

Before rename, perform read-only reconciliation. After rename, verify redirects and access to:

- v2.0.0 tag;
- v2.0.0 release;
- historical assets;
- historical commit links.

Update applicable v3 identities and metadata without renaming or replacing historical v2
release assets.

Existing user settings/data must not be broken without an explicit migration policy.

## 24. v3 release boundary

G9 may publish v3.0.0 only after exact-final-main qualification.

The release must include only artifacts produced from the exact qualified release source.
No artifact built from another source SHA may be renamed and published as v3.0.0.

Signing/notarization capability must be evaluated against actual available credentials and
platform capability before release. Missing external capability is a blocker to that claim,
not permission for an unsafe workaround.

## 25. Foundation freeze classification

V3_ARCHITECTURE_DIRECTION=APPROVED
V3_FOUNDATION_CONTENT=APPROVED
V3_FOUNDATION_FREEZE=READY
V3_IMPLEMENTATION=NOT_STARTED

NEXT_SAFE_ACTION=
COMPLETE_G0_DOCUMENTATION_ONLY_FREEZE_AND_INTEGRATION
