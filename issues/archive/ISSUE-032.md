# ISSUE-032 — PSD export: eight-bit opacity detection drops U16 alpha

State: Archived
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/111
Contribution-Priority: Low
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Merged PSD opacity detection uses `q255` for U16 samples and can remove their non-opaque alpha plane.
Review mapping: `E19`, CHANGED; severity Low, reduced from Medium.
The supported claim is bounded precision loss, not demonstrated ordinary visual visibility.

## Reach-and-Impact

Trigger: all canvas pixels have alpha near one, such as `65500/65535`, in a U16 document.
[S] Eight-bit rounding reports opacity and export removes the merged alpha plane.
[S] Layer alpha remains separate, so merged-image consumers can disagree with the layer stack.
[O] Public export produced three channels before the fix and preserves four channels with decoded alpha `65500` afterward.
No ordinary visual artifact is claimed.

## Evidence

- [S] `crates/io/src/psd_export.rs:60-62,505` uses the eight-bit quantizer for non-F32 opacity detection.
- [S] `crates/io/src/psd_export.rs:605-611` re-renders without matte and removes alpha after that decision.
- [S] `crates/compose/src/psblend.rs:94-117` preserves fractional alpha rather than quantizing it to eight bits.

## Prior-Art

Coverage: original 102-record inventory, current PR #111 thread/diff, and main source checked on 2026-10-06.
PR #71 preserves `q255` and covers U8 near-opacity, not U16 target precision (https://github.com/storytold/photocraft/pull/71).
PR #111 by @MikeeI merged on 2026-10-05 and preserves merged alpha at exported U16 precision.
GitHub Discussions are disabled; project Discord history was inaccessible.

## Proposed-Change

Determine opacity using the actual target sample precision for U8, U16, and F32.

## Scope-and-Constraints

- Preserve the existing eight-bit rounding behavior for genuine U8 output.
- Keep the no-matte rerender only when quantized target alpha is truly opaque.
- Do not claim a visible image defect from the small precision difference alone.

## Verification

[O] `cargo test -p photocraft-io --test composite merged_alpha_written_when_transparent -- --exact` passed in isolated target/issue-032.
The test decodes serialized PSD output and requires four merged channels and U16 alpha `65500`.
An earlier shared Cargo target returned contradictory stale results; only isolated post-fix evidence is accepted.
[O] Isolated full IO tests passed: 195 tests across 18 suites; affected Clippy passed with warnings denied.
[O] Layering passed for 26 crates; all 20 packages passed the WebAssembly gate.
Existing WebAssembly-only unused-constant warnings remain unchanged.
[S] Independent xhigh review approved the complete diff and precision-specific decision.
The review traced both opacity callers and the serialized regression; it did not run additional tests.
[S] Current main evaluates merged alpha at U8, U16, or F32 target precision (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/io/src/psd_export.rs#L539-L545).

## Publication-Blockers

None.

## Next-Action

Summary: —
Action: None.
Done-When: None.

## Pull-Request-Implementation

Branch: fix/preserve-u16-merged-alpha
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Determine merged PSD opacity at the actual exported sample precision.
Commit: `50e5ff9f7f1ecd831aeca0e34ead835529bbe7d3`
Push: `origin/fix/preserve-u16-merged-alpha`.
Checks:
- Baseline merged export: three channels, missing alpha.
- Isolated focused post-fix test: passed with alpha 65500.
- Full isolated IO suite: 195 passed across 18 suites.
- Affected Clippy with warnings denied, formatting, layering, and all 20 WebAssembly packages: passed.
- Independent xhigh source review: approved.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Title: Preserve merged PSD alpha at the exported sample precision

### Problem

Merged PSD export uses eight-bit rounding to decide whether the alpha plane is fully opaque.
For a U16 document, alpha `65500/65535` rounds to opaque at eight-bit precision, so export drops a representable alpha value.
Layer alpha remains separate; this affects the merged composite.
No ordinary visible artifact is claimed.

### Change

Decide opacity at the actual exported precision: retain U8 quantization, match U16 encoding, and compare F32 alpha with `1.0`.
Leave the existing no-matte rerender and channel-removal paths unchanged.

### Verification

- Extended the existing `merged_alpha_written_when_transparent` test through public byte export and PSD parsing.
- Before the fix, the regression observed three channels instead of four.
- After the fix, an isolated run verifies four channels, merged transparency, and decoded U16 alpha `65500`.
- The full IO suite passed: 195 tests across 18 suites.
- Affected Clippy with warnings denied, formatting, dependency layering, and all 20 WebAssembly packages passed.
- Independent source review found no blocker.

The fixture is synthetic; no Photoshop-produced corpus or visual comparison is claimed.
Existing WebAssembly unused-constant warnings are unchanged.

### AI assistance

This contribution was prepared with OpenAI Codex through the Oh My Pi/MOMP agent framework.
An independent `openai-codex/gpt-6.1-sol` reviewer at xhigh effort reviewed the bounded diff.
The implementation agent executed the reported checks; review approval is not additional runtime evidence.

## Archive

Archive-Reason: Merged
Detail: None.
Evidence: https://github.com/storytold/photocraft/pull/111
Checked: 2026-10-06
