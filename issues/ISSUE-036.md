# ISSUE-036 — group composition: opacity mixes straight-alpha colors directly

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/108
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Pass-through group opacity and mask mixing interpolate straight RGB and alpha independently.
Review mapping: `E23`, VALID; severity Medium.

## Reach-and-Impact

Trigger: an opaque red child in a 50-percent pass-through group over transparent black.
[S] The group equation yields `[0.5,0,0,0.5]` instead of straight RGBA `[1,0,0,0.5]`.
[S] Compositing that result over white darkens red to `0.75` instead of `1.0`.
[O] The compositor regression reproduced the same defect with HDR red `2`: output red `1` instead of `2`.

## Evidence

- [S] `crates/compose/src/lib.rs:35-39` defines Buffer samples as straight-alpha RGBA.
- [S] `crates/compose/src/lib.rs:838-844` interpolates all four components directly.
- [S] `crates/doc/src/lib.rs:469-471` makes ordinary groups pass-through by default.
- [S] `crates/compose/src/psblend.rs:92-117` uses proper straight-alpha source-over for normal layer composition.
- [O] The existing `pass_through_group_opacity_mixes` extension failed before the fix and passed afterward.
- [O] An isolated build passed the complete compose/GPU test commands and affected-crate Clippy.
- [O] `groups_masks_and_clipping` passed with a temporary mandatory-GPU assertion, proving real pixel comparisons rather than a skip.
- [O] The temporary assertion was removed; Git confirms the worktree matches the pushed commit.
- [S] Upstream `7e7864afae8f779afff063a68edf208e30fe592c` leaves the changed CPU and shader paths unchanged.

## Prior-Art

Coverage: local ledger and all 102 upstream issue/PR records, public releases, and relevant PR discussions checked.
No matching root cause or active implementation was found.
Adjustment-preview work in https://github.com/storytold/photocraft/pull/85 is distinct from pass-through group coverage.
`ISSUE-009` concerns unnecessary snapshots, not incorrect interpolation.
GitHub Discussions are disabled; project Discord history was inaccessible.

## Proposed-Change

Interpolate premultiplied colors and alpha for group coverage, then safely convert back to straight RGBA.

## Scope-and-Constraints

- Preserve zero-alpha handling, group masks, clipped-layer ordering, and supported blend behavior.
- Do not remove snapshots needed for interpolation while optimizing copies under `ISSUE-009`.
- Scope the correction to coverage mixing rather than redefining all compositor color-space policy.
- The unchanged clipping-difference branch still has its prior RGB clamps and approximation.
- The F32 HDR oracle is CPU-side; executed GPU group parity uses the existing RGBA8 fixtures.

## Verification

[O] `cargo test -p photocraft-compose -p photocraft-gpu` passed all emitted suites.
[O] `cargo clippy -p photocraft-compose -p photocraft-gpu --all-targets -- -D warnings` passed.
[O] The focused GPU parity command passed with GPU initialization made mandatory for the local run.
[O] Formatting and Git diff checks passed.
[O] `cargo xtask layers` passed for 26 crates; `cargo xtask wasm` passed for all 20 selected packages.
The wasm check emitted existing unused-constant warnings in CMS and engine code.
An independent GPT-6.1 Sol review at runtime-verified xhigh effort found no source blocker.

## Publication-Blockers

None.

## Next-Action

Summary: Await upstream group coverage review
Action: Monitor the submitted PR for actionable maintainer feedback or CI failures.
Done-When: Feedback is addressed or the PR reaches a terminal upstream decision.

## Pull-Request-Implementation

Branch: fix/premultiplied-group-coverage
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Mix pass-through group coverage in premultiplied space and return straight-alpha samples.
Commit: c3c28d7e5d7e6d167fe828461a79e954062a3542
Push: origin/fix/premultiplied-group-coverage
Checks:
- CPU regression: failed before, passed after.
- Affected-crate tests and Clippy: passed.
- GPU group parity with skip disabled: passed; instrumentation removed.
- Layering and WebAssembly: passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/premultiplied-group-coverage`
Title: Mix pass-through group coverage in premultiplied space

### Problem

Pass-through group opacity and masks currently interpolate straight RGB and alpha independently.
Over a transparent backdrop this attenuates RGB twice when the result is subsequently composited.
For an opaque F32 child `[2,0,0,1]` in a half-opacity group, the CPU renderer returns `[1,0,0,0.5]` instead of `[2,0,0,0.5]`.

### Changes

- Weight the before/after composites by their alpha and group coverage.
- Interpolate premultiplied colour, then convert back to the buffer's straight-alpha representation.
- Return transparent black for zero combined alpha without dividing by zero.
- Apply the same equation to the GPU coverage shader.
- Extend the existing group-opacity test with the transparent-backdrop HDR case.

The shader's quantization, channel-mixing, and clipping-difference modes retain their existing equations.
The unchanged clipping-difference approximation and its RGB clamps are outside this fix.

### Verification

- The CPU regression failed before the fix and passed afterward.
- `cargo test -p photocraft-compose -p photocraft-gpu`: passed.
- `cargo clippy -p photocraft-compose -p photocraft-gpu --all-targets -- -D warnings`: passed.
- `groups_masks_and_clipping` also passed with GPU initialization temporarily made mandatory, so actual GPU/CPU pixels were compared.
- The temporary verification assertion was removed and is not in the contribution diff.
- Formatting and Git diff checks passed.

The new HDR oracle is CPU-side; the existing GPU group-parity fixtures use RGBA8 layers.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
