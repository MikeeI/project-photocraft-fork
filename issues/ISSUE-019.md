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
Source: `upstream/main@47f4abfed49e0d2f5b9277287b27dee632530ba4`

## Root-Cause

[S] Independent `PcraftWriter::save_dir` calls can publish conflicting manifests and garbage-collect objects from stale snapshots without transaction coordination.
Review mapping: `E6`, VALID; severity High.

## Reach-and-Impact

Trigger: two directory-bundle writers overlap on one `.pcraft` path with different object sets.
[O] A controlled Linux reproduction delayed writer A immediately after a new object rename; writer B's stale-snapshot GC removed the object before A published its manifest, and final bundle loading failed.
The reproduction proves this interleaving is possible, not its production frequency.

## Evidence

- [S] `save_dir` snapshots existing objects, publishes its manifest, then garbage-collects from that earlier snapshot (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/store.rs#L387-L425).
- [S] Current `store.rs::write_atomic` delegates file replacement to the exclusive-temp `atomic_write` implementation merged in PR #230 (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/store.rs#L457-L461; https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/atomic.rs#L117-L148).
- [S] Desktop autosave uses ID-derived keys in a shared per-user recovery directory (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/apps/photocraft/src/services.rs#L164-L170).
- [S] Each autosave worker serializes only its own queue and does not coordinate independent writer instances (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/autosave.rs#L131-L147).

## Prior-Art

Coverage: upstream issue #203, merged PR #230, and current source reviewed on 2026-10-06; targeted searches found no cross-writer bundle-GC fix.
PR #230 adds exclusive temporary files and atomic file replacement but does not serialize the directory transaction across object enumeration, manifest publication, and GC (https://github.com/storytold/photocraft/pull/230).
`ISSUE-014` owns destructive single-file overwrite; `ISSUE-018` owns same-session document-ID collisions.
Contribution fit: the controlled reproduction demonstrates a distinct bundle-level stale-GC failure.

## Proposed-Change

Serialize each target's directory-bundle object enumeration, writes, manifest publication, and garbage collection across processes.
Unique temporary files and process-local locks alone cannot protect objects referenced by another writer's committed manifest.

## Scope-and-Constraints

- Preserve incremental content reuse and complete-manifest publication.
- Hold cross-process coordination from object enumeration through final garbage collection.
- Unique temp files and process-local locks do not coordinate independent processes.
- Do not infer power-loss durability from atomic rename or claim a reproduced race.

## Verification

Status: controlled cross-process reproduction and focused regression pass.
- Baseline: on Ubuntu 24.04.5, x86_64, Linux 6.8.0-107-generic, `strace` delayed writer A after its successful object rename; writer B completed a baseline save while A was paused; A then published a manifest whose object had been removed, and `load_path` failed.
- Fixed branch: writer B blocked on `flock(3, LOCK_EX)` until writer A finished; both writers exited successfully and final `load_path` verification passed.
- Regression: `cargo test --locked -p photocraft-format --test roundtrip directory_incremental_and_gc -- --exact` passed.

## Publication-Blockers

- Upstream issue/PR prior art and the exact current PR draft remain unresolved.

## Next-Action

Summary: Complete upstream PR evidence
Action: Search current upstream prior art and prepare the exact pull request draft.
Done-When: Record search coverage and exact target/body without publishing.

## Pull-Request-Implementation

Branch: fix/concurrent-native-publication
Base: `upstream/main@47f4abfed49e0d2f5b9277287b27dee632530ba4`
Scope: Coordinate directory-bundle publication and garbage collection across processes so stale snapshots cannot remove objects from a committed manifest.
Commit: `7b272bd`
Push: `origin/fix/concurrent-native-publication`
Checks:
- `cargo test --locked -p photocraft-format` → 68 passed.
- `cargo clippy --locked -p photocraft-format --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → passed; 27 crates, no layering violations.
- `cargo xtask wasm` → passed from this worktree.
- `cargo xtask test-corpus` → all pinned corpora verified; corpus suites passed from this worktree.
- Controlled cross-process baseline failed to load after stale GC; fixed branch serialized B behind A and loaded successfully.
