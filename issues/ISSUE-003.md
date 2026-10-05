# ISSUE-003 — compose: repeated full-pattern conversion

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/104
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

Coverage: local ledger, upstream issues, open/closed PRs, and releases searched on 2026-10-05.
Discussions are disabled; no matching root-cause implementation was found in the searched material.
Related banding work https://github.com/storytold/photocraft/pull/71 does not prepare patterns across tiles.
Currentness: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c` leaves `crates/compose` unchanged.

## Proposed-Change

Share demand-prepared immutable patterns through the existing render context, including its bands.
Bound retained buffers with a fixed internal byte budget and preserve the current individual path when capacity is exhausted.
Use a conservative 64 MiB retained-pixel budget and 64-entry cap; this is not a total-render-memory limit.

## Scope-and-Constraints

- Preserve the document CMYK context, bilinear sampling, premultiplication, rotation, phase, scale, and wrapping.
- Do not eagerly prepare unused patterns or introduce a persistent global cache or user-facing setting.
- Coordinate initialization between render workers without holding a cache lock across dependent parallel work.
- Call-scoped lifetime is not a memory bound; account for retained buffers separately from in-flight work.

## Performance-Evidence

[S] T render tiles can repeat P pattern-pixel conversions T times and allocate T buffers of 16P bytes cumulatively.
[S] Retaining several prepared patterns instead would retain the sum of their buffer sizes until eviction or release.
[O] A 6000×4000 RGB pattern-fill fixture uses a 1024-square pattern, scale 0.73, angle 17°, and phase (3, 7).
[O] Eight Rayon workers, three warm release samples: median 3543.639 → 1216.037 ms.
[O] Cold calls: 3671.821 → 1108.691 ms; output digest matches at `6b547cf803b5d41b`.
Command: `RAYON_NUM_THREADS=8 <binary> pattern 6000 4000 3`.
The temporary `review-perf` example used separate base and contribution target directories on the shared host.
[S] This fixture retains one 16 MiB converted buffer rather than preparing it independently for every tile.
Peak heap and driver memory were not measured; over-budget fallback conversions remain unbounded by the retained-buffer cap.

## Verification

- [O] `cargo test --locked -p photocraft-compose`: 97 unit and three integration tests passed.
- [O] Existing `identity_placement_tiles_exactly` now verifies shared reuse and zero-budget output equivalence.
- [O] `cargo clippy --locked -p photocraft-compose --all-targets -- -D warnings`: passed.
- [O] `cargo xtask layers` and `cargo xtask wasm`: passed.
- [S] Independent GPT-6.1 Sol/xhigh review verified reservation ordering, immutable pattern identity, worker CMYK scope, and call-owned lifetime.
- Runtime RGB parity does not establish measured CMYK-profile parity or peak-memory improvement.

## Publication-Blockers

None.
The user explicitly requested immediate publication of completed PRs on 2026-10-05.

## Next-Action

Summary: Await upstream pattern review
Action: Respond to substantive maintainer feedback on PR #104.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/reuse-pattern-tiles`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Invocation-local bounded pattern preparation shared by CPU render tiles and bands.
Commit: `4803da25fe7a121bce219b4a09da039d76d1dcc3`
Push: `MikeeI/project-photocraft-fork:perf/reuse-pattern-tiles`
Checks:
- Affected-crate tests, Clippy, layering, and WASM passed in the independent worktree.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(compose): reuse bounded pattern preparation across render tiles`

```markdown
## Summary

Pattern fills and effects repeatedly convert the entire source pattern into premultiplied float pixels for each CPU render tile.
This shares demand-prepared immutable patterns within a rendering invocation, including streaming bands.

The preparation owner reserves capacity before initialization and releases its mutex before conversion.
It retains at most 64 MiB of prepared pixel buffers and 64 entries.
Patterns that exceed that capacity use the existing direct-conversion path.
This bounds retained cache payload, not total render memory or concurrent fallback allocations.

Pattern lookup precedence, bilinear sampling, wrapping, phase, rotation, scale, and premultiplication stay with their existing owners.
Preparation executes in the requesting worker's document CMYK scope; no converted data persists across rendering invocations.

## Validation

- `cargo test --locked -p photocraft-compose`: 97 unit tests and 3 integration tests passed.
- The existing pattern-placement test now checks reuse and zero-budget fallback equivalence.
- `cargo clippy --locked -p photocraft-compose --all-targets -- -D warnings`: passed.
- `cargo xtask layers` and `cargo xtask wasm`: passed.
- Independent source review covered concurrency, lookup identity, CMYK scope, fallback, and lifetime.

A synthetic 6000×4000 RGB release fixture used a 1024×1024 pattern, scale 0.73, angle 17°, and phase (3, 7).
With eight Rayon workers and three warm samples, median composition time was 3543.639 ms before and 1216.037 ms after.
The before/after output digest matched.
The prepared pattern occupies 16 MiB in this fixture.
These shared-host timings are workload-specific; peak heap and tagged-CMYK runtime parity were not measured.

This branch is independent of other proposed compositor optimizations.
It is based on ff53be7; the affected compose sources remain unchanged at upstream 7e7864a.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
