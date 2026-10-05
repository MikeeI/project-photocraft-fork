# ISSUE-001 — compose: per-pixel gradient stop preparation

State: Investigating
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

[S] Gradient sampling rebuilds unchanged stop tables inside the pixel loop.
Review mapping: `P1`, VALID; severity High.

## Reach-and-Impact

[S] CPU canvas and live-preview composition reaches `paint_fx` for gradient overlays, strokes, and glows.
[S] Each painted pixel repeats table construction and stop conversion; actual UI latency remains unmeasured.

## Evidence

- [S] `crates/compose/src/effects.rs:448-450` collects color stops and nonempty opacity stops on every sample.
- [S] `crates/compose/src/effects.rs:474-480,577-587` invokes that sampling for each pixel with positive coverage.
- [S] `crates/compose/src/lib.rs:597-637` separately repeats selected stop-color conversions for gradient fills.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending measurement and upstream ownership search.

## Proposed-Change

Prepare color and opacity stops before the pixel loops in the existing paint/fill owners.
Keep preparation within the document's CMYK scope and preserve the existing interpolation exactly.

## Scope-and-Constraints

- Preserve stop order, duplicate positions, empty-stop behavior, alpha, and floating-point interpolation.
- Do not replace exact sampling with an approximate LUT or add a persistent cache.
- Gains overlap with reduced effect working area in `ISSUE-002`; do not add independent speedup estimates.

## Performance-Evidence

[S] For N painted pixels, nonempty color tables are constructed N times rather than once per preparation scope.
[S] Nonempty opacity tables incur another N constructions; fill gradients repeat conversion without those allocations.
Measurement: none; no runtime speedup or observed user harm is claimed.

## Verification

- Compare allocation counts, conversion calls, and release preview latency on an identical gradient workload.
- Compare pixels for RGB, CMYK, Lab, opacity stops, and duplicate stop positions.
- Existing `gradient_overlay_follows_angle_and_reverse` is partial coverage, not a complete color-contract proof.

## Publication-Blockers

- Representative baseline and behavior-preserving implementation verification are missing.
- Upstream prior art and contribution fit remain unresolved.
- The user authorized publication of a verified fix; the exact PR draft and required evidence are pending.

## Next-Action

Summary: Measure gradient preparation
Action: Capture allocation and conversion counts for a release gradient-overlay preview with nonempty opacity stops.
Done-When: Record the workload, command, source revision, counts, and latency without claiming an implemented improvement.
