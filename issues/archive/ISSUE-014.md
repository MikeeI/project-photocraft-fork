# ISSUE-014 — file save: destructive overwrite before successful publication

State: Archived
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/230
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Reliability
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] At the recorded source revision, desktop, engine, and automation writers truncated existing destinations before replacement bytes were fully written.
Review mapping: `E1`, VALID; severity Critical.

## Reach-and-Impact

Trigger: overwrite an existing document when a write fails after truncation.
[O] Controlled Linux failures destroyed valid PSDs through both desktop Save and CLI conversion after each caller reported an error.
Production frequency remains unknown.

## Evidence

- [S] `apps/photocraft/src/services.rs` used `std::fs::write` for desktop document publication.
- [S] `crates/engine/src/file_cmds.rs` used `std::fs::write` for engine document publication.
- [S] `crates/automation/src/files.rs::save` used `std::fs::write` for shared CLI and MCP exports.
- [O] The desktop and CLI overwrite failures are recorded in Bug-Reproduction.
- [S] Merged upstream PR #230 routes these callers through shared atomic publication, including CLI and MCP paths.

## Bug-Reproduction

Environment: Linux; baseline source `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`; disposable files under `/tmp/issue-014-repro`.
Desktop: opened a 14,281,130-byte 2400×1500 RGB PSD with SHA-256 `36a112efb0bf35a87cfec59c1ceb952c7d25fc6ee6ca491954af11df1bfd3144` in the Xvfb app and painted a brush stroke.
`ui.inspect` showed dirty revision 2; `prlimit --pid 59650 --fsize=1024:1024` plus Ctrl+S made the write fail with `File too large (os error 27)`.
The UI remained dirty, but the prior PSD became 1,024 bytes and `photocraft-cli info` could no longer parse it.
CLI baseline: `/root/projects/project-photocraft-fork/target/issue-014/debug/photocraft-cli convert docs/images/photocraft-demo.jpg /tmp/issue-014-repro/engine-target.psd` produced a valid 14,281,130-byte PSD with SHA-256 `36a112efb0bf35a87cfec59c1ceb952c7d25fc6ee6ca491954af11df1bfd3144`.
The unchanged `crates/automation/src/files.rs::save` used `std::fs::write`; `trap '' XFSZ && ulimit -f 1 && /root/projects/project-photocraft-fork/target/issue-014/debug/photocraft-cli convert docs/images/photocraft-demo.jpg /tmp/issue-014-repro/engine-target.psd` exited 1 with `File too large (os error 27)`.
The destination changed from SHA-256 `36a112efb0bf35a87cfec59c1ceb952c7d25fc6ee6ca491954af11df1bfd3144` to `8fa812f7db98f19c4a3e795f18219e5218900e9483834f796f9d575274c10a86` and shrank to 512 bytes.
`/root/projects/project-photocraft-fork/target/issue-014/debug/photocraft-cli info /tmp/issue-014-repro/engine-target.psd` failed with unexpected EOF.

## Prior-Art

Coverage: inspected upstream issue #203, merged PR #230, and current `main` service/save files on 2026-10-05.
Upstream issue #203 reports the same destructive overwrite across desktop, engine, and automation writers and is closed as completed: https://github.com/storytold/photocraft/issues/203.
Merged PR #230 introduces shared `photocraft_format::atomic_write` and routes desktop, engine, automation `files::save`, CLI conversion, MCP, and capability-scoped writes through it: https://github.com/storytold/photocraft/pull/230.
This finding is a duplicate of that completed upstream correction and must not produce a second PR.
`ISSUE-019` remains distinct; this archive does not claim to resolve its directory-bundle garbage-collection lifecycle.

## Proposed-Change

Superseded by upstream PR #230, which centralizes document publication through a shared atomic writer.

## Scope-and-Constraints

- Do not submit a duplicate implementation of the root cause fixed by PR #230.
- Keep `ISSUE-019`'s directory-bundle garbage-collection behavior open and separate.
- Do not infer stronger durability guarantees than the upstream implementation documents.

## Verification

Status: fixed upstream; the merged PR and current `main` sources directly cover the reproduced writers.
- [O] Baseline desktop and CLI failures are reproduced above with before/after sizes, hashes, readability, dirty state, and write errors.
- [S] PR #230 reports passing formatting, Clippy, engine/automation/CLI tests, layering, wasm, and panic-hunt gates.
- [S] Current upstream desktop services and automation file saves call `photocraft_format::atomic_write`.
The upstream gates were not rerun independently for this archived duplicate.

## Publication-Blockers

None.

## Next-Action

Summary: —
Action: None.
Done-When: None.

## Archive

Archive-Reason: Fixed-Elsewhere
Detail: None.
Evidence: https://github.com/storytold/photocraft/issues/203; https://github.com/storytold/photocraft/pull/230
Checked: 2026-10-05
