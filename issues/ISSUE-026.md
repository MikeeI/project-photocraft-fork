# ISSUE-026 — canvas geometry: surface traversal omits smart-filter masks

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
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
- [S] Current main's mask-aware image traversal visits `sm.cache` but omits `sm.filter_mask.surface`; Image Size and `translate_doc` reuse that traversal (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/engine/src/image_cmds.rs#L24-L55; https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/engine/src/image_cmds.rs#L109-L178).

## Prior-Art

Coverage: current main source and merged PR #70 reviewed on 2026-10-06.
PR #70 explicitly leaves Canvas Size and Crop smart-filter-mask translation unresolved (https://github.com/storytold/photocraft/pull/70).
This is partial prior art, not a fix; Image Size resampling is also affected in current source.

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

- Image Size and post-translation refresh behavior need runtime verification.
- Implementation, focused checks, and the exact external draft remain unresolved.

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
