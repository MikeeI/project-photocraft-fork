# ISSUE-012 — UI: orphaned thumbnail texture handles

State: Investigating
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

Coverage: local ledger checked on 2026-10-05; distinct from display-LUT ownership in `ISSUE-013`.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: potentially bounded resource-lifecycle correction; runtime confirmation and prior art remain open.

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
[S] Retention follows distinct previously displayed keys rather than current live-layer count.
Measurement: actual GPU allocation, release timing, and session growth rate are unknown.

## Verification

- Inspect thumbnail key counts and texture resources across repeated New/Duplicate/Delete/Close cycles.
- Verify undo/redo recreates removed thumbnails and subsequent deletion releases them again.

## Publication-Blockers

- Runtime resource-lifecycle confirmation and restoration verification are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Inspect thumbnail resource retention
Action: Track thumbnail keys and texture resources across repeated layer creation, deletion, and document closure.
Done-When: Record the operation sequence and retained counts after closure, distinguishing live handles from driver residency.
