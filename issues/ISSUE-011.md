# ISSUE-011 — compose: repeated vector-mask compilation

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

[S] CPU mask evaluation recompiles unchanged vector geometry for each requested render tile.
Review mapping: `P11`, VALID; severity Medium.

## Reach-and-Impact

[S] CPU content, group, and adjustment masking reaches `vector_mask_values` during tile rendering.
[S] Complex nonempty enabled paths repeat flattening, edge construction, and sorting.
Measurement: compilation share of total composition latency remains unmeasured.

## Evidence

- [S] `crates/compose/src/lib.rs:492-493,530` invokes vector-mask evaluation from content rendering.
- [S] `crates/vector/src/lib.rs:40-45,74-85` compiles a rasterizer before each coverage calculation.
- [S] `crates/vector/src/raster.rs:64-92` constructs and sorts edges during compilation.
- [S] `crates/vector/src/raster.rs:143-169` renders through immutable geometry with local row state.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending complex-path measurements and upstream ownership search.

## Proposed-Change

Compile each needed vector mask once per render call and share the immutable geometry between tiles.
Compute coverage for each requested rectangle using separate local rendering state.

## Scope-and-Constraints

- Preserve flattening tolerance, density, path operations, fill rules, inversion, and disabled or empty-mask semantics.
- Do not allocate a full-canvas mask merely to avoid geometry compilation.
- The GPU combined-mask cache already exists; this finding is specific to direct CPU evaluation.
- Do not introduce initialization waits that can deadlock with nested Rayon work.

## Performance-Evidence

[S] T tiles currently cause T geometry compilations for the same enabled nonempty mask.
[S] Pixel coverage remains necessary for every output region after compilation reuse.
[O] Synthetic 6000×4000 RGB fixture with a 1000-point polygon and eight Rayon workers: output digest matched.
[O] Three warm release samples measured median 444.330 ms before and 493.314 ms after.
[O] Cold calls were 410.603 → 624.191 ms; digest `8e8bafeb6140d949`.
Command: `RAYON_NUM_THREADS=8 <binary> vector 6000 4000 3`.
The shared host was busy; this result establishes no improvement and does not justify publication.

## Verification

- Compare flattening and edge-sort counts plus release composition latency on a complex unchanged mask.
- Compare coverage for path operations, fill rules, inversion, density, and tile boundaries.

## Publication-Blockers

- The measured fixture did not improve; isolate compilation cost and repeat under controlled load before adoption.
- Geometry retained across the render has no byte/count budget; peak-memory tradeoffs remain unmeasured.
- No PR draft is approved as ready; the source branch is pushed but intentionally not published.

## Next-Action

Summary: Resolve vector performance regression
Action: Determine whether representative complex masks justify retained geometry and initialization overhead.
Done-When: Record controlled timing and memory evidence supporting adoption, revision, or rejection.

## Pull-Request-Implementation

Branch: `perf/reuse-vector-mask-geometry`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Render-owned compiled vector geometry with rectangle-local mutable raster state.
Commit: `1f24dd6770cba27db290bf0d40177fd09eaf04f0`
Push: `MikeeI/project-photocraft-fork:perf/reuse-vector-mask-geometry`
Checks:
- Compose: 97 unit and three integration tests passed; vector: 23 passed and one ignored.
- Independent Clippy, layering, and WASM checks passed.
- GPT-6.1 Sol/xhigh source review found no blocker; it does not establish a performance benefit.
- Worktree: `.git/omp-worktrees/issue-011`; publication paused on the observed performance result.
