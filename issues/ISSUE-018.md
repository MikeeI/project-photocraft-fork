# ISSUE-018 — session: colliding persisted document IDs share autosave ownership

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

[S] Session admission accepts duplicate persisted `DocId` values used as autosave ownership keys.
Review mapping: `E5`, CHANGED; severity High.
The corrected proposal preserves already unique identities and accounts for callers that capture IDs before admission.

## Reach-and-Impact

Trigger: open two native bundles containing the same document ID and edit both.
[S] Both documents select the same Autosaver and recovery path, allowing one snapshot to replace the other.
No multi-document recovery experiment was executed.

## Evidence

- [S] `crates/format/src/lib.rs:88-90` defaults to preserving IDs.
- [S] `crates/format/src/convert.rs:499-503` restores the serialized document ID.
- [S] `crates/engine/src/lib.rs:278-285` admits the document without detecting an existing identical ID.
- [S] `apps/photocraft/src/services.rs:149` keys both the saver and recovery bundle by that ID.
- [S] `crates/engine/src/smart_cmds.rs:618-624` captures `child_id` before admission and then builds a SmartLink.

## Prior-Art

Coverage: local ledger checked on 2026-10-05.
`ISSUE-013` concerns LUT cleanup on reopening; it does not own simultaneous document-identity collisions.
`ISSUE-019` concerns overlapping writer publication rather than session admission.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending identity and smart-child compatibility verification.

## Proposed-Change

Preserve already unique IDs and replace colliding runtime document IDs at session admission.
Make identity-dependent callers bind to the actually admitted document ID rather than an invalidated pre-admission value.

## Scope-and-Constraints

- Preserve layer IDs and document-internal references unless a separately justified remapping is required.
- Do not unconditionally reassign IDs behind SmartLink callers.
- Single-session uniqueness does not establish cross-process recovery isolation.

## API-and-Compatibility

Session admission and callers caching identity must agree on the effective document ID.
Preserve Edit Contents save-back behavior and recovery-entry provenance.

## Verification

Status: source-traced; no session identity experiment executed.
- Open two same-ID bundles, edit both, and verify distinct runtime IDs and independent recoverable snapshots.
- Open and save a Smart-Object child and verify its parent link remains valid.

## Publication-Blockers

- Duplicate-ID recovery and SmartLink preservation evidence are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce document identity collision
Action: Open two disposable same-ID native bundles and trace session IDs and recovery keys during autosave.
Done-When: Record admitted IDs, saver keys, persisted snapshots, and the existing SmartLink identity contract.
