# ISSUE-009 — compose: unnecessary backdrop snapshots

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Performance
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Group and adjustment composition copies original backdrop buffers beyond their actual read dependencies.
Review mapping: `P9`, VALID; severity Medium.

## Reach-and-Impact

[S] CPU tile composition snapshots pass-through groups even when neither mixing nor visible clipped layers need it.
[S] Ordinary adjustment composition creates two full clones although only the adjusted result needs separate storage.
Measurement: memory-bandwidth pressure and live-preview latency remain unmeasured.

## Evidence

- [S] `crates/compose/src/lib.rs:836-852` clones before determining whether mixing or clipping uses the snapshot.
- [S] `crates/compose/src/lib.rs:880-898` clones twice and only mutates the adjusted buffer before final assignment.
- [S] `crates/compose/src/lib.rs:895-898` reads each original pixel before overwriting that same backdrop index.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching record.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: potentially local buffer-ownership correction; measurements and upstream ownership search remain open.

## Proposed-Change

Create pass-through snapshots only when mixing or visible clipped layers require the original backdrop.
Clone adjustment output directly from the backdrop and read each original backdrop pixel before its assignment.

## Scope-and-Constraints

- Preserve snapshots required by clipping, channel restrictions, and Blend If.
- Preserve masks, opacity, blend ordering, quantization, and alpha behavior.
- Do not replace required independent mutable buffers with aliased storage.

## Performance-Evidence

[S] Each unnecessary N-pixel float-RGBA snapshot allocates a Vec and copies 16N pixel bytes.
[S] Unlike Surface clones, these Buffer clones copy their pixel vectors.
Measurement: no allocator, bandwidth, or end-to-end timing comparison has run.

## Verification

- Compare copied bytes, allocations, and release preview latency for groups and adjustments.
- Compare output with clipping, masks, opacity, channel restrictions, and Blend If.

## Publication-Blockers

- Representative copy counts, timing, and output-equivalence evidence are missing.
- Upstream prior art and the exact PR draft remain pending; publication is authorized conditional on verification.

## Next-Action

Summary: Measure redundant backdrop copies
Action: Capture buffer-copy volume during pass-through-group and adjustment composition.
Done-When: Record the layer conditions, rendered area, allocation and copy counts, command, and latency.
