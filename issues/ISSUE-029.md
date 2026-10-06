# ISSUE-029 — native loading: fresh layer IDs leave stale variable targets

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`

## Root-Cause

[S] Native loading remaps layer IDs but copies document variables with their original layer references.
Review mapping: `E16`, VALID; severity Medium.

## Reach-and-Impact

Trigger: load a data-driven template with `preserve_ids=false`, then apply a visibility or text data set.
[S] Variable lookup uses stale IDs, skipping absent targets or reaching an unintended matching layer.
[S] The loader also selects remapping automatically for implausibly distant stored IDs.

## Evidence

- [S] `crates/format/src/convert.rs::Loader::id` records stored-to-loaded layer IDs in `id_map`.
- [S] `crates/format/src/convert.rs::Loader::document` remaps slices but copies variables unchanged.
- [S] `crates/format/src/store.rs::load` retries with `preserve_ids=false` for far-ahead identifiers.
- [S] `crates/engine/src/variables_cmds.rs::apply_to_doc` resolves targets through `def.layer`.
- [O] Before the fix, applying `showBadge=false` after fresh-ID load left `badge` visible.
- [O] After the fix, the same data set hides `badge` and leaves `photo` visible.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-018` concerns runtime document identity, not persisted layer-reference remapping.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: The remapped-target fix is bounded and verified; upstream fit remains unresolved.

## Proposed-Change

Remap valid `VariableDef.layer` references through `id_map`; leave dangling targets unchanged and prevent them from aliasing fresh layers.

## Scope-and-Constraints

- Preserve normal `preserve_ids=true` behavior and valid variable bindings.
- Reuse the existing mapping owner rather than introducing a second ID allocator.
- Keep layer-comp and slice remapping intact.

## Verification

Status: reproduced before the fix and verified after.

- Applying `image.applyDataSet` after fresh-ID loading now changes the intended layer.

## Publication-Blockers

- Upstream issues, PRs, discussions, and releases remain unsearched.
- Required GPT-6.1 Sol/xhigh review cannot be run with the current configured model.
- The exact PR draft and approval remain outstanding.

## Next-Action

Summary: Search upstream prior art
Action: Check upstream issues, pull requests, discussions, and releases for this root cause.
Done-When: Record coverage, candidates, classifications, and contribution fit.

## Pull-Request-Implementation

Branch: fix/remap-variable-layer-ids
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Map variable layer references during fresh-ID loads without redirecting dangling targets.
Commit: `842831ca2663e30f57154284d520ad03228dfe1b`
Push: `origin/fix/remap-variable-layer-ids`
Checks:
- `cargo fmt --all` → passed.
- `cargo test --quiet -p photocraft-format` → passed.
- `cargo test --quiet -p photocraft-engine` → 596 passed, 9 ignored.
- `cargo test --quiet -p photocraft-engine fresh_id_load_remaps_variable_targets_before_applying_a_data_set` → 1 passed.
- `cargo clippy --quiet -p photocraft-format --all-targets -- -D warnings` → passed.
- `cargo clippy --quiet -p photocraft-engine --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → 27 crates, no violations.
- `cargo xtask wasm` → all 21 checks passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
