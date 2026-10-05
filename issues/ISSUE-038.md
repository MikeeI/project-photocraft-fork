# ISSUE-038 — file picker: read failures collapse into cancellation

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: UI
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] The native Open picker converts a selected file's read error into the same absence result as user cancellation.
Review mapping: `E25`, VALID; severity Medium.

## Reach-and-Impact

Trigger: select a file that is unreadable or disappears before the picker reads it.
[S] The UI receives no file and therefore shows neither the promised open error nor a notice.
No native-dialog read-failure experiment was executed.

## Evidence

- [S] `crates/ui-egui/src/file_open.rs:1-5` promises visible status and notice reporting for failed opens.
- [S] `apps/photocraft/src/services.rs:109-112` uses `std::fs::read(&path).ok()?` after selection.
- [S] `crates/ui-egui/src/lib.rs:483-488` only reports errors when selected bytes were returned.
- [S] `crates/ui-egui/src/menus.rs:212-218` reports successful command completion after the silent path.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a native-dialog failure reproduction and service-contract review.

## Proposed-Change

Distinguish cancellation from read failure with a fallible picker contract or separate path selection and reading.
Pass actual read failures to the existing open-error reporting owner.

## Scope-and-Constraints

- Preserve silent genuine cancellation and existing import-error reporting.
- Keep native picker behavior distinct from browser inbox delivery.
- Do not replace concrete read errors with a generic success or cancellation value.

## API-and-Compatibility

Picker providers and consumers must migrate together if their return type changes to `Result<Option<_>, _>`.
Existing cancellation behavior remains an intentional non-error result.

## Verification

Status: source-traced; no native-dialog experiment executed.
- Select an unreadable or removed disposable file and require a concrete status/notice error.
- Cancel the picker normally and verify no error is shown.

## Publication-Blockers

- Native-dialog failure and genuine-cancellation behavior need verification.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce silent picker read failure
Action: Exercise native File Open with a selected file that cannot be read and compare genuine cancellation.
Done-When: Record selection outcome, read error, command result, status, and visible notice behavior.
