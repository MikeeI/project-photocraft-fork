# ISSUE-018 — session: colliding persisted document IDs share autosave ownership

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/114
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
The isolated focused post-fix UI regression passed; the full engine/UI verification passed 723 tests across six suites, with five ignored.
[O] Formatting and affected Clippy with warnings denied, dependency layering, and all 20 WebAssembly packages passed.
[S] Independent xhigh review approved the admission invariant and SmartLink caller migration.
The review found no further identity-dependent admission caller; semantic references were unavailable.
The regression proves distinct autosave requests, not two recovered on-disk bundles.

## Publication-Blockers

None.

## Next-Action

Summary: Await upstream identity review
Action: Address review feedback on the submitted session identity correction.
Done-When: Upstream closes or merges the PR.

## Pull-Request-Implementation

Branch: fix/unique-session-document-ids
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Resolve colliding document identities at admission and migrate identity-dependent callers.
Commit: `a57ce8c9751f14771800d0b9118ece316715cbde`
Push: `origin/fix/unique-session-document-ids`.
Checks:
- Isolated baseline regression: one autosave request instead of two.
- Isolated focused post-fix UI regression: passed.
- Full engine/UI verification: 723 tests passed across six suites; five ignored.
- Formatting, affected Clippy with warnings denied, layering, and all 20 WebAssembly packages: passed.
- Independent xhigh source review: approved.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Title: Disambiguate colliding document identities at session admission
Target: `storytold/photocraft:main`
Head: `MikeeI:fix/unique-session-document-ids`

### Problem

Session admission accepts duplicate persisted document IDs.
Autosave ownership and recovery snapshots use that ID, so equal IDs can suppress an autosave for a second dirty document.
Identity-dependent callers must also use the ID actually admitted into the session.
The regression demonstrates suppressed autosave requests; two on-disk recovery bundles were not separately inspected.

### Change

Keep an incoming ID when it is not already open in the session.
When it collides, allocate until a unique live ID is found before admitting the document.
Edit Contents now reads the admitted child ID before registering its SmartLink.

### Verification

- The existing autosave regression failed before the fix with one request instead of two.
- The isolated post-fix regression passed with distinct IDs and equal revisions.
- The full engine/UI verification passed: 723 tests across six suites, with five ignored.
- Formatting, affected Clippy with warnings denied, dependency layering, and all 20 WebAssembly packages passed.
- Independent source review found no blocker and inspected session admission callers.

This establishes uniqueness among documents simultaneously open in one Session; it does not establish cross-process recovery isolation or verify two completed on-disk recovery bundles.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
