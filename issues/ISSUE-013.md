# ISSUE-013 — GPU canvas: orphaned display LUT resources

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
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

Coverage: local ledger checked on 2026-10-05; distinct from thumbnail-handle ownership in `ISSUE-012`.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved until cleanup and same-ID reopening are verified together.

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

[S] Each retained 33-cubed RGBA8 LUT represents 143,748 nominal texel bytes plus GPU and binding overhead.
[S] The existing map has no closed-document pruning path; only explicit same-key `None` removes a LUT.
Measurement: no runtime growth rate, actual VRAM footprint, exhaustion, or corrected speedup is established.

## Verification

- Close and reopen the same color-managed `.pcraft` file and verify LUT reconstruction and identical display output.
- Inspect retained LUT keys and resources after repeated distinct-document Open/Close cycles and completed GPU work.
- Verify live previews and other open documents keep their LUTs and signatures valid.

## Publication-Blockers

- Runtime retention evidence and same-ID reopening verification are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Inspect LUT cleanup lifecycle
Action: Trace LUT resources and display signatures across closing and reopening the same color-managed native file.
Done-When: Record document IDs, keys, signatures, retained resources, and display output without changing cleanup behavior.
