# ISSUE-035 — effect cache: surface identity omits mask default pixels

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

[S] CPU effect-map identity hashes allocated surface tiles but omits the default pixel of unallocated mask regions.
Review mapping: `E22`, VALID; severity Medium.

## Reach-and-Impact

Trigger: render a shadowed raster with a tileless Reveal All mask, then replace it with tileless Hide All.
[S] The mask fingerprints match despite opposite coverage, allowing old shadow or glow maps to survive.
No application render comparison was executed.

## Evidence

- [S] `crates/doc/src/lib.rs:114-118` creates tileless masks differing only in their default sample.
- [S] `crates/compose/src/lib.rs:966-990` fingerprints tiles and mask flags but not surface defaults or format.
- [S] `crates/compose/src/lib.rs:1058-1066` reuses effect maps for the matching key.
- [S] `crates/compose/src/effects.rs:1046-1058` paints cached exterior shadow coverage.
- [S] `crates/engine/src/commands.rs:544-554,1049-1054` replaces the mask through normal commands.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; `ISSUE-004` was read for duplicate comparison.
`ISSUE-004` owns repeated metadata derivation cost, not missing identity inputs or stale effect results.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a warm-cache versus cold-cache image comparison.

## Proposed-Change

Include the surface default pixel and pixel format in effect-cache surface identity.

## Scope-and-Constraints

- Preserve tile pinning and copy-on-write identity guarantees.
- Do not introduce global cache purging as a workaround for incomplete identity.
- Coordinate any metadata optimization in `ISSUE-004` with the corrected semantic inputs.

## Verification

Status: source-traced; no application render comparison executed.
- Render Reveal All, switch to Hide All, and compare the warm-cache result with a cold-cache render.
- Exterior effects must not retain coverage from the previous mask.

## Publication-Blockers

- Deterministic warm/cold-cache output mismatch needs runtime verification.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Compare mask effect-cache results
Action: Render one shadowed raster across a Reveal All to Hide All transition with warm and cleared caches.
Done-When: Record mask defaults, cache conditions, expected coverage, and exact differing output pixels.
