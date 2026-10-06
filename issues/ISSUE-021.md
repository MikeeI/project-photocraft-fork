# ISSUE-021 — smart objects: failed save-back still removes the edited child

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Reliability
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`

## Root-Cause

[S] Smart-child close discards the save-back error, removes the link, and permits removal of the edited document.
Review mapping: `E8`, VALID; severity High.

## Reach-and-Impact

Trigger: edit a smart-object child, then delete or rasterize its original parent layer before closing the child.
[S] The promised parent update can fail while `file.close` still reports success and removes the only edited child.
[O] Controlled engine and desktop scenarios verified the corrected error path; no irreversible user data loss was induced.

## Evidence

- [S] At the recorded base, `smart_cmds::on_close` ignored `commit_child` errors before `Session::close` removed documents.
- [S] Single, batch, channel, and headless close callers ignored the old `Session::close` result.
- [O] The new engine regression failed before the fix because `file.close` returned success after the missing-parent-layer commit failed.
- [O] After the fix, live `file.close` returned `no such layer LayerId(6)` and kept the dirty linked child open.

## Prior-Art

GitHub issue searches on 2026-10-06 for `"smart object" close save` and `"Edit Contents" close` returned no matches.
The upstream pull-request search for `"smart object" close child` and commit search for `smart object close save-back` returned none.
`ISSUE-020` owns stale close-prompt selection; `ISSUE-025` owns explicit discard despite successful save-back.
Those causes are distinct from this finding's ignored required save-back error.
Gaps: Upstream discussions and release notes were not searched.
Contribution fit: Keep this failed-save-back root cause separate and stack ISSUE-025 on its public close API.

## Proposed-Change

Make the close precondition fallible and propagate save-back errors before removing the child or its link.
Migrate single and batch close callers to the resulting error contract.

## Scope-and-Constraints

- Preserve the dirty child's contents, link, and editability when its parent update fails.
- Preserve the existing clone-before-commit rollback of the parent document.
- Do not suppress errors in batch close or silently convert failures to successful removal.

## API-and-Compatibility

`Session::close` now returns `Result<Option<DocState>>`; callers must propagate save-back errors.
`ISSUE-025` must carry explicit discard policy through this failure-propagating close boundary.

## Verification

Status: engine and live UI failure paths verified on 2026-10-06.
- [O] Before the fix, `failed_smart_child_close_preserves_document_and_link` failed because `file.close` succeeded.
- [O] After the fix, that engine test verifies error propagation, unchanged child pixels and parent revision, and retained link.
- [O] `file.closeAll` returns the same save-back error without removing the failed child.
- [O] Live UI screenshot `/tmp/photocraft-issue021-live-ccy1IX/output/issue021-close-failure.png` shows the error and both open documents.
- [O] Live partial `file.closeOthers` failure preserved parent and child tabs and resynchronized two views; screenshot `/tmp/photocraft-issue021-live-ccy1IX/output/issue021-partial-close-error.png`.

## Publication-Blockers

- The independent GPT-6.1 Sol/xhigh review is unavailable in this session; do not substitute another model.
- The exact PR draft and user approval remain outstanding.
- `ISSUE-025` shares the close API boundary and must remain a separate, explicitly stacked contribution.

## Next-Action

Summary: Obtain GPT-6.1 review
Action: Obtain the required independent GPT-6.1 Sol/xhigh review of the source diff.
Done-When: Record the review outcome and resolve every required source correction.

## Pull-Request-Implementation

Branch: fix/preserve-failed-smart-child
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Propagate required smart-child save-back failures and preserve child contents and links.
Commit: `71e724e32fc1e882a718b20264f37fa1d87b592a`
Push: `origin/fix/preserve-failed-smart-child`
Checks:
- `cargo test --quiet -p photocraft-engine -p photocraft-automation -p photocraft-ui-egui` → 1121 passed, 12 ignored.
- Affected Clippy, formatting, and `cargo xtask layers` passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
