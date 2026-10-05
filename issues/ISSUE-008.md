# ISSUE-008 — UI: channel-view thumbnail invalidation

State: Implementing
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Channel thumbnails use a general revision that also changes for view-only channel actions.
Review mapping: `P8`, VALID; severity High.

## Reach-and-Impact

[S] Channel target and visibility changes invalidate thumbnails on the next active Channels-panel repaint.
[S] The unchanged document is composited again; exact thumbnail reduction may process the full document.
Measurement: click latency and repeated composition cost remain unmeasured.

## Evidence

- [S] `crates/engine/src/channel_cmds.rs:120-127,996-1013,1047-1073` changes view state and bumps revision.
- [S] `crates/ui-egui/src/lib.rs:895-942` keys thumbnail generation on that revision and document ID.
- [S] `crates/ui-egui/src/channels_panel.rs:57` requests those thumbnails from the panel.
- [S] `crates/compose/src/lib.rs:276-286` uses full exact reduction when the proxy conditions do not hold.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: potentially bounded invalidation correction; measurements and upstream ownership search remain open.

## Proposed-Change

Track document ID and a `Weak<Document>` identity alongside the thumbnail cache's seen revision.
Reuse thumbnails across view-only revisions when the immutable document snapshot is identical.
Rebuild on snapshot changes and preserve the single active-document cache slot.

## Scope-and-Constraints

- Do not use an unpinned raw address as snapshot identity.
- Keep true pixel, channel, quick-mask, document-switch, and undo/redo invalidation.
- Keep channel-view canvas behavior independent from thumbnail pixels.
- Avoid strong snapshot retention that unnecessarily pins document pixels.

## Performance-Evidence

[S] Every view-only revision currently triggers composite generation and recreation of the channel textures.
[S] Without a faithful proxy, reduced output still requires full source-region composition.
Measurement: no observed click delay, rebuild count, or speedup is claimed.

## Verification

- Count rebuilds and measure click latency during target and visibility changes on an unchanged snapshot.
- Verify rebuilding after actual pixel/channel/quick-mask edits, document switching, and undo/redo.

## Publication-Blockers

- Representative rebuild counts, timing, and snapshot-invalidation verification are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Verify channel snapshot invalidation
Action: Observe thumbnail reuse across view changes and invalidation across edits, undo, and same-ID reopening.
Done-When: Record those outcomes and complete independent UI gates before committing.

## Pull-Request-Implementation

Branch: `perf/retain-channel-thumbnails`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Key the channel cache by DocId and Weak document snapshot identity.
Commit: Pending.
Push: Pending.
Checks:
- Independent source review found no blocker; runtime sequence and screenshots remain pending.
- Worktree: `.git/omp-worktrees/issue-008`; unfinished checks paused to prioritize finished PRs.
