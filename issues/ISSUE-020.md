# ISSUE-020 — close workflow: stale dirty list misses smart-object parent edits

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Close All and Exit keep the initially dirty document list even when saving a child makes its parent dirty.
Review mapping: `E7`, CHANGED; severity High.
Recomputing dirty documents requires decisions scoped to document ID and revision, not repeated unconditional prompts.

## Reach-and-Impact

Trigger: a clean saved parent has a dirty Edit Contents child; choose Save for that child during Close All or Exit.
[S] Child Save updates the parent in memory, but the initial prompt list contains no parent entry.
[S] Finishing the list can close or exit without persisting or asking about the newly dirty parent.
No UI sequence was executed.

## Evidence

- [S] `crates/ui-egui/src/discard_ui.rs:47-66` captures affected dirty IDs once.
- [S] `crates/ui-egui/src/lib.rs:494-497` routes child Save to smart-object save-back.
- [S] `crates/engine/src/smart_cmds.rs:638-651` edits the parent and marks only the child saved.
- [S] `crates/ui-egui/src/discard_ui.rs:92-104` completes the original list without discovering new dirty parents.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-021` owns failed child commit handling; `ISSUE-025` owns explicit discard semantics.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending the parent-clean/child-dirty UI sequence.

## Proposed-Change

Recompute affected document revisions after each Save and honor answered decisions per `DocId` and revision.
Ask about newly dirty parents while avoiding repeated prompts for an unchanged revision explicitly discarded already.

## Scope-and-Constraints

- Preserve Cancel and Save-dialog cancellation without closing documents.
- Keep stable-ID targeting when tabs move during a prompt.
- Integrate after the engine close/error/discard contracts in `ISSUE-021` and `ISSUE-025` are defined.
- Keep this policy inside the existing close workflow rather than adding a generic workflow engine.

## Verification

Status: source-traced; no UI close sequence executed.
- Save a dirty child during Close All and Exit and require a subsequent prompt for its newly dirty parent.
- An unchanged explicitly discarded revision must not trigger an endless sequence of repeated prompts.

## Publication-Blockers

- Close All, Exit, and revision-scoped decision behavior need UI reproduction.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce stale close prompt list
Action: Close a clean-parent/dirty-child session and choose Save for the child while tracing dirty revisions.
Done-When: Record prompts, save-back effects, parent persistence, and the final close or exit decision.
