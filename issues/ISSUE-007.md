# ISSUE-007 — UI: repeated uncached content bounds

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/116
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Info-panel selection bounds and canvas layer edges bypass the existing exact tile-bounds cache.
Review mapping: `P7`, CHANGED; severity reduced from High to Medium and cold-cache costs added to acceptance.

## Reach-and-Impact

[S] Repaints with a selection in the Info tab or enabled layer edges repeat direct bounds computation.
[S] The direct algorithm traverses and sorts tile metadata and scans relevant boundary tiles.
[A] Reusing the existing cache is beneficial only if representative cold and warm costs justify it.

## Evidence

- [S] `crates/ui-egui/src/panels.rs:843` computes selection bounds before the separate info-sample cache gate.
- [S] `crates/ui-egui/src/canvas.rs:942-947` directly computes active-layer surface bounds for layer edges.
- [S] `crates/raster/src/lib.rs:105-125` skips enclosed interior tiles in the existing direct algorithm.
- [S] `crates/compose/src/bounds.rs:47-68` checks weak tile identity and encoded defaults but scans every cold miss.

## Prior-Art

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
PR #71's proxy and navigator changes do not route selection or layer-edge bounds through the existing exact cache.
The changed UI sources remain unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

## Proposed-Change

Route both UI call sites through `photocraft_compose::bounds::content_bounds` without a new cache layer.
Adopt the change only if cold and warm workload measurements show an acceptable overall result.

## Scope-and-Constraints

- Preserve exact bounds for default pixels, sparse tiles, changing content, inversion, and undo/redo.
- Use the existing weak-identity validation, not raw pointer fingerprints alone.
- Do not describe the current algorithm as an unconditional full-image pixel scan.
- Cache initialization can cost more than the existing boundary-only scan on dense content.

## Performance-Evidence

[S] A direct call allocates and sorts T tile entries and scans boundary or otherwise unenclosed tiles.
[S] Warm cached bounds still visit T tiles but avoid their repeated pixel scans.
[O] At 6000×4000 with 21 calls, dense 384-tile RGBA8 bounds cost 4.3803 → 6.3636 ms cold and 3.1213 → 0.0107 ms warm.
[O] Sparse two-tile warm medians were 0.3238 → 0.0001 ms; default-heavy 384-tile warm medians were 66.2243 → 0.0296 ms.
[O] All compared bounds matched; the actual 6000×4000 before/after UI surfaces rendered successfully.
The cold-cache regression and unmeasured changing-selection workflow limit the claim to unchanged content with repeated queries.

## Verification

- Compare cold and warm bounds calls on dense and sparse surfaces with unchanged and changing selections.
- Verify exact bounds after shrinking, movement, inversion, and undo/redo.
- Reject or revise the proposed routing if cold regressions outweigh representative warm gains.

## Publication-Blockers

None.
The draft bounds the benefit to warm unchanged surfaces and discloses cold and changing-selection gaps.

## Next-Action

Summary: Await upstream bounds review
Action: Respond to substantive maintainer feedback on PR #116.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/reuse-ui-content-bounds`
Base: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`
Scope: Route the two UI bounds consumers through the existing exact cache.
Commit: `9a910c02d4b53acec8436a25a9950ac4f5887ce6`
Push: `MikeeI/project-photocraft-fork:perf/reuse-ui-content-bounds`
Checks:
- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and all 20 WASM checks passed.
- Before/after 6000×4000 UI screenshots were rendered and inspected.
- Independent GPT-6.1 Sol/xhigh source review found no blocker.
- Changed canvas/panel sources are unchanged at upstream `7e7864a`.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(ui): reuse exact cached content bounds`

```markdown
## Summary

Route the Info-panel selection bounds and canvas layer-edge bounds through the existing exact tile-bounds cache.
No second cache or UI-specific invalidation rule is added.

## Validation

- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and `cargo xtask wasm` passed.
- For a 6000×4000 dense 384-tile surface, 21 bounds calls measured 4.3803 → 6.3636 ms cold and 3.1213 → 0.0107 ms warm.
- Sparse two-tile and default-heavy 384-tile warm cases also improved; every compared rectangle matched.
- A cold-cache penalty remains, and changing-selection UI latency was not measured.
- Before: ![Cached bounds before](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-007-before.png)
- After: ![Cached bounds after](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-007-after.png)

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
