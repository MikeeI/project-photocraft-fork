# ISSUE-035 — effect cache: surface identity omits mask default pixels

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/105
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
[O] The existing compositor test now reproduces stale shadow pixels and verifies equal warm/cold output after the fix.

## Evidence

- [S] `crates/doc/src/lib.rs:114-118` creates tileless masks differing only in their default sample.
- [S] `crates/compose/src/lib.rs:966-990` fingerprints tiles and mask flags but not surface defaults or format.
- [S] `crates/compose/src/lib.rs:1058-1066` reuses effect maps for the matching key.
- [S] `crates/compose/src/effects.rs:1046-1058` paints cached exterior shadow coverage.
- [S] `crates/engine/src/commands.rs:544-554,1049-1054` replaces the mask through normal commands.
- [O] The added Reveal All → Hide All regression failed before the fix at its warm/cold pixel comparison.
- [O] The fixed focused regression passed; an isolated build passed all 100 compose tests and affected-crate Clippy.
- [S] Upstream `7e7864afae8f779afff063a68edf208e30fe592c` leaves the affected compositor and dependency sources unchanged.

## Prior-Art

Coverage: local ledger and all 102 upstream issue/PR records, public releases, and relevant PR discussions checked.
No matching upstream root cause or active implementation was found.
`ISSUE-004` owns metadata derivation cost, not incomplete cache identity.
Related rendering changes in https://github.com/storytold/photocraft/pull/71 do not repair this cache key.
GitHub Discussions are disabled; project Discord history was inaccessible.

## Proposed-Change

Include the surface default pixel and pixel format in effect-cache surface identity.

## Scope-and-Constraints

- Preserve tile pinning and copy-on-write identity guarantees.
- Do not introduce global cache purging as a workaround for incomplete identity.
- Coordinate any metadata optimization in `ISSUE-004` with the corrected semantic inputs.

## Verification

[O] Regression: `cargo test -p photocraft-compose --lib tests::effect_maps_are_cached_and_invalidated_by_pixel_changes -- --exact` failed before and passed after the fix.
[O] `cargo test -p photocraft-compose` passed 97 unit and 3 integration tests in an isolated target directory.
[O] `cargo clippy -p photocraft-compose --all-targets -- -D warnings` passed.
[O] `cargo fmt -p photocraft-compose` and `git diff --check` passed.
[O] `cargo xtask layers` passed for 26 crates; `cargo xtask wasm` passed for all 20 selected packages.
The wasm check emitted existing unused-constant warnings in CMS and engine code.
An independent GPT-6.1 Sol review at runtime-verified xhigh effort found no source blocker.

## Publication-Blockers

None.

## Next-Action

Summary: Await upstream cache review
Action: Monitor the submitted PR for actionable maintainer feedback or CI failures.
Done-When: Feedback is addressed or the PR reaches a terminal upstream decision.

## Pull-Request-Implementation

Branch: fix/hash-mask-default-pixels
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Include surface default pixels and format in CPU effect-cache identity.
Commit: 8be4613ff402bae48455d6dbcefeb9c36244f191
Push: origin/fix/hash-mask-default-pixels
Checks:
- Focused regression: failed before, passed after.
- Affected-crate tests: 100 passed.
- Affected-crate Clippy and Git diff checks: passed.
- Layering and WebAssembly: passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/hash-mask-default-pixels`
Title: Fix stale effect maps after default-only mask changes

### Problem

Replacing a tileless Reveal All mask with Hide All can leave the previous exterior shadow visible.
Both masks have identical allocated tiles and flags, but different default pixels.
The effect-cache fingerprint omitted that default, allowing stale effect maps to be reused.

### Changes

- Include the surface pixel format and default samples in effect-cache identity.
- Retain the order-independent tile fingerprint and existing copy-on-write tile pinning.
- Extend the existing cache regression to compare every warm-cache pixel with an otherwise identical cold-cache render.

### Verification

The regression failed before this change and passes afterward.
It first establishes a visible shadow, replaces only the mask, then requires the shadow pixel to return to the white background.

- `cargo test -p photocraft-compose`: 97 unit and 3 integration tests passed.
- `cargo clippy -p photocraft-compose --all-targets -- -D warnings`: passed.
- `cargo fmt -p photocraft-compose` and `git diff --check`: passed.

This corrects CPU effect-map identity; it does not change GPU cache policy or claim a performance improvement.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
