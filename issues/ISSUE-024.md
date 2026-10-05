# ISSUE-024 — indexed color: RGB reconstruction loses original palette indices

State: Investigating
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

[S] Color Table reconstructs pixel indices from mutable expanded RGB instead of retaining original index identity.
Review mapping: `E11`, CHANGED; severity High.
The correction spans index creation/import, editing, snapshots, and native persistence rather than one alpha branch.

## Reach-and-Impact

Trigger: make an indexed color transparent and remove transparency, or merge two entry colors then recolor one.
[S] Zero-alpha pixels are skipped on later edits; equal colors map to the lower index and lose independent identity.
No palette-edit sequence was executed.

## Evidence

- [S] `crates/engine/src/mode_cmds.rs:202-204` keeps quantized expanded pixels and a palette.
- [S] `crates/engine/src/mode_cmds.rs:254-255` promises that pixels retain their indices during table changes.
- [S] `crates/engine/src/mode_cmds.rs:282-285,303-315` removes transparency metadata but skips zero-alpha pixels.
- [S] `crates/doc/src/mode.rs:21-29` resolves equal RGB colors to the lower palette index.
- [S] `crates/doc/src/lib.rs:613-614` and `crates/format/src/manifest.rs:65-67` store the palette, not original indices.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: a local alpha-only patch is insufficient; data ownership and compatibility need investigation first.

## Proposed-Change

Retain original indices when quantizing or importing indexed data and derive expanded appearance from those indices.
Carry index identity through editing, undo snapshots, and native save/load.

## Scope-and-Constraints

- Preserve the intentional expanded rendering representation while giving index identity one authoritative owner.
- Do not infer lost distinct indices from equal RGB colors or promise recovery of ambiguous old saves.
- Avoid an alpha-only fix that leaves duplicate-color index collapse unresolved.
- The current PR authorization includes the index data model and required native persistence mapping.

## API-and-Compatibility

A new persisted index owner requires compatible native schema mapping and an explicit policy for old indexless saves.
Index identity must remain coherent with geometry, mode conversion, import, and undo rather than becoming a stale cache.

## Verification

Status: source-traced; no palette or persistence experiment executed.
- Change transparency on and off and verify the indexed region is visible again without undo.
- Make two palette colors equal, recolor one independently, then repeat after native save/load.

## Publication-Blockers

- Runtime palette failures and a bounded index-lifecycle compatibility design are missing.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Reproduce palette identity loss
Action: Trace transparency reversal and equal-color palette edits on a disposable indexed image.
Done-When: Record original index populations, table edits, expanded pixels, and where identity becomes unrecoverable.

## Pull-Request-Implementation

Branch: fix/retain-palette-indices
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Preserve indexed pixel identity across palette edits, import, undo, geometry, and native persistence.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
