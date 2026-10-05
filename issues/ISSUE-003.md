# ISSUE-003 — compose: repeated full-pattern conversion

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

[S] Per-tile pattern painting repeatedly prepares the same complete immutable pattern image.
Review mapping: `P3`, CHANGED; severity High.

## Reach-and-Impact

[S] CPU pattern effects and pattern fills without a valid imported fill cache reach `Tile::new` per render tile.
[S] Large image-based patterns amplify conversion and allocation work even for small output rectangles.
[A] Net memory and latency benefit depends on pattern size, reuse, and the retained-buffer budget.

## Evidence

- [S] `crates/compose/src/pattern.rs:19-31` reads and premultiplies every pattern pixel into a new float buffer.
- [S] `crates/compose/src/lib.rs:519-522,610-611` prepares patterns inside tile rendering unless a fill cache applies.
- [S] `crates/compose/src/effects.rs:560-561` repeats preparation for pattern effects.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending workload measurements and upstream ownership search.

## Proposed-Change

Share demand-prepared immutable patterns through the existing render context, including its bands.
Bound retained buffers with a fixed internal byte budget and preserve the current individual path when capacity is exhausted.
Choose the budget from measured memory pressure; no value has been established or approved.

## Scope-and-Constraints

- Preserve the document CMYK context, bilinear sampling, premultiplication, rotation, phase, scale, and wrapping.
- Do not eagerly prepare unused patterns or introduce a persistent global cache or user-facing setting.
- Coordinate initialization between render workers without holding a cache lock across dependent parallel work.
- Call-scoped lifetime is not a memory bound; account for retained buffers separately from in-flight work.

## Performance-Evidence

[S] T render tiles can repeat P pattern-pixel conversions T times and allocate T buffers of 16P bytes cumulatively.
[S] Retaining several prepared patterns instead would retain the sum of their buffer sizes until eviction or release.
Measurement: none; no peak-heap reduction or latency gain is established.

## Verification

- Compare conversion counts, peak heap, and release latency for one large pattern and several distinct patterns.
- Compare exact output for rotation, phase, scale, CMYK, wrap-around, and budget-exhaustion fallback.

## Publication-Blockers

- Representative conversion counts, peak heap, budget selection, and output equivalence are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Measure pattern conversion pressure
Action: Capture full-pattern conversion counts and peak heap for repeated and distinct image-based patterns.
Done-When: Record workload sizes, reuse, command, source revision, counts, peak heap, and latency.
