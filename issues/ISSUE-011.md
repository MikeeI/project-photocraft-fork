# ISSUE-011 — compose: repeated vector-mask compilation

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/112
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

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
Discussions are disabled; no matching CPU mask-compilation reuse was found.
Compose and vector sources remain unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

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
[O] A fixed 64-ellipse mask contains 23,296 flattened vertices; the 24MP canvas has 384 render tiles.
[O] Pinned eight-worker run, five warm samples: complex mask 348.166 → 186.692 ms, polygon control 224.063 → 201.107 ms.
[O] Pinned one-worker run: complex mask 1317.971 → 596.294 ms, polygon control 1055.144 → 539.898 ms.
[O] One-tile 256² control, one worker, nine warm samples: 3.336 → 3.543 ms.
[O] All paired output digests matched; complex 24MP `ae961bbdbab3cc85`, one-tile `2906b7b72fe349fb`.
[O] `/usr/bin/time` on the 24MP complex fixture: peak RSS 502760 → 500472 KiB; this is not evidence of universal memory savings.
Commands: `env RAYON_NUM_THREADS=<1|8> taskset -c <0|0-7> <binary> <vector-curves|vector> <6000 4000|256 256> <5|9>`.
The host remained shared; CPU affinity reduces scheduling variation but does not make these isolated benchmarks.
Decision: adopt for repeated complex geometry, disclose small-case overhead and render-lifetime retention.

## Verification

- Compare flattening and edge-sort counts plus release composition latency on a complex unchanged mask.
- Compare coverage for path operations, fill rules, inversion, density, and tile boundaries.

## Publication-Blockers

None.
The draft scopes the observed benefit and discloses the absence of a geometry byte budget.

## Next-Action

Summary: Await upstream vector review
Action: Respond to substantive maintainer feedback on PR #112.
Done-When: Record the upstream decision or requested follow-up.

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
- Fixed complex-mask and negative-control runs establish workload-specific adoption evidence.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(compose): reuse compiled vector masks within each render`

```markdown
## Summary

Compile each required vector mask once per CPU render and share immutable geometry across tiles and bands.
Coverage rasterization keeps rectangle-local mutable state; flattening tolerance, path operations, fill rules, density, inversion, and disabled/empty behavior stay unchanged.
The render owns the cache, so document edits cannot reuse a prior render's geometry.
This does not allocate a full-canvas mask or replace the separate GPU combined-mask cache.

## Validation

- Compose tests: 97 unit and 3 integration tests passed.
- Vector tests: 23 passed, 1 ignored.
- Affected-crate Clippy with `--all-targets -- -D warnings`, formatting, layering, and WASM checks passed.
- Independent source review found no introduced blocker.

A fixed 6000×4000 RGB fixture uses 64 cubic ellipses with 23,296 flattened vertices.
With eight Rayon workers pinned to CPUs 0–7, five warm samples measured median composition time of 348.166 ms before and 186.692 ms after.
The fixture digest matched (`ae961bbdbab3cc85`).
A 1000-point polygon control measured 224.063 → 201.107 ms with matching output.
A one-tile 256² complex-mask control measured 3.336 → 3.543 ms, showing that reuse has overhead when there is nothing to reuse.

These are shared-host observations, not isolated benchmarks or a universal speedup claim.
An earlier unpinned polygon run regressed; the complex-path case and both controls were fixed before the final comparison.
Peak process RSS on the 24MP complex fixture was 502760 → 500472 KiB.
Geometry is retained until the render ends, without a byte budget; documents with many distinct masks may retain more geometry than the old tile-local path.
The measured single-mask workload does not bound that tradeoff.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
