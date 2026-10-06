# ISSUE-033 — flat export: omitted channels lack document-level loss warnings

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

[S] Flat conversion discards saved extra channels and Quick Mask before codec warning generation can see them.
Review mapping: `E20`, CHANGED; severity Medium.
Document-level warnings must cover mode-specific early returns as well as the general flat conversion path.

## Reach-and-Impact

Trigger: export a one-raster document with a saved selection or Quick Mask to a flat format.
[S] Reopening cannot recover those omitted channels, while the export warning list does not describe that loss.
A baseline regression test reproduced the missing warning; no standalone export/reopen experiment was run.

## Bug-Reproduction

Environment: Ubuntu 24.04.5, x86_64, `rustc 1.99.0 (b940084d7 2026-09-28)`.
Baseline: `upstream/main@47f4abfed49e0d2f5b9277287b27dee632530ba4`.
Command: `cargo test --locked -p photocraft-io --test modes indexed_png_is_palette_png -- --exact`.
Observed: the test failed because warnings only contained `written as an 8-bit palette PNG (2 colours)`.
Currentness: Flat-export files were unchanged from source base `5ada60c35b2fcf4e6567b2d3ee39d4ed318454ad` to this base.
The rebased regression and gates passed.

## Evidence

- [S] `crates/io/src/lib.rs:85-91` promises warnings about approximated or dropped data.
- [S] `crates/io/src/flat.rs:125-151,204-210` carries only surface samples and selected metadata.
- [S] `crates/io/src/flat.rs:254-259` can return mode-specific output before `document_to_image` runs.
- [S] `crates/io/src/flat.rs:267-274` obtains codec warnings from the already reduced image representation.

## Prior-Art

Coverage: broad prior-art searches and local ledger checked 2026-10-05; branch-specific PR searches checked 2026-10-06.
Issue searches `“saved channels” export OR “Quick Mask” export` and `flat export channels mask loss` returned no matches.
PR search `flat export channel warning OR Quick Mask` returned #195 and #71; both are unrelated.
PR search `saved channels export conversion` returned #70, which covers canvas geometry rather than flat-export data loss.
PR #195 covers PSD masks and layer fidelity, PR #71 covers export performance, and PR #70 covers geometry; none owns this warning loss.
The v0.2.0 notes mention only an unrelated Indexed Color forced-colour panic fix: https://github.com/storytold/photocraft/releases/tag/v0.2.0
The discussions route returned 404 and could not be searched: https://github.com/storytold/photocraft/discussions
`ISSUE-034` concerns PSD channel-budget exhaustion, not flat conversion's missing document-level warnings.
No open upstream or fork PR uses `fix/warn-flat-channel-loss` (checked 2026-10-06).
Contribution fit: distinct document-level warnings now cover both general and mode-specific flat-export branches.

## Proposed-Change

Determine omitted extra-channel and Quick Mask warnings before dispatching to flat-export branches.
Merge those warnings into every affected result, including mode-specific early returns.

## Scope-and-Constraints

- Report actual loss without claiming that every flat codec must gain editable document-channel support.
- Do not place the entire correction only in `document_to_image`.
- Preserve existing fatal codec errors, metadata, color conversion, and other fidelity warnings.

## Verification

Status: verified by baseline reproduction, focused warning/reopen assertions, current-base crate gates, and corpus tests.
- `cargo test --locked -p photocraft-io --test modes indexed_png_is_palette_png -- --exact` → passed.
- `cargo test --locked -p photocraft-io --test modes duotone_exports_the_inks_as_rgb -- --exact` → passed.
- `cargo test --quiet --locked -p photocraft-io` → passed on current base.
- `cargo clippy --quiet --locked -p photocraft-io --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → 27 crates, no layering violations.
- `cargo xtask wasm` → all 21 wasm-compatible crates passed.
- `cargo xtask test-corpus` → passed.

## Publication-Blockers

- The discussions route was unavailable (404); no discussion history was inspected.
- The exact PR draft is not finalized.
- Independent GPT-6.1 Sol/xhigh review evidence is unavailable in this runtime; GPT-6 Luna medium is not a substitute.

## Next-Action

Summary: Obtain GPT-6.1 review
Action: Obtain independent GPT-6.1 Sol/xhigh review evidence for the rebased implementation.
Done-When: Record the review result and draft truthful disclosure language before publication.

## Pull-Request-Implementation

Branch: fix/warn-flat-channel-loss
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Report saved-channel and Quick Mask omissions across every flat-export branch.
Commit: `30ea8eb0062a8139d64e35e0b82db323f17667b4`
Push: `origin/fix/warn-flat-channel-loss` at this commit; updated with `--force-with-lease`.
Checks:
- Current-base `photocraft-io` tests, including warning/reopen regressions → passed.
- Strict Clippy for `photocraft-io` → passed.
- `cargo xtask layers` (27 crates), `cargo xtask wasm` (21 checks), and `cargo xtask test-corpus` → passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
