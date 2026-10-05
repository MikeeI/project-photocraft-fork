# ISSUE-036 — group composition: opacity mixes straight-alpha colors directly

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
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
The arithmetic was evaluated independently; the application rendering path was not executed.

## Evidence

- [S] `crates/compose/src/lib.rs:35-39` defines Buffer samples as straight-alpha RGBA.
- [S] `crates/compose/src/lib.rs:838-844` interpolates all four components directly.
- [S] `crates/doc/src/lib.rs:469-471` makes ordinary groups pass-through by default.
- [S] `crates/compose/src/psblend.rs:92-117` uses proper straight-alpha source-over for normal layer composition.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; `ISSUE-009` was read for duplicate comparison.
`ISSUE-009` concerns unnecessary snapshots, not incorrect interpolation of snapshots that are required.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a narrow group-render comparison.

## Proposed-Change

Interpolate premultiplied colors and alpha for group coverage, then safely convert back to straight RGBA.

## Scope-and-Constraints

- Preserve zero-alpha handling, group masks, clipped-layer ordering, and supported blend behavior.
- Do not remove snapshots needed for interpolation while optimizing copies under `ISSUE-009`.
- Scope the correction to coverage mixing rather than redefining all compositor color-space policy.

## Verification

Status: equation evaluated; no application rendering reproduction executed.
- Render half-opacity red over transparency and require `[1,0,0,0.5]` before quantization.
- Compare the result over an opaque white background with the expected red value `1.0`.

## Publication-Blockers

- Runtime group rendering and affected clipping/mask compatibility evidence are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce group alpha darkening
Action: Render an opaque red child inside a half-opacity pass-through group on a transparent canvas.
Done-When: Record raw composite RGBA and white-background output against the analytic expected values.
