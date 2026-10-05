# ISSUE-034 — PSD export: full channel budget silently drops Quick Mask

State: Submitted
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/115
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
[O] Full isolated IO tests passed: 195 tests across 18 suites; affected Clippy passed with warnings denied.
[O] Formatting, layering across 26 crates, and all 20 WebAssembly packages passed.
[S] Independent xhigh review approved the bounded diff and traced warning ownership and resource consistency.

## Publication-Blockers

None.

## Next-Action

Summary: Await upstream Quick Mask warning review
Action: Address review feedback on the submitted capacity-loss warning.
Done-When: Upstream closes or merges the PR.

## Pull-Request-Implementation

Branch: fix/warn-quick-mask-capacity
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Warn when the PSD channel budget excludes a present Quick Mask.
Commit: `b243ccb6a5f30cddcb81d2dce7d389af1d3205c9`
Push: `origin/fix/warn-quick-mask-capacity`.
Checks:
- Baseline capacity warning regression: failed with empty warnings.
- Focused post-fix capacity regression: passed.
- Isolated full IO tests: 195 passed across 18 suites.
- Affected Clippy with warnings denied, formatting, layering, and all 20 WebAssembly packages: passed.
- Independent xhigh source review: approved.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/warn-quick-mask-capacity`
Title: Warn when the PSD channel limit drops a Quick Mask

### Problem

PSD export silently omits a present Quick Mask when saved alpha channels already consume the channel budget.
At exact capacity, the existing saved-channel truncation warning does not fire, leaving this separate loss unreported.

### Change

Emit a Quick Mask-specific warning at the branch that omits it.
Keep the 56-channel limit, saved-channel ordering, and resource 1022 behavior unchanged.

### Verification

- The public export regression failed before the fix with an empty warning list.
- The isolated post-fix regression verifies the Quick Mask warning, 56 output channels, and no resource 1022 for the omitted mask.
- Existing below-capacity Quick Mask roundtrip coverage remains in the same test.
- The full IO suite passed: 195 tests across 18 suites.
- Affected Clippy with warnings denied, formatting, dependency layering, and all 20 WebAssembly packages passed.
- Independent source review found no blocker.

The regression fixture is synthetic U8 RGB and exercises exact capacity; no separate-channel content assertions are claimed.
The `document_to_psd` convenience wrapper still discards warnings; this fix verifies public `export` warnings.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.
