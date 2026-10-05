# ISSUE-029 — native loading: fresh layer IDs leave stale variable targets

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Native loading remaps layer IDs but copies document variables with their original layer references.
Review mapping: `E16`, VALID; severity Medium.

## Reach-and-Impact

Trigger: load a data-driven template with `preserve_ids=false`, then apply a visibility or text data set.
[S] Variable lookup uses stale IDs, skipping absent targets or reaching an unintended matching layer.
[S] The loader also selects remapping automatically for implausibly distant stored IDs.
No template load/apply experiment was executed.

## Evidence

- [S] `crates/format/src/convert.rs:313-321` builds the old-to-new layer `id_map`.
- [S] `crates/format/src/convert.rs:488-498,526` remaps slice references but clones variables unchanged.
- [S] `crates/format/src/store.rs:166-170` retries loading with fresh IDs for far-ahead identifiers.
- [S] `crates/engine/src/variables_cmds.rs:161-173` resolves visibility/text targets through `def.layer`.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-018` concerns runtime document identity, not persisted layer-reference remapping.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a remapped template/data-set reproduction.

## Proposed-Change

Remap `VariableDef.layer` references through the existing loaded-layer `id_map`.
Keep the behavior for genuinely dangling references explicit rather than inventing replacement targets.

## Scope-and-Constraints

- Preserve normal `preserve_ids=true` behavior and valid variable bindings.
- Reuse the existing mapping owner rather than introducing a second ID allocator.
- Keep layer-comp and slice remapping intact.

## Verification

Status: source-traced; no remapped-template experiment executed.
- Load a template with `preserve_ids=false`, apply a visibility or text data set, and inspect its intended target.

## Publication-Blockers

- Runtime data-set targeting after ID remapping needs verification.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce remapped variable targeting
Action: Load a disposable native template with fresh IDs and apply one data-set value through the engine.
Done-When: Record stored and loaded layer IDs, variable references, and the actual modified layer.
