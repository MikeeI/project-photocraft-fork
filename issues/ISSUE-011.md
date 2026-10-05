# ISSUE-011 — compose: repeated vector-mask compilation

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
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
Measurement: no speedup is established; simple rectangular paths may offer little benefit.

## Verification

- Compare flattening and edge-sort counts plus release composition latency on a complex unchanged mask.
- Compare coverage for path operations, fill rules, inversion, density, and tile boundaries.

## Publication-Blockers

- Representative geometry-compilation measurements and coverage equivalence are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Measure vector-mask compilation work
Action: Capture geometry compilation counts and timing for a complex mask rendered across multiple tiles.
Done-When: Record path complexity, tile count, tolerance, command, compilation counts, and composition latency.
