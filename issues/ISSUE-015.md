# ISSUE-015 — recovery: delete snapshots before durable replacement

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

[S] Launch recovery deletes every listed recovery entry regardless of load success or replacement persistence.
Review mapping: `E2`, VALID; severity Critical.

## Reach-and-Impact

Trigger: a transient recovery read failure, or another crash after recovery but before the next successful autosave.
[S] Failed recovery loses its retained input; successful recovery initially exists only in memory after deletion.
No crash or irreversible loss was induced; the restart scenario used only a disposable config directory.

## Evidence

- [S] `apps/photocraft/src/services.rs::recover` deleted each entry after attempting its load, including failed loads.
- [S] `crates/format/src/autosave.rs::Autosaver::discard` removes the recovery bundle and sidecar.
- [S] `crates/ui-egui/src/prefs_ui.rs::load` adopts only returned documents; the old service discarded their source first.
- [S] `crates/ui-egui/src/prefs_ui.rs::autosave` records a revision when the callback accepts a snapshot, not when it persists.

## Prior-Art

Issue searches on 2026-10-06 (`autosave recovery`, `recovery discard`, `"crash recovery"`) found no direct match.
PR searches for `autosave recovery`, `recovery load`, and `autosave key` found no matching fix.
- https://github.com/storytold/photocraft/pull/114 covers document-ID collisions, not recovery ownership.
- https://github.com/storytold/photocraft/pull/230 covers atomic replacement, not recovery-entry cleanup.
- https://github.com/storytold/photocraft/pull/267 covers portable paths, not recovery-entry cleanup.
No matching commit was found.
`ISSUE-017` owns asynchronous save acknowledgment and changes the same callback; its API adds document-instance identity.
Its callback is `(doc, document_instance_id, revision, path)`; this branch instead carries `(doc, revision, path, recovery_key)`.

## Proposed-Change

Keep recovery entries after failed loads and report their errors.
Retain successfully loaded entries until a confirmed replacement save or explicit discard owns their removal.
Track the original recovery entry independently from any newly assigned runtime document identity.

## Scope-and-Constraints

- Preserve recoverable input when loading, session admission, or replacement persistence fails.
- Do not interpret a queued autosave as a persisted replacement; coordinate with `ISSUE-017`.
- Remove only the recovery entry owned by the confirmed replacement or discard decision.

## Verification

Status: implementation and restart behavior verified on 2026-10-06.
- [O] A dirty 8×8 document persisted as `doc-1` and was recovered on two isolated restarts.
- [O] Both launches still reported `doc-901: manifest JSON: expected ident at line 1 column 2`.
- [O] Screenshot `/tmp/photocraft-issue015-visual-6f21e9b3/output/issue015-recovery-restart-one.png` shows the recovered tab and error notice.

## Publication-Blockers

- The independent GPT-6.1 Sol/xhigh review is unavailable in this session; do not substitute another model.
- The exact PR draft and user approval remain outstanding.
- Coordinate the overlapping autosave callback contract with `ISSUE-017` before choosing the contribution branch base.

## Next-Action

Summary: Obtain GPT-6.1 review
Action: Obtain the required independent GPT-6.1 Sol/xhigh review of the current source diff.
Done-When: Record the exact review outcome and close every resulting blocker.

## Pull-Request-Implementation

Branch: fix/retain-recovery-snapshots
Base: `upstream/main@a96a621deea97d4b1ecd173b8b921587e33f3ca5`
Scope: Retain recovery input until successful replacement or explicit discard owns deletion.
Commit: `0a665dd89dc3debed3b5aa46732fe34f842dc599`
Push: `origin/fix/retain-recovery-snapshots`
Checks:
- `cargo test --quiet -p photocraft-format` → 70 passed.
- `cargo test --quiet -p photocraft` → 36 passed.
- `cargo test --quiet -p photocraft-ui-egui` → 481 passed, 3 ignored.
- `cargo clippy --quiet -p photocraft-format -p photocraft-ui-egui -p photocraft --all-targets -- -D warnings` → passed.
- `cargo xtask layers` → 27 crates, no violations.
- `cargo xtask wasm` → all 21 package checks passed; baseline unused-code warnings remain.
- `cargo fmt --all -- --check && git diff --check` → passed.
The user authorized implementation and publication of a verified fix PR on 2026-10-05.
