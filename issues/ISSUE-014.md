# ISSUE-014 — file save: destructive overwrite before successful publication

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Reliability
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Desktop and engine document writers truncate the final destination before the replacement is fully written.
Review mapping: `E1`, VALID; severity Critical.

## Reach-and-Impact

Trigger: overwrite an existing PSD or native bundle when a write fails after truncation.
[S] The last good file can become empty or incomplete despite the caller reporting an error and retaining dirty state.
Runtime frequency and actual data loss have not been observed.

## Evidence

- [S] `apps/photocraft/src/services.rs:125` implements the desktop writer with `std::fs::write`.
- [S] `crates/ui-egui/src/lib.rs:507-512` exports, writes, then updates the path and saved revision.
- [S] `crates/engine/src/file_cmds.rs:87-91,164-167` also publishes directly to the destination.
- [S] `crates/format/src/store.rs:423-430` offers atomic native publication, but these callers bypass it.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-019` concerns overlapping atomic writers, not direct destination truncation.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved until the real writer failure path is reproduced and upstream ownership is checked.

## Proposed-Change

Write encoded bytes completely to an exclusively owned temporary sibling, then atomically replace the destination.
Apply the correction to both desktop and engine document publication boundaries.

## Scope-and-Constraints

- Preserve export errors, warnings, save paths, and dirty-state updates only after successful publication.
- Clean up only the current operation's temporary file.
- Coordinate publication semantics with `ISSUE-019`; a shared deterministic temporary name is insufficient.
- Atomic replacement does not by itself establish power-loss durability.

## Verification

Status: source-traced; no failed-write experiment executed.
- Fail an overwrite after writing begins and compare the destination with the original bytes.
- Verify the original document remains readable and the edited document remains dirty.

## Publication-Blockers

- Controlled failed-write reproduction and platform-specific replacement behavior remain unverified.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce failed document overwrite
Action: Exercise an existing-file save on a disposable failure-injected filesystem through the actual document writer.
Done-When: Record the command, revision, write failure, before/after bytes, readability, and dirty state.
