# ISSUE-019 — native storage: shared temporary files break concurrent publication

State: Investigating
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: Not published.
Contribution-Priority: High
Root-Cause-Confidence: High
Finding-Category: Reliability
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Independent writers targeting the same native path share a deterministic, non-exclusively created temporary file.
Review mapping: `E6`, VALID; severity High.

## Reach-and-Impact

Trigger: overlapping writer instances or app processes target the same recovery bundle or native file.
[S] One writer can rename the shared temporary inode while another still writes through an open handle.
[A] The corrupting interleaving and its frequency have not been observed in a runtime experiment.

## Evidence

- [S] `crates/format/src/store.rs:455-457` derives one temporary path, writes it, and renames it.
- [S] `crates/format/src/store.rs:383-415` publishes bundle objects and a manifest before garbage collection.
- [S] `apps/photocraft/src/services.rs:66-67,149` uses a shared per-user recovery directory and ID-derived keys.
- [S] `crates/format/src/autosave.rs:125-135` serializes one worker queue, not independent writer instances.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
`ISSUE-014` is direct destination truncation; `ISSUE-018` is duplicate identity inside one session.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending a controlled overlapping-writer reproduction.

## Proposed-Change

Create exclusively owned temporary siblings and publish only the current writer's complete output.
Coordinate directory-bundle transactions across processes through manifest publication and garbage collection.

## Scope-and-Constraints

- Preserve incremental content reuse and the existing last-complete-publication contract.
- A process-local mutex cannot coordinate separate processes.
- Unique temporary files alone do not prevent overlapping directory garbage collection.
- Do not infer power-loss durability from atomic rename or claim a reproduced race.

## Verification

Status: source-traced interleaving only; no concurrent-write experiment executed.
- Force two writers to overlap temporary creation, writing, and rename at one disposable destination.
- Verify every visible file is a complete committed output and directory snapshots remain loadable during GC overlap.

## Publication-Blockers

- The proposed overlapping-writer interleaving and directory transaction behavior need controlled execution.
- Upstream prior art, verified implementation, required review evidence, and the exact draft remain unresolved.

## Next-Action

Summary: Reproduce overlapping native writers
Action: Force two isolated writers to overlap publication at one disposable native destination.
Done-When: Record writer ordering, temporary inode ownership, rename results, visible bytes, and bundle loadability.

## Pull-Request-Implementation

Branch: fix/concurrent-native-publication
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Isolate temporary writers and coordinate native directory publication with garbage collection across processes.
Commit: Pending.
Push: Pending.
Checks:
- Pending.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.
