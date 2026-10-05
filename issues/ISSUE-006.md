# ISSUE-006 — UI: redundant active-layer clone

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/121
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The Layers panel clones the active layer despite already retaining the owning document snapshot.
Review mapping: `P6`, CHANGED; severity reduced from High to Medium because actual UI cost is unmeasured.

## Reach-and-Impact

[S] Each Layers-panel repaint with an active layer performs the clone.
[S] Large surfaces copy tile-map metadata; active groups recursively copy their subtree metadata.
[S] COW shares pixel buffers, so this is not a full raster-pixel copy.

## Evidence

- [S] `crates/ui-egui/src/panels.rs:1066-1068` retains the document Arc and then calls `.cloned()` on the layer.
- [S] `crates/ui-egui/src/panels.rs:1102-1153,1259-1274` reads controls before executing collected actions.
- [S] `crates/raster/src/lib.rs:41-45` derives Clone for a surface containing a tile-to-Arc BTreeMap.

## Prior-Art

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
PR #71's panel change routes navigator image drawing through its proxy helper; it does not address active-layer metadata cloning.
The changed panel source remains unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

## Proposed-Change

Remove `.cloned()` and borrow the active layer from the local `Arc<Document>` for the panel's read phase.
Keep the owning Arc alive and preserve the existing action execution order.

## Scope-and-Constraints

- Do not remove the document Arc clone that owns the snapshot lifetime.
- Do not introduce a cache, new shared mutation, or refactoring of unrelated controls.
- Distinguish copied strings, vectors, tile maps, and Arc counts from shared pixel data.

## Performance-Evidence

[S] The read-only panel previously cloned the active layer, including tile-map or recursive group metadata; pixel buffers remain COW-shared.
[S] The change removes that `Layer::clone` while retaining the owning document `Arc` and deferring actions until reads finish.
[O] The active-group UI screenshot showed the same group and controls after the change.
No controlled frame-latency or allocation-count improvement is claimed.

## Verification

- `cargo test --locked -p photocraft-ui-egui` passed; active-group controls were inspected in the offscreen screenshot.
- Frame latency and allocation counts were not measured under controlled conditions.

## Publication-Blockers

None.
The draft states the clone removed and explicitly disclaims an unmeasured latency effect.

## Next-Action

Summary: Await upstream layer-borrow review
Action: Respond to substantive maintainer feedback on PR #121.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `perf/borrow-active-layer`
Base: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`
Scope: Borrow the active layer from the existing local document Arc.
Commit: `28cbb514c86cbfaed9925d319260495c23f72706`
Push: `MikeeI/project-photocraft-fork:perf/borrow-active-layer`
Checks:
- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and all 20 WASM checks passed.
- Before/after screenshots were rendered and inspected; the active-group view is unchanged.
- Independent GPT-6.1 Sol/xhigh source review found no blocker.
- Changed panel source is unchanged at upstream `7e7864a`.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `perf(ui): borrow the active layer from its document snapshot`

```markdown
## Summary

Borrow the active layer from the local document `Arc` instead of cloning it for the Layers panel's read-only controls.
The owning document snapshot remains alive through the read phase, and collected actions still run afterward.
This removes a `Layer::clone` of surface tile-map or recursive group metadata; copy-on-write pixel buffers were not deep-copied before this change.

## Validation

- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and `cargo xtask wasm` passed.
- No controlled frame-latency or allocation-count result is claimed.
- Before: ![Active group before](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-006-before.png)
- After: ![Active group after](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-006-after.png)

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
