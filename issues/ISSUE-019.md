# ISSUE-019 — native storage: stale bundle GC removes objects referenced by manifests

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Reliability
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`

## Root-Cause

[S] Independent `PcraftWriter::save_dir` calls can publish conflicting manifests and garbage-collect objects from stale snapshots without transaction coordination.
Review mapping: `E6`, VALID; severity High.

## Reach-and-Impact

Trigger: two directory-bundle writers overlap on one `.pcraft` path with different object sets.
[O] A controlled Linux reproduction delayed writer A immediately after a new object rename; writer B's stale-snapshot GC removed the object before A published its manifest, and final bundle loading failed.
The reproduction proves this interleaving is possible, not its production frequency.

## Evidence

- [S] `save_dir` enumerates existing objects before publishing its manifest, then garbage-collects from that earlier snapshot (https://github.com/storytold/photocraft/blob/a96a621deea97d4b1ecd173b8b921587e33f3ca5/crates/format/src/store.rs#L387-L425).
- [S] `write_atomic` uses per-file atomic replacement from PR #230; it does not serialize the directory transaction (https://github.com/storytold/photocraft/blob/a96a621deea97d4b1ecd173b8b921587e33f3ca5/crates/format/src/store.rs#L457-L461; https://github.com/storytold/photocraft/blob/a96a621deea97d4b1ecd173b8b921587e33f3ca5/crates/format/src/atomic.rs#L117-L148).
- [S] Desktop autosave writes ID-keyed bundles into one shared `Recovery` directory (https://github.com/storytold/photocraft/blob/a96a621deea97d4b1ecd173b8b921587e33f3ca5/apps/photocraft/src/services.rs#L55-L58; https://github.com/storytold/photocraft/blob/a96a621deea97d4b1ecd173b8b921587e33f3ca5/apps/photocraft/src/services.rs#L148-L153).
- [S] Each `Autosaver` owns one worker queue and does not coordinate other instances' directory writers (https://github.com/storytold/photocraft/blob/a96a621deea97d4b1ecd173b8b921587e33f3ca5/crates/format/src/autosave.rs#L46-L72; https://github.com/storytold/photocraft/blob/a96a621deea97d4b1ecd173b8b921587e33f3ca5/crates/format/src/autosave.rs#L125-L140).

## Prior-Art

Coverage: upstream issue #203, merged PR #230, v0.2.0 release notes, and current source at `a96a621` reviewed on 2026-10-06.
Two issue and two PR searches for `save_dir object manifest` and `bundle lock concurrent GC` returned no matches.
PR #230 adds atomic replacement for individual files, not serialization across object enumeration, manifest publication, and GC (https://github.com/storytold/photocraft/pull/230).
The v0.2.0 release notes mention no concurrent directory-bundle fix (https://github.com/storytold/photocraft/releases/tag/v0.2.0).
The Discussions route returned HTTP 404; discussion history remains unsearched (https://github.com/storytold/photocraft/discussions).
`ISSUE-014` owns destructive single-file overwrite; `ISSUE-018` owns same-session document-ID collisions.
Contribution fit: the controlled reproduction demonstrates a distinct bundle-level stale-GC failure.

## Proposed-Change

Serialize directory-bundle transactions sharing a canonical parent from object enumeration through final garbage collection.
Use one stable parent lock to keep bundle deletion from splitting waiters without leaving a persistent lock file per document.

## Scope-and-Constraints

- Preserve incremental content reuse and complete-manifest publication.
- Hold the stable parent-level lock from object enumeration through final garbage collection.
- Accept sibling-bundle serialization to bound lock-file growth and preserve one lock inode across bundle deletion.
- Do not infer power-loss durability or production frequency from atomic rename or the controlled race.

## Verification

Status: controlled baseline reproduction and rebased source checks pass.
- Baseline: `strace` delayed writer A after its object rename at `47f4abf`; writer B's stale-snapshot GC removed it, and final loading failed. The affected upstream `store.rs` is unchanged through `a96a621`.
- Fix: the shared parent lock made writer B wait for A; both saves completed and final bundle loading passed.
- Full `photocraft-format` tests passed; all three modified directory-save tests passed again after isolating lock files inside cleaned temporary roots.
- Strict format Clippy and `cargo fmt --package photocraft-format -- --check` passed after the cleanup.
- `cargo xtask layers` passed with 27 crates and no violations; `cargo xtask wasm` passed all 21 packages.
- `cargo xtask test-corpus --changed` verified pinned corpora and passed corpus suites before the test-root cleanup.

## Publication-Blockers

- The exact final PR draft is pending; discussion history could not be inspected because the route returned HTTP 404.
- The required independent GPT-6.1 Sol/xhigh review and truthful extra-high public disclosure are unavailable in this runtime.
- Human approval of the exact current draft and target remains pending; no PR was created.

## Next-Action

Summary: Prepare exact PR draft
Action: Record the complete title, body, target, and truthful required disclosure before requesting approval.
Done-When: The exact current draft and target are recorded for human review without unsupported review or model claims.

## Pull-Request-Implementation

Branch: fix/concurrent-native-publication
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Serialize directory-bundle publication and garbage collection across processes without stale-snapshot object deletion.
Commit: `f50b015329e9242842f4d41fa47162b953849d4b`
Push: `origin/fix/concurrent-native-publication`
Checks:
- `cargo test --locked --quiet -p photocraft-format` → passed; three directory-save tests passed again after temp-root cleanup.
- `cargo clippy --locked --quiet -p photocraft-format --all-targets -- -D warnings` → passed.
- `cargo fmt --package photocraft-format -- --check` → passed.
- `cargo xtask layers` → 27 crates, no layering violations.
- `cargo xtask wasm` → all 21 wasm-compatible packages passed.
- `cargo xtask test-corpus --changed` → all pinned corpora verified; corpus suites passed.
