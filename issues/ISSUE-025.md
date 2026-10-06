# ISSUE-025 — smart objects: explicit discard still commits child edits

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`

## Root-Cause

[S] The prompt advances after Don't Save without carrying that choice, so replay closes every dirty linked child through default save-back.
Review mapping: `E12`, VALID; severity Medium.

## Reach-and-Impact

Trigger: edit Smart-Object contents, close the child, and choose `Don't Save`.
[S] Save-back changes the parent before any later normal Save, which can persist rejected edits.
Parent undo can limit the consequence; no irreversible data loss was induced.

## Evidence

- [S] `crates/ui-egui/src/discard_ui.rs:100-111` replayed only the target document and lost the discard decision.
- [S] `crates/engine/src/commands.rs:262-265` used `Session::close` without a discard policy.
- [S] `crates/engine/src/smart_cmds.rs:657-664` committed every dirty linked child before removal.

## Prior-Art

Local ledger checked 2026-10-06; no matching root cause.
Issue searches for `"Don't Save" "smart object"` and `smart object discard close` returned no matches.
PR search `"Don't Save" "Edit Contents"` and commit search `smart object discard child close` found no matches.
Gaps: Upstream discussions and release notes were not searched.
`ISSUE-021` covers failed save-back; this finding covers successful but unwanted save-back.
Contribution fit: the correction is implemented, but its current branch is stacked on `ISSUE-021`.

## Proposed-Change

Carry the explicit discard decision into the close boundary and prevent automatic save-back on that path.
Preserve separately documented save-back behavior for callers that did not request discard.

## Scope-and-Constraints

- Preserve parent undo history and the existing Save and Cancel paths.
- Keep save-back as the default for close callers without an explicit discard decision.
- Coordinate the close policy with `ISSUE-021` and stale close prompts in `ISSUE-020`.
- Keep failed save-back and explicit discard as separate root causes.

## API-and-Compatibility

`file.close`, `file.closeAll`, and `file.closeOthers` accept optional `discardedDocuments` stable document IDs.
`Session::close` keeps the commit default; `Session::close_with_policy` exposes explicit commit or discard behavior.
The UI passes only IDs answered Don't Save; Save and Cancel remain on their existing paths.

## Verification

Status: pre-fix engine reproduction and corrected native UI behavior verified on 2026-10-06.
- [O] Pre-fix `cargo test --quiet -p photocraft-engine explicitly_discarded_smart_child_does_not_update_parent` failed at `71e724e32fc1e882a718b20264f37fa1d87b592a`: revision `4 -> 5` (`left: 5`, `right: 4`).
- [O] Actual `File > Close Others` and a physical `Don't Save` click closed the child while retaining the clean parent.
- [O] Parent revision stayed `4`; pixel `(12, 12)` stayed `[1.0, 0.0, 0.0, 1.0]`; the child and third tab were removed.
- [O] Inspected prompt: `/tmp/photocraft-issue025-live/io/issue025-dont-save-prompt.png`.
- [O] Inspected result: `/tmp/photocraft-issue025-live/io/issue025-after-discard.png`.

## Publication-Blockers

- The branch is stacked on `ISSUE-021` commit `71e724e32fc1e882a718b20264f37fa1d87b592a`; a standalone upstream PR needs that dependency resolved.
- Required independent GPT-6.1 Sol/xhigh review is unavailable in this session; do not substitute another model.
- Upstream discussions and release notes remain unsearched.
- The exact PR draft and user approval remain outstanding.

## Next-Action

Summary: Obtain GPT-6.1 review
Action: Obtain independent GPT-6.1 Sol/xhigh review of the stacked source diff.
Done-When: Record its outcome and resolve blockers affecting a standalone upstream branch.

## Pull-Request-Implementation

Branch: fix/discard-smart-child-edits
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`, stacked on `fix/preserve-failed-smart-child@71e724e32fc1e882a718b20264f37fa1d87b592a`
Scope: Carry explicit discard IDs through single and bulk close while preserving default save-back, Save, and Cancel behavior.
Commit: `7d0a23b4095f9c0251e8884e796c15ec42ac08f8`
Push: `origin/fix/discard-smart-child-edits`
Checks:
- `cargo test --quiet -p photocraft-engine -p photocraft-ui-egui` → 1076 passed, 12 ignored.
- `cargo test --quiet -p photocraft-ui-egui` → 479 passed, 3 ignored.
- `cargo clippy --quiet -p photocraft-engine -p photocraft-ui-egui --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → 27 crates, no violations.
- `cargo xtask wasm` → all 21 package checks passed; unrelated unused-code warnings remain.
- `cargo fmt --all` → passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
