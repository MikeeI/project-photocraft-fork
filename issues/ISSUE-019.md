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
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Independent `PcraftWriter::save_dir` calls can publish conflicting manifests and garbage-collect objects from stale snapshots without transaction coordination.
Review mapping: `E6`, VALID; severity High.

## Reach-and-Impact

Trigger: two directory-bundle writers overlap on one `.pcraft` path with different object sets.
[S] A writer's GC uses an object snapshot taken before another writer publishes a newer manifest, so it can remove an object that manifest reuses.
[A] This interleaving is source-derived, not runtime-reproduced; production frequency is unknown.

## Evidence

- [S] `save_dir` snapshots existing objects, publishes its manifest, then garbage-collects from that earlier snapshot (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/store.rs#L387-L425).
- [S] Current `store.rs::write_atomic` delegates file replacement to the exclusive-temp `atomic_write` implementation merged in PR #230 (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/store.rs#L457-L461; https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/atomic.rs#L117-L148).
- [S] Desktop autosave uses ID-derived keys in a shared per-user recovery directory (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/apps/photocraft/src/services.rs#L164-L170).
- [S] Each autosave worker serializes only its own queue and does not coordinate independent writer instances (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/autosave.rs#L131-L147).

## Prior-Art

Coverage: issue #203, merged PR #230, and current upstream source reviewed on 2026-10-06; targeted search found no cross-writer bundle-GC fix.
PR #230 adds exclusive temporary files and atomic file replacement but does not serialize the directory transaction across object enumeration, manifest publication, and GC (https://github.com/storytold/photocraft/pull/230).
`ISSUE-014` owns destructive single-file overwrite; `ISSUE-018` owns same-session document-ID collisions.
Contribution fit: verify the stale-snapshot bundle-GC interleaving before implementation.

## Proposed-Change

Serialize each target's directory-bundle object enumeration, writes, manifest publication, and garbage collection across processes.
Unique temporary files and process-local locks alone cannot protect objects referenced by another writer's committed manifest.

## Scope-and-Constraints

- Preserve incremental content reuse and complete-manifest publication.
- Hold cross-process coordination from object enumeration through final garbage collection.
- Unique temp files and process-local locks do not coordinate independent processes.
- Do not infer power-loss durability from atomic rename or claim a reproduced race.

## Verification

Status: source-proven directory-GC interleaving; no runtime reproduction performed.
- Seed shared object `X`; let writer A snapshot it, publish a manifest excluding it, and pause A before GC.
- Let writer B snapshot the still-present `X` and publish a manifest reusing it; resume A's stale GC and verify B's manifest remains loadable with every referenced object present.

## Publication-Blockers

- The source-derived interleaving still needs controlled execution in a disposable directory.
- The cross-process coordination choice, implementation, focused checks, and exact PR draft remain unresolved.

## Next-Action

Summary: Reproduce bundle-GC interleaving
Action: Coordinate two independent writers so stale-snapshot garbage collection overlaps a newer manifest publication.
Done-When: Record ordering, final manifest object closure, and bundle loadability.

## Pull-Request-Implementation

Branch: fix/concurrent-native-publication
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Coordinate directory-bundle publication and garbage collection across processes so stale snapshots cannot remove objects from a committed manifest.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
