# ISSUE-022 — PSD import: redundant-mask cleanup deletes the selected real mask

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/130
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
[O] The post-fix direct-import regression preserves Shape content and the selected real-mask sample `64/255`.
[O] The same existing test exports and reimports the mask sample without loss.
[O] Full IO tests passed: 195 across 18 suites; affected Clippy passed with warnings denied.
[O] Formatting, dependency layering, and all 20 WebAssembly packages passed.
[S] Independent xhigh review approved the shared source-selection predicate and both cleanup guards.
The fixture is synthetic, and neither the reimported layer kind nor every mask property is asserted.

## Publication-Blockers

None.

## Next-Action

Summary: Await upstream real-mask review
Action: Address review feedback on the submitted PSD mask correction.
Done-When: Upstream closes or merges the PR.

## Pull-Request-Implementation

Branch: fix/preserve-real-psd-mask
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Preserve selected real PSD masks while still eliminating redundant synthetic coverage.
Commit: `b09bebdea57bf5a1adeea8166316f3b83bbc7ed0`
Push: `origin/fix/preserve-real-psd-mask`.
Checks:
- Baseline direct-import regression: failed at missing selected mask.
- Post-fix direct import preserves shape and the real-mask sample; export/reimport retains that sample.
- Full IO suite: 195 passed across 18 suites.
- Affected Clippy with warnings denied, formatting, layering, and all 20 WebAssembly packages: passed.
- Independent xhigh source review: approved.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/preserve-real-psd-mask`
Title: Preserve selected real masks during PSD import

### Problem

PSD import can select a real user mask from channel `-3` and then discard it because the layer also marks vector-rendered mask coverage.
The synthetic-cleanup decision must use the same channel-and-metadata predicate that selected the mask.

### Change

Share the selected-real-mask predicate between mask decoding and both cleanup paths.
Preserve the chosen real mask; keep removing synthetic coverage when only the ordinary `-2` mask is selected.

### Verification

- The direct public-import regression failed before the fix because the selected real mask was removed from a synthetic Shape layer.
- The isolated post-fix regression preserves Shape content and mask sample `64/255`.
- The same existing test exports and reimports the mask sample.
- Full IO tests passed: 195 tests across 18 suites.
- Affected Clippy with warnings denied, formatting, dependency layering, and all 20 WebAssembly packages passed.
- Independent source review found no blocker.

The fixture is synthetic, not a Photoshop-produced PSD.
The roundtrip verifies one mask sample; it does not assert the reimported layer kind or every mask property.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
