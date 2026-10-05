# ISSUE-037 — proxy rendering: artboard rectangles retain full-size coordinates

State: Archived
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/131
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
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
[O] Isolated offscreen captures used the same 6000×4000 right-side artboard script at 1440×900.

## Evidence

- [S] `crates/compose/src/proxy.rs:35-71` shrinks surfaces and canvas but not `Group::artboard.rect`.
- [S] `crates/compose/src/lib.rs:283-284` chooses proxy thumbnails for eligible large documents.
- [S] `crates/compose/src/lib.rs:801-803` rejects an artboard whose rectangle misses the render area.
- [S] `crates/ui-egui/src/filter_dialog.rs:371-374` calls `proxy_document` without the thumbnail guard.
- [S] `crates/io/src/lib.rs:135-138` embeds generated native previews.

## Prior-Art

Coverage: original 102-record inventory, current PR #131 thread/diff, and main source checked on 2026-10-06.
PR #131 by @MikeeI merged on 2026-10-05; it scales artboard bounds during recursive proxy creation.
`ISSUE-010` concerns source-row copy volume, not unscaled clipping geometry.
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
[O] Separate isolated baseline and patched UI builds ran on the GPU with no fallback and `lastRefresh=filter-preview`, 1,500,000 proxy pixels.
The baseline canvas and Navigator showed transparent checkerboard where the artboard content should be.
The patched canvas showed the red artboard content and the Navigator showed its reduced thumbnail.
[S] Current `upstream/main@47f4abfed49e0d2f5b9277287b27dee632530ba4` scales artboard rectangle edges in `shrink_layer` before recursing through group children (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/compose/src/proxy.rs).

## Publication-Blockers

None.

## Next-Action

Summary: —
Action: None.
Done-When: None.

## Pull-Request-Implementation

Branch: fix/scale-proxy-artboards
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Scale artboard clipping rectangles inside proxy creation for all proxy consumers.
Commit: `6851f0b4185f8dfe88b2be0d1bff8323260e9853`
Push: `origin/fix/scale-proxy-artboards`.
Checks:
- Baseline proxy-artboard regression: failed at `(3,2)`.
- Isolated post-fix pixel comparison, full compose tests, and affected Clippy: passed.
- Formatting, layering, and all 20 WebAssembly packages: passed.
- Independent xhigh source reviews: approved.
- Same-scene isolated offscreen baseline/patched captures: inspected; baseline content was clipped, patched artboard and Navigator rendered.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/scale-proxy-artboards`
Title: Scale artboard bounds in proxy documents

### Problem

Proxy creation downsamples raster content and canvas dimensions but leaves artboard clipping rectangles in full-resolution coordinates.
For a right-side board in a large document, thumbnail and interactive filter-preview proxies can clip away the board contents.

### Change

Scale both half-open artboard edges using signed ceil division, matching proxy pixel sampling even for negative coordinates.
Apply the correction inside recursive proxy creation so all proxy callers use the same geometry.

### Verification

- The existing artboard regression failed before the fix at proxy pixel `(3,2)`.
- The post-fix integration test compares every 4× proxy pixel against its source sample.
- Full compose tests passed: 97 unit tests and three integration tests.
- Affected Clippy, formatting, dependency layering, and all 20 WebAssembly packages passed.
- Separate isolated baseline and patched offscreen builds used the same 6000×4000 artboard scene.
- Both UI captures executed the real GPU filter-preview path with no fallback.
- The baseline clipped the artboard content; the patched canvas and Navigator rendered the reduced artboard.
- Two independent source reviews found no blocker.

The UI comparison verifies this synthetic scene; it does not claim performance improvement or broad document-corpus coverage.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.

## Archive

Archive-Reason: Merged
Detail: None.
Evidence: https://github.com/storytold/photocraft/pull/131
Checked: 2026-10-06
