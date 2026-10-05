# ISSUE-017 — autosave: queue acknowledgment suppresses failed-save retries

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Reliability
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The UI marks a revision autosaved when the service merely queues its snapshot, before the worker writes it.
Review mapping: `E4`, VALID; severity High.

## Reach-and-Impact

Trigger: a dirty document reaches the autosave interval while recovery storage is unavailable.
[S] A later worker failure does not clear the UI acknowledgment, so unchanged revisions are excluded from retry.
No worker failure or resulting crash loss was observed.

## Evidence

- [S] `apps/photocraft/src/services.rs:146-151` returns success immediately after `Autosaver::request`.
- [S] `crates/format/src/autosave.rs:79-90` queues a job rather than confirming a write.
- [S] `crates/format/src/autosave.rs:132-140` records the actual fallible result later in the worker.
- [S] `crates/ui-egui/src/prefs_ui.rs:217-224` skips acknowledged revisions and records queue success as saved.
- [S] `crates/format/src/autosave.rs:93-102` exposes results, but inspected app/UI callers do not consume them.

## Prior-Art

Coverage: local ledger and production completion consumers checked on 2026-10-05; no matching record.
`ISSUE-015` is the separate recovery-deletion policy.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending failure/retry reproduction and upstream ownership search.

## Proposed-Change

Return completion results containing document ID, revision, and success or error to the UI.
Confirm only successfully persisted revisions; report failures and leave them eligible for retry.

## Scope-and-Constraints

- Preserve background saving and snapshot coalescing without blocking the UI on filesystem work.
- Distinguish queued, in-flight, completed, and superseded revisions.
- Do not apply stale completions to closed or unrelated documents.

## API-and-Compatibility

The service contract must distinguish queue acceptance from completed persistence.
The existing statistics-only `last_result` cannot identify the completed revision on its own.

## Verification

Status: source-traced; no worker/failure experiment executed.
- Fail an autosave, restore writable storage, and wait another interval without editing the document.
- Confirm the same revision retries and the original failure is visible.

## Publication-Blockers

- Completed-revision acknowledgment and unchanged-revision retry evidence are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce failed autosave suppression
Action: Observe one unchanged dirty revision across a failed autosave and the next writable interval.
Done-When: Record queued and completed revisions, worker result, UI acknowledgment, retry behavior, and error visibility.
