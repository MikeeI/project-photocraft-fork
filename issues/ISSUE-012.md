# ISSUE-012 — UI: orphaned thumbnail texture handles

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/119
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The application thumbnail map retains handles after their layers or documents cease to be live.
Review mapping: `P12`, VALID; severity Medium.

## Reach-and-Impact

[S] Showing newly created or duplicated layers adds thumbnail keys during the application lifetime.
[S] Deleting layers or closing documents does not remove those keys from the thumbnail map.
Measurement: cumulative live-resource growth has not been observed at runtime; no exhaustion is claimed.

## Evidence

- [S] `crates/ui-egui/src/lib.rs:787-797` updates existing handles or inserts new thumbnail keys.
- [S] Production references to `thumbs` contain lookup, update, and insert but no remove, clear, or retain.
- [S] `crates/doc/src/lib.rs:50-61,505-516` provides fresh IDs for new or duplicated layers.
- [S] `crates/engine/src/lib.rs:288-295` closes documents without owning the UI thumbnail map.

## Prior-Art

Coverage: upstream issues, open/closed PRs, releases, and local ledger searched on 2026-10-05.
No matching thumbnail-handle pruning correction was found; this remains distinct from display-LUT ownership in ISSUE-013.
The changed UI source remains unchanged at `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`.

## Proposed-Change

Sweep thumbnail entries against live documents and their layer/mask IDs when document membership or revision changes.
Drop orphaned handles and regenerate thumbnails when undo or later visibility requires them again.

## Scope-and-Constraints

- Preserve live thumbnail handles and mask targeting; do not scan pixel tiles merely to establish liveness.
- Avoid a full layer-liveness sweep on every unchanged repaint.
- Do not retain document pixel snapshots solely for cleanup.
- Restored persisted IDs need not be globally fresh; growth claims concern distinct retained keys.

## Performance-Evidence

[S] Each retained 64-square RGBA thumbnail represents 16,384 nominal texel bytes plus map and resource overhead.
[O] Focused runtime sequence (baseline/populated/deleted/undo-recreated/closed textures): `1/3/3/3/3` before and `1/3/1/2/1` after.
[O] The candidate drops layer/mask handles on delete and remaining handles on close; the layer thumbnail regenerates on undo.
These are texture-manager counts, not measured GPU-resident bytes or document-session growth rates.

## Verification

- Inspect thumbnail key counts and texture resources across repeated New/Duplicate/Delete/Close cycles.
- Verify undo/redo recreates removed thumbnails and subsequent deletion releases them again.

## Publication-Blockers

None.
Texture-manager counts, rather than GPU residency or memory exhaustion, define the runtime claim.

## Next-Action

Summary: Await upstream thumbnail-cleanup review
Action: Respond to substantive maintainer feedback on PR #119.
Done-When: Record the upstream decision or requested follow-up.

## Pull-Request-Implementation

Branch: `fix/prune-orphaned-thumbnails`
Base: `upstream/main@7e7864afae8f779afff063a68edf208e30fe592c`
Scope: Retain the union of live layer/mask thumbnail keys after document snapshot or membership changes.
Commit: `f31ebfea58dfc3d179256204662c0a1e0be1f801`
Push: `MikeeI/project-photocraft-fork:fix/prune-orphaned-thumbnails`
Checks:
- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and all 20 WASM checks passed.
- Texture counts across generation, delete, undo, and close were `1/3/3/3/3` before and `1/3/1/2/1` after.
- Before/after UI screenshots were rendered and inspected.
- Independent GPT-6.1 Sol/xhigh source review found no blocker.
- Changed UI source is unchanged at upstream `7e7864a`.

## Publication-Draft

Target: `storytold/photocraft:main`
Title: `fix(ui): prune thumbnails for removed layers and documents`

```markdown
## Summary

Track the immutable document snapshots used for layer thumbnails.
When document membership or a document snapshot changes, retain only keys for live layer and mask IDs across every open document.
Undo and later visibility can regenerate removed thumbnails; the sweep avoids rescanning unchanged document trees.
Weak document identities detect replacement without retaining document pixels, including same-ID reopen.

## Validation

- `cargo test --locked -p photocraft-ui-egui`: 227 unit and 11 integration tests passed.
- UI crate Clippy with `--all-targets -- -D warnings`, `cargo xtask layers`, and `cargo xtask wasm` passed.
- Texture counts across thumbnail generation, delete, undo restoration, and close were `1/3/3/3/3` before and `1/3/1/2/1` after.
- The runtime counts reflect egui's texture manager, not measured GPU residency or a memory-exhaustion threshold.
- Before: ![Thumbnail lifecycle before](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-012-before.png)
- After: ![Thumbnail lifecycle after](https://raw.githubusercontent.com/MikeeI/project-photocraft-fork/personal/project/evidence/issue-012-after.png)

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
```
