# ISSUE-006 — UI: redundant active-layer clone

State: Implementing
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The Layers panel clones the active layer despite already retaining the owning document snapshot.
Review mapping: `P6`, CHANGED; severity reduced from High to Medium because actual UI cost is unmeasured.

## Reach-and-Impact

[S] Each Layers-panel repaint with an active layer performs the clone.
[S] Large surfaces copy tile-map metadata; active groups recursively copy their subtree metadata.
[S] COW shares pixel buffers, so this is not a full raster-pixel copy.

## Evidence

- [S] `crates/ui-egui/src/panels.rs:1066-1068` retains the document Arc and then calls `.cloned()` on the layer.
- [S] `crates/ui-egui/src/panels.rs:1102-1153,1259-1274` reads controls before executing collected actions.
- [S] `crates/raster/src/lib.rs:41-45` derives Clone for a surface containing a tile-to-Arc BTreeMap.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: potentially small borrowing correction; measurements and upstream ownership search remain open.

## Proposed-Change

Remove `.cloned()` and borrow the active layer from the local `Arc<Document>` for the panel's read phase.
Keep the owning Arc alive and preserve the existing action execution order.

## Scope-and-Constraints

- Do not remove the document Arc clone that owns the snapshot lifetime.
- Do not introduce a cache, new shared mutation, or refactoring of unrelated controls.
- Distinguish copied strings, vectors, tile maps, and Arc counts from shared pixel data.

## Performance-Evidence

[S] Clone cost scales with active-layer metadata and recursively with an active group's subtree and tile maps.
Measurement: allocation count, frame latency, and user-visible impact have not been measured.

## Verification

- Compare per-frame allocations and timing for a large active raster layer and a nested active group.
- Visually verify blend, opacity, fill, and lock controls after the borrowing change.

## Publication-Blockers

- Representative clone-cost evidence and actual UI verification are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Complete borrowing UI verification
Action: Verify active-group controls and finish the independent UI gates.
Done-When: Record representative interaction evidence and complete checks before committing.

## Pull-Request-Implementation

Branch: `perf/borrow-active-layer`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Borrow the active layer from the existing local document Arc.
Commit: Pending.
Push: Pending.
Checks:
- Independent source review found no blocker.
- Earlier shared-target checks are not accepted as branch-specific evidence.
- Worktree: `.git/omp-worktrees/issue-006`; independent gates remain incomplete after publication reprioritization.
