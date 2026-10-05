# ISSUE-005 — UI: offscreen layer thumbnail work

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/117
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The active Layers panel requests thumbnails for scrolled-out rows without checking thumbnail visibility.
Review mapping: `P5`, VALID; severity High.

## Reach-and-Impact

[S] Each repaint traverses all rows surviving isolation and filtering, not only visible rows.
[S] Offscreen thumbnail requests still compute tile fingerprints and may regenerate textures.
Measurement: frame-time impact on long layer stacks has not been measured.

## Evidence

- [S] `crates/ui-egui/src/lib.rs:669-697` reaches the dock from the application UI entry point.
- [S] `crates/ui-egui/src/panels.rs:1162-1188` iterates all rows inside `ScrollArea::show`.
- [S] `crates/ui-egui/src/panels.rs:1359-1365` requests layer and mask thumbnails unconditionally.
- [S] `crates/ui-egui/src/lib.rs:754,772,805-811` traverses surface tiles before checking thumbnail cache hits.

## Prior-Art

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
PR #71 changes navigator proxy texture ownership, not offscreen Layers-row thumbnail generation.
No matching offscreen-row correction was found; changed panel/canvas sources remain unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

## Proposed-Change

Gate thumbnail generation and drawing with `ui.is_rect_visible` for each layer or mask thumbnail rectangle.
Retain row layout and interactions even when its thumbnail is offscreen.

## Scope-and-Constraints

- Preserve scrolling geometry, selection, mask targeting, rename, and drag-and-drop.
- Do not return early from the entire row or assume fixed heights for effect and smart-filter subrows.
- Keep the existing content cache so scrolling into view checks current content.
- Do not claim continuous idle repainting or work in inactive tabs.

## Performance-Evidence

[S] Offscreen cache hits traverse surface tiles; misses also sample thumbnails and create or update textures.
[O] In an 800×600 UI fixture with 300 hidden raster layers and masks, the full surface reported 670 textures before and 81 after.
[O] Frame medians were 46.978 ms before and 49.951 ms after under unrelated compiler load; these do not establish faster frames.
The observed improvement is reduced offscreen thumbnail resources/work, not an end-to-end latency claim.

## Verification

- In the 300-hidden-layer fixture, texture count fell from 670 to 81; the full row remains in the UI and only thumbnail image work is gated.
- Before/after screenshots show the same visible layer-panel layout.

## Publication-Blockers

None.
The draft makes no latency claim and reports the offscreen texture-count reduction.

## Next-Action

Summary: Await upstream thumbnail-culling review
Action: Respond to substantive maintainer feedback on PR #117.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/skip-offscreen-thumbnails`
Base: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`
Scope: Gate only offscreen thumbnail image work, preserving row interactions and outside decorations.
Commit: `6b0581879b911cbc7e0d3bf44d7eb958fe3ca661`
Push: `MikeeI/project-photocraft-fork:perf/skip-offscreen-thumbnails`
Checks:
- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- `cargo clippy --locked -p photocraft-ui-egui --all-targets -- -D warnings`, `cargo xtask layers`, and all 20 WASM checks passed.
- Before/after UI screenshots were rendered and inspected; they preserve the same visible layer-panel layout.
- Independent GPT-6.1 Sol/xhigh source review found no blocker.
- Changed panel/canvas sources are unchanged at upstream `7e7864a`.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(ui): skip offscreen layer thumbnail generation`

```markdown
## Summary

Gate layer and mask thumbnail generation and drawing on whether each thumbnail rectangle is visible.
Rows retain their layout, interactions, decorations, and mask targeting while scrolled offscreen.

## Validation

- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and `cargo xtask wasm` passed.
- In an 800×600 fixture with 300 hidden raster layers and masks, texture count was 670 before and 81 after.
- Frame medians were 46.978 ms before and 49.951 ms after under unrelated compiler load; no frame-speedup claim is made.
- Before: ![Offscreen layer list before](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-005-before.png)
- After: ![Offscreen layer list after](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-005-after.png)

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
