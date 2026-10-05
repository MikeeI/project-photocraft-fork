# ISSUE-031 — PSD import: white-unmatting clips supported HDR samples

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/106
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The inverse white-matte operation clamps every recovered sample to `[0,1]`, including supported float data.
Review mapping: `E18`, VALID; severity Medium.

## Reach-and-Impact

Trigger: import a flattened transparent 32-bit RGB PSD containing HDR samples.
[S] With matted red `1.5`, alpha `0.5`, and white `1`, the inverse should produce `2.0` but returns `1.0`.
[S] Negative recovered float values are likewise clipped by this operation.
[O] A serialized synthetic RGB32 PSD imported `[1,0,0,0.5]` before the fix and `[2,-0.5,0,0.5]` afterward.

## Evidence

- [S] `crates/io/src/pixels.rs:134-143` defines matte/inverse behavior and applies universal clamping.
- [S] `crates/io/src/psd_import.rs:386-389,498-513` uses this inverse on supported float merged-image import.
- [S] `crates/color/src/lib.rs:95-125` otherwise preserves stored F32 range while quantizing integer samples.
- [O] The fixture contains big-endian F32 planes `[1.5,0.25,0.5,0.5]` and uses public `import`.
- [S] Integer surface encoders retain their clipping; the SDR `merged_composite` caller now owns its previous clamp.
- [S] Upstream `7e7864afae8f779afff063a68edf208e30fe592c` leaves the affected PSD IO path unchanged.

## Prior-Art

Coverage: local ledger and all 102 upstream issue/PR records, public releases, and relevant PR discussions checked.
The banded import in https://github.com/storytold/photocraft/pull/71 still calls the clipping helper.
No duplicate or active implementation was found; a fresh upstream PR search for `unmatte` returned no matches.
GitHub Discussions are disabled; project Discord history was inaccessible.

## Proposed-Change

Preserve float range in the inverse matte calculation and leave integer clamping to integer quantization.

## Scope-and-Constraints

- Preserve the defined zero-alpha behavior and model-specific white value.
- Do not silently convert float documents to an SDR representation.
- Verify the accepted transparent PSD representation rather than substituting an unrelated compositor example.

## Verification

[O] `cargo test -p photocraft-io --test psd_structure merged_composite_unmattes_photoshop_white -- --exact` failed before and passed after the fix.
[O] The regression imports serialized PSD bytes and asserts both F32 depth and recovered pixel values.
[O] `cargo fmt -p photocraft-io` and `git diff --check` passed.
[O] Isolated `cargo test -p photocraft-io` and affected-crate Clippy passed.
[O] `cargo xtask layers` passed for 26 crates; `cargo xtask wasm` passed for all 20 selected packages.
The wasm check emitted existing unused-constant warnings in CMS and engine code.
An independent GPT-6.1 Sol review at runtime-verified xhigh effort found no source blocker.
The synthetic fixture is not a Photoshop-produced interoperability corpus.

## Publication-Blockers

None.

## Next-Action

Summary: Await upstream HDR import review
Action: Monitor the submitted PR for actionable maintainer feedback or CI failures.
Done-When: Feedback is addressed or the PR reaches a terminal upstream decision.

## Pull-Request-Implementation

Branch: fix/preserve-hdr-unmatte
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Preserve supported F32 sample range while undoing the PSD merged-image white matte.
Commit: 739e26337e5461374f82aa7c5bb7f482c31119a2
Push: origin/fix/preserve-hdr-unmatte
Checks:
- Serialized F32 PSD import regression: failed before, passed after.
- Independent xhigh review and Git diff checks: passed.
- Affected-crate tests, Clippy, layering, and WebAssembly: passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/preserve-hdr-unmatte`
Title: Preserve HDR samples when importing white-matted PSD composites

### Problem

Importing a flattened transparent RGB32 PSD clips recovered HDR and negative samples to `[0,1]`.
The shared `unmatte` helper clamps its inverse even when the destination surface stores unrestricted F32 samples.

A serialized one-pixel fixture stores merged RGB `[1.5,0.25,0.5]` with alpha `0.5`.
Under the existing white-matte convention, its straight RGBA is `[2,-0.5,0,0.5]`.
Before this change, public `import` instead returns `[1,0,0,0.5]`.

### Changes

- Remove normalized-range clipping from the inverse-matte arithmetic.
- Leave U8/U16 clipping to the existing integer surface encoders.
- Keep the existing SDR `merged_composite` preview output bounded at its own call site.
- Extend the existing import regression with serialized RGB32 PSD bytes and assert both sample depth and recovered values.

The zero-alpha behavior and model-specific white values are unchanged.
This is a focused follow-up to the merged-image import path touched by #71, not a change to its banding.

### Verification

- The serialized F32 import regression failed before the fix and passed afterward.
- `cargo test -p photocraft-io`: passed.
- `cargo clippy -p photocraft-io --all-targets -- -D warnings`: passed.
- `cargo fmt -p photocraft-io` and `git diff --check`: passed.

The new fixture is synthetic, not a Photoshop-produced interoperability sample.
This PR does not make the separate eight-bit `merged_composite` preview API HDR-preserving.
No external PSD corpus coverage is claimed.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
