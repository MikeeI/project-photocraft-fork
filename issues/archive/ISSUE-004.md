# ISSUE-004 — compose: repeated effect metadata derivation

State: Archived
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

[S] Render tiles repeatedly derive unchanged layer bounds and effect identities from complete surface metadata.
Review mapping: `P4`, CHANGED; severity High.

## Reach-and-Impact

[S] CPU refreshes of effect layers and groups repeat metadata work for every intersecting render tile.
[S] Warm bounds caches avoid pixel rescans but still traverse tile metadata under a shared mutex.
[A] Actual mutex waiting and UI latency depend on workload and scheduling and remain unmeasured.

## Evidence

- [S] `crates/compose/src/lib.rs:651,911-912,1052-1059` queries bounds and identity inside tile processing.
- [S] `crates/compose/src/lib.rs:966-994` traverses surface tiles, groups, masks, and effect data for identity.
- [S] `crates/compose/src/bounds.rs:47-68` holds the bounds-cache mutex across tile lookup and union.
- [S] `crates/compose/src/lib.rs:461-475,650-651,1053` proves bounds depend on differing canvas arguments.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending measurements and upstream ownership search.

## Proposed-Change

Derive effect identities once per render call and share immutable metadata between tiles and bands.
Prepare culling-context bounds separately from document-canvas bounds; never key bounds by layer alone.

## Scope-and-Constraints

- Preserve the artificial large canvas used by culling and the actual canvas used for effect regions.
- Preserve existing map invalidation, COW pinning, and demand-driven effect-map construction.
- Do not build unused effects eagerly or introduce a second persistent cache.
- Describe residual metadata traversal, not repeated full-pixel scans already avoided by the bounds cache.

## Performance-Evidence

[S] T render tiles and S surface tiles can incur `O(T*S)` metadata visits despite warm pixel bounds.
[S] Group identity additionally traverses children; source proves locking scope but not measured contention.
[S] Eight off-canvas 24MP COW-shared effect layers reduce the derived cull-bound calls from 3072 to eight.
[O] Eight-worker medians: off-canvas fixture 207.982 → 192.292 ms; ordinary halo fixture 1253.727 → 1554.348 ms.
[O] Earlier halo run: 1854.270 → 1953.310 ms; all paired output digests matched.
[O] One-worker, CPU-0-pinned off-canvas median: 583.057 → 520.490 ms over nine warm samples.
These shared-host measurements show a workload tradeoff, not a robust general improvement.
Decision: reject the current 144-line metadata-cache diff for upstream publication.
The modest culling benefit does not justify its complexity and observed ordinary-effect slowdown.
This rejects the tested implementation, not the existence of repeated metadata work or every possible future approach.

## Verification

- Count bounds calls, metadata visits, identity derivations, mutex waiting, and release latency.
- Compare output and culling for raster, fill, group, and artboard layers across tile and thread counts.

## Publication-Blockers

The measured tradeoff does not justify publishing this implementation as a performance improvement.

## Next-Action

Summary: —
Action: None.
Done-When: None.

## Pull-Request-Implementation

Branch: `perf/prepare-effect-metadata`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Prepare render-owned effect metadata with distinct culling and document-canvas bounds.
Commit: `a0941198d7e0a5c504e7ad8b32c6f67b1c511efe`
Push: `MikeeI/project-photocraft-fork:perf/prepare-effect-metadata`
Checks:
- Independent compose tests: 97 unit and three integration tests passed; Clippy and layering passed.
- WASM passed for all 20 packages; GPT-6.1 Sol/xhigh source review found no correctness blocker.
- The verified experimental branch remains available; no upstream PR was created.

## Archive

Archive-Reason: Not-Worth-Pursuing
Detail: None.
Evidence: The paired workloads in Performance-Evidence did not establish a satisfactory benefit/complexity tradeoff.
Checked: 2026-10-05
