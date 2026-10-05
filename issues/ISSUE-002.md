# ISSUE-002 — compose: redundant effect application halo

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/110
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

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
Discussions are disabled; no matching application-halo correction was found.
Related banding work https://github.com/storytold/photocraft/pull/71 has a distinct root cause.
The affected compose sources remain unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

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

None.
Remaining coverage limits are explicit in the draft; publication is authorized by the current user request.

## Next-Action

Summary: Await upstream halo review
Action: Respond to substantive maintainer feedback on PR #110.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/trim-effect-application-halo`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Remove application halos while retaining complete neighborhood effect maps.
Commit: `ca81013cb18d18e118587499f988a4e41db68c39`
Push: `MikeeI/project-photocraft-fork:perf/trim-effect-application-halo`
Checks:
- Independent compose tests: 97 unit and three integration tests passed; Clippy and layering passed.
- Existing tile-parity test now compares full output at U8/U16/F32 and tile sizes 1/7/33.
- WASM passed for all 20 packages; independent GPT-6.1 Sol/xhigh source review found no blocker.
- Formatting and diff checks passed; no fork tracking files enter the contribution.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(compose): avoid redundant effect-halo rendering`

```markdown
## Summary

Both regular and clipped-effect composition render content over the requested output rectangle rather than inflating it by the effect margin.
Full bounds-plus-margin effect maps remain unchanged, retaining neighborhood inputs while avoiding repeated halo content work per tile.
Absolute paint coordinates, effect reach, clipping, and map construction keep their existing owners.

## Validation

- `cargo test --locked -p photocraft-compose`: 97 unit and 3 integration tests passed.
- The existing tile-parity test now compares complete shadow-plus-stroke output at U8/U16/F32 and tile sizes 1, 7, and 33.
- `cargo clippy --locked -p photocraft-compose --all-targets -- -D warnings`: passed.
- `cargo xtask layers` and `cargo xtask wasm`: passed.
- Formatting, diff checks, and independent source review completed.

A synthetic 6000×4000 RGB shadow fixture with eight Rayon workers and five warm samples measured 3018.106 ms before and 1621.630 ms after.
The fixture output digest matched (`87052d1c71274d2e`).
These uncontrolled shared-host timings are workload-specific, not a universal speedup or blanket pixel-equivalence claim.
The base is ff53be7; affected compose sources remain unchanged at upstream 7e7864a.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
