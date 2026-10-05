# Issue and Pull Request Tracking

Read this index at the start of every agent session before repository work.
`FORMAT.md` owns research, lifecycle, drafting, implementation, and publication rules.
Each linked `issues/ISSUE-NNN.md` is the complete authoritative record for one root cause.
This file owns `Next finding ID` and projects current issue-file state.
`Next-Action` is the 2–6 word `Next-Action/Summary` projection from the issue record.
When a row disagrees with its issue file, correct the row from the issue file in the same task.

Next finding ID: ISSUE-014

## Open-Findings

| ID | Finding | State | Authorized-Work | Publication-Target | Contribution-Priority | Next-Action | External-Reference |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [ISSUE-001](issues/ISSUE-001.md) | compose: per-pixel gradient stop preparation | Investigating | Not-Selected | Not-Selected | High | Measure gradient preparation | Not published. |
| [ISSUE-002](issues/ISSUE-002.md) | compose: redundant effect application halo | Investigating | Not-Selected | Not-Selected | High | Measure effect halo work | Not published. |
| [ISSUE-003](issues/ISSUE-003.md) | compose: repeated full-pattern conversion | Investigating | Not-Selected | Not-Selected | Medium | Measure pattern conversion pressure | Not published. |
| [ISSUE-004](issues/ISSUE-004.md) | compose: repeated effect metadata derivation | Investigating | Not-Selected | Not-Selected | Medium | Measure effect metadata traversal | Not published. |
| [ISSUE-005](issues/ISSUE-005.md) | UI: offscreen layer thumbnail work | Investigating | Not-Selected | Not-Selected | High | Measure offscreen thumbnail work | Not published. |
| [ISSUE-006](issues/ISSUE-006.md) | UI: redundant active-layer clone | Investigating | Not-Selected | Not-Selected | Medium | Measure active-layer clone cost | Not published. |
| [ISSUE-007](issues/ISSUE-007.md) | UI: repeated uncached content bounds | Investigating | Not-Selected | Not-Selected | Medium | Compare cold and warm bounds | Not published. |
| [ISSUE-008](issues/ISSUE-008.md) | UI: channel-view thumbnail invalidation | Investigating | Not-Selected | Not-Selected | High | Count view-only thumbnail rebuilds | Not published. |
| [ISSUE-009](issues/ISSUE-009.md) | compose: unnecessary backdrop snapshots | Investigating | Not-Selected | Not-Selected | Medium | Measure redundant backdrop copies | Not published. |
| [ISSUE-010](issues/ISSUE-010.md) | compose: unused proxy source-column copies | Investigating | Not-Selected | Not-Selected | Medium | Measure proxy row-copy volume | Not published. |
| [ISSUE-011](issues/ISSUE-011.md) | compose: repeated vector-mask compilation | Investigating | Not-Selected | Not-Selected | Medium | Measure vector-mask compilation work | Not published. |
| [ISSUE-012](issues/ISSUE-012.md) | UI: orphaned thumbnail texture handles | Investigating | Not-Selected | Not-Selected | Medium | Inspect thumbnail resource retention | Not published. |
| [ISSUE-013](issues/ISSUE-013.md) | GPU canvas: orphaned display LUT resources | Investigating | Not-Selected | Not-Selected | Medium | Inspect LUT cleanup lifecycle | Not published. |

## Archived-Findings

| ID | Finding | Authorized-Work | Publication-Target | Contribution-Priority | Archive-Reason | External-Reference |
| --- | --- | --- | --- | --- | --- | --- |
