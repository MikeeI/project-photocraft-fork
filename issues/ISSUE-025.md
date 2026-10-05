# ISSUE-025 — smart objects: explicit discard still commits child edits

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The UI's explicit Don't Save decision reaches an engine close path that automatically commits dirty smart children.
Review mapping: `E12`, VALID; severity Medium.

## Reach-and-Impact

Trigger: edit Smart-Object contents, close the child, and choose `Don't Save`.
[S] The rejected changes are copied into the parent and can be persisted by a later normal parent Save.
Parent undo can limit the consequence; no UI reproduction was executed.

## Evidence

- [S] `crates/ui-egui/src/discard_ui.rs:165-173` advances the parked close action after explicit discard.
- [S] `crates/engine/src/commands.rs:254-257` calls `Session::close` without a discard policy.
- [S] `crates/engine/src/lib.rs:292-293` calls `on_close` before removing the child.
- [S] `crates/engine/src/smart_cmds.rs:660-661` commits every dirty linked child on that path.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-021` is failed commit handling; this finding applies even when the unwanted commit succeeds.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending explicit-discard reproduction and close API review.

## Proposed-Change

Carry the explicit discard decision into the close boundary and prevent automatic save-back on that path.
Preserve separately documented save-back behavior for callers that did not request discard.

## Scope-and-Constraints

- Do not remove parent undo history or change Save and Cancel behavior to disguise the contradiction.
- Coordinate the engine close contract with `ISSUE-021` and the UI orchestration in `ISSUE-020`.
- Keep existing frontend behavior explicit when changing close parameters or policy.

## Verification

Status: source-traced; no UI discard sequence executed.
- Edit a child, choose `Don't Save`, and compare parent content and dirty state with their pre-edit values.

## Publication-Blockers

- Explicit-discard behavior and other frontend close contracts need verification.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Reproduce ignored smart-child discard
Action: Close a visibly edited smart child with Don't Save and inspect the parent before any later save.
Done-When: Record the selected decision, parent pixels, parent revision, and child removal.

## Pull-Request-Implementation

Branch: fix/discard-smart-child-edits
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Carry explicit discard decisions into smart-child close without changing Save or Cancel behavior.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
