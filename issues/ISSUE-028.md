# ISSUE-028 — image rotation: lock bypass omits nested layers

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

[S] Whole-image rotation bypasses position/all locks only at the root; recursive transformation rejects locked descendants.
Review mapping: `E15`, VALID; severity Medium.

## Reach-and-Impact

Trigger: rotate an image containing an unlocked group with a position-locked or fully locked raster child.
[S] The child lock rejects the entire operation even though the same root-level locked raster is deliberately allowed.
[S] Clone-before-commit preserves the original document on failure; partial mutation is not claimed.

## Evidence

- [S] `crates/engine/src/mode_cmds.rs::rotate_arbitrary` clears root `position`/`all` locks before calling `transform_layer`.
- [S] `crates/engine/src/transform_cmds.rs::transform_layer` recursively rejects locked group children.
- [S] `crates/engine/src/lib.rs::Session::edit` commits its candidate only after success, so failed rotation leaves live state unchanged.
- [O] The 30° nested-lock reproduction failed before the fix with `Other("layer \"Locked pixels\" is locked")`.
- [O] After the fix, root and grouped layers rotate with equal composites and all five lock flags retained.
- [O] Nested locked `Background` layers also retain their name and lock flags after rotation.

## Prior-Art

Coverage: Local issue ledger checked on 2026-10-05; no matching root cause.
Gaps: Upstream issues, PRs, discussions, and releases remain unsearched.
Contribution fit: The bounded fix is verified; upstream target fit remains unresolved.

## Proposed-Change

Propagate the whole-image lock-bypass policy recursively while preserving stored locks and Free Transform restrictions.

## Scope-and-Constraints

- Do not weaken ordinary Free Transform lock enforcement.
- Do not leave descendants unlocked after success or failure.
- Preserve root Background handling, descendant lock state, and transactional rollback.

## Verification

Status: reproduced before the fix and verified after.

- The 30° regression passes with equal root/group composites and all stored lock flags unchanged.
- Nested locked `Background` layers retain their name and lock flags after rotation.
- `cargo fmt --all` → passed.
- `cargo test --quiet -p photocraft-engine` → 596 passed, 9 ignored.
- `cargo test --quiet -p photocraft-engine whole_image_rotation_bypasses_nested_locks_without_changing_them` → 1 passed.
- `cargo clippy --quiet -p photocraft-engine --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → 27 crates, no violations.
- `cargo xtask wasm` → all 21 checks passed.

## Publication-Blockers

- Upstream issues, PRs, discussions, and releases remain unsearched.
- Required GPT-6.1 Sol/xhigh review cannot be run with the current configured model.
- The exact PR draft and approval remain outstanding.

## Next-Action

Summary: Search upstream prior art
Action: Check upstream issues, pull requests, discussions, and releases for this root cause.
Done-When: Record coverage, candidates, classifications, and contribution fit.

## Pull-Request-Implementation

Branch: fix/rotate-nested-locked-layers
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Propagate whole-image lock bypass recursively without weakening Free Transform or changing stored locks.
Commit: `fb066652a0f5b6fbacdd969c66c24a903942a7d7`
Push: `origin/fix/rotate-nested-locked-layers`
Checks:
- `cargo fmt --all` → passed.
- `cargo test --quiet -p photocraft-engine` → 596 passed, 9 ignored.
- `cargo test --quiet -p photocraft-engine whole_image_rotation_bypasses_nested_locks_without_changing_them` → 1 passed.
- `cargo clippy --quiet -p photocraft-engine --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → 27 crates, no violations.
- `cargo xtask wasm` → all 21 checks passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
