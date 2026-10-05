# ISSUE-007 — UI: repeated uncached content bounds

State: Implementing
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

Coverage: local ledger checked on 2026-10-05; both UI call sites share this record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved until cold/warm measurements establish a net benefit.

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
[O] At 6000×4000 with 21 repeated calls, dense 384-tile RGBA8 bounds cost 4.3803 → 6.3636 ms cold.
[O] Dense warm medians were 3.1213 → 0.0107 ms; sparse two-tile medians were 0.3238 → 0.0001 ms.
[O] Default-heavy 384-tile warm medians were 66.2243 → 0.0296 ms; returned bounds matched.
These function-level measurements do not prove a net UI benefit during changing selections.

## Verification

- Compare cold and warm bounds calls on dense and sparse surfaces with unchanged and changing selections.
- Verify exact bounds after shrinking, movement, inversion, and undo/redo.
- Reject or revise the proposed routing if cold regressions outweigh representative warm gains.

## Publication-Blockers

- Cold/warm timing, allocation evidence, and net-benefit acceptance are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Verify bounds adoption tradeoff
Action: Compare changing-selection UI workloads and exact bounds after edits against the measured cold penalty.
Done-When: Record runtime parity and a supported adoption or rejection decision.

## Pull-Request-Implementation

Branch: `perf/reuse-ui-content-bounds`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Route the two UI bounds consumers through the existing exact cache.
Commit: Pending.
Push: Pending.
Checks:
- Source review found no correctness blocker; cold regression prevents an unconditional performance claim.
- Worktree: `.git/omp-worktrees/issue-007`; UI evidence and independent gates remain incomplete.
