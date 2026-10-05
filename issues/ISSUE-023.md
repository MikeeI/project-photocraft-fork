# ISSUE-023 — flat export: native shortcut bypasses active compositing properties

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/122
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Single-layer native export eligibility omits vector masks, Blend If, and effective channel exclusions.
Review mapping: `E10`, VALID; severity High.

## Reach-and-Impact

Trigger: export one otherwise eligible raster layer with a half-canvas vector mask as PNG or TIFF.
[S] The shortcut exports original raster bytes rather than the masked visible composite.
[S] Supported active Blend If and channel restrictions can be bypassed by the same eligibility omission.
[O] A direct PNG export of an enabled vector mask lacked alpha before the fix.

## Evidence

- [S] `crates/io/src/flat.rs:82-98` promises an unmasked layer but checks only the pixel mask and basic properties.
- [S] `crates/io/src/flat.rs:126-151` copies native surface samples without composition after eligibility succeeds.
- [S] `crates/compose/src/lib.rs:492-501,530-533` applies vector coverage in the reference compositor.
- [S] `crates/compose/src/lib.rs:667-679,708-710,773-784` defines and applies effective blending restrictions.

## Prior-Art

Coverage: upstream inventory covered all 102 issue/PR records, relevant discussions, and public releases; no exact root-cause duplicate was found.
GitHub Discussions are disabled; project Discord history was inaccessible.
`ISSUE-033` concerns omitted channel-loss warnings, not incorrect visible composite pixels.

## Proposed-Change

Reject the native shortcut when an enabled vector mask or an effective supported blending restriction changes output.
Reuse existing compositor predicates for channel restrictions and Blend If.

## Scope-and-Constraints

- Preserve native depth and color-model fidelity for genuinely eligible surfaces.
- Do not reject unsupported-mode metadata merely because it is nondefault if the compositor intentionally ignores it.
- Keep disabled vector masks and other visually inactive properties distinct from active restrictions.

## Verification

[O] The extended `single_layer_native_export_strips_opaque_alpha` regression failed before the fix.
The isolated post-fix test decodes PNG output and matches every pixel to the CPU composite with U16 alpha retained.
[O] Full IO tests passed: 195 across 18 suites; affected Clippy passed with warnings denied.
[O] Formatting, dependency layering, and all 20 WebAssembly packages passed.
Main's source review confirmed the shortcut now requires no active vector mask, effective channel restriction, or Blend If.

## Publication-Blockers

None.

## Next-Action

Summary: Await upstream flat export review
Action: Address review feedback on the submitted compositing eligibility fix.
Done-When: Upstream closes or merges the PR.

## Pull-Request-Implementation

Branch: fix/flat-export-composite-eligibility
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Exclude active masks and effective blending restrictions from direct native-surface flat export.
Commit: `bf29b83c54820bf00b76674f1197ea1ad20c74c5`
Push: `origin/fix/flat-export-composite-eligibility`.
Checks:
- Baseline enabled-vector-mask PNG export: alpha missing.
- Isolated post-fix test: decoded U16 PNG pixels match the CPU composite.
- Full IO suite: 195 passed across 18 suites.
- Affected Clippy with warnings denied, formatting, layering, and all 20 WebAssembly packages: passed.
Main source review: approved the predicate boundaries.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/flat-export-composite-eligibility`
Title: Preserve compositing properties in single-layer flat export

### Problem

The single-layer native export shortcut can bypass active compositing properties.
An enabled vector mask is ignored, so a PNG export can lack the visible alpha mask.
Effective Blend If and channel restrictions also must not take this shortcut.

### Change

Reject the shortcut for an enabled vector mask and reuse existing compositor predicates for effective channel restrictions and Blend If.
Leave the direct native path available when these properties do not alter the visible result.

### Verification

- The existing `single_layer_native_export_strips_opaque_alpha` regression failed before the fix for an enabled vector mask.
- The isolated post-fix test decodes PNG output and compares every pixel with the CPU composite, including U16 alpha.
- Full IO tests passed: 195 tests across 18 suites.
- Affected Clippy with warnings denied, formatting, dependency layering, and all 20 WebAssembly packages passed.
- Main reviewed the predicate boundaries against the compositing implementations.

Runtime coverage in this regression is for the vector mask; Blend If and channel exclusion boundaries were reviewed in source.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
