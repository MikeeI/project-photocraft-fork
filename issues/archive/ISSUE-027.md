# ISSUE-027 — history: unchanged bit depth clears redo

State: Archived
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/133
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] At the recorded source commit, `convert_depth` detected an equal depth inside `Session::edit`.
That successful no-op still recorded a history step and cleared the redo branch.
Review mapping: `E14`, VALID; severity Medium.
The correction is limited to this explicitly unchanged-depth command and does not establish a universal no-op history rule.

## Reach-and-Impact

Trigger: undo an edit, then execute the command for the depth that is already active.
[S] The baseline command returned successfully without conversion but recorded a history step.
[O] The existing regression reproduced the revision change and lost redo before the fix.
[O] After the fix, the regression confirms unchanged revision and successful redo.

## Evidence

- [S] Baseline `crates/engine/src/image_cmds.rs:317-319` checked equality inside `s.edit`.
- [S] `crates/engine/src/lib.rs:335-358` records each successful edit closure.
- [S] `crates/ops/src/lib.rs:49-52` clears redo when a step is recorded.
- [O] Existing `image_cmds::tests::mode_and_depth_conversions` failed before the fix on its revision assertion.
- [S] Current `crates/engine/src/image_cmds.rs:316-321` returns before editing when the current depth matches.
- [O] The updated existing test confirms revision stability and redo, while earlier real conversions remain undoable.

## Prior-Art

Coverage: original targeted history searches, current PR #133 thread/diff, and main source checked on 2026-10-06.
PR #133 by @MikeeI merged on 2026-10-05; it preserves redo only for this unchanged-depth command.
The broader no-op-history policy remains unestablished.
Contribution fit: the fix claims only unchanged-depth history preservation.

## Proposed-Change

Detect identical depth before entering `Session::edit`, leaving document revision and redo unchanged.

## Scope-and-Constraints

- Limit the correction to the explicitly recognized unchanged-depth case.
- Do not impose generic deep-document equality checks on every edit.
- Preserve real depth conversion and normal undo/redo behavior.

## Verification

Status: reproduced before the fix and verified after it.
- Existing `image_cmds::tests::mode_and_depth_conversions` reproduced revision change before the fix.
- The same test now verifies unchanged revision and preservation of the undone conversion's redo.
- `cargo test -p photocraft-engine`: 513 passed across three suites; five ignored.
- `cargo test -p photocraft-engine --test panic_hunt -- --ignored`: one passed.
- Engine Clippy, formatting, dependency layering, and all 20 WebAssembly package checks passed.
- WebAssembly emitted existing `photocraft-cms` dead-code warnings for `PAR_MIN_PIXELS` and `PAR_CHUNK_PIXELS`.
- Independent xhigh review approved the patch and found no source blocker.
[S] Current `upstream/main@47f4abfed49e0d2f5b9277287b27dee632530ba4` returns from `convert_depth` before opening `Session::edit` when depth is unchanged (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/engine/src/image_cmds.rs).

Test decision: update — existing `image_cmds::tests::mode_and_depth_conversions` covers this no-op redo contract.

## Publication-Blockers

None known.

## Next-Action

Summary: —
Action: None.
Done-When: None.

## Pull-Request-Implementation

Branch: `fix/preserve-noop-depth-history`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Preserve revision and redo when a requested bit depth already matches the document.
Commit: `5c8b7703b54f4a68bf46c027f79d422dec8766a7`
Push: Pushed to `origin/fix/preserve-noop-depth-history`.
Checks: Engine tests, panic hunt, Clippy, formatting, layering, and all WebAssembly package checks passed.
Review: Independent GPT-6.1 Sol review at xhigh approved; no source correction requested.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/preserve-noop-depth-history`
Title: Selecting current bit depth must preserve redo

### Problem

Selecting the already-current bit depth still entered `Session::edit`.
Although no pixels changed, history recorded a step and cleared the user's redo branch.

### Change

Return before opening an edit transaction when the active document already has the requested depth.
Real depth conversions continue through the existing undoable path.

### Verification

- The existing engine regression reproduced the revision change before the fix.
- The updated test verifies revision stability and successful redo after selecting the current depth.
- Engine tests, the ignored panic hunt, Clippy, formatting, layering, and all 20 WebAssembly package checks passed.
- Independent xhigh review approved the source patch.

The fix covers only unchanged bit-depth commands; it makes no claim about other no-op commands.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Archive

Archive-Reason: Merged
Detail: None.
Evidence: https://github.com/storytold/photocraft/pull/133
Checked: 2026-10-06
