# Codebase Research — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

## Scope
- **Depth:** moderate · **Reads:** 16 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the `timeout` exit-124 rule; no boot smoke in this chunk) · `.claude/rules/testing.md` — read in full, 9 Session Additions applied (counting result lines only under `--no-fail-fast`; attributing `cargo test` results on one merged stream; searching `tests/`, `examples/*/tests/` and the owning crate's `src/` before calling a standing check unaffected; the off-unix check after a `#[cfg(unix)]` move — none moved here)
- **Platform issues consulted:** none — no runner-only bullet: both shas Setup 5a read are green 17/17
- **External inputs:** `inputs#I1` — upstream/main `17de049b1`, 7 commits past `23b9b26a9` (four Taffy-main bumps, the cursor fix already taken, `:focus-within`, a WPT pin); one conflict in `mutator.rs`; `keyboard_compat.rs` not retired; no Taffy release past 0.15.0

## Files inspected
- `packages/blitz-dom/src/document.rs` (1630-1660, 1740-1800) — `snapshot_node_and` marks the changed set only when the state intersects `FOCUS | CHECKED` (1644-1647); `clear_focus` (1759) and `set_focus_to` (1772) as HEAD holds them, with the `#[cfg(feature = "tracing")]` `Focussed node` INFO line inside `set_focus_to`; `focus_next_node`/`focus_prev_node` filter by `is_focussable` (1745, 1753). Upstream's `38ca4e777` adds to `clear_interaction_state_for_removed_node` (941), `clear_focus` and `set_focus_to`: a `FOCUS_WITHIN` walk over the ancestor chain through `snapshot_node_and(…, ElementState::FOCUS_WITHIN, …)`, a `request_redraw()` where focus is set and cleared, and an early `return false` in `set_focus_to` when the node is not in the document — placed after the same-node check and before the tracing line.
- `packages/blitz-dom/src/mutator.rs` (690-790, 950-970) — `add_children_to_parent` holds our changed-set insert for the new parent (731-733) and each moved in-document child (744); the trial-merged body (inputs#I1) conflicts only at 731, our three-line `if` against upstream's `focus_within_old_chain` declaration; every other upstream hunk (the chain record at the loop's head, the transfer after reattachment) auto-merges beside our loop. `flush`'s autofocus calls `set_focus_to` on a node `process_added_subtree` picked when it entered the document (962-966, 1049).
- `packages/blitz-dom/src/events/keyboard.rs` (15-50) — the Tab arm of `handle_key_or_input_event` holds `old_focus` and its own `request_redraw()` (29, 41-43); the trial merge removes exactly those four lines, our activation arms below untouched.
- `packages/blitz-dom/src/traversal.rs` (286, 301) — `node_element_ancestors` and `maybe_node_element_ancestors`, which upstream's hunks call, already stand at HEAD.
- `packages/blitz-dom/src/stylo.rs` (474-479) — `NonTSPseudoClass::FocusWithin => false` at 477, the line upstream's `stylo.rs` hunk replaces.
- `packages/blitz-dom/src/config.rs` (47, 73) — `shell_provider` and `clock` fields; upstream's `focus_redraw.rs` sets `shell_provider` with `..Default::default()`, so our added `clock` field does not break it.
- `packages/blitz-dom/src/document_changed_set_tests.rs` (110-141) — `changed_set_focus_move_marks_both_nodes_and_hover_marks_nothing` asserts the set after a focus move equals exactly `{first, second}`; `FOCUS_WITHIN` changes do not mark, so it holds on the take.
- `tests/blitz-tests/Cargo.toml` (13-59) — auto-discovery kept (no `autotests = false`), one `[[test]]` table `all` with `test = false`, `keyboard-types` among the dev-dependencies (the crate `focus_redraw.rs` imports).
- `.github/scripts/test_blitz_tests_targets.py` (full) — pins those two lines; a new file directly under `tests/` needs no manifest line.
- upstream's `tests/blitz-tests/tests/focus_redraw.rs` and `focus_within.rs` at `17de049b1` (`git show`) — four redraw tests on a hand-written `RecordingShell` (`focusing_a_node_requests_a_redraw` · `refocusing_the_same_node_requests_nothing` · `blurring_requests_a_redraw` · `tabbing_to_the_next_node_requests_a_redraw`, each `assert_eq!` on an exact count) and seven `:focus-within` tests (ancestors and blur, `display: contents`, focus before the first layout, removal, a moved subtree, a subtree moved out of the document, a node outside the document refused). Neither prints, reads an env var or installs a subscriber (§Sweeps).
- `tests/blitz-tests/tests/interaction_state_teardown.rs` (31-216) — its `RecordingShell` records `set_ime_enabled`, never redraws; no redraw count to move.
- `.andromeda/architecture.md:85` ([Unsupported features]) and `.andromeda/a11y-plan.md:155` — both state that `:focus-within` never matches, citing `stylo.rs:476-478`; a take carrying `38ca4e777` falsifies both lines (expected amendments at the wrap).

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-19T18-50-26-phase/tree-query-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`, plane rust)
- **set_focus_to** — called by `focus_next_node` @ `packages/blitz-dom/src/document.rs:1746`, `focus_prev_node` @ `:1754`, `flush` (autofocus) @ `packages/blitz-dom/src/mutator.rs:964`, `handle_dom_event` @ `packages/blitz-dom/src/events/mod.rs:142`, `:172`, `handle_pointerdown` @ `packages/blitz-dom/src/events/pointer.rs:574`, `handle_click` @ `:771`, `:795`, `:823`, `:849`, `click_button` @ `:980`, dioxus-native-dom's `set_focus` @ `packages/dioxus-native-dom/src/events.rs:298`, blitz-vibey-script's `focus` @ `packages/blitz-vibey-script/src/dom/element.rs:1010`, the browser `Toolbar` @ `apps/browser/src/toolbar.rs:60`, and tests (`document_changed_set_tests.rs:121`, `:126`, `snapshot.rs:664`, `accessibility_active_descendant_unread.rs:58`, `interaction_state_teardown.rs:201`, `range_input_interaction.rs:60`, `stale_interaction_state.rs:190`, `preact.rs:98`). Every caller passes a node reached through the document (a hit, a traversal, an id lookup, a node just added); none is a node outside it on the stand's paths, so the new refusal moves no stand reading. Upstream's change keeps the signature: no caller is edited.
- **clear_focus** — `handle_click` @ `packages/blitz-dom/src/events/pointer.rs:957`, blitz-vibey-script `blur` @ `packages/blitz-vibey-script/src/dom/element.rs:1017`, `set_focus` @ `packages/dioxus-native-dom/src/events.rs:302`, `document_changed_set_tests.rs:142`, `stale_interaction_state.rs:195`. Signature kept.
- **clear_interaction_state_for_removed_node** — `remove_node_from_tree` @ `packages/blitz-dom/src/document.rs:901`, `process_removed_subtree` @ `packages/blitz-dom/src/mutator.rs:1068`. Signature kept.
- **add_children_to_parent** — `append_children` @ `packages/blitz-dom/src/mutator.rs:707`, `insert_nodes_before` @ `:714`. Private, signature kept.

## Patterns detected
- **Partial take by no-commit cherry-pick** (`.andromeda/architecture.md:278`): the last sync took `fb8d6c15e` alone as `275950dc0` with its `(cherry picked from commit …)` trailer, the merge base unmoved and the Taffy commits named for the next sync.
- **State changes through `snapshot_node_and`** (`packages/blitz-dom/src/document.rs:1635-1649`): a state bit flipped under a snapshot when a style rule depends on it; the changed-set mark keyed on `FOCUS | CHECKED` alone. Upstream's `FOCUS_WITHIN` writes take this path, so they snapshot for restyle and do not mark.
- **Upstream test files as auto-discovered targets** (`tests/blitz-tests/Cargo.toml:48-59`; `.github/scripts/test_blitz_tests_targets.py:18-25`): a merged file becomes its own target with no manifest line; `all.rs` takes upstream's `mod` lines and stays `test = false`.
- **Headless documents carry no shell** (no `shell_provider` set in `packages/blitz-test-harness/src`, `packages/escher-driver/src`, `examples/seven_guis/src` — §Sweeps): a `request_redraw()` is `ShellProvider`'s default no-op (`packages/blitz-traits/src/shell.rs:12`).

## Conventions to follow
- **Coupled pins move as one set** (CLAUDE.md Critical Warnings; `.claude/rules/security.md` §Dependencies): Taffy's git line comes with its lock lines or not at all.
- **A conflict resolution is named and minimal** (`.andromeda/architecture.md` §Established Decisions, the 2026-10-10 sync's one recorded upstream-code edit): the `mutator.rs` resolution keeps both sides' lines in their order and is recorded as the take's non-additive edit.
- **Moved counts are attributed by name** (`.andromeda/test-plan.md:330`): every result-line and passed delta named to its file; the two 2026-10-19 chunks' moves since CI#39111579925 are attributed beside upstream's.

## New files to create
- `tests/blitz-tests/tests/focus_within.rs` — upstream's file from `38ca4e777`, byte-identical (where the take carries `38ca4e777`)
- `tests/blitz-tests/tests/focus_redraw.rs` — upstream's file from `38ca4e777`, byte-identical (where the take carries `38ca4e777`)
- `escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/` — the take's readings (lock, counts, sink, merge-surface rows, CI tallies)

## Files to modify
- `packages/blitz-dom/src/document.rs` — upstream's `38ca4e777` hunks in `clear_interaction_state_for_removed_node`, `clear_focus`, `set_focus_to`, as written
- `packages/blitz-dom/src/events/keyboard.rs` — upstream's removal of the Tab arm's `old_focus` and redraw lines, as written
- `packages/blitz-dom/src/mutator.rs` — upstream's `focus_within_old_chain` record and transfer in `add_children_to_parent`, the conflict resolved by keeping our changed-set `if` and then upstream's declaration
- `packages/blitz-dom/src/node/node.rs` — upstream's `add_focus_within`, `remove_focus_within`, `contains_focus`, as written
- `packages/blitz-dom/src/stylo.rs` — upstream's `FocusWithin` match on `ElementState::FOCUS_WITHIN`, as written
- `tests/blitz-tests/tests/all.rs` — upstream's two `mod` lines, the file staying byte-identical to upstream's
- `wpt/WPT_COMMIT` — upstream's `17de049b1` pin, as written
- `Cargo.toml` — only under the merge shape: upstream's git Taffy line
- `Cargo.lock` — only under the merge shape: upstream's Taffy source line and the `cssparser-macros` dependency line
- `packages/blitz-dom/src/layout/inline.rs` — only under the merge shape: `1a11761bc`'s one added `place_floated_box` argument

## Sweeps
- Upstream's Rust delta past `fb8d6c15e` (`git diff fb8d6c15e upstream/main -- '*.rs'`), added lines matched against `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(|tracing::|debug!|trace!|info!|warn!|error!|log::|Instant::now|SystemTime::now`: 0 of 379 added lines (`grep -cE '^\+[^+]'` = 379) — no log, print, env read, subscriber, listener or wall-clock read.
- The same two test files' added lines against that pattern plus `env!`: 0 hits — the silence census rows (env · print · subscriber) read 0 · 0 · 0 per file.
- `Instant::now()|SystemTime::now()` in `packages/blitz-dom/src` outside `clock.rs` (`git grep`): HEAD none · trial-merged tree (`git merge-tree --write-tree HEAD upstream/main`) none.
- `git diff fb8d6c15e upstream/main -- packages/blitz-dom/assets/default.css`: 0 lines — the user-agent stylesheet is untouched.
- `focus-within` over `examples/seven_guis/` (`command grep -rc`): 0 files with a hit — the stand styles nothing on it.
- `focus-within|focus_within|FOCUS_WITHIN` over `.rs`/`.css`/`.html` in `examples packages tests apps` (`command grep -rn`): 5 hits, all in `examples/assets/servo.css` and `apps/readme/assets/github-markdown.css` · 0 changed · 5 no-change (upstream's demo and readme stylesheets; no escher check reads them).
- `shell_provider` over `packages/blitz-test-harness/src packages/escher-driver/src examples/seven_guis/src packages/dioxus-native-dom/src` (`command grep -rn`): 1 hit (`packages/dioxus-native-dom/src/mutation_writer.rs:259`, a sub-document inheriting its parent's provider) · 0 changed — no headless document sets a shell.
- `impl ShellProvider for` over `packages tests examples apps` (`command grep -rln`): 6 files · 0 changed — `document.rs`'s `zoom_tests` and `mutator.rs`'s `mutator_requests_redraw_only_after_mutation` count redraws on zoom and on a mutation (no focus move); `interaction_state_teardown.rs` records IME only; `pointer_leave.rs` cursors; blitz-shell and blitz-traits the providers themselves.
- `is_inert|is_focussable` over `packages/blitz-dom/src/document.rs`, `events/*.rs`, `mutator.rs`: 5 hits · 0 changed — the focus filters live in the callers (`document.rs:1745`, `:1753`, `mutator.rs:1042`) and the flush sites (`mutator.rs:345`, `:483`); `set_focus_to` reads neither, so upstream's refusal sits beside the `inert` rule, not inside it.

## Scope premise closure
Every `[inferred]` bullet of `scope.md` re-read against the reads above:
- The conflict caveat ("a clean or resolved textual merge can still fail to build or move a reading") — VERIFIED as the open case it is: `focus_redraw.rs` asserts exact redraw counts our fork's other redraw sites could move (§Open questions). scope.md amended to say so.
- Upstream's two new test files run — VERIFIED: auto-discovery kept, `all.rs` `test = false`, both files compile against our API (`config.rs:47`; `keyboard-types` dev-dependency). scope.md amended (auto-discovered, no manifest line).
- CARRY 3, the files `38ca4e777` reaches and F1's focusability — VERIFIED: `node.rs`, `events/keyboard.rs`, `document.rs`; `set_focus_to` reads no `inert`, the filter is its callers'. scope.md amended.
- CARRY 4, no wall-clock read added — VERIFIED (§Sweeps).
- Engine behaviour our checks rest on — VERIFIED and extended: stand CSS and `default.css` untouched; the snapshot reads the tree's focus; `FOCUS_WITHIN` writes do not mark the changed set; the added redraws are a headless no-op. scope.md amended.

## Open questions
- Do upstream's four `focus_redraw.rs` counts hold on our engine — in particular `tabbing_to_the_next_node_requests_a_redraw`'s exactly one redraw on a Tab `KeyDown` through `handle_ui_event`, where our fork's activation and changed-set paths sit on the same event? → blocks: implementation-scope (a red count is a gate failure; its fix is a named non-additive edit or a recorded reading, never a loosened assertion).
- The pin shape — merge `17de049b1` (git Taffy back in the root manifest), cherry-pick `38ca4e777` and `17de049b1` alone, or a narrower take → blocks: plan-decision (P4).
