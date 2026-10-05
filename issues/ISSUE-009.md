# ISSUE-009 — compose: unnecessary backdrop snapshots

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/109
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Group and adjustment composition copies original backdrop buffers beyond their actual read dependencies.
Review mapping: `P9`, VALID; severity Medium.

## Reach-and-Impact

[S] CPU tile composition snapshots pass-through groups even when neither mixing nor visible clipped layers need it.
[S] Ordinary adjustment composition creates two full clones although only the adjusted result needs separate storage.
Measurement: memory-bandwidth pressure and live-preview latency remain unmeasured.

## Evidence

- [S] `crates/compose/src/lib.rs:836-852` clones before determining whether mixing or clipping uses the snapshot.
- [S] `crates/compose/src/lib.rs:880-898` clones twice and only mutates the adjusted buffer before final assignment.
- [S] `crates/compose/src/lib.rs:895-898` reads each original pixel before overwriting that same backdrop index.

## Prior-Art

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
Discussions are disabled; no matching snapshot-ownership correction was found.
The affected compose sources remain unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

## Proposed-Change

Create pass-through snapshots only when mixing or visible clipped layers require the original backdrop.
Clone adjustment output directly from the backdrop and read each original backdrop pixel before its assignment.

## Scope-and-Constraints

- Preserve snapshots required by clipping, channel restrictions, and Blend If.
- Preserve masks, opacity, blend ordering, quantization, and alpha behavior.
- Do not replace required independent mutable buffers with aliased storage.

## Performance-Evidence

[S] Each unnecessary N-pixel float-RGBA snapshot allocates a Vec and copies 16N pixel bytes.
[S] Unlike Surface clones, these Buffer clones copy their pixel vectors.
[O] Four pass-through groups with one Invert adjustment each, 6000×4000 RGB, eight Rayon workers, five warm samples: 1736.108 → 866.259 ms.
[O] The output digest matched (`ca6f7d23acd70325`); cold calls were 2052.855 → 790.061 ms.
Command: `RAYON_NUM_THREADS=8 <binary> backdrop 6000 4000 5`.
These shared-host observations do not establish allocator counts, bandwidth, or universal UI improvement.

## Verification

- Compare copied bytes, allocations, and release preview latency for groups and adjustments.
- Compare output with clipping, masks, opacity, channel restrictions, and Blend If.

## Publication-Blockers

None.
The current user request authorizes publication of the verified independent fix.

## Next-Action

Summary: Await upstream backdrop review
Action: Respond to substantive maintainer feedback on PR #109.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/avoid-backdrop-copies`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Allocate pass-through snapshots on demand and remove the redundant adjustment snapshot.
Commit: `3bf3d140d865b88a72c85075aabb07b8fa07d668`
Push: `MikeeI/project-photocraft-fork:perf/avoid-backdrop-copies`
Checks:
- Independent compose tests: 97 unit and three integration tests passed; Clippy and layering passed.
- WASM passed for all 20 packages; independent GPT-6.1 Sol/xhigh source review found no blocker.
- Formatting and diff checks passed; no fork tracking files enter the contribution.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(compose): avoid unnecessary backdrop snapshots`

```markdown
## Summary

Pass-through groups snapshot the backdrop only when mixing or visible clipped layers need the original pixels.
Otherwise their children composite directly into the existing backdrop.
The adjustment branch now clones once into its mutable result and reads each original backdrop pixel before replacing it.

Required snapshots for channel restrictions and Blend If remain.
Clipping, masks, opacity, quantization, blending, and alpha handling stay on their existing paths.
Each eliminated float-RGBA snapshot avoids allocating and copying 16 bytes per rendered pixel; this is a source-derived payload count, not allocator profiling.

## Validation

- `cargo test --locked -p photocraft-compose`: 97 unit and 3 integration tests passed.
- `cargo clippy --locked -p photocraft-compose --all-targets -- -D warnings`: passed.
- `cargo xtask layers` and `cargo xtask wasm`: passed.
- Formatting, diff checks, and independent source review completed.

A synthetic 6000×4000 RGB fixture with four pass-through groups, each applying an Invert adjustment, used eight Rayon workers and five warm samples.
Median composition time was 1736.108 ms before and 866.259 ms after; the output digest matched (`ca6f7d23acd70325`).
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
