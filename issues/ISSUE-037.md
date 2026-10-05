# ISSUE-037 — proxy rendering: artboard rectangles retain full-size coordinates

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

[S] Proxy creation shrinks document dimensions and raster content but leaves artboard clipping rectangles unchanged.
Review mapping: `E24`, CHANGED; severity Medium.
Changing only thumbnail proxy eligibility cannot repair interactive callers that build proxies directly.

## Reach-and-Impact

Trigger: a large document has an artboard positioned toward the right side of its canvas.
[S] The original rectangle can lie entirely outside the smaller proxy and cause its contents to disappear.
[S] Native thumbnails and interactive filter previews both reach proxy rendering through different callers.
No thumbnail or live-preview reproduction was executed.

## Evidence

- [S] `crates/compose/src/proxy.rs:35-71` shrinks surfaces and canvas but not `Group::artboard.rect`.
- [S] `crates/compose/src/lib.rs:283-284` chooses proxy thumbnails for eligible large documents.
- [S] `crates/compose/src/lib.rs:801-803` rejects an artboard whose rectangle misses the render area.
- [S] `crates/ui-egui/src/filter_dialog.rs:371-374` calls `proxy_document` without the thumbnail guard.
- [S] `crates/io/src/lib.rs:135-138` embeds generated native previews.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; `ISSUE-010` was read for duplicate comparison.
`ISSUE-010` concerns source-row copy volume, not unscaled clipping geometry.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending both thumbnail and interactive-preview comparisons.

## Proposed-Change

Scale artboard rectangles inside `shrink_layer` with the proxy factor and consistent coordinate rounding.

## Scope-and-Constraints

- Preserve negative coordinates, group nesting, and clipping behavior at rounding boundaries.
- Do not present a `proxy_faithful`-only guard change as a complete fix.
- Keep this geometry correction separate from sampling-cost optimization in `ISSUE-010`.

## Verification

Status: source-traced; no rendering reproduction executed.
- Render a 6000-by-4000 document with a right-side artboard as a thumbnail and an interactive filter preview.
- Require the same visible artboard placement at the corresponding reduced coordinates.

## Publication-Blockers

- Thumbnail and interactive-preview geometry evidence is missing.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

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
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
