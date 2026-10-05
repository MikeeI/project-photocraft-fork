# ISSUE-016 — file open: script events retarget the imported save path

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] `open_file` assigns the path to the active document after open-event scripts can change that document.
Review mapping: `E3`, VALID; severity Critical.

## Reach-and-Impact

Trigger: an `openDocument` event executes `file.new` while opening an existing PSD.
[S] The new canvas receives the original PSD path; normal Save can overwrite that PSD with unrelated content.
No script-driven overwrite was executed.

## Evidence

- [S] `crates/ui-egui/src/file_open.rs:34-36` calls `open_bytes`, then changes `session.active_mut().path`.
- [S] `crates/ui-egui/src/lib.rs:418-424` admits the imported document before running `document_opened`.
- [S] `crates/engine/src/automate_cmds.rs:99-108,264-265` dispatches event commands without preserving the active tab.
- [S] `crates/engine/src/lib.rs:278-285` makes a newly added document active.
- [S] `crates/ui-egui/src/menus.rs:220-226` uses the active document's stored path for Save.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Atomic publication in `ISSUE-014` cannot correct a wrongly owned destination path.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending event-driven reproduction and upstream ownership search.

## Proposed-Change

Assign the original path to the imported document before dispatching open-event scripts.
Keep bare-byte imports pathless rather than treating a display name as a save location.

## Scope-and-Constraints

- Preserve script execution and its intentional active-document changes.
- Preserve brush and gradient imports, which do not create document tabs.
- Track imported identity explicitly if the path cannot be assigned before event dispatch.

## Verification

Status: source-traced; no UI/script experiment executed.
- Register an `openDocument` event with `file.new`, open a disposable PSD, and inspect both document paths.
- Saving the helper canvas must require its own destination and leave the original PSD unchanged.

## Publication-Blockers

- Event-driven path ownership and safe-save reproduction are missing.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce open-event path retargeting
Action: Open a disposable PSD with an open-event script that creates another document and inspect both paths.
Done-When: Record imported and active document IDs, path assignments, event commands, and Save destination selection.
