# Issue and Pull Request Tracking

Read this index at the start of every agent session before repository work.
`FORMAT.md` owns research, lifecycle, drafting, implementation, and publication rules.
Each linked `issues/ISSUE-NNN.md` is the complete authoritative record for one root cause.
This file owns `Next finding ID` and projects current issue-file state.
`Next-Action` is the 2–6 word `Next-Action/Summary` projection from the issue record.
When a row disagrees with its issue file, correct the row from the issue file in the same task.

Correctness review `E1`–`E25` maps in order to `ISSUE-014`–`ISSUE-038`.
Each record preserves its final review classification, severity, evidence limits, and corrected proposal.

Next finding ID: ISSUE-039

## Open-Findings

| ID | Finding | State | Authorized-Work | Publication-Target | Contribution-Priority | Next-Action | External-Reference |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [ISSUE-001](issues/ISSUE-001.md) | compose: per-pixel gradient stop preparation | Investigating | Pull-Request-Implementation | New-pull-request | High | Measure gradient preparation | Not published. |
| [ISSUE-002](issues/ISSUE-002.md) | compose: redundant effect application halo | Investigating | Pull-Request-Implementation | New-pull-request | High | Measure effect halo work | Not published. |
| [ISSUE-003](issues/ISSUE-003.md) | compose: repeated full-pattern conversion | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Measure pattern conversion pressure | Not published. |
| [ISSUE-004](issues/ISSUE-004.md) | compose: repeated effect metadata derivation | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Measure effect metadata traversal | Not published. |
| [ISSUE-005](issues/ISSUE-005.md) | UI: offscreen layer thumbnail work | Investigating | Pull-Request-Implementation | New-pull-request | High | Measure offscreen thumbnail work | Not published. |
| [ISSUE-006](issues/ISSUE-006.md) | UI: redundant active-layer clone | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Measure active-layer clone cost | Not published. |
| [ISSUE-007](issues/ISSUE-007.md) | UI: repeated uncached content bounds | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Compare cold and warm bounds | Not published. |
| [ISSUE-008](issues/ISSUE-008.md) | UI: channel-view thumbnail invalidation | Investigating | Pull-Request-Implementation | New-pull-request | High | Count view-only thumbnail rebuilds | Not published. |
| [ISSUE-009](issues/ISSUE-009.md) | compose: unnecessary backdrop snapshots | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Measure redundant backdrop copies | Not published. |
| [ISSUE-010](issues/ISSUE-010.md) | compose: unused proxy source-column copies | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Measure proxy row-copy volume | Not published. |
| [ISSUE-011](issues/ISSUE-011.md) | compose: repeated vector-mask compilation | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Measure vector-mask compilation work | Not published. |
| [ISSUE-012](issues/ISSUE-012.md) | UI: orphaned thumbnail texture handles | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Inspect thumbnail resource retention | Not published. |
| [ISSUE-013](issues/ISSUE-013.md) | GPU canvas: orphaned display LUT resources | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Inspect LUT cleanup lifecycle | Not published. |
| [ISSUE-014](issues/ISSUE-014.md) | file save: destructive overwrite before successful publication | Investigating | Not-Selected | Not-Selected | High | Reproduce failed document overwrite | Not published. |
| [ISSUE-015](issues/ISSUE-015.md) | recovery: delete snapshots before durable replacement | Investigating | Not-Selected | Not-Selected | High | Reproduce recovery snapshot deletion | Not published. |
| [ISSUE-016](issues/ISSUE-016.md) | file open: script events retarget the imported save path | Investigating | Not-Selected | Not-Selected | High | Reproduce open-event path retargeting | Not published. |
| [ISSUE-017](issues/ISSUE-017.md) | autosave: queue acknowledgment suppresses failed-save retries | Investigating | Not-Selected | Not-Selected | High | Reproduce failed autosave suppression | Not published. |
| [ISSUE-018](issues/ISSUE-018.md) | session: colliding persisted document IDs share autosave ownership | Investigating | Not-Selected | Not-Selected | High | Reproduce document identity collision | Not published. |
| [ISSUE-019](issues/ISSUE-019.md) | native storage: shared temporary files break concurrent publication | Investigating | Not-Selected | Not-Selected | High | Reproduce overlapping native writers | Not published. |
| [ISSUE-020](issues/ISSUE-020.md) | close workflow: stale dirty list misses smart-object parent edits | Investigating | Not-Selected | Not-Selected | High | Reproduce stale close prompt list | Not published. |
| [ISSUE-021](issues/ISSUE-021.md) | smart objects: failed save-back still removes the edited child | Investigating | Not-Selected | Not-Selected | High | Reproduce failed smart-child close | Not published. |
| [ISSUE-022](issues/ISSUE-022.md) | PSD import: redundant-mask cleanup deletes the selected real mask | Investigating | Not-Selected | Not-Selected | High | Reproduce real PSD mask loss | Not published. |
| [ISSUE-023](issues/ISSUE-023.md) | flat export: native shortcut bypasses active compositing properties | Investigating | Not-Selected | Not-Selected | High | Compare masked flat export | Not published. |
| [ISSUE-024](issues/ISSUE-024.md) | indexed color: RGB reconstruction loses original palette indices | Investigating | Not-Selected | Not-Selected | High | Reproduce palette identity loss | Not published. |
| [ISSUE-025](issues/ISSUE-025.md) | smart objects: explicit discard still commits child edits | Investigating | Not-Selected | Not-Selected | Medium | Reproduce ignored smart-child discard | Not published. |
| [ISSUE-026](issues/ISSUE-026.md) | canvas geometry: surface traversal omits smart-filter masks | Investigating | Not-Selected | Not-Selected | Medium | Reproduce smart-filter mask misalignment | Not published. |
| [ISSUE-027](issues/ISSUE-027.md) | history: unchanged bit depth clears redo | Investigating | Not-Selected | Not-Selected | Medium | Reproduce unchanged-depth redo loss | Not published. |
| [ISSUE-028](issues/ISSUE-028.md) | image rotation: lock bypass omits nested layers | Investigating | Not-Selected | Not-Selected | Medium | Compare nested locked rotation | Not published. |
| [ISSUE-029](issues/ISSUE-029.md) | native loading: fresh layer IDs leave stale variable targets | Investigating | Not-Selected | Not-Selected | Medium | Reproduce remapped variable targeting | Not published. |
| [ISSUE-030](issues/ISSUE-030.md) | native import: reordered ZIP entries defeat format detection | Investigating | Not-Selected | Not-Selected | Medium | Reproduce reordered bundle rejection | Not published. |
| [ISSUE-031](issues/ISSUE-031.md) | PSD import: white-unmatting clips supported HDR samples | Investigating | Not-Selected | Not-Selected | Medium | Reproduce HDR unmatte clipping | Not published. |
| [ISSUE-032](issues/ISSUE-032.md) | PSD export: eight-bit opacity detection drops U16 alpha | Investigating | Not-Selected | Not-Selected | Low | Compare U16 merged alpha | Not published. |
| [ISSUE-033](issues/ISSUE-033.md) | flat export: omitted channels lack document-level loss warnings | Investigating | Not-Selected | Not-Selected | Medium | Inspect flat channel-loss warnings | Not published. |
| [ISSUE-034](issues/ISSUE-034.md) | PSD export: full channel budget silently drops Quick Mask | Investigating | Not-Selected | Not-Selected | Medium | Reproduce Quick Mask capacity loss | Not published. |
| [ISSUE-035](issues/ISSUE-035.md) | effect cache: surface identity omits mask default pixels | Investigating | Not-Selected | Not-Selected | Medium | Compare mask effect-cache results | Not published. |
| [ISSUE-036](issues/ISSUE-036.md) | group composition: opacity mixes straight-alpha colors directly | Investigating | Not-Selected | Not-Selected | Medium | Reproduce group alpha darkening | Not published. |
| [ISSUE-037](issues/ISSUE-037.md) | proxy rendering: artboard rectangles retain full-size coordinates | Investigating | Not-Selected | Not-Selected | Medium | Compare proxy artboard clipping | Not published. |
| [ISSUE-038](issues/ISSUE-038.md) | file picker: read failures collapse into cancellation | Investigating | Not-Selected | Not-Selected | Medium | Reproduce silent picker read failure | Not published. |

## Archived-Findings

| ID | Finding | Authorized-Work | Publication-Target | Contribution-Priority | Archive-Reason | External-Reference |
| --- | --- | --- | --- | --- | --- | --- |
