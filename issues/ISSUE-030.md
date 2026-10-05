# ISSUE-030 — native import: reordered ZIP entries defeat format detection

State: Investigating
Authorized-Work: Not-Selected
Publication-Target: Not-Selected
External-Reference: Not published.
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-05
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Normal import identifies native bundles only when the first local ZIP entry is `manifest.json`.
Review mapping: `E17`, VALID; severity Medium.
This is an integration-contract failure; the narrow detector behavior itself is documented.

## Reach-and-Impact

Trigger: unpack a valid `.pcraft` bundle and re-zip it with a blob or preview entry first.
[S] The native loader supports named lookup, but normal import routes the reordered bundle to the flat-image decoder.
No repack/import experiment was executed.

## Evidence

- [S] `crates/format/src/zip.rs:1-3` explicitly supports bundles re-zipped by another tool.
- [S] `crates/format/src/lib.rs:96-100` tests only the first local entry name.
- [S] `crates/io/src/lib.rs:110-123` uses that test to choose the native loader or flat import.

## Prior-Art

Coverage: local ledger checked on 2026-10-05; no matching root cause.
Gaps: upstream issues, PRs, discussions, and releases not searched.
Contribution fit: unresolved pending verification at the actual import boundary.

## Proposed-Change

Route declared `.pcraft` input through the native loader regardless of ZIP entry ordering.
Preserve magic-based recognition for native content when its input name has no native extension.

## Scope-and-Constraints

- Preserve malformed-bundle errors rather than silently reinterpreting declared native files as flat images.
- Do not claim ZIP64, encrypted, or multi-disk support; those are explicitly unsupported.
- Verify normal UI and CLI import, not only the lower-level ZIP reader.

## Verification

Status: source-traced; no reordered bundle imported.
- Repackage a native bundle with a non-manifest first entry and open it through `photocraft_io::import`.

## Publication-Blockers

- Reordered native bundle acceptance through the real import boundary needs verification.
- Upstream prior art, authorized work, target, exact draft, and publication approval remain unresolved.

## Next-Action

Summary: Reproduce reordered bundle rejection
Action: Repackage a disposable native bundle with a non-manifest first entry and compare direct and normal imports.
Done-When: Record ZIP entry order, input name, native loader result, and ordinary import result.
