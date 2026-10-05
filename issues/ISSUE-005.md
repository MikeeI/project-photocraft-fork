# ISSUE-005 — UI: offscreen layer thumbnail work

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
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

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: potentially bounded UI correction; measurements and upstream ownership search remain open.

## Proposed-Change

Gate thumbnail generation and drawing with `ui.is_rect_visible` for each layer or mask thumbnail rectangle.
Retain row layout and interactions even when its thumbnail is offscreen.

## Scope-and-Constraints

- Preserve scrolling geometry, selection, mask targeting, rename, and drag-and-drop.
- Do not return early from the entire row or assume fixed heights for effect and smart-filter subrows.
- Keep the existing content cache so scrolling into view checks current content.
- Do not claim continuous idle repainting or work in inactive tabs.

## Performance-Evidence

[S] Offscreen cache hits traverse all tiles of the relevant surfaces and masks.
[S] Cache misses additionally perform up to 64-square thumbnail sampling and texture creation or update.
Measurement: no frame-time improvement or workload frequency has been observed.

## Verification

- Compare thumbnail requests, tile visits, and UI frame time on an identical long layer stack.
- Visually inspect scrolling changed layers into view, mask targeting, rename, and drag-and-drop.

## Publication-Blockers

- Representative frame measurements and actual UI verification are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Measure offscreen thumbnail work
Action: Capture thumbnail requests and tile visits during repaints of a scrolled long Layers panel.
Done-When: Record visible and total row counts, command or interaction sequence, counters, and frame timing.
