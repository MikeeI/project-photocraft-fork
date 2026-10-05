# ISSUE-034 — PSD export: full channel budget silently drops Quick Mask

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

[S] PSD channel-limit warnings count saved extra channels but omit a present Quick Mask that has no remaining slot.
Review mapping: `E21`, VALID; severity Medium.

## Reach-and-Impact

Trigger: export opaque RGB with 53 saved extra channels and an active Quick Mask.
[S] Saved channels exactly consume the supported budget, so their truncation warning does not fire.
[S] Quick Mask pixels and their identifying resource are omitted without a warning.
No capacity-bound export was executed.

## Evidence

- [S] `crates/io/src/psd_export.rs:614-618` sets the budget and warns only when `doc.channels` exceeds it.
- [S] `crates/io/src/psd_export.rs:621-627` silently returns no Quick Mask ID when no slot remains.
- [S] `crates/io/src/psd_export.rs:658-662` writes resource 1022 only when Quick Mask was included.
- [S] `crates/io/src/lib.rs:85-91` promises warnings for dropped data.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-033` owns flat-format warning loss; PSD uses a separate capacity-dependent branch.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a narrow channel-capacity export check.

## Proposed-Change

Emit an explicit Quick Mask loss warning when the mask is present but no channel slot remains.

## Scope-and-Constraints

- Preserve the supported PSD channel limit and existing saved-channel ordering.
- Do not silently displace another channel or invent an unapproved reservation policy.
- Keep the identifying resource consistent with the channel actually written.

## Verification

Status: source-traced; no channel-capacity experiment executed.
- Export opaque RGB with 53 extra channels and Quick Mask and inspect warnings, channel count, and resource 1022.

## Publication-Blockers

- Capacity-bound Quick Mask omission and warning behavior need verification.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce Quick Mask capacity loss
Action: Export a disposable opaque RGB document whose 53 saved channels leave no Quick Mask slot.
Done-When: Record input channels, Quick Mask state, warning list, output channels, and resource presence.
