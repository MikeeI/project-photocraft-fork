# ISSUE-033 — flat export: omitted channels lack document-level loss warnings

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

[S] Flat conversion discards saved extra channels and Quick Mask before codec warning generation can see them.
Review mapping: `E20`, CHANGED; severity Medium.
Document-level warnings must cover mode-specific early returns as well as the general flat conversion path.

## Reach-and-Impact

Trigger: export a one-raster document with a saved selection or Quick Mask to a flat format.
[S] Reopening cannot recover those omitted channels, while the export warning list does not describe that loss.
No export/reopen experiment was executed.

## Evidence

- [S] `crates/io/src/lib.rs:85-91` promises warnings about approximated or dropped data.
- [S] `crates/io/src/flat.rs:125-151,204-210` carries only surface samples and selected metadata.
- [S] `crates/io/src/flat.rs:254-259` can return mode-specific output before `document_to_image` runs.
- [S] `crates/io/src/flat.rs:267-274` obtains codec warnings from the already reduced image representation.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-034` concerns PSD channel-budget exhaustion, not flat conversion's missing document-level warnings.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending warning verification at both flat-export branch families.

## Proposed-Change

Determine omitted extra-channel and Quick Mask warnings before dispatching to flat-export branches.
Merge those warnings into every affected result, including mode-specific early returns.

## Scope-and-Constraints

- Report actual loss without claiming that every flat codec must gain editable document-channel support.
- Do not place the entire correction only in `document_to_image`.
- Preserve existing fatal codec errors, metadata, color conversion, and other fidelity warnings.

## Verification

Status: source-traced; no warning or reopen comparison executed.
- Export a saved-selection document through the general path and verify the specific channel-loss warning.
- Exercise a mode-specific path and verify the same document-level omission is also reported.

## Publication-Blockers

- General and mode-specific export warning behavior needs focused verification.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Inspect flat channel-loss warnings
Action: Compare warnings and reopened channels for general and mode-specific flat exports with saved extra channels.
Done-When: Record export branches, input channels, warning lists, and the data absent after reopening.
