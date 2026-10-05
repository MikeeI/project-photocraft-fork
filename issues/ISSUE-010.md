# ISSUE-010 — compose: unused proxy source-column copies

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

[S] Proxy downsampling materializes each sampled source row before discarding all but every kth column.
Review mapping: `P10`, VALID; severity Medium.

## Reach-and-Impact

[S] Large-document proxies for previews and eligible thumbnails downsample raster, mask, cache, and channel surfaces.
[S] The existing adjustment proxy is revision-cached; this is proxy-build work, not every slider-frame work.
Measurement: source-byte volume and proxy-build latency remain unmeasured.

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
Measurement: those counts do not establish proportional memory-bus traffic or latency improvement.

## Verification

- Compare copied encoded bytes, temporary allocations, and release proxy-build latency at fixed factors.
- Compare output bytes across pixel types, defaults, sparse tiles, and negative coordinates.

## Publication-Blockers

- Representative byte counts, timing, and encoded-output equivalence are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Complete encoded proxy verification
Action: Measure the applied encoded sampler and finish the remaining WASM gate.
Done-When: Record representative performance, complete checks, and the independent PR handoff.

## Pull-Request-Implementation

Branch: `perf/stride-proxy-source-pixels`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Add raster-owned encoded nearest-neighbor sampling without unused source-column copies.
Commit: Pending.
Push: Pending.
Checks:
- Independent compose tests: 97 unit and three integration tests passed; 14 raster tests passed.
- Clippy and layering passed; WASM remains incomplete.
- Existing proxy test covers F32 NaN payloads, signed zero, sparse defaults, and negative coordinates.
- Source review found no blocker; worktree `.git/omp-worktrees/issue-010` remains unpublished.
