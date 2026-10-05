# ISSUE-002 — compose: redundant effect application halo

State: Implementing
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Content rendering and effect application retain a halo after neighborhood results already exist in effect maps.
Review mapping: `P2`, VALID; severity High.

## Reach-and-Impact

[S] Every CPU render tile intersecting a visible effect layer reaches this path, including warm-cache refreshes.
[S] Clipped effect layers use the same enlarged-content pattern.
[S] Larger temporary buffers and pixel loops are source-proven; end-to-end impact is unmeasured.

## Evidence

- [S] `crates/compose/src/lib.rs:904-912,1143-1157` inflates the content rectangle by the effect margin.
- [S] `crates/compose/src/lib.rs:1048-1086` builds and shares maps over the complete effect region.
- [S] `crates/compose/src/effects.rs:1022-1226` applies maps pixel-locally and writes only `backdrop.rect`.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending numerical equivalence and upstream ownership search.

## Proposed-Change

Restrict content rendering and effect application to the output rectangle in both effect branches.
Keep the halo exclusively where effect-map neighborhood calculations require it.

## Scope-and-Constraints

- Preserve full map regions, map invalidation, effect reach, culling, clipping order, and absolute paint coordinates.
- Do not shrink shadow, distance-field, bevel, or other neighborhood inputs during map construction.
- Nested groups, vector masks, knockout, and clipped effects require explicit output comparison.

## Performance-Evidence

[S] A full 256-square render tile processes `(256 + 2m)^2` content pixels for margin m but outputs only `256^2`.
[S] The enlarged region also sizes effect working buffers after map lookup.
[O] Synthetic 6000×4000 RGB shadow fixture, eight Rayon workers, five warm samples: 3018.106 → 1621.630 ms.
[O] Before/after digest: `87052d1c71274d2e`; cold calls were 4563.393 → 3949.545 ms.
Shared-host timing is not a controlled end-to-end UI benchmark.

## Verification

- Compare cold and warm refreshes for processed pixels, allocations, and release latency.
- Run existing `effects_render_identically_in_tiles` and `parallel_tiles_match_single_pass` checks after implementation.
- Compare old and new output across tile boundaries, clipped effects, vector masks, shadows, strokes, and bevels.

## Publication-Blockers

- Numerical before/after equivalence and representative timing evidence are missing.
- Existing tile tests alone do not cover every affected effect combination.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Complete halo verification
Action: Resume unfinished WASM validation and complete the independent PR handoff.
Done-When: Record complete gates, commit, push, and an evidence-scoped draft.

## Pull-Request-Implementation

Branch: `perf/trim-effect-application-halo`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Remove application halos while retaining complete neighborhood effect maps.
Commit: Pending.
Push: Pending.
Checks:
- Independent compose tests: 97 unit and three integration tests passed; Clippy and layering passed.
- Existing tile-parity test now compares full output at U8/U16/F32 and tile sizes 1/7/33.
- WASM remains incomplete; independent source review found no blocker.
- Worktree: `.git/omp-worktrees/issue-002`; further work paused when the user prioritized finished PRs.
