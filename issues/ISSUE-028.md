# ISSUE-028 — image rotation: lock bypass omits nested layers

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

[S] Arbitrary whole-image rotation bypasses position/all locks only on root layers before recursive transformation.
Review mapping: `E15`, VALID; severity Medium.

## Reach-and-Impact

Trigger: rotate an image containing an unlocked group with a position-locked or fully locked raster child.
[S] The child lock rejects the entire operation even though the same root-level locked raster is deliberately allowed.
[S] Clone-before-commit preserves the original document on failure; partial mutation is not claimed.

## Evidence

- [S] `crates/engine/src/mode_cmds.rs:101-106` explicitly permits locked layers for whole-image rotation.
- [S] `crates/engine/src/transform_cmds.rs:73-80` checks locks again while recursively visiting children.
- [S] `crates/engine/src/lib.rs:338-345` installs the candidate document only after successful completion.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a root-versus-group rotation comparison.

## Proposed-Change

Forward the whole-image rotation lock-bypass policy recursively while preserving every stored lock flag.

## Scope-and-Constraints

- Do not weaken ordinary Free Transform lock enforcement.
- Do not leave descendants unlocked after success or failure.
- Preserve special Background-layer handling and transactional rollback.

## Verification

Status: source-traced; no rotation sequence executed.
- Rotate the same locked raster at the root and inside a group and compare transformed output.
- Verify original lock flags survive successful rotation and failed transformation.

## Publication-Blockers

- Nested-lock rotation behavior and unchanged lock-state evidence are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Compare nested locked rotation
Action: Run arbitrary image rotation on equivalent root-level and grouped locked raster layers.
Done-When: Record command parameters, errors or output geometry, and before/after lock flags.
