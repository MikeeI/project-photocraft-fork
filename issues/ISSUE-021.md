# ISSUE-021 — smart objects: failed save-back still removes the edited child

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

[S] Smart-child close discards the save-back error, removes the link, and permits removal of the edited document.
Review mapping: `E8`, VALID; severity High.

## Reach-and-Impact

Trigger: edit a smart-object child, then delete or rasterize its original parent layer before closing the child.
[S] The promised parent update can fail while `file.close` still reports success and removes the only edited child.
No command sequence or actual data loss was executed.

## Evidence

- [S] `crates/engine/src/smart_cmds.rs:638-649` fails if the linked layer is absent or no longer smart.
- [S] `crates/engine/src/smart_cmds.rs:656-663` ignores `commit_child` errors and removes the link.
- [S] `crates/engine/src/lib.rs:288-295` unconditionally removes the document after `on_close`.
- [S] `crates/engine/src/commands.rs:254-257` reports successful close; `smart_cmds.rs:829-839` promises save-back.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
This is error propagation, distinct from stale close prompts in `ISSUE-020` and ignored discard in `ISSUE-025`.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a command-level failure reproduction.

## Proposed-Change

Make the close precondition fallible and propagate save-back errors before removing the child or its link.
Migrate single and batch close callers to the resulting error contract.

## Scope-and-Constraints

- Preserve the dirty child's contents, link, and editability when its parent update fails.
- Preserve the existing clone-before-commit rollback of the parent document.
- Do not suppress errors in batch close or silently convert failures to successful removal.

## API-and-Compatibility

`Session::close` and callers must distinguish an invalid index, a failed required save-back, and successful removal.
Coordinate explicit discard policy with `ISSUE-025` without weakening the failure path.

## Verification

Status: source-traced; no command-level reproduction executed.
- Remove the linked smart layer after child editing and attempt `file.close` on the child.
- Require a visible error, unchanged parent, and an open, dirty, linked child.

## Publication-Blockers

- Failed-save-back close and batch-close propagation evidence are missing.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Reproduce failed smart-child close
Action: Attempt to close an edited smart child after replacing its linked parent layer in a disposable session.
Done-When: Record the error, parent state, child contents, link retention, and document membership.

## Pull-Request-Implementation

Branch: fix/preserve-failed-smart-child
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Propagate required smart-child save-back failures and preserve child contents and links.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
