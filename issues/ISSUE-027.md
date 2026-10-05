# ISSUE-027 — history: unchanged bit depth clears redo

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: Medium
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The unchanged-depth early return occurs inside a successful edit transaction that still records history.
Review mapping: `E14`, VALID; severity Medium.
The history-preserving no-op contract is implied by this explicitly unchanged conversion, not a universal command rule.

## Reach-and-Impact

Trigger: edit an eight-bit document, undo, then execute `image.mode.bits8`.
[S] No depth conversion occurs, but a new history step clears the previously available redo branch.
[A] The contract inference should be confirmed against intended no-op command semantics before publication.
No command sequence was executed.

## Evidence

- [S] `crates/engine/src/image_cmds.rs:317-319` detects equal depth inside `s.edit`.
- [S] `crates/engine/src/image_cmds.rs:422-424` enables depth commands for any open document.
- [S] `crates/engine/src/lib.rs:345-356` publishes and records every successful edit closure.
- [S] `crates/ops/src/lib.rs:49-52` clears redo when recording a new step.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, releases, and an explicit no-op-history policy not established.
Contribution fit: unresolved until the behavior and intended unchanged-depth contract are confirmed.

## Proposed-Change

Detect identical depth before entering `Session::edit`, leaving document revision and redo unchanged.

## Scope-and-Constraints

- Limit the correction to the explicitly recognized unchanged-depth case.
- Do not impose generic deep-document equality checks on every edit.
- Preserve real depth conversion, dirty tracking, and normal undo/redo behavior.

## Verification

Status: source-traced; no history sequence executed.
- Edit, undo, select the current bit depth, and verify unchanged revision and redo availability.
- Confirm a real depth conversion still creates its normal undoable history step.

## Publication-Blockers

- Runtime redo-loss reproduction and the intended unchanged-depth contract need confirmation.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Reproduce unchanged-depth redo loss
Action: Trace history and revision across edit, undo, and selection of the already active bit depth.
Done-When: Record before/after history, redo availability, unchanged pixel depth, and the applicable no-op contract.

## Pull-Request-Implementation

Branch: fix/preserve-noop-depth-history
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Preserve revision and redo when an unchanged bit depth bypasses conversion.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
