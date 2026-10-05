# ISSUE-022 — PSD import: redundant-mask cleanup deletes the selected real mask

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

[S] Rendered-layer cleanup discards a selected real pixel mask using the outer synthetic-mask flag.
Review mapping: `E9`, CHANGED; severity High.
The corrected condition must follow actual mask-source selection, not the presence of real-mask metadata alone.

## Reach-and-Impact

Trigger: import a shape, text, or smart layer with vector-derived user-mask coverage and a separate real channel `-3`.
[S] Import chooses the real mask, then clears it; re-export cannot preserve the dropped editable mask.
[O] A synthetic shape passed to public `psd_to_document` lost its selected real mask before the fix.
The fixture asserts Shape content explicitly; serializing its empty vector tags first had produced a misleading raster case.

## Evidence

- [S] `crates/io/src/psd_import.rs:111-115` selects the real mask only when both metadata and channel `-3` exist.
- [S] `crates/io/src/psd_import.rs:267-276` clears the resulting mask for rendered layers based on outer flag bit 3.
- [S] `crates/io/src/psd_import.rs:284-299` distinguishes redundant vector coverage in a neighboring path.
- [S] `crates/io/src/psd_export.rs:159-164` exports a pixel mask only from `l.mask`.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Vector-mask compilation cost in `ISSUE-011` does not concern PSD mask-source ownership.
Upstream inventory covered all 102 issue/PR records, relevant discussions, and public releases; no matching root cause was found.
GitHub Discussions are disabled; project Discord history was inaccessible.

## Proposed-Change

Use the same actual source-selection predicate as `record_mask` when deciding whether coverage is redundant.
Preserve a selected real mask; remove only synthetic coverage already represented by the cached rendered content.

## Scope-and-Constraints

- Do not preserve all synthetic masks indiscriminately and introduce double-masking.
- Real-mask metadata without channel `-3` does not prove that the real mask was selected.
- Preserve real mask values, defaults, density, and flags through import and re-export.

## Verification

[O] The existing `vector_rendered_mask_not_doubled_on_shapes` extension failed at the missing selected real mask.
The source now shares actual real-mask selection across import and both synthetic-cleanup paths.
Post-fix isolated verification is queued behind the two active Cargo chains.
[S] Independent xhigh review approved the shared source-selection predicate and both cleanup guards.
The direct synthetic Shape assertion reproduces the defect; the byte roundtrip checks mask sample preservation.
No Photoshop corpus, complete mask-property coverage, or runtime coverage of both cleanup paths is claimed.

## Publication-Blockers

- Post-fix mask preservation, exported roundtrip, and required gates remain pending.

## Next-Action

Summary: Verify selected mask preservation
Action: Run isolated IO verification when a bounded build slot becomes available.
Done-When: The selected real mask survives import/export and the scoped PR is published.

## Pull-Request-Implementation

Branch: fix/preserve-real-psd-mask
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Preserve selected real PSD masks while still eliminating redundant synthetic coverage.
Commit: Pending.
Push: Pending.
Checks:
- Baseline direct-import regression: failed at missing selected mask.
- Post-fix isolated IO gates: queued.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
