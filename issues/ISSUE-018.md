# ISSUE-018 — session: colliding persisted document IDs share autosave ownership

State: Implementing
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
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
[O] An isolated existing UI regression admitted two equal-ID documents at equal dirty revisions and observed only one autosave request.
This proves suppression at the autosave ownership boundary, not a completed disk-recovery experiment.

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
Upstream inventory covered all 102 issue/PR records, relevant discussions, and public releases; no matching root cause was found.
GitHub Discussions are disabled; project Discord history was inaccessible.

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

[O] The extended `prefs_ui::tests::autosave_runs_for_dirty_documents` failed before the fix: one request instead of two.
The admission loop and SmartLink caller migration are implemented.
The isolated focused regression and required crate/portability checks are running.
[S] Independent xhigh review approved the admission invariant and SmartLink caller migration.
The review found no further identity-dependent admission caller; semantic references were unavailable.
The regression proves distinct autosave requests, not two recovered on-disk bundles.

## Publication-Blockers

- Post-fix runtime checks and SmartLink compatibility remain pending.

## Next-Action

Summary: Verify unique admission fix
Action: Finish the isolated validation chain and review before publishing.
Done-When: Both copies autosave independently, affected tests pass, and the scoped PR is published.

## Pull-Request-Implementation

Branch: fix/unique-session-document-ids
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Resolve colliding document identities at admission and migrate identity-dependent callers.
Commit: Pending.
Push: Pending.
Checks:
- Isolated baseline autosave regression: failed with one request instead of two.
- Post-fix verification: running in target/issue-018.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
