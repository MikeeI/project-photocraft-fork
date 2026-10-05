# ISSUE-030 — native import: reordered ZIP entries defeat format detection

State: Archived
Authorized-Work: Pull-Request-Implementation
Publication-Target: New-pull-request
External-Reference: https://github.com/storytold/photocraft/pull/137
Contribution-Priority: Medium
Root-Cause-Confidence: High
Finding-Category: Correctness
Created: 2026-10-05
Updated: 2026-10-06
Source: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`

## Root-Cause

[S] Baseline `is_pcraft` recognized native bundles only when the first local ZIP entry was `manifest.json`.
[S] Normal IO import relied on that detector and otherwise routed the bytes to the flat-image codec.
Review mapping: `E17`, VALID; severity Medium.
The fix restores the normal import contract while retaining manifest-first detection as the fast path.

## Reach-and-Impact

Trigger: re-zip a valid `.pcraft` bundle so another entry precedes `manifest.json`, then open it normally.
[S] The baseline detector rejected that entry order even though the native loader supports named lookup.
[O] The baseline public-import regression failed with `Codec(UnknownFormat)`.
[O] The post-fix public import accepts uppercase `.PCRAFT` and extensionless filenames.

## Evidence

- [S] Baseline `crates/format/src/lib.rs:94-100` inspected only the first local filename.
- [S] `crates/format/src/zip.rs:188-190` supports named central-directory lookup.
- [S] Baseline `crates/io/src/lib.rs:110-124` used the detector before flat-codec fallback.
- [O] `crates/io/tests/comps_artboards.rs:220-236` reorders entries and covers declared, extensionless, and malformed names.
- [O] Baseline `crates/format/src/zip.rs` offset regression panicked on overflowing addition before the checked-arithmetic fix.
- [O] Reordered bundles opened through the CLI and controlled desktop app after the fix.
- [S] Current `crates/format/src/zip.rs:128-195` checks central-entry ranges and local/data offsets before access.

## Prior-Art

Coverage: targeted upstream issue/PR searches and current PR #137 thread/diff checked on 2026-10-06.
PR #137 by @MikeeI merged on 2026-10-05 and resolves the same ZIP entry-order detection defect.
The original search found no prior result; upstream discussions and releases were not searched.
Contribution fit: a bounded import correction for bundles already supported by the native loader.

## Proposed-Change

Route declared `.pcraft` basenames to the native loader and detect extensionless native archives by manifest lookup.
Keep the first-entry fast path and return format errors rather than overflowing on malformed ZIP offsets.

## Scope-and-Constraints

- Preserve malformed declared-native errors and keep bare `pcraft` from counting as a filename extension.
- Reject malformed central-directory extents before reading entries or advancing offsets.
- Do not claim ZIP64, encrypted, or multi-disk support.
- Verify the shared IO importer through both the CLI and desktop app paths.

## Verification

Status: baseline import and offset-overflow regressions reproduced; post-fix checks and app paths passed.
- Baseline reordered `photocraft_io::import` failed with `Codec(UnknownFormat)`.
- Baseline `zip::tests::offset_access_rejects_overflow` panicked on `usize` addition; the updated test passes.
- Existing `synthetic_pcraft_roundtrip_keeps_everything` passes reordered `.PCRAFT`, extensionless, bare-`pcraft` PNG, and malformed `.pcraft` cases.
- `cargo test -p photocraft-format -p photocraft-io`: 252 passed across 24 suites.
- Affected Clippy with warnings denied, formatting, dependency layering, and all 20 WebAssembly packages passed.
- CLI `info` opened the reordered archive with `.pcraft` and no extension; ZIP listing showed `extra.txt` first.
- Controlled desktop `app.open` opened both filenames; `ui.inspect` showed the loaded 2400×1500 document.
- WebAssembly emitted existing `photocraft-cms` dead-code warnings for `PAR_MIN_PIXELS` and `PAR_CHUNK_PIXELS`.
- Independent xhigh source review approved the final patch.
[S] Current `upstream/main@47f4abfed49e0d2f5b9277287b27dee632530ba4` recognizes the manifest through central-directory lookup when it is not the first ZIP entry (https://github.com/storytold/photocraft/blob/47f4abfed49e0d2f5b9277287b27dee632530ba4/crates/format/src/lib.rs#L102-L116).

Test decision: update — existing `synthetic_pcraft_roundtrip_keeps_everything` covers public import routing.
Crash regression: `zip::tests::offset_access_rejects_overflow` was added to reproduce the required ZIP offset panic.

## Publication-Blockers

None.

## Next-Action

Summary: —
Action: None.
Done-When: None.

## Pull-Request-Implementation

Branch: `fix/import-reordered-native-zip`
Base: `upstream/main@ff53be714db50b8b190381eb0a9ec2b1ffab6715`
Scope: Recognize reordered `.pcraft` ZIPs through native import and reject overflowing ZIP offsets.
Commit: `305be5e`
Push: `origin/fix/import-reordered-native-zip`.
Checks:
- Baseline public import and ZIP offset regressions failed as described.
- Format and IO suites, affected Clippy, formatting, layering, and all 20 WebAssembly packages passed.
- Actual CLI and controlled desktop imports passed for extension-bearing and extensionless bundles.
- Independent GPT-6.1 Sol review at xhigh approved the source patch.

The user authorized implementation and publication of a verified fix PR on 2026-10-05.

## Publication-Draft

Target: `storytold/photocraft:main`
Head: `MikeeI:fix/import-reordered-native-zip`
Title: Detect PhotoCraft archives independent of ZIP entry order

### Problem

Normal import rejected valid `.pcraft` bundles when another local ZIP entry preceded `manifest.json`.
The native loader already supports named manifest lookup, but the public IO importer sent such files to the flat-image decoder.

### Change

Route actual `.pcraft` filename suffixes to the native loader and sniff extensionless bundles by manifest lookup.
Retain the manifest-first fast path and check ZIP entry extents and offsets before access.

### Verification

- The existing public IO regression failed before the fix with `Codec(UnknownFormat)`.
- It now covers reordered `.PCRAFT`, extensionless import, valid PNG content named `pcraft`, and malformed `.pcraft` errors.
- The ZIP offset overflow regression panicked before the fix and now returns an error.
- The format and IO suites passed: 252 tests across 24 suites.
- Affected Clippy, formatting, dependency layering, and all 20 WebAssembly packages passed.
- The actual CLI opened the reordered archive with and without its extension.
- The controlled desktop app opened both files; `ui.inspect` confirmed the loaded 2400×1500 document.
- Independent xhigh review approved the final source patch.

Reordered sniffed ZIPs are parsed before normal loading parses them again; no performance improvement is claimed.
ZIP64, encrypted archives, and multi-disk archives remain unsupported.

### Disclosure

Investigated thoroughly with GPT-6.1 Sol (extra high reasoning effort), using [Oh My Pi](https://github.com/can1357/oh-my-pi) as the agent framework.
I reviewed this contribution with GPT-6.1 Sol at xhigh reasoning effort.

This report is not generic or unreviewed AI-generated output.
Its claims were checked against the cited evidence, and it includes the relevant detail intended to help maintainers resolve the issue.

If reports like this are not useful to the project, please let me know and I will refrain from submitting similar ones.
My intent is to help without wasting maintainer time or energy or discouraging their work.

Thank you for your work.

## Archive

Archive-Reason: Merged
Detail: None.
Evidence: https://github.com/storytold/photocraft/pull/137
Checked: 2026-10-06
