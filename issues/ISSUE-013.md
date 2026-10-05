# ISSUE-013 — GPU canvas: orphaned display LUT resources

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/118
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] GPU document cleanup omits the separate display-LUT map and its UI-side signature cache.
Review mapping: `P13`, CHANGED; severity Medium.
The corrected proposal couples LUT removal with signature invalidation; LUT-only cleanup is incomplete.

## Reach-and-Impact

[S] Display transforms, proofing, or HDR preview can allocate a per-document LUT retained after document closure.
[S] Repeated distinct document keys can accumulate resources throughout the renderer lifetime.
[S] The original LUT-only proposal could leave a valid-looking signature for a missing resource after reopening.
That last risk concerns the proposed change, not an observed current color-rendering defect.

## Evidence

- [S] `crates/ui-egui/src/canvas.rs:524-525` provides live document and preview keys to GPU retention.
- [S] `crates/ui-egui/src/gpu_canvas.rs:339-369` removes document textures but omits LUT-map pruning.
- [S] `crates/ui-egui/src/gpu_canvas.rs:577-600` allocates and binds a separate RGBA8 3D LUT texture.
- [S] `crates/ui-egui/src/canvas.rs:774-777` accepts a cached display signature before rebuilding a LUT.
- [S] `crates/format/src/lib.rs:88-90` defaults to preserved IDs; `convert.rs:499-503` restores the stored DocId.
- [S] `crates/engine/src/lib.rs:148-163,278-285` does not replace the loaded document ID during insertion.

## Prior-Art

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
Upstream GPU preview/HDR work is preserved; no matching LUT/signature pruning correction was found.
The rebased implementation also clears the introduced closed-document filter-preview upload marker and the new adjustment-preview owner.

## Proposed-Change

Remove orphaned LUT keys and their matching `pc-display-lut` signature entries in the same retention lifecycle.
Apply LUT pruning even when no orphaned document texture triggers the current cleanup branch.
Keep valid document and preview keys live.

## Scope-and-Constraints

- Preserve color management on reopening the same native file with the same persisted document ID.
- Preserve proof colors, gamut warnings, HDR preview, live previews, and the existing renderer-lock ownership.
- Treat GPU resources and UI signature metadata as coupled state; do not introduce LUT-only cleanup.
- GPU residency may outlive handle removal until submitted work completes.

## Performance-Evidence

[S] A 33-cubed RGBA8 LUT represents 143,748 nominal texel bytes plus GPU and binding overhead; actual residency was not measured.
[S] Renderer LUTs and UI-side identity signatures are removed together, independently of orphan canvas textures.
Measurement: resource-count and VRAM deltas are unavailable; no speedup or exhaustion threshold is claimed.

## Verification

- Close and reopen the same color-managed `.pcraft` file and verify LUT reconstruction and identical display output.
- Inspect retained LUT keys and resources after repeated distinct-document Open/Close cycles and completed GPU work.
- Verify live previews and other open documents keep their LUTs and signatures valid.

## Publication-Blockers

None.
The tested same-ID preview lifecycle and source-reviewed LUT pruning are documented without VRAM or speedup claims.

## Next-Action

Summary: Await upstream LUT-lifecycle review
Action: Respond to substantive maintainer feedback on PR #118.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `fix/prune-display-lut-state`
Base: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`
Scope: Co-own LUT signatures with renderer resources and prune both at document synchronization.
Commit: `94ce4c08a7a25ccc4ecf0b9413743035d75aa706`
Push: `MikeeI/project-photocraft-fork:fix/prune-display-lut-state`
Checks:
- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and all 20 WASM checks passed.
- The existing `display_p3_is_converted_on_gpu_and_cpu_canvases` test verifies same-ID native reopen, filter-preview resource recreation, and adjustment-preview owner clearing/rebuild.
- Observed display samples: GPU `[216, 124, 64]`, CPU `[217, 123, 65]`; existing tolerances passed.
- The final UI branch was rebased onto upstream `7e7864a`; GPT-6.1 Sol/xhigh reviewed the rebased delta and found no introduced blocker.
- Formatting, diff checks, and the branch test update were complete before final validation.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `fix(ui): prune closed-document display and preview resources`

```markdown
## Summary

Prune closed-document display LUTs together with their cached identity signatures, even when no orphan canvas texture triggers cleanup.
Clear closed-document filter and adjustment preview owners before collecting live GPU keys; same-ID reopen must rebuild rather than trust stale upload metadata.
Retain upstream's HDR preview and `gpu_canvas_lut` paths, and preserve resources for documents and previews that remain live.

## Validation

- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and `cargo xtask wasm` passed.
- The existing Display P3 integration test closed and reopened the same native document ID, verified preview texture release/reupload, and confirmed adjustment-preview state clears and rebuilds.
- GPU sample `[216, 124, 64]` and CPU sample `[217, 123, 65]` passed the existing color tolerances after reopen.
- Before: ![Display P3 canvas before](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-013-before.png)
- After: ![Display P3 canvas after](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-013-after.png)

No direct GPU-residency measurement or speedup claim is made.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
