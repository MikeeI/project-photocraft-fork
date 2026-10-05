# ISSUE-001 — compose: per-pixel gradient stop preparation

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/103
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

Coverage: local ledger plus upstream issues, open/closed PRs, and release history searched on 2026-10-05.
Upstream discussions are disabled; no matching root-cause implementation was found in the searched material.
Related rendering work: https://github.com/storytold/photocraft/pull/71 and https://github.com/storytold/photocraft/pull/84.
Neither owns CPU per-pixel gradient-stop preparation.
Currentness: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c` leaves `crates/compose` unchanged from the recorded base.

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
[O] Release RGB overlay, 6000×4000, eight Rayon workers, five warm samples: median 362.752 → 235.813 ms.
[O] Same-size gradient fill: median 129.264 → 117.642 ms.
[O] Before/after F32-output digests match: overlay `f1e5009d3b711253`; fill `9b8574593cec74df`.
Commands: `RAYON_NUM_THREADS=8 <binary> gradient-effect 6000 4000 5` and `gradient-fill 6000 4000 5`.
The temporary `review-perf` example used separate release target directories for the base and contribution.
These synthetic shared-host measurements do not establish universal UI, tagged-CMYK, or GPU speedups.

## Verification

- [O] `cargo test --locked -p photocraft-compose`: 97 unit and three integration tests passed.
- [O] `cargo clippy --locked -p photocraft-compose --all-targets -- -D warnings`: passed.
- [O] `cargo xtask layers`: 26 crates, no violations.
- [O] `cargo xtask wasm`: all 20 checked packages passed; existing unrelated dead-code warnings remain.
- [O] Formatting and `git diff --check` passed.
- [S] Independent GPT-6.1 Sol/xhigh review found no blocking defect in stop order, alpha, or CMYK-scope ownership.
- Runtime parity is limited to the exercised fixtures; other edge contracts were source-reviewed.

## Publication-Blockers

None.
The user explicitly requested immediate publication of completed PRs on 2026-10-05.

## Next-Action

Summary: Await upstream gradient review
Action: Respond to substantive maintainer feedback on PR #103.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/prepare-gradient-stops`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Prepare CPU effect and fill gradient stops outside their pixel loops.
Commit: `c5036cf9b6a64ff03121f7052792d5a97653f72c`
Push: `MikeeI/project-photocraft-fork:perf/prepare-gradient-stops`
Checks:
- Affected-crate tests, Clippy, layering, WASM, and diff checks passed in the independent worktree.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(compose): prepare gradient stops outside pixel loops`

```markdown
## Summary

CPU gradient effects rebuild color and opacity stop vectors for every painted pixel.
Gradient fills also repeat stop-color conversion inside the pixel loop.
This prepares those values once per paint/fill call while retaining the existing interpolation.

Effect and fill semantics remain separate: empty effects use their grayscale ramp, while empty fills stay transparent.
Preparation is invocation-local rather than a persistent cache, so converted stops are not reused across document color contexts.
The public sampling API remains available.

## Validation

- `cargo test --locked -p photocraft-compose`: 97 unit tests and 3 integration tests passed.
- `cargo clippy --locked -p photocraft-compose --all-targets -- -D warnings`: passed.
- `cargo xtask layers` and `cargo xtask wasm`: passed.
- Formatting and diff checks passed.
- Independent source review covered stop ordering, duplicate positions, alpha, and CMYK-scope ownership.

A synthetic 6000×4000 RGB release workload with eight Rayon workers and five warm samples measured:

- Gradient overlay: median 362.752 ms before, 235.813 ms after.
- Gradient fill: median 129.264 ms before, 117.642 ms after.

Before/after output digests matched for both fixtures.
These are shared-host synthetic CPU measurements, not a claim of universal UI, tagged-CMYK, or GPU speedup.
This branch is based on ff53be7; the affected compose sources remain unchanged at upstream 7e7864a.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
