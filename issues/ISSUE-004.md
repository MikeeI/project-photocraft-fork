# ISSUE-004 — compose: repeated effect metadata derivation

State: Investigating
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
Measurement: none; no thread-scaling or end-to-end speedup is established.

## Verification

- Count bounds calls, metadata visits, identity derivations, mutex waiting, and release latency.
- Compare output and culling for raster, fill, group, and artboard layers across tile and thread counts.

## Publication-Blockers

- Measured metadata cost and context-preserving output verification are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Measure effect metadata traversal
Action: Capture repeated bounds and identity work during a warm-cache refresh of a large effect layer.
Done-When: Record tile counts, metadata visits, measured waiting, command, source revision, and latency.
