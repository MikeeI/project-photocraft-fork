# project-photocraft-fork

<essential-rule>
AGENTS.md is the sole authoritative project context file.
Read and edit AGENTS.md directly.
</essential-rule>

## Development Rules

Before launching agents, apply skill-xray, skill-expert, and skill-brutal to the task.
Surface expert-level issues, non-obvious issues, blindspots, stale assumptions, and hidden dependencies.
Also surface missed constraints, edge cases, false positives, verification gaps, overclaims, and weak assumptions.
Identify improvement potential, inefficiencies, and what is wrong without softening.
Use these findings to design safe slices, sequencing, checks, and boundaries for complete agent results.

Every agent prompt must require skill-xray, skill-expert, and skill-brutal for the assigned scope before acting.
It must surface non-obvious issues, blindspots, stale assumptions, hidden dependencies, and edge cases.
It must also surface verification gaps, overclaims, failure modes, weak assumptions, and what is wrong.
The agent must adjust its approach, challenge its assumptions, and flag misleading or incomplete output risks.

Implementation assignments must cover existing patterns, callers, exported-symbol consumers, and failure modes.
They must also cover concurrency safety and lifecycle cleanup.
Each assignment must state `Test decision: none` or `Test decision: update`.
`update` must name the exact existing test that follows an intentional contract change.
Never request new tests.
Prohibit broad edits, unrelated cleanup, and unassigned files.

No vague agents.
Each assignment needs exact targets, non-goals, evidence anchors, acceptance criteria, and an output contract.

Commit completed units continuously.
Before each commit, use skill-git-commit-format to determine whether staged effects are one coherent unit.
The skill owns commit-message format and evidence.
After the boundary is valid, run the repository-owned commit and push workflow.
Do not commit every trivial edit immediately or defer unrelated work into one end-of-session commit.

Every project-level quality command is quiet by default and verbose on demand.
This policy applies regardless of language or toolchain.
It covers Make targets, package scripts, Python CLIs, shell quality gates, and test runners.
Successful checks print only compact status such as `format: ok`, `lint: ok`, `test: ok`, or `check: ok`.
On failure, exit non-zero and print the failing step, exit code, and enough output to act without rerunning.
Full raw output must remain available through `--verbose`, `VERBOSE=1`, or the underlying tool's verbose mode.
New quality commands and future language setup must follow this policy instead of inventing another logging contract.

# Repository Guidelines

## Project Overview

PhotoCraft is a native, open-source, Photoshop-comparable image editor written only in Rust.
Its goal is 1:1 Photoshop menus, shortcuts, behavior, and file fidelity, with better performance and agent-driven features.
Use **PhotoCraft** in user-facing text and lowercase machine identifiers such as `photocraft-*` and `ai.storyteller.photocraft`.
Related crafting apps follow the `{Function}Craft` naming convention but do not share code.

## Fork & Upstream Contribution Intent

