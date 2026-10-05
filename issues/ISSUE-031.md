# ISSUE-031 — PSD import: white-unmatting clips supported HDR samples

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

[S] The inverse white-matte operation clamps every recovered sample to `[0,1]`, including supported float data.
Review mapping: `E18`, VALID; severity Medium.

## Reach-and-Impact

Trigger: import a flattened transparent 32-bit RGB PSD containing HDR samples.
[S] With matted red `1.5`, alpha `0.5`, and white `1`, the inverse should produce `2.0` but returns `1.0`.
[S] Negative recovered float values are likewise clipped by this operation.
No PSD import reproduction was executed.

## Evidence

- [S] `crates/io/src/pixels.rs:134-143` defines matte/inverse behavior and applies universal clamping.
- [S] `crates/io/src/psd_import.rs:386-389,498-513` uses this inverse on supported float merged-image import.
- [S] `crates/color/src/lib.rs:95-125` otherwise preserves stored F32 range while quantizing integer samples.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a float PSD import sample and range comparison.

## Proposed-Change

Preserve float range in the inverse matte calculation and leave integer clamping to integer quantization.

## Scope-and-Constraints

- Preserve the defined zero-alpha behavior and model-specific white value.
- Do not silently convert float documents to an SDR representation.
- Verify the accepted transparent PSD representation rather than substituting an unrelated compositor example.

## Verification

Status: source-traced numeric counterexample; no PSD runtime import executed.
- Import a flattened F32 PSD with matted red `1.5` and alpha `0.5` and inspect the recovered sample `2.0`.

## Publication-Blockers

- A supported transparent float PSD fixture and importer output need verification.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce HDR unmatte clipping
Action: Import a disposable transparent float PSD with a known white-matted HDR sample.
Done-When: Record encoded samples, alpha, imported sample type, and expected versus actual straight color values.
