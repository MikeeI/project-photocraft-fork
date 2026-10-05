# ISSUE-034 — PSD export: full channel budget silently drops Quick Mask

State: Implementing
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
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
[O] A capacity-bound public PSD export omitted Quick Mask without a warning before the fix.

## Evidence

- [S] `crates/io/src/psd_export.rs:614-618` sets the budget and warns only when `doc.channels` exceeds it.
- [S] `crates/io/src/psd_export.rs:621-627` silently returns no Quick Mask ID when no slot remains.
- [S] `crates/io/src/psd_export.rs:658-662` writes resource 1022 only when Quick Mask was included.
- [S] `crates/io/src/lib.rs:85-91` promises warnings for dropped data.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-033` owns flat-format warning loss; PSD uses a separate capacity-dependent branch.
Upstream inventory covered all 102 issue/PR records, relevant discussions, and public releases.
https://github.com/storytold/photocraft/pull/63 surfaces warnings but does not generate this missing capacity warning.
No duplicate or active implementation was found.
GitHub Discussions are disabled; project Discord history was inaccessible.

## Proposed-Change

Emit an explicit Quick Mask loss warning when the mask is present but no channel slot remains.

## Scope-and-Constraints

- Preserve the supported PSD channel limit and existing saved-channel ordering.
- Do not silently displace another channel or invent an unapproved reservation policy.
- Keep the identifying resource consistent with the channel actually written.

## Verification

[O] The extended `channel_options_spot_and_quick_mask_roundtrip` failed before the fix with an empty warning list.
The focused post-fix run passed, asserting a Quick Mask warning, 56 output channels, and no resource 1022.
Isolated full IO, Clippy, layering, and WebAssembly verification is running in target/issue-034.
[S] Independent xhigh review approved the bounded diff and confirmed unchanged saved-channel priority and resource ownership.

## Publication-Blockers

- The isolated required verification chain remains pending.

## Next-Action

Summary: Verify Quick Mask loss warning
Action: Finish the active isolated validation chain, then publish the independently reviewed warning fix.
Done-When: Required gates pass and the scoped warning PR is published.

## Pull-Request-Implementation

Branch: fix/warn-quick-mask-capacity
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Warn when the PSD channel budget excludes a present Quick Mask.
Commit: `b243ccb6a5f30cddcb81d2dce7d389af1d3205c9`
Push: `origin/fix/warn-quick-mask-capacity`.
Checks:
- Baseline capacity warning regression: failed with empty warnings.
- Focused post-fix capacity regression: passed.
- Full isolated verification: running.
- Independent xhigh source review: approved.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
