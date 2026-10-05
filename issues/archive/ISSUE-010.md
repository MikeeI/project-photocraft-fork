# ISSUE-010 — compose: unused proxy source-column copies

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

[S] Proxy downsampling materializes each sampled source row before discarding all but every kth column.
Review mapping: `P10`, VALID; severity Medium.

## Reach-and-Impact

[S] Large-document proxies for previews and eligible thumbnails downsample raster, mask, cache, and channel surfaces.
[S] The existing adjustment proxy is revision-cached; this is proxy-build work, not every slider-frame work.
[O] The tested encoded gather removes temporary source-row copies but regresses measured factor-four latency.

## Evidence

- [S] `crates/compose/src/proxy.rs:25-30` copies a full row and selects only strided pixels from it.
- [S] `crates/raster/src/lib.rs:444-466` allocates and copies every requested encoded pixel byte.
- [S] `crates/compose/src/proxy.rs:35-69` applies this path throughout the document's downsampled surfaces.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending measurements and review of the raster-owned read boundary.

## Proposed-Change

Read only needed encoded pixels through a strided raster-owned row reader or direct tilewise copies.
Write into the already allocated output row without materializing unused source columns.

## Scope-and-Constraints

- Preserve exact encoded bytes, default pixels, negative coordinates, Euclidean indexing, and factor clamping.
- Do not introduce float decoding and re-encoding that may change F32 bit patterns or NaNs.
- Retain nearest-neighbor proxy semantics rather than changing sampling quality.

## Performance-Evidence

[S] For dense source width W, height H, and factor k, copied source pixels are approximately `W*H/k`.
[S] Output needs approximately `W*H/k^2` sampled pixels, excluding edge rounding.
[S] At factor four, four-byte samples every 16 bytes still touch essentially every conventional 64-byte cache line.
[O] Initial 24MP factor-four medians: baseline 28.930 ms, candidate 34.312 ms.
[O] After hoisting coordinate calculations and iterating encoded source chunks: baseline 24.418 ms, candidate 32.718 ms.
[O] Reversed-order CPU-0-pinned comparison, 15 warm samples: baseline 17.210 ms, candidate 19.500 ms.
[O] All output digests matched (`2d0a963e0d932325`).
The host remained shared, but neither implementation established a latency benefit.
Decision: reject this candidate rather than exchange efficient contiguous copies for a slower gather.
Other factors and architectures are not proven slower; these results do not invalidate every future approach.

## Verification

- Compare copied encoded bytes, temporary allocations, and release proxy-build latency at fixed factors.
- Compare output bytes across pixel types, defaults, sparse tiles, and negative coordinates.

## Publication-Blockers

The measured factor-four regression blocks publication as a performance improvement.

## Next-Action

Summary: —
Action: None.
Done-When: None.

## Pull-Request-Implementation

Branch: `perf/stride-proxy-source-pixels`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Add raster-owned encoded nearest-neighbor sampling without unused source-column copies.
Commit: `857a732baaa36b0a723cd86bdde3b3aa1cc145ea`
Push: `MikeeI/project-photocraft-fork:perf/stride-proxy-source-pixels`
Checks:
- Independent compose tests: 97 unit and three integration tests passed; 14 raster tests passed.
- Clippy, layering, and all 20 WASM package checks passed.
- Existing proxy test covers F32 NaN payloads, signed zero, sparse defaults, and negative coordinates.
- GPT-6.1 Sol/xhigh reviewed the final iterator delta and found no correctness blocker.
- The verified experimental branch remains available; no upstream PR was created.

## Archive

Archive-Reason: Not-Worth-Pursuing
Detail: None.
Evidence: Three paired factor-four comparisons in Performance-Evidence failed the performance adoption gate.
Checked: 2026-10-05
