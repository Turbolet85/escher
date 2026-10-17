# Codebase Research — 2026-10-17-upstream-sync-engine-debts

## Scope
- **Depth:** deep (a mature-codebase merge touching the engine, the driver, the bridge and 12 checks) · **Reads:** 19 · **Globs/Greps:** 21 (plus one typed sweep script and three graph queries)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the boot-smoke `timeout` exit; no boot smoke in this chunk) · `.claude/rules/testing.md` — read in full, 8 Session Additions, two applied: 2026-10-06 (a platform-bound key arm is compiled out on macOS — read the `#[cfg]` above an arm before trusting a grep hit; applied to `ACTION_MODIFIER`) and 2026-10-10 (search `tests/` for a changed table's readers before calling a check unaffected; applied to the key sweep and the target pin)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read one green run) and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the orchestrator's git measurement of `upstream/main` at the take-up (ls-remote then fetch, `1e5479ae6`; 14 commits past `434e06615`; both trial merges; the lock package set at both pins; the Taffy line per commit)

## Files inspected
- `escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/{scope,research}.md` (full scope; research's lists) — the last sync's shape: one merge commit, the pin as a measured sha, every both-sides file read, the lock refreshed for the two `escher-*` packages, files listed as "upstream's, by the merge"
- `packages/blitz-dom/src/accessibility.rs` at the trial merge `7ac45106` (1-300) — 5 conflict regions, listed in inputs#I1; our trimmed `aria-label` read (accname 2C) merges outside them at the merged :161-167; upstream's own `aria-label` and `aria-selected` sit in the last region (merged :213-221)
- `git diff 434e06615 HEAD -- packages/blitz-dom/src/accessibility.rs` (whole, +57 −1) — ours is the `labels` collection, the label→input `push_labelled_by` loop, the trimmed non-empty `aria-label`, and the three closed-set state reads
- `tests/blitz-tests/tests/accessibility_label.rs` at `upstream/main` (assertion lines) — 9 tests; every `aria-label` it asserts is non-blank (`"A node"`, `"Ignored"`, `"From aria-label"`, `"Fallback"`); `aria_labelledby_takes_precedence_over_aria_label_and_content` and `falls_back_to_aria_label_when_aria_labelledby_targets_have_no_text` fix the order labelledby > aria-label; no case reads a blank label or a `<label for>`
- `tests/blitz-tests/tests/accessibility_description.rs` at `upstream/main` (imports) — `accesskit`, `blitz_dom::DocumentConfig`, `blitz_html`, `blitz_traits::shell`, `std::sync::Arc`, `test_that::prelude::*`; 0 env, 0 print
- `packages/dioxus-native-dom/src/snapshot.rs` (154-161, 170-195, 625-640) — `enabled` from the `disabled` attribute (:154-156); `selected`/`pressed`/`invalid` from the AccessKit node (:159-161); `name` = the non-empty label else the `labelled_by` names (:178-195); a unit test sending `keyboard_types::Key::ArrowRight` (:635)
- `packages/escher-driver/src/execute.rs` (1-14, 455-501, 540-562) — `Key as KeyboardKey` from `blitz_test_harness` (:3-4); `ACTION_MODIFIER = Modifiers::SUPER` under `cfg(target_os = "macos")` (:461-462); `keyboard_key` maps our `Key` to 12 `KeyboardKey` values (:488-501), 11 named plus Space as `Character(" ")`; its unit test repeats the table (:545-556)
- `packages/escher-driver/src/command.rs` (62-81) — `Key` is our own enum (`Key::Tab` … `Key::End`, `KEY_NAMES`), not keyboard-types'
- `packages/blitz-dom/src/events/keyboard.rs` (named-key lines) — `Key::Tab` :27 (upstream's own rename covers it); ours `Key::Enter` :111 (activation), `Key::ArrowRight`/`ArrowUp`/`ArrowLeft`/`ArrowDown`/`Home`/`End` :151-154 (range); `Key::Character` :51, :129 unchanged in 0.8
- `packages/blitz-test-harness/src/lib.rs` (28-34) — re-exports `keyboard_types::{Key, Modifiers}` (:32), not `NamedKey`
- `packages/blitz-test-harness/src/input.rs` (named-key lines) — only `Key::Character` (:81, :224) and `Modifiers::default()`; nothing to migrate
- `tests/blitz-tests/Cargo.toml` (13-56) — dev-dependencies hold `blitz-html` :18, `accesskit` :33, `keyboard-types` :36, `test-that` :39; one `[[test]]` table
- `.github/scripts/test_blitz_tests_targets.py` (14-26) — asserts shape only: `autotests` is not `False`, and `test == [{name: all, path: tests/all.rs, test: false}]`
- `Cargo.lock` (tomllib) — `dioxus-html 0.7.10` → `keyboard-types 0.7.0`; `blitz-traits`, `blitz-test-harness` → 0.7.0 at HEAD; `winit-core` → 0.8.3
- `packages/escher-driver/Cargo.toml:5`, `packages/escher-telemetry/Cargo.toml:5` — `version.workspace = true`
- `.github/scripts/coverage_floor.py` (1-70) and `.github/scripts/ci-leg.sh:42-43` — the floor sums `LF`/`LH` of every `SF` record under a crate directory; the leg runs `cargo llvm-cov --workspace --locked --lcov` then `report`
- `target/ci-logs/coverage.log` (:2151-2201) — `packages/dioxus-native-dom 2472/2934 = 84.25 % (floor 83 %)`
- `packages/dioxus-native-dom/src/keyboard_compat.rs` at `upstream/main` (structure) — 564 lines, 530 `=>` arms; `convert_modifiers` :10, `convert_location` :14, `convert_key` :23, `convert_named_key` :31-343, `convert_code` :345-564
- `.andromeda/playbook.md` (escalate patterns, :36-69) — "Boundary widening" :36-38 and "Boundary widening by upstream merge" :60-62 escalate; "A field inside a ratified carrier" :40-42 is routine only for closed value sets
- `deny.toml` — no `sources` line

## Graph impact (trace `.andromeda/runs/2026-10-17T12-18-00-phase/tree-query-2026-10-17-upstream-sync-engine-debts.json`, rust plane; lines below are editor lines, the graph's 0-indexed `line` + 1)
- **keyboard_key** — 2 callers: `execute/press()` @ `packages/escher-driver/src/execute.rs:478`, `execute/tests/each_schema_key_stands_for_one_keyboard_key()` @ `:562`. The migration is local to the file; no signature changes.
- **label_bound_input_element** — 2 callers: `build_accessibility_tree()` @ `packages/blitz-dom/src/accessibility.rs:43`, `handle_click()` @ `packages/blitz-dom/src/events/pointer.rs:780`. The conflict resolution keeps our call site in the builder; `handle_click` is untouched.
- **build_accessibility_tree** — 3 production callers (`Document::accessibility_tree` @ `packages/blitz-dom/src/document.rs:195`, `BlitzApplication::handle_blitz_shell_event` @ `packages/blitz-shell/src/application.rs:82`, `DioxusDocument::accessibility_tree` @ `packages/dioxus-native-dom/src/dioxus_document.rs:298`) and 12 test callers in `accessibility_hidden.rs` (6), `accessibility_names.rs` (1), `accessibility_roles.rs` (2), `accessibility_states.rs` (3). Its signature does not change; the tests re-prove the resolution.

## Patterns detected
- **The sync's shape** (escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/scope.md): one merge commit at a measured pin, conflicts named with their resolution, a de-fork where upstream now carries our behaviour, the lock taken whole with our two workspace packages refreshed, counts re-read on the merged tree.
- **Upstream reads overlapping ours** (accessibility.rs, trial merge :161-167 vs :213-221): upstream's `aria-selected` (`parse::<bool>`) accepts exactly `"true"`/`"false"` and sets the same boolean ours sets there; ours also accepts padded values. So upstream's read never changes an outcome ours decides, and dropping it is behaviour-preserving. Upstream's `aria-label` (unfiltered) differs from ours only on a blank value, which our contract says names nothing.
- **Accname order by construction**: upstream's labelledby loop calls `builder.clear_labelled_by()` once it has a target, then pushes the targets and sets a computed label. Run after our label loop, an element with both a `<label for>` and `aria-labelledby` reads the labelledby name: accname 2B before 2D.
- **The macOS-only arm** (`execute.rs:461-462`): `ACTION_MODIFIER` is `Modifiers::SUPER` under `cfg(target_os = "macos")`. keyboard-types 0.8 renames it `META` (upstream's own `util.rs` hunk). A Linux build compiles the other arm, so the rename is witnessed only by the macOS CI job (testing.md 2026-10-06; test-plan history 2026-10-10-driver-cli: the macOS leg is the only witness of the select-all modifier).
- **Two keyboard-types in one graph**: blitz moves to 0.8 while Dioxus 0.7 stays on 0.7. Upstream bridges them with `keyboard_compat.rs` in dioxus-native-dom. Our Dioxus apps (`crud.rs`, `cells.rs`) keep Dioxus's 0.7 `Key` and need no edit.

## Conventions to follow
- **The key-type seam**: escher-driver names keyboard types only through `blitz_test_harness`'s re-exports (`execute.rs:3-4`; arch §Standard Contracts → Test harness). `NamedKey` is not re-exported (`lib.rs:32`), so the migration adds `NamedKey` to that re-export line. The alternative, a direct `keyboard-types` dependency in escher-driver, is the one arch rules out. Leaned per arch §Standard Contracts → Test harness.
- **Stand checks name `keyboard_types` directly** (nine files, `use keyboard_types::Key;` / `{Key, Modifiers}`) through blitz-tests' own dev-dependency (`tests/blitz-tests/Cargo.toml:36`), which the merge moves to 0.8 with the workspace. Their migration is `Key::X` → `Key::Named(NamedKey::X)`, importing `NamedKey` beside `Key`.
- **Check messages** keep the act-by-id rule: a failure message names a layout mode, a task and an index, never an id or screen text (test-plan history 2026-10-07-act-by-id). The migration changes how a key is spelled, never a message or an expected table.
- **One read per attribute** in the accessibility builder (arch extract's anti-pattern; security-plan §Input Validation rows).

## New files to create
- `escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/evidence/` — the chunk's evidence: the merge record, the conflict resolutions, the lock review, the counts, the moved readings, the sink reading, the coverage reading, the CI run and the per-OS tallies
- `packages/dioxus-native-dom/src/keyboard_compat.rs` — upstream's new file, by the merge
- `tests/blitz-tests/tests/accessibility_description.rs` — upstream's new file, by the merge
- `tests/blitz-tests/tests/accessibility_label.rs` — upstream's new file, by the merge

## Files to modify
- `Cargo.toml` — upstream's, by the merge (members, workspace version, in-repo pins, keyboard-types, test-that; taffy only at `1e5479ae6`)
- `Cargo.lock` — the conflict: regenerated from the resolved manifests at the pin, then the two escher workspace entries to 0.3.0-beta.4
- `justfile` — upstream's, by the merge (the `bump` recipe leaves)
- `apps/bump/Cargo.toml` — deleted by the merge
- `apps/bump/src/main.rs` — deleted by the merge
- `packages/blitz-dom/src/accessibility.rs` — the conflict: both sides kept, upstream's duplicate `aria-label` and `aria-selected` reads dropped, our label loop before upstream's labelledby loop
- `packages/blitz-dom/src/events/keyboard.rs` — upstream's Tab rename by the merge, and our Enter and range arms migrated to `NamedKey`
- `packages/blitz-dom/src/layout/inline.rs` — upstream's, by the merge (#1198; at `1e5479ae6` also the `place_floated_box` argument)
- `packages/blitz-dom/src/node/text.rs` — upstream's, by the merge
- `packages/blitz-dom/src/util.rs` — upstream's, by the merge
- `packages/blitz-shell/src/convert_events.rs` — upstream's, by the merge
- `packages/blitz-vibey-script/src/dom/element.rs` — upstream's, by the merge
- `packages/blitz-vibey-script/src/runtime.rs` — upstream's, by the merge
- `packages/blitz-vibey-script/tests/preact.rs` — upstream's, by the merge
- `packages/dioxus-native-dom/src/events.rs` — upstream's, by the merge
- `packages/dioxus-native-dom/src/lib.rs` — upstream's `mod keyboard_compat;`, by the merge, beside our six hunks
- `packages/dioxus-native-dom/src/snapshot.rs` — the unit test's `Key::ArrowRight` migrated
- `packages/blitz-test-harness/src/lib.rs` — `NamedKey` added to the `keyboard_types` re-export
- `packages/escher-driver/src/execute.rs` — `keyboard_key`, its unit test, and the macOS `ACTION_MODIFIER` migrated
- `tests/blitz-tests/tests/all.rs` — upstream's two `mod` lines, by the merge (still `test = false`)
- `tests/blitz-tests/tests/range_input_interaction.rs` — key names migrated
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` — key names migrated
- `tests/blitz-tests/tests/stand_contrast.rs` — key names migrated
- `tests/blitz-tests/tests/stand_diff.rs` — key names migrated
- `tests/blitz-tests/tests/stand_keyboard.rs` — key names migrated
- `tests/blitz-tests/tests/stand_session_fresh.rs` — key names migrated
- `tests/blitz-tests/tests/stand_session_state.rs` — key names migrated
- `tests/blitz-tests/tests/stand_settle.rs` — key names migrated
- `tests/blitz-tests/tests/stand_snapshot_state.rs` — key names migrated
- `tests/blitz-tests/tests/stand_snapshot_text.rs` — key names migrated
- `.github/scripts/ci-leg.sh` — the coverage leg's two `cargo llvm-cov` lines skip upstream's `keyboard_compat.rs` by filename (P4 fork 3, inputs#I2)
- `.github/scripts/test_ci_workflows.py` — the coverage leg's pin also reads the exclusion's flag

## Sweeps
- Named keyboard-types keys and `Modifiers::SUPER` over every tracked `*.rs` (`scratchpad/keysweep.py`: `\b(Key|KeyboardKey)::(?!Character|Named|ALL)[A-Z]…` and `Modifiers::SUPER`, with each file's `use` line read): 24 files hit · 15 changed (above) · 9 no-change — `apps/browser/src/{nav,toolbar}.rs`, `examples/todomvc/src/app.rs`, `examples/seven_guis/src/tasks/{crud,cells}.rs` (Dioxus's `Key`, keyboard-types 0.7 through `dioxus-html 0.7.10`); `packages/escher-driver/src/command.rs` (our own `Key` enum); `packages/blitz-dom/src/node/text.rs`, `util.rs`, `packages/blitz-shell/src/convert_events.rs`, `packages/blitz-vibey-script/tests/preact.rs` (upstream migrates them itself, listed above as "by the merge"). Re-derived; inputs#I1's rough count is superseded.
- Upstream log, print and env sites over `git diff 434e06615 23b9b26a9 -- '*.rs'` (`^\+.*(tracing::|debug!|trace!\(|info!|warn!|error!|println!|eprintln!|env::var|log::)`): 0 added of 1,324 added lines (control: the same file's `^+` count, 1,324) · 5 removed, all in `apps/bump/src/main.rs`.
- `bump` over CLAUDE.md and the five masters that name it (`command grep -n -i bump`): 11 hits, all docs, each a wrap amendment (scope §What it builds), 0 code edits.
- `synthetic_click_event` callers (`command grep -rn` over packages, examples, tests): 9 hits — 3 engine `synthetic_click_event_data` calls (signature unchanged), 6 vibey-script test calls passing `Modifiers::empty()`/`SHIFT`, which keyboard-types 0.8 keeps · 0 changed.
- Files and targets (`ls tests/blitz-tests/tests/*.rs | wc -l`): 125 `.rs` at HEAD, `all.rs` included, so 124 targets; after the merge 127 and 126.

## Open questions
- none — the three plan-decision questions P3 carried were answered at P4 (inputs#I2): the pin is `23b9b26a9`; the founder's «Утвердить оба (Рекомендую)» puts `aria-labelledby` names inside the ratified name crossing and ratifies `aria-description` to the adapter as upstream's behaviour, the merge keeping upstream's reads as written; the coverage leg excludes `keyboard_compat.rs` by filename. The `coverage_floor.py` branch of fork 3 was not taken, and its two files left the list.
