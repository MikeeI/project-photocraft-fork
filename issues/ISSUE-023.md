# ISSUE-023 — flat export: native shortcut bypasses active compositing properties

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
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
No exported-image comparison was executed.

## Evidence

- [S] `crates/io/src/flat.rs:82-98` promises an unmasked layer but checks only the pixel mask and basic properties.
- [S] `crates/io/src/flat.rs:126-151` copies native surface samples without composition after eligibility succeeds.
- [S] `crates/compose/src/lib.rs:492-501,530-533` applies vector coverage in the reference compositor.
- [S] `crates/compose/src/lib.rs:667-679,708-710,773-784` defines and applies effective blending restrictions.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-033` concerns omitted channel-loss warnings, not incorrect visible composite pixels.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a compositor-to-export pixel comparison.

## Proposed-Change

Reject the native shortcut when an enabled vector mask or an effective supported blending restriction changes output.
Reuse existing compositor predicates for channel restrictions and Blend If.

## Scope-and-Constraints

- Preserve native depth and color-model fidelity for genuinely eligible surfaces.
- Do not reject unsupported-mode metadata merely because it is nondefault if the compositor intentionally ignores it.
- Keep disabled vector masks and other visually inactive properties distinct from active restrictions.

## Verification

Status: source-traced; no export comparison executed.
- Compare a PNG export of one half-canvas vector-masked raster with its reference composite.
- Review shortcut eligibility for active Blend If and effective channel exclusions.

## Publication-Blockers

- Visible export mismatch and corrected eligibility behavior need focused verification.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Compare masked flat export
Action: Export a one-layer vector-masked document and compare its decoded pixels with the CPU composite.
Done-When: Record layer properties, export format, expected mask coverage, and differing pixel coordinates.

## Pull-Request-Implementation

Branch: fix/flat-export-composite-eligibility
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Exclude active masks and effective blending restrictions from direct native-surface flat export.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