- Official upstream: [storytold/photocraft](https://github.com/storytold/photocraft).
- This checkout is [MikeeI/project-photocraft-fork](https://github.com/MikeeI/project-photocraft-fork), not an independent product.
- The goal is evidence-backed upstream issues, comments, and pull requests, favoring small, high-value corrections.
- `origin` is `git@github.com:MikeeI/project-photocraft-fork.git`.
- `upstream` is `git@github.com:storytold/photocraft.git`; the contribution base is `upstream/main`.
- Work in `personal`, tracking `origin/personal`; this branch owns fork-only context and contribution tracking.
- Keep `main` free of personal commits; create upstream contribution branches or worktrees from current `upstream/main`.
- Never base upstream pull requests on `personal` or include its tracking commits in contribution diffs.
- `ISSUES.md` owns the compact finding overview and global allocator.
- Each `issues/ISSUE-NNN.md` owns one root cause's complete durable record.
- `FORMAT.md` owns research, drafting, authorization, approval, lifecycle, and publication rules.
- Apply `skill-fork-contribution-tracking` for ledger, personal-branch, and upstream handoff work.
- Apply `skill-maintainer-communication` before external issues, pull requests, reviews, comments, or discussions.
- Apply `skill-semantic-compression-3` when authoring tracking content.
- Apply `skill-git-commit-format`, respecting explicit upstream contribution conventions.
- Search existing work first and follow current upstream templates and disclosure rules.
- Recommend a pull request when a bounded verified fix is ready and no active implementation owns it.
- Otherwise recommend a comment, new issue, or continued investigation according to the evidence.
- Never choose `Authorized-Work` or `Publication-Target` on the user's behalf.
- `Research-and-Reporting` permits research, issues, and comments, but no source implementation.
- `Pull-Request-Implementation` authorizes only the implementation scope recorded for that finding.
- Reproduce claimed bugs against current upstream and run the narrowest conclusive verification.
- Publish one coherent root cause per issue, comment, or pull request.
- Keep fork-only context, ledgers, configuration, and personal commits out of upstream contribution diffs.

## Finding and Contribution Ledger

- Read root `ISSUES.md` at the start of every agent session before repository work.
- `Next finding ID` in `ISSUES.md` is the sole global allocator; start with `ISSUE-001`.
- Read `FORMAT.md` and the selected issue record before contribution-tracking work.
- Keep open records in `issues/`; archived records belong in `issues/archive/`.
- Read archived records only when selected by the user or needed for a plausible duplicate comparison.
- Before adding a finding, search the index and relevant records for the same symptom or root cause.
- Allocate `Next finding ID`, create the record, add its index row, and increment the allocator together.
- IDs remain permanent; never reuse, renumber, or scope them by subsystem, status, session, or contribution type.
- Update the record and index together after state, authorization, target, priority, next action, or reference changes.
- Every record follows `FORMAT.md`; correct projection disagreements from the authoritative issue record.
- New findings use `State: Investigating`, `Authorized-Work: Not-Selected`, and `Publication-Target: Not-Selected`.
- Set `External-Reference: Not published.` until an external reference exists.
- Keep findings Investigating until currentness, prior art, impact, and correction value are evidence-backed.
- Clone detectors, AST matches, text similarity, shared names, and TODOs produce candidates only.
- Duplication findings require shared change pressure, realistic drift, and simpler consolidation.
- The user selects each finding's `Authorized-Work`.
- `Research-and-Reporting` must not implement the finding.
- Authorized implementation stays within recorded scope after research resolves callers and failure modes.
- Pull-request work must verify behavior, commit, push, and reach `PR-Ready` before publication.
- Show the exact draft and target before publishing to official upstream.
- Publish only after user approval of that exact current draft and target.
- Any draft or target change requires showing the complete current draft and target again before publication.
- Record the final external URL immediately after publication.
- Run the skill's read-only validator after every ledger mutation.
- Keep `FORMAT.md`, `ISSUES.md`, `issues/`, and fork-only `AGENTS.md` changes out of upstream contribution diffs.

### External publication approval

External issue, comment, review, discussion, and pull request writes require approval.
Before publication, read current contribution guidance and explain applicable project policy.
The human must be able to review and own every submission statement.
Fork commits, pushes, tracking updates, and source implementation follow the active repository contract.

## Architecture & Data Flow

UI, CLI, authenticated TCP control, and MCP dispatch engine commands by ID into a session and document model.
The CPU compositor is the reference oracle; the wgpu compositor accelerates supported rendering.
Document import/export goes through `photocraft-io`; `photocraft-format` owns lossless `.pcraft` persistence.
The browser uses the same Rust UI through eframe, WebGPU/WebGL2, and generated wasm-bindgen glue.
Never hand-write JavaScript or TypeScript for this project.

### Key Directories

- `crates/geom,cms,color,raster`: L0 geometry, ICC management, pixel formats, blend math, and COW tiles.
- `crates/psd,codecs`: standalone formats without workspace dependencies.
- `crates/doc`: L1 pure-data documents, layers, masks, adjustments, effects, and smart objects.
- `crates/ops,paint,algo,text,vector`: L2 history, brushes, imaging, typography, and shapes.
- `crates/compose,gpu,format`: L3 CPU/GPU composition and native persistence.
- `crates/io,plugins`: L4 document interchange and sandboxed WebAssembly plugins.
- `crates/engine`: L5 sessions and command registry.
- `crates/ui-egui,automation`: L6 UI shell and MCP automation.
- `crates/testkit`: test helpers; `crates/raw`: camera RAW support.
- `apps/photocraft`: desktop app and TCP control server.
- `apps/photocraft-cli`: headless conversion, inspection, batch actions, and MCP.
- `apps/photocraft-web`: Trunk/wasm-bindgen browser app.
- `xtask/`: repository-owned layering, CI, wasm, corpus, parity, statistics, and release commands.
- `packaging/`: platform installers; `assets/`: fonts, icons, and dictionaries.

`cargo xtask layers` enforces the dependency graph; register new crates in `xtask/src/layers.rs`.
Use lower layers and only the explicit intra-layer dependencies permitted by that table.
`psd`, `codecs`, and `cms` have no workspace dependencies.
Nothing below the UI layer may depend on egui, eframe, winit, or rfd.

## Development Commands

Run commands from the workspace root unless stated otherwise.

```sh
cargo run --release -p photocraft -- path/to/image.psd
cargo run -p photocraft-cli -- commands --filter blur
cargo test -p <affected-crate>
cargo clippy -p <affected-crate> --all-targets -- -D warnings
cargo xtask layers
cargo xtask wasm
cargo xtask parity
cargo xtask ci
```

`cargo xtask ci` runs formatting checks, Clippy, tests, layering, and wasm checks.
Run `cargo xtask wasm` after L0–L6 changes.
Run `cargo xtask parity` after command additions and commit regenerated `docs/parity.md`.
For added or changed commands, also run:

```sh
cargo test -p photocraft-engine --test panic_hunt -- --ignored
```

## Code Conventions

### Never crash

Document safety outranks feature work: malformed files, commands, MCP parameters, settings, and full disks must return actionable errors.
Fix a crash before building on it; unfinished features return unsupported errors rather than panicking.

- Non-test code forbids `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, and `unimplemented!`.
- Return crate-owned errors through `Result` and `?`; never use a fallback that silently corrupts documents.
- A provably infallible literal is the sole `expect` exception, with `#[allow(clippy::expect_used)]` and an explanatory message.
- The workspace forbids unsafe code.
- Check input-derived indices with `get()`, string character boundaries, allocation caps, and numeric overflow.
- Use checked or saturating arithmetic where appropriate; guard division by zero and NaN/infinite casts.
- Bound recursion with depth limits or seen sets, because documents may be deep or cyclic.
- Recover poisoned locks through `PoisonError::into_inner` and handle thread joins as results.
- Keep unwind behavior so the app shell can catch escaped panics around command dispatch and import/export without losing documents.
- Shell panic guards are a last resort, not permission to introduce panic paths.
- Every crash fix requires a synthetic regression test that previously panicked.
- New crates deny Clippy's unwrap, expect, panic, unimplemented, todo, and unreachable lints.
- `clippy.toml` permits panic-prone operations and indexing in tests only.

### Commands and UI ownership

- New user-visible behavior belongs in `crates/engine/src/<area>_cmds.rs` with `specs()`, registered in `commands.rs`.
- Commands define ID, label, menu path, shortcut, parameter documentation, enabled predicate, and run function.
- Use the exact ID in `crates/ui-egui/src/menu_catalog.rs` to activate the matching menu item automatically.
- Only view/window state such as zoom, panels, and screen mode belongs in `menus.rs` and `UI_COMMANDS`.
- Command runs return errors for arbitrary parameters and document states; validate before indexing, dividing, or allocating.
- Commands respect selections, masks, locks, color model, depth, and one history step per action.
- UI state lives in `crates/ui-egui/src/state.rs` with serde for control-channel access.
- Use `theme::Tokens` and `widgets::*`, not hard-coded colors or radii.
- New command modules avoid growing shared registration and state files unnecessarily.

### Pixel and persistence invariants

- Depths 8/16/32f and RGB/Gray/CMYK/Lab color models are runtime data; public pixel APIs must not assume `u8`.
- Never assume sRGB; conversions use `photocraft-cms`, `Transform`, and `transform::cached`.
- Doc-field additions must update `crates/format/src/manifest.rs` and `convert.rs` with `#[serde(default)]` for old saves.
- The format crate deliberately fails compilation on missing document fields to prevent silent persistence loss.
- Map PSD equivalents in `crates/io` and preserve unknown PSD blocks verbatim.
- Keep filesystem access behind native target guards or platform services so L0–L6 remains wasm-compatible.
- Process tiles in parallel with Rayon, skip empty tiles, and cache by revision rather than scanning surfaces every frame.
- Benchmark heavy operations on 24–36 MP images in release and record before/after timings in the dev log.

### Clean-room and assets

Match proprietary editors only through behavioral observation and public specifications, never copied code, shaders, profiles, or assets.
Contributions use MIT OR Apache-2.0; retain `LICENSE-MIT`, `LICENSE-APACHE`, and `NOTICE`.
Third-party assets need permissive licenses beside the assets and an `ATTRIBUTION.md` row with path, title, author, source, and license.
Original assets also require attribution rows.
ArtCraft logos in `docs/brand/` are not open source; `docs/brand/LICENSE-brand.txt` governs them.
Demo images must be public-domain art, never personal photos.
Asset requests must not expose personal names, email addresses, or local project identity in headers or URLs.
Use a mainstream browser User-Agent rather than upstream's project-identifying example.

## Important Files

- `docs/architecture.md`: detailed crate graph and engine/UI seam.
- `docs/development.md`: build, debugging, rendering, environment variables, and automation recipes.
- `docs/contributing.md`: contribution policy and command checklist.
- `docs/control-protocol.md`: authenticated JSON control and MCP protocol.
- `docs/ui-design.md`: themes, widgets, and design tokens.
- `docs/roadmap.md`: milestones M0–M12 and current focus.
- `docs/parity.md`: generated menu coverage; never lower `crates/ui-egui/src/parity.rs`'s `FLOOR`.
- `docs/releasing.md`: versioning, release branch, signing, and packaging specifics.
- `crates/<name>/README.md`: crate APIs where available.
- `Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `rustfmt.toml`, and `clippy.toml`: existing tooling owners.

Upstream references sibling `../craftrules` for reusable standards, not shared code.
Its `README.md`, `standards/never-crash.md`, and `release/playbook.md` are external prerequisites when relevant.
Do not assume that sibling checkout exists or create it as part of fork initialization.
`docs/release-playbook.md` points to the shared release recipe.
Upstream keeps research in gitignored `plan/` and development notes in gitignored `log/`.
Use `project/` with uppercase filenames for new fork-owned planning documents.
Append completed work, measured numbers, and remaining work to `log/devlog.md`.
Use the roadmap's current focus, then missing parity with existing algorithms, when selecting authorized feature work.

## Runtime/Tooling Preferences

- Rust stable 1.90+ and edition 2024 are the upstream baseline; use Cargo and retain its lockfile.
- `.cargo/config.toml` maps `cargo xtask` to the workspace's `xtask` binary.
- Linux desktop builds need `libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libgtk-3-dev`.
- The web target is `wasm32-unknown-unknown`; run Trunk from `apps/photocraft-web`.
- Use release builds for interaction; dev profiles already optimize image code and dependencies.
- Live automation uses `--control`, a control token file, and explicit read/write roots; see `docs/control-protocol.md`.
- Parallel Cargo work may use `CARGO_TARGET_DIR=target/agent-<name>`; each target directory can approach 10 GB.
- Keep manifests valid because the `crates/*` workspace glob reads every crate; stage new manifests outside that glob before atomic publication.

## Testing & QA

Upstream requires tests for source changes and graceful-failure tests for new commands.
This project-specific requirement overrides generic advice against adding tests when the upstream contract requires them.
Test behavior, undo/redo, disabled states, invalid parameters, and multiple pixel depths at the relevant boundary.
Format crates use round-trip, synthetic-generator, oracle, malformed-input, and fuzz tests.
Keep PSD corpus results and the parity floor from regressing; raise `FLOOR` when parity rises.
Corpus tests can silently skip absent files, so do not report corpus coverage without the actual corpus.
Run affected-crate tests and Clippy, plus `cargo xtask layers`, before completing source work.
Fork-only tracking changes use the bundled ledger validator and Git diff checks; no application behavior is changed.

UI changes require an inspected PNG from the offscreen snapshot example or `ui.screenshot` on a controlled app instance.
Capture only the app window; macOS screenshots may raise it because occluded windows are not rendered.
Attach before/after screenshots to UI pull requests.

```sh
cargo run --release -p photocraft-ui-egui --example snapshot -- --out ui.png
```
