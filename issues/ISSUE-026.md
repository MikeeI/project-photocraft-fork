# ISSUE-026 — canvas geometry: surface traversal omits smart-filter masks

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

[S] The shared canvas surface traversal visits Smart-Object caches but omits their document-coordinate filter masks.
Review mapping: `E13`, VALID; severity Medium.

## Reach-and-Impact

Trigger: resize or translate a canvas containing a renderable Smart Object with a nonuniform filter mask.
[S] Image Size refreshes the smart content immediately with an unscaled mask.
[S] Canvas translation can initially move the cache correctly but exposes mask misalignment on the next refresh.
[O] The pre-fix Image Size defect reproduced; post-fix regressions verify resampling and delayed Canvas Size refresh.

## Evidence

- [S] `crates/engine/src/image_cmds.rs:20-49` omits `sm.filter_mask.surface` from the shared mask-aware traversal.
- [S] Image Size and `translate_doc` call the traversal with `masks=true`; crop clipping uses `masks=false`.
- [S] `canvas_geom::refresh(Refresh::All)` re-renders Smart Objects; `smart_cmds::mask_mix` samples filter masks in document coordinates.
- [O] Before the fix, the Image Size regression differed at the masked pixel: `[1.0, 0.0, 0.0, 1.0]` versus `[1.0, 0.0, 0.0, 0.6313726]` after disabling the mask.
- [O] Post-fix contract tests cover Image Size resampling and Canvas Size translation before a later smart-object refresh.

## Prior-Art

Coverage: upstream main at `a96a621deea97d4b1ecd173b8b921587e33f3ca5` and merged PR #70 reviewed on 2026-10-06.
PR #70 explicitly leaves Canvas Size and Crop smart-filter-mask translation unresolved (https://github.com/storytold/photocraft/pull/70).
This is partial prior art, not a fix; Image Size resampling is also affected in current source.

## Proposed-Change

Visit `sm.filter_mask.surface` with the mask flag when mask processing is enabled in the existing traversal.

## Scope-and-Constraints

- Preserve filter-mask interpolation policy and ordinary smart-object movement.
- Avoid double transformation in paths that already remap the mask separately.
- Do not replace source-unavailable cache fallback with a new rendering policy.

## Verification

Status: pre-fix defect reproduction and post-fix engine behavior verified on 2026-10-06.
- [O] Before the fix, `cargo test --quiet -p photocraft-engine image_size_scales_smart_filter_mask_before_refresh` failed with the expected masked-pixel difference.
- [O] After the fix, `cargo test --quiet -p photocraft-engine smart_filter_mask_before_refresh` → 2 passed.
- [O] `cargo test --quiet -p photocraft-engine` → 596 passed, 9 ignored.
- [O] `cargo clippy --quiet -p photocraft-engine --all-targets -- -D warnings` passed after both regressions.
- [O] `cargo xtask layers` → 27 crates, no violations.
- [O] `cargo xtask wasm` → all 21 package checks passed; existing unused-code warnings remain.
- [O] `cargo fmt --all` passed.

## Publication-Blockers

- The independent GPT-6.1 Sol/xhigh review is unavailable in this session; do not substitute another model.
- Upstream issue, release, and discussion searches remain incomplete; PR #70 is the recorded prior-art check.
- The exact PR draft and user approval remain outstanding.

## Next-Action

Summary: Obtain GPT-6.1 review
Action: Obtain the required independent GPT-6.1 Sol/xhigh review of the current source diff.
Done-When: Record the review outcome and resolve every remaining publication blocker.

## Pull-Request-Implementation

Branch: fix/transform-smart-filter-masks
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Transform smart-filter masks during mask-aware canvas geometry and keep disabled masks aligned.
Commit: `92df7cb2698c633e1d8cfdc34dca9587cfdeecd4` and `b91eafe95455ed523addd6d82b678981531c79ac`
Push: `origin/fix/transform-smart-filter-masks`
Checks:
- `cargo test --quiet -p photocraft-engine` → 596 passed, 9 ignored.
- `cargo test --quiet -p photocraft-engine smart_filter_mask_before_refresh` → 2 passed.
- `cargo clippy --quiet -p photocraft-engine --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → 27 crates, no violations.
- `cargo xtask wasm` → all 21 package checks passed; existing unused-code warnings remain.
- `cargo fmt --all` → passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
