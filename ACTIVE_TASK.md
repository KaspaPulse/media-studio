# ACTIVE TASK

## Task

KASPAPULSE_MEDIA_STUDIO_V3

## Status

IN_PROGRESS

## Current phase

G2 — SOURCE ACQUISITION

## Authorization

Continuous owner authorization covers G0 → G9 for this task, including branches,
source/tests/docs/governance changes, required Cargo changes, commits, push/PR/merge after
gates, repository rename to `KaspaPulse/media-studio`, and v3.0.0 release publication.

Excluded:
- force push/history rewrite;
- rewriting/deleting v2 tags/releases/assets;
- credential rotation;
- DNS/unrelated infrastructure;
- DRM circumvention;
- unrelated repositories.

## Completed and verified

### G0 — Foundation Freeze
PASS on exact main:
`27c68ff509477d53e454a4f6f80b7eeffb8e7d15`
tree:
`2428660026f840f73631b8d0b31da32157038014`

### G1 — Product / Domain Model
VERIFIED_SUCCESS_LOCAL_CHECKPOINT

Commit:
`6270f011ef293482063573bc5f97a2574b502f2a`

Tree:
`8d2c2652fe8c79ef7e3f62e82b85690e8bb2c62b`

Implemented ProductIdentity, InputSource, Locale, UiDirection, AppViewState,
MirrorPolicy, and FocusPolicy foundations.

Affected-surface qualification:
fmt PASS; lib tests PASS; lib clippy -D warnings PASS; xtask verify PASS.

## G2 objective

Implement general source acquisition:

- LocalFile remains read-only and original is never overwritten/moved/deleted.
- RemoteUrl uses the pinned yt-dlp helper.
- yt-dlp external config is ignored.
- plugin loading is disabled.
- final downloaded path is reported explicitly and validated inside the job directory.
- no newest-file guessing remains in the successful path.
- no DRM circumvention.
- support claim remains bounded to URLs handled by bundled yt-dlp.

Do not implement the final v3 UI in G2.

## Do not repeat

Do not repeat G0 CI or G1 qualification unless a relevant validity predicate changes.

## Next safe action

Implement the smallest G2 Rust acquisition slice, write unit tests for argument/path
invariants, and run affected-surface qualification only.
