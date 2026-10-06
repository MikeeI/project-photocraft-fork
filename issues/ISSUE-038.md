# ISSUE-038 — file picker: read failures collapse into cancellation

State: Implementing
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: UI
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@eaf92dcd9df738117991502de1dd16137dbcd3b2`

## Root-Cause

[S] The native Open picker converts a selected file's read error into the same absence result as user cancellation.
Review mapping: `E25`, VALID; severity Medium.

## Reach-and-Impact

Trigger: select a native file that disappears or cannot be read before `std::fs::read`.
[S] The upstream callback used absence for both user cancellation and read failure.
The UI consequently treated a selected-but-unreadable file as cancellation, not an open error.
Browser picks remain distinct: they arrive later through the inbox.

## Bug-Reproduction

[O] Ubuntu 24.04.5 / Rust 1.99.0: a temporary pre-fix UI test removed a selected path before read.
The original callback returned no file; `status_error` stayed false and `notices` stayed empty.
The test was synthetic, not an OS dialog; its one-off command was not retained.

## Evidence

- [S] Upstream `apps/photocraft/src/services.rs` (`Services::native`) used `std::fs::read(&path).ok()?` after selection.
- [S] `open_dialog_file` in `crates/ui-egui/src/lib.rs` ignores absence; `open_failed` in `file_open.rs` reports errors.
- [O] `picker_cancellation_stays_silent_and_read_failures_are_reported` verifies `Ok(None)` stays silent.
- [O] The same test reads a removed selected path through `read_picked_file` and observes an error status and notice.

## Prior-Art

Coverage: Local open records and upstream GitHub issues/PRs searched 2026-10-06; no exact match.
Terms: picker/read failure, `rfd FileDialog`, `pick_open`, and post-selection disappearance.

- [S] PR #63 added `open_failed` for path opens but retained native `.ok()?`; related reporter work, not a duplicate.
  https://github.com/storytold/photocraft/pull/63
- [S] PR #23 and issue #5 concern Open Recent, not selected-file read errors.
  https://github.com/storytold/photocraft/pull/23
  https://github.com/storytold/photocraft/issues/5
- [S] v0.2.0 release notes have no native picker read-failure fix.
  https://github.com/storytold/photocraft/releases/tag/v0.2.0
- [O] GitHub Discussions returned 404.
  https://github.com/storytold/photocraft/discussions
- [O] Public-index searches found no PhotoCraft result for linked Discord; server content was not directly searchable.
  https://discord.gg/artcraft
- [S] Local ISSUE-016 concerns script-event save-path retargeting, a different root cause.

Contribution fit: distinct UI error-propagation defect; a bounded fix reuses the existing reporter.

## Proposed-Change

Return native picker read failures as errors while preserving `Ok(None)` for cancellation.
Route File Open errors through `open_failed`; keep browser inbox delivery asynchronous.

## Scope-and-Constraints

- Preserve silent genuine cancellation and existing import-error reporting.
- Keep native picker behavior distinct from browser inbox delivery.
- Do not replace concrete read errors with a generic success or cancellation value.

## API-and-Compatibility

Public `PickOpenFn`: `Box<dyn FnMut() -> Result<Option<(String, Vec<u8>)>, String>>`.
All in-repository providers and consumers were migrated.
Cancellation remains `Ok(None)`; web inbox delivery remains asynchronous.
Downstream `Services` providers face a source-level API change; persistence and control protocol are unchanged.

## Verification

Status: behavior and UI surface verified on 2026-10-06.

- Rebased onto current `upstream/main@eaf92dcd9df738117991502de1dd16137dbcd3b2`; upstream had no changes in the eight touched paths.
- Cancellation remains silent; a removed selected path produces a path-qualified error, status error, and notice.
- An inspected offscreen snapshot rendered “Couldn't open unreadable.psd: Permission denied (os error 13)”.
- No real native OS picker was driven; the regression used a selected path removed before reading.
- Dependency-inclusive strict Clippy hit warnings in unchanged `photocraft-cms` and `photocraft-engine`; package-only `--no-deps` strict checks passed.

## Publication-Blockers

- Required independent GPT-6.1 Sol/xhigh review is unavailable in this runtime; GPT-6 Luna medium is not a substitute.
- The exact upstream PR draft is not finalized, and no pull request has been opened.
- Do not publish until the exact current target and full draft are shown and the user approves them.

## Next-Action

Summary: Obtain GPT-6.1 review
Action: Obtain the required independent GPT-6.1 Sol review at xhigh of the pushed commit and regression evidence.
Done-When: Record verified model/effort identity and actionable review; otherwise retain the publication blocker.

## Pull-Request-Implementation

Branch: fix/report-picker-read-errors
Base: `upstream/main@eaf92dcd9df738117991502de1dd16137dbcd3b2`
Scope: Distinguish native picker read failures from cancellation and propagate them to visible error reporting.
Commit: `69cdfbbccaaf7a70de759d6d422897013904ec4d`
Push: `origin/fix/report-picker-read-errors` (explicit `--force-with-lease` update)
Checks:
- `cargo test --quiet --locked -p photocraft` → 36 passed.
- `cargo test --quiet --locked -p photocraft-ui-egui` → 477 passed; 3 ignored.
- Strict package Clippy (`--no-deps`, `-D warnings`) passed for `photocraft`, `photocraft-ui-egui`, and wasm `photocraft-web`.
- `cargo check --target wasm32-unknown-unknown --quiet --locked -p photocraft-web` → passed.
- `cargo xtask layers` → 27 crates, no violations.
- `cargo xtask wasm` → all 21 wasm-compatible crates passed.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
