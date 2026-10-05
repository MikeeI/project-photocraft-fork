# ISSUE-022 — PSD import: redundant-mask cleanup deletes the selected real mask

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

[S] Rendered-layer cleanup discards a selected real pixel mask using the outer synthetic-mask flag.
Review mapping: `E9`, CHANGED; severity High.
The corrected condition must follow actual mask-source selection, not the presence of real-mask metadata alone.

## Reach-and-Impact

Trigger: import a shape, text, or smart layer with vector-derived user-mask coverage and a separate real channel `-3`.
[S] Import chooses the real mask, then clears it; re-export cannot preserve the dropped editable mask.
No PSD fixture roundtrip was executed.

## Evidence

- [S] `crates/io/src/psd_import.rs:111-115` selects the real mask only when both metadata and channel `-3` exist.
- [S] `crates/io/src/psd_import.rs:267-276` clears the resulting mask for rendered layers based on outer flag bit 3.
- [S] `crates/io/src/psd_import.rs:284-299` distinguishes redundant vector coverage in a neighboring path.
- [S] `crates/io/src/psd_export.rs:159-164` exports a pixel mask only from `l.mask`.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Vector-mask compilation cost in `ISSUE-011` does not concern PSD mask-source ownership.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a representative PSD mask roundtrip.

## Proposed-Change

Use the same actual source-selection predicate as `record_mask` when deciding whether coverage is redundant.
Preserve a selected real mask; remove only synthetic coverage already represented by the cached rendered content.

## Scope-and-Constraints

- Do not preserve all synthetic masks indiscriminately and introduce double-masking.
- Real-mask metadata without channel `-3` does not prove that the real mask was selected.
- Preserve real mask values, defaults, density, and flags through import and re-export.

## Verification

Status: source-traced; no real-fixture or synthetic PSD roundtrip executed.
- Import and re-export a layer with independent real channel `-3` and compare its mask values.
- Verify purely synthetic coverage is still not applied twice.

## Publication-Blockers

- Representative real-mask preservation and synthetic-only rendering evidence are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce real PSD mask loss
Action: Trace a PSD carrying synthetic coverage and an independent real user mask through import and re-export.
Done-When: Record the selected channel, imported LayerMask, exported channels, and mask-pixel differences.
