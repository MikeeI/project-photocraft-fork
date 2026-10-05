# ISSUE-037 — proxy rendering: artboard rectangles retain full-size coordinates

State: Implementing
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

[S] Proxy creation shrinks document dimensions and raster content but leaves artboard clipping rectangles unchanged.
Review mapping: `E24`, CHANGED; severity Medium.
Changing only thumbnail proxy eligibility cannot repair interactive callers that build proxies directly.

## Reach-and-Impact

Trigger: a large document has an artboard positioned toward the right side of its canvas.
[S] The original rectangle can lie entirely outside the smaller proxy and cause its contents to disappear.
[S] Native thumbnails and interactive filter previews both reach proxy rendering through different callers.
[O] The existing artboard integration regression failed at proxy pixel `(3,2)` before the fix and passed afterward.
Actual thumbnail and interactive-preview PNG evidence remains pending.

## Evidence

- [S] `crates/compose/src/proxy.rs:35-71` shrinks surfaces and canvas but not `Group::artboard.rect`.
- [S] `crates/compose/src/lib.rs:283-284` chooses proxy thumbnails for eligible large documents.
- [S] `crates/compose/src/lib.rs:801-803` rejects an artboard whose rectangle misses the render area.
- [S] `crates/ui-egui/src/filter_dialog.rs:371-374` calls `proxy_document` without the thumbnail guard.
- [S] `crates/io/src/lib.rs:135-138` embeds generated native previews.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; `ISSUE-010` was read for duplicate comparison.
`ISSUE-010` concerns source-row copy volume, not unscaled clipping geometry.
Upstream inventory covered all 102 issue/PR records, relevant discussions, and public releases; no exact duplicate was found.
GitHub Discussions are disabled; project Discord history was inaccessible.

## Proposed-Change

Scale artboard rectangles inside `shrink_layer` with the proxy factor and consistent coordinate rounding.

## Scope-and-Constraints

- Preserve negative coordinates, group nesting, and clipping behavior at rounding boundaries.
- Do not present a `proxy_faithful`-only guard change as a complete fix.
- Keep this geometry correction separate from sampling-cost optimization in `ISSUE-010`.

## Verification

[O] The extended `contents_are_clipped_and_background_painted` failed before the fix and passed afterward.
It compares every reduced pixel against source coordinates `(4*x,4*y)` for an unaligned artboard rectangle.
[O] Full compose verification passed: 97 unit tests and three integration tests.
[O] Formatting, affected Clippy, dependency layering, and all 20 WebAssembly packages passed.
[S] Two independent source reviews approved signed ceil scaling of both half-open edges.
Before/after offscreen UI captures must still demonstrate the thumbnail and interactive filter-preview callers.

## Publication-Blockers

- Inspected before/after UI captures and the exact publication draft remain pending.

## Next-Action

Summary: Compare proxy artboard clipping
Action: Render a large right-side artboard through the thumbnail and interactive filter-preview paths.
Done-When: Record proxy factors, original and reduced bounds, clipping regions, and inspected output images.

## Pull-Request-Implementation

Branch: fix/scale-proxy-artboards
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Scale artboard clipping rectangles inside proxy creation for all proxy consumers.
Commit: Pending.
Push: Pending.
Checks:
- Baseline proxy-artboard regression: failed at `(3,2)`.
- Focused and full compose tests: passed.
- Formatting, affected Clippy, layering, and WebAssembly: passed.
- Independent source reviews: approved.
- Offscreen UI captures: pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
