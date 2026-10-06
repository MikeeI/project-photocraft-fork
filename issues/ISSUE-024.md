# ISSUE-024 — indexed color: RGB reconstruction loses original palette indices

State: Implementing
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`

## Root-Cause

[S] Indexed documents retained expanded pixels, not exact palette-entry IDs.
`image.mode.colorTable` reconstructed IDs from RGB and skipped zero-alpha pixels, making duplicate colors and transparent entries non-invertible.
Review mapping: `E11`, CHANGED; severity High.

## Reach-and-Impact

Trigger: unset transparency after assigning it to used pixels, or merge two entry colors then recolor one independently.
[O] The pre-fix regression left the transparent pixel invisible and rendered both formerly distinct entries blue after recoloring one green.
The affected scope is Indexed Color editing; real-user frequency is unmeasured.

## Evidence

- [S] At `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`, `crates/engine/src/mode_cmds.rs::color_table` reconstructed IDs from expanded RGB and skipped alpha-zero samples.
- [S] `crates/doc/src/mode.rs::ColorTable::nearest` chose the lower index for equal colors; upstream `LayerM` had no assignment plane.
- [S] The native manifest stored `DocM::color_table` but no per-pixel identity sidecar.
- [O] Test-only commit `4f4df70e1a32a1aef1fcd7dd19570083c95dd1a8`, based on that upstream revision, failed `mode_cmds::tests::color_table_presets_and_entries`: transparency remained zero and both entries stayed blue.

## Prior-Art

Coverage: GitHub issue and PR searches on 2026-10-06 used `indexed palette`, `"Color Table" indexed`, `indexed transparency`, `palette index`, `ColorTable`, and `"duplicate colors" palette`.
PR #17 fixed an empty-centers panic when forced colors filled the palette, not lost entry identity: https://github.com/storytold/photocraft/pull/17
Issue #213 mentions indexed ABR texture coverage, not Color Table identity: https://github.com/storytold/photocraft/issues/213
Release v0.2.0 lists PR #17; the discussions route returned HTTP 404: https://github.com/storytold/photocraft/releases/tag/v0.2.0 and https://github.com/storytold/photocraft/discussions
Contribution fit: no matching issue or PR surfaced; this is a distinct, bounded correctness fix.

## Proposed-Change

Store palette assignments and pre-transparency alpha with indexed rasters, then carry them through edits, undo, geometry, imports, and native persistence.
Reuse exact IDs only when transformations preserve identity; otherwise quantize into the active table.

## Scope-and-Constraints

- Keep expanded pixels as the rendering representation and add one authoritative per-raster identity owner.
- Never infer distinct IDs from equal RGB values in legacy indexless documents.
- Preserve exact assignments only when a data path proves their identity; quantize other output into the selected palette.
- Keep this correction within Indexed Color; do not alter unrelated mode behavior.

## API-and-Compatibility

The native manifest adds an optional per-layer sidecar with defaults for old saves.
Legacy Indexed documents still load without guessed IDs; Color Table edits reject missing identity data until explicit reconversion.
Switching out of Indexed Color clears sidecars, and undo restores them with the document snapshot.

## Verification

Status: implemented, committed, pushed, and verified on the rebased contribution branch.
- Baseline: the exact `color_table_presets_and_entries` regression failed on test-only commit `4f4df70e1a32a1aef1fcd7dd19570083c95dd1a8`.
- `cargo test --locked -p photocraft-doc -p photocraft-format -p photocraft-ops -p photocraft-io -p photocraft-engine` → 926 passed across 44 suites; 9 ignored.
- Strict Clippy across the five affected crates → passed.
- `cargo xtask layers` → 27 crates, no violations; `cargo xtask wasm` → all 21 checks passed.
- `cargo xtask test-corpus -p io -p engine` → corpus suites passed, including Photoshop and psd-tools PSD oracles.
- Corpus fetching ran in an isolated worktree at `a2236b04a9eb65ca0dffe4ae293065fc405f989d`; only the xtask User-Agent was overridden to installed Chrome 150.

## Publication-Blockers

- The required GPT-6.1 Sol review at xhigh has not occurred; the current runtime is `openai-codex/gpt-6-luna:medium`.
- The exact disclosure-bearing PR draft has not been finalized or shown; do not create an upstream PR without approval of its exact current draft and target.

## Next-Action

Summary: Prepare Sol-reviewed PR draft
Action: Obtain verifiable GPT-6.1 Sol xhigh review, finalize the exact PR draft, then show its target and body for approval.
Done-When: Model and effort are verified and the complete draft and target are shown; publication remains gated on approval.

## Pull-Request-Implementation

Branch: fix/retain-palette-indices
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Preserve indexed pixel identity across palette edits, import, undo, geometry, and native persistence.
Commit: a2236b04a9eb65ca0dffe4ae293065fc405f989d
Push: origin/fix/retain-palette-indices
Checks:
- `cargo test --locked -p photocraft-doc -p photocraft-format -p photocraft-ops -p photocraft-io -p photocraft-engine` → 926 passed.
- `cargo clippy --locked -p photocraft-doc -p photocraft-format -p photocraft-ops -p photocraft-io -p photocraft-engine --all-targets -- -D warnings` → passed.
- `cargo xtask layers` and `cargo xtask wasm` → passed.
- `cargo xtask test-corpus -p io -p engine` → passed against pinned corpora.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
