# ISSUE-008 — UI: channel-view thumbnail invalidation

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/120
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

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
PR #71 includes related proxy-thumbnail work but does not change the revision key or view-only snapshot identity.
No matching channel-thumbnail invalidation correction was found; changed UI sources remain unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

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

[S] Channel-target changes bump session revision while retaining the same immutable document snapshot.
[O] The focused lifecycle run observed view reuse, then invalidation after pixel edit, undo, and same-ID document reopen.
[O] The baseline recreated thumbnails on the view-only selection; the candidate reused texture IDs.
No click-latency or general speedup is claimed.

## Verification

- Count rebuilds and measure click latency during target and visibility changes on an unchanged snapshot.
- Verify rebuilding after actual pixel/channel/quick-mask edits, document switching, and undo/redo.

## Publication-Blockers

None.
The draft confines claims to the observed snapshot reuse and invalidation sequence.

## Next-Action

Summary: Await upstream channel-cache review
Action: Respond to substantive maintainer feedback on PR #120.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/retain-channel-thumbnails`
Base: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`
Scope: Key the channel cache by DocId and Weak document snapshot identity.
Commit: `b8a67edc7c19b6fc2d1cb901d82fa925c04586c7`
Push: `MikeeI/project-photocraft-fork:perf/retain-channel-thumbnails`
Checks:
- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and all 20 WASM checks passed.
- View-only selection reused texture IDs; pixel edit, undo, and same-ID reopen invalidated them.
- Before/after Channels-panel screenshots were rendered and inspected.
- Independent GPT-6.1 Sol/xhigh source review found no blocker.
- Changed UI source is unchanged at upstream `7e7864a`.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(ui): retain channel thumbnails across view-only revisions`

```markdown
## Summary

Key channel thumbnails by active document ID and weak document-snapshot identity.
View-only channel commands can bump session revision without changing pixels, so the same immutable snapshot reuses its textures.
Pixel edits, undo, document switches, and preserved-ID reopen create a different snapshot and rebuild thumbnails.
The weak key validates allocation identity without retaining the document's pixel data.

## Validation

- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and `cargo xtask wasm` passed.
- Runtime sequence: view-only selection reused IDs; pixel edit, undo, and same-ID reopen invalidated them.
- No click-latency or general speedup claim is made.
- Before: ![Channels panel before](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-008-before.png)
- After: ![Channels panel after](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-008-after.png)

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
