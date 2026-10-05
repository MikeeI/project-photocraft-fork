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
| [ISSUE-001](issues/ISSUE-001.md) | compose: per-pixel gradient stop preparation | Submitted | Pull-Request-Implementation | New-pull-request | High | Await upstream gradient review | https://github.com/storytold/photocraft/pull/103 |
| [ISSUE-002](issues/ISSUE-002.md) | compose: redundant effect application halo | Submitted | Pull-Request-Implementation | New-pull-request | High | Await upstream halo review | https://github.com/storytold/photocraft/pull/110 |
| [ISSUE-003](issues/ISSUE-003.md) | compose: repeated full-pattern conversion | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream pattern review | https://github.com/storytold/photocraft/pull/104 |
| [ISSUE-005](issues/ISSUE-005.md) | UI: offscreen layer thumbnail work | Submitted | Pull-Request-Implementation | New-pull-request | High | Await upstream thumbnail-culling review | https://github.com/storytold/photocraft/pull/117 |
| [ISSUE-006](issues/ISSUE-006.md) | UI: redundant active-layer clone | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream layer-borrow review | https://github.com/storytold/photocraft/pull/121 |
| [ISSUE-007](issues/ISSUE-007.md) | UI: repeated uncached content bounds | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream bounds review | https://github.com/storytold/photocraft/pull/116 |
| [ISSUE-008](issues/ISSUE-008.md) | UI: channel-view thumbnail invalidation | Submitted | Pull-Request-Implementation | New-pull-request | High | Await upstream channel-cache review | https://github.com/storytold/photocraft/pull/120 |
| [ISSUE-009](issues/ISSUE-009.md) | compose: unnecessary backdrop snapshots | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream backdrop review | https://github.com/storytold/photocraft/pull/109 |
| [ISSUE-011](issues/ISSUE-011.md) | compose: repeated vector-mask compilation | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream vector review | https://github.com/storytold/photocraft/pull/112 |
| [ISSUE-012](issues/ISSUE-012.md) | UI: orphaned thumbnail texture handles | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream thumbnail-cleanup review | https://github.com/storytold/photocraft/pull/119 |
| [ISSUE-013](issues/ISSUE-013.md) | GPU canvas: orphaned display LUT resources | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream LUT-lifecycle review | https://github.com/storytold/photocraft/pull/118 |
| [ISSUE-014](issues/ISSUE-014.md) | file save: destructive overwrite before successful publication | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce failed document overwrite | Not published. |
| [ISSUE-015](issues/ISSUE-015.md) | recovery: delete snapshots before durable replacement | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce recovery snapshot deletion | Not published. |
| [ISSUE-016](issues/ISSUE-016.md) | file open: script events retarget the imported save path | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce open-event path retargeting | Not published. |
| [ISSUE-017](issues/ISSUE-017.md) | autosave: queue acknowledgment suppresses failed-save retries | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce failed autosave suppression | Not published. |
| [ISSUE-018](issues/ISSUE-018.md) | session: colliding persisted document IDs share autosave ownership | Submitted | Pull-Request-Implementation | New-pull-request | High | Await upstream identity review | https://github.com/storytold/photocraft/pull/114 |
| [ISSUE-019](issues/ISSUE-019.md) | native storage: shared temporary files break concurrent publication | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce overlapping native writers | Not published. |
| [ISSUE-020](issues/ISSUE-020.md) | close workflow: stale dirty list misses smart-object parent edits | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce stale close prompt list | Not published. |
| [ISSUE-021](issues/ISSUE-021.md) | smart objects: failed save-back still removes the edited child | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce failed smart-child close | Not published. |
| [ISSUE-022](issues/ISSUE-022.md) | PSD import: redundant-mask cleanup deletes the selected real mask | Submitted | Pull-Request-Implementation | New-pull-request | High | Await upstream real-mask review | https://github.com/storytold/photocraft/pull/130 |
| [ISSUE-023](issues/ISSUE-023.md) | flat export: native shortcut bypasses active compositing properties | Submitted | Pull-Request-Implementation | New-pull-request | High | Await upstream flat export review | https://github.com/storytold/photocraft/pull/122 |
| [ISSUE-024](issues/ISSUE-024.md) | indexed color: RGB reconstruction loses original palette indices | Investigating | Pull-Request-Implementation | New-pull-request | High | Reproduce palette identity loss | Not published. |
| [ISSUE-025](issues/ISSUE-025.md) | smart objects: explicit discard still commits child edits | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Reproduce ignored smart-child discard | Not published. |
| [ISSUE-026](issues/ISSUE-026.md) | canvas geometry: surface traversal omits smart-filter masks | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Reproduce smart-filter mask misalignment | Not published. |
| [ISSUE-027](issues/ISSUE-027.md) | history: unchanged bit depth clears redo | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream bit-depth history review | https://github.com/storytold/photocraft/pull/133 |
| [ISSUE-028](issues/ISSUE-028.md) | image rotation: lock bypass omits nested layers | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Compare nested locked rotation | Not published. |
| [ISSUE-029](issues/ISSUE-029.md) | native loading: fresh layer IDs leave stale variable targets | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Reproduce remapped variable targeting | Not published. |
| [ISSUE-030](issues/ISSUE-030.md) | native import: reordered ZIP entries defeat format detection | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Reproduce reordered bundle rejection | Not published. |
| [ISSUE-031](issues/ISSUE-031.md) | PSD import: white-unmatting clips supported HDR samples | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream HDR import review | https://github.com/storytold/photocraft/pull/106 |
| [ISSUE-032](issues/ISSUE-032.md) | PSD export: eight-bit opacity detection drops U16 alpha | Submitted | Pull-Request-Implementation | New-pull-request | Low | Await upstream merged alpha review | https://github.com/storytold/photocraft/pull/111 |
| [ISSUE-033](issues/ISSUE-033.md) | flat export: omitted channels lack document-level loss warnings | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Inspect flat channel-loss warnings | Not published. |
| [ISSUE-034](issues/ISSUE-034.md) | PSD export: full channel budget silently drops Quick Mask | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream Quick Mask warning review | https://github.com/storytold/photocraft/pull/115 |
| [ISSUE-035](issues/ISSUE-035.md) | effect cache: surface identity omits mask default pixels | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream cache review | https://github.com/storytold/photocraft/pull/105 |
| [ISSUE-036](issues/ISSUE-036.md) | group composition: opacity mixes straight-alpha colors directly | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream group coverage review | https://github.com/storytold/photocraft/pull/108 |
| [ISSUE-037](issues/ISSUE-037.md) | proxy rendering: artboard rectangles retain full-size coordinates | Submitted | Pull-Request-Implementation | New-pull-request | Medium | Await upstream artboard proxy review | https://github.com/storytold/photocraft/pull/131 |
| [ISSUE-038](issues/ISSUE-038.md) | file picker: read failures collapse into cancellation | Investigating | Pull-Request-Implementation | New-pull-request | Medium | Reproduce silent picker read failure | Not published. |

## Archived-Findings

| ID | Finding | Authorized-Work | Publication-Target | Contribution-Priority | Archive-Reason | External-Reference |
| --- | --- | --- | --- | --- | --- | --- |
| [ISSUE-004](issues/archive/ISSUE-004.md) | compose: repeated effect metadata derivation | Pull-Request-Implementation | New-pull-request | Medium | Not-Worth-Pursuing | Not published. |
| [ISSUE-010](issues/archive/ISSUE-010.md) | compose: unused proxy source-column copies | Pull-Request-Implementation | New-pull-request | Medium | Not-Worth-Pursuing | Not published. |
