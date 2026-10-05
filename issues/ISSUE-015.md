# ISSUE-015 — recovery: delete snapshots before durable replacement

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Reliability
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Launch recovery deletes every listed recovery entry regardless of load success or replacement persistence.
Review mapping: `E2`, VALID; severity Critical.

## Reach-and-Impact

Trigger: a transient recovery read failure, or another crash after recovery but before the next successful autosave.
[S] Failed recovery loses its retained input; successful recovery initially exists only in memory after deletion.
No crash sequence or actual loss was executed.

## Evidence

- [S] `apps/photocraft/src/services.rs:161-166` conditions document adoption on success but deletes unconditionally.
- [S] `crates/format/src/autosave.rs:176-184` removes the bundle directory and sidecar.
- [S] `crates/ui-egui/src/prefs_ui.rs:108-115` installs recovered documents only after the service returns.
- [S] `crates/ui-egui/src/prefs_ui.rs:204-212` initially defers autosave for a full interval.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-017` owns autosave completion acknowledgment, not deletion during recovery admission.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending recovery reproduction and upstream ownership search.

## Proposed-Change

Keep recovery entries after failed loads and report their errors.
Retain successfully loaded entries until a confirmed replacement save or explicit discard owns their removal.
Track the original recovery entry independently from any newly assigned runtime document identity.

## Scope-and-Constraints

- Preserve recoverable input when loading, session admission, or replacement persistence fails.
- Do not interpret a queued autosave as a persisted replacement; coordinate with `ISSUE-017`.
- Remove only the recovery entry owned by the confirmed replacement or discard decision.

## Verification

Status: source-traced; no recovery or crash experiment executed.
- Force a recovery load failure and verify the bundle and sidecar remain available.
- Recover successfully, terminate before replacement autosave, and recover the same changes again.

## Publication-Blockers

- Failed-load and repeated-crash preservation evidence is missing.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Reproduce recovery snapshot deletion
Action: Trace one disposable recovery entry through a failed load and an immediate post-recovery restart.
Done-When: Record entry existence, load results, session adoption, and whether the second launch can recover it.

## Pull-Request-Implementation

Branch: fix/retain-recovery-snapshots
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Retain recovery input until successful replacement or explicit discard owns deletion.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
