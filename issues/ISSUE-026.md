# ISSUE-026 — canvas geometry: surface traversal omits smart-filter masks

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The shared canvas surface traversal visits Smart-Object caches but omits their document-coordinate filter masks.
Review mapping: `E13`, VALID; severity Medium.

## Reach-and-Impact

Trigger: resize or translate a canvas containing a renderable Smart Object with a nonuniform filter mask.
[S] Image Size refreshes the smart content immediately with an unscaled mask.
[S] Canvas translation can initially move the cache correctly but exposes mask misalignment on the next refresh.
No geometry/filter sequence was executed.

## Evidence

- [S] `crates/engine/src/image_cmds.rs:20-48` omits `sm.filter_mask.surface` from mask-aware traversal.
- [S] `crates/engine/src/image_cmds.rs:128-142,149-162` uses that traversal for resizing and translation.
- [S] `crates/engine/src/canvas_geom.rs:103,180-195` transforms smart placement and refreshes content.
- [S] `crates/engine/src/smart_cmds.rs:255-266,302-320` samples the filter mask in placed document coordinates.
- [S] `crates/engine/src/smart_cmds.rs:402-410` already moves the filter mask during ordinary object movement.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a focused filter-mask geometry comparison.

## Proposed-Change

Visit `sm.filter_mask.surface` with the mask flag when mask processing is enabled in the existing traversal.

## Scope-and-Constraints

- Preserve filter-mask interpolation policy and ordinary smart-object movement.
- Avoid double transformation in paths that already remap the mask separately.
- Do not replace source-unavailable cache fallback with a new rendering policy.

## Verification

Status: source-traced; no geometry experiment executed.
- Double an image containing a sharp half-image smart-filter mask and compare the expected mask boundary.
- Translate the canvas, refresh the smart object, and verify the mask remains aligned with its content.

## Publication-Blockers

- Resize and post-translation refresh behavior need runtime verification.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Reproduce smart-filter mask misalignment
Action: Resize and translate a disposable smart-object document with a sharp nonuniform filter mask.
Done-When: Record transforms, mask bounds, refresh timing, and expected versus actual effect boundaries.

## Pull-Request-Implementation

Branch: fix/transform-smart-filter-masks
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Include smart-filter masks in mask-aware canvas geometry traversal without double transforms.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
