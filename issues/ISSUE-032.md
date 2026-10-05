# ISSUE-032 — PSD export: eight-bit opacity detection drops U16 alpha

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Low
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Merged PSD opacity detection uses `q255` for U16 samples and can remove their non-opaque alpha plane.
Review mapping: `E19`, CHANGED; severity Low, reduced from Medium.
The supported claim is bounded precision loss, not demonstrated ordinary visual visibility.

## Reach-and-Impact

Trigger: all canvas pixels have alpha near one, such as `65500/65535`, in a U16 document.
[S] Eight-bit rounding reports opacity and export removes the merged alpha plane.
[S] Layer alpha remains separate, so merged-image consumers can disagree with the layer stack.
No export/decode experiment or visible artifact was observed.

## Evidence

- [S] `crates/io/src/psd_export.rs:60-62,505` uses the eight-bit quantizer for non-F32 opacity detection.
- [S] `crates/io/src/psd_export.rs:605-611` re-renders without matte and removes alpha after that decision.
- [S] `crates/compose/src/psblend.rs:94-117` preserves fractional alpha rather than quantizing it to eight bits.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: low-priority precision correction pending a sample-level export comparison.

## Proposed-Change

Determine opacity using the actual target sample precision for U8, U16, and F32.

## Scope-and-Constraints

- Preserve the existing eight-bit rounding behavior for genuine U8 output.
- Keep the no-matte rerender only when quantized target alpha is truly opaque.
- Do not claim a visible image defect from the small precision difference alone.

## Verification

Status: source-traced; no U16 export/decode executed.
- Export U16 alpha `65500/65535` and verify a merged transparency plane retaining the value `65500`.

## Publication-Blockers

- Sample-level merged-alpha preservation evidence is missing.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Compare U16 merged alpha
Action: Export and decode a near-opaque U16 document and inspect the merged transparency plane.
Done-When: Record input alpha, output depth, channel presence, decoded alpha, and unchanged U8 expectations.

## Pull-Request-Implementation

Branch: fix/preserve-u16-merged-alpha
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Determine merged PSD opacity at the actual exported sample precision.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
