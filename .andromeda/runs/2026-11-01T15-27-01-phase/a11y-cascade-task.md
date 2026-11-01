# Task — a11y · 2026-11-01-upstream-sync-ui-graph

## The chunk
Takes DioxusLabs/blitz `upstream/main`, measured at `e42ee1f25`, into the fork as its ninth upstream take: 33 commits to take or leave, the shape the founder's fork at P4 (a cherry-pick onto the registry pins, that plus winit beta.4, the whole merge, or nothing), each shape measured by P3 on a trial worktree.
Resolves the predicted conflicts keeping both sides' intent, with the changed set's contract, the clock invariant, the select model and its list, the hit stops and the scrolled-box bounds unchanged; any edit to upstream-owned code beyond a conflict, a gate failure or the panic fork is excluded, and each one made is named and recorded as non-additive.
Carries the folded freight: the free-disk step of `matrix_test` fixed or removed, the orphan comment in blitz-dom's `lib.rs` resolved, a `blitz-paint/font-embolden` run joined to a local gate, a controlled run of the key-press panic at `node.rs:1775` with upstream's `node.rs` read (the fix is the founder's fork), and the merge surfaces of the engine debts and the select list re-read row by row.
Re-runs and re-reads every escher proof on the take: the `fast`, `doc`, `a11y` and `audit` legs, the stand checks in both layout modes, the incremental/non-incremental oracle, fork CI with every job settled, the per-OS tallies with each moved count explained, and the sink allowlist against the taken delta.
Re-proves the a11y target (WCAG SC 2.1.1, 1.4.3, 2.4.3) through its standing checks; it builds nothing of the UI graph and adds no crate, port, env var or listener of ours.

## Reached files
- Cargo.lock · conflict surface
- Cargo.toml
- examples/seven_guis/src/tasks/cells.rs · conflict surface
- packages/blitz-dom/assets/default.css · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-dom/src/stylo.rs · conflict surface
- packages/blitz-paint/src/text.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · conflict surface
- .github/workflows/ci.yml · edits the free-disk step of the `matrix_test` job
- packages/blitz-dom/src/lib.rs · edits the orphan comment at lines 96-97
- .andromeda/residuals.md · edits the left-out-delta line and the merge-surfaces line
- .andromeda/architecture.md · edits the `Upstream sync:` line
- .github/scripts/ci-leg.sh · re-reads the `fast`, `doc`, `a11y` and `audit` legs
- .github/scripts/test_blitz_tests_targets.py · re-reads the pinned blitz-tests target list
- packages/escher-telemetry/src/format.rs · re-reads the sink allowlist
- packages/blitz-test-harness/src/contrast.rs · re-reads the `None` conditions of `Harness::text_contrast`
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads `stand_keyboard`
- tests/blitz-tests/tests/stand_contrast.rs · re-reads `stand_contrast`
- tests/blitz-tests/tests/inert_subtree.rs · re-reads `inert_subtree`
- tests/blitz-tests/tests/stand_element_ids.rs · re-reads `stand_element_ids`
- tests/blitz-tests/tests/stand_id_persistence.rs · re-reads `stand_id_persistence`
- tests/blitz-tests/tests/stand_id_edits.rs · re-reads `stand_id_edits`
- tests/blitz-tests/tests/select_popup.rs · re-reads the one row that skips the key press
- tests/blitz-tests/tests/select_popup_painted.rs · re-reads the font-embolden witness
- tests/blitz-tests/tests/accessibility_hidden.rs · re-reads `accessibility_*`
- tests/blitz-tests/tests/accessibility_roles.rs · re-reads `accessibility_*`
- tests/blitz-tests/tests/accessibility_states.rs · re-reads `accessibility_*`
- tests/blitz-tests/tests/accessibility_label.rs · re-reads `accessibility_*`
- tests/blitz-tests/tests/accessibility_description.rs · re-reads `accessibility_*`
- tests/blitz-tests/tests/accessibility_names.rs · re-reads `accessibility_*`
- tests/blitz-tests/tests/accessibility_merged_states.rs · re-reads `accessibility_*`
- tests/blitz-tests/tests/accessibility_active_descendant_unread.rs · re-reads `accessibility_*`

## Needs
- The `a11y` leg, run on the take through `bash .github/scripts/ci-leg.sh a11y`: its fourteen files, its job in `ci.yml`, and the eight of them that also run in `stand-repeat`.
- The tier-0 target as the standing checks re-prove it: focus order, Enter and Space activation, contrast, and accessibility-tree identity.
- `stand_keyboard`'s stated Tab orders (the lean tasks, `LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`, the list walks), in both layout modes, with `stand_load_page`, `stand_cells_served`, `focusability_updates`, `select_interaction` and `stand_act_select` beside it.
- `stand_contrast`'s three tables and row counts, the load page's and Cells' ratios, and the painted-pixel witnesses: a value a moved pin shifts is re-measured and recorded, never loosened.
- `Harness::text_contrast`'s `None` conditions and a stand contrast row, re-read where a take carries `6fca6308d` (`mix-blend-mode`, `isolation`), `ac6f9f582` (`background-clip: text`), `e5929f9e2` (`-webkit-text-fill-color`) or `f6edb9701` (a failed stylesheet); the visual lint's `contrast` reading goes through that same harness call.
- The changed set's contract, unchanged through the `mutator.rs` conflict and `fe33b778b`'s narrower invalidation: what marks `changed_nodes`, what does not, and the tree refresh a non-empty set drives.
- The select model and its list through the `mutator.rs` and `node.rs` conflicts: the list's keys, clicks, close paths and focus, `ComboBox` and `expanded` in the tree, `select_popup` and `stand_act_popup` asserting it, and the range input's arm beside it.
- The select list's merge surface, sixteen files, re-read row by row against the take, its colour constants on `SelectListView` and the pair `select_popup_painted` measures at the default sheet included.
- The hit stops and `inert` through the `node.rs` conflict: a hit passing through an inert element, the focus fixup at the mutator's flush, `inert_subtree`'s rows, with `0980cce5f`'s and `ebacd68bd`'s hit-testing hunks where a take carries them.
- The key-press panic at `node.rs:1775` (`position_in_containing_block`): a focused control's anonymous block dropped when an ancestor gains `hidden`, a key press panicking on any control, a controlled run at the base and a read of upstream's `node.rs`; `select_popup` skips the key press in that one row until the founder's fork is answered.
- The clock invariant through the `scrolling.rs` conflict: the keyboard and pointer timings read on the document's clock, the double click's window and the select's typeahead among them.
- `scroll_into_view` and `visible_region` through the `scrolling.rs` conflict, as Cells' arrows and a page scroll that closes a select list use them.
- The scrolled-box bounds in `writing_mode.rs` and `absolute_position` in `node.rs`, as the snapshot's `bounds` and client rects and the select list's rows read them; `ae774d7f4` (unrounded layout) can move those numbers.
- blitz-dom's default sheet through its conflict and `dee854c76` (`<picture>`'s UA rule): the hidden-content, control and select values a check reads off it keep their meaning.
- Cells through the `cells.rs` conflict: `cells-grid` as one Tab stop with its name, its arrows and Enter, the editor's commit and focus return, and its focused-cell style.
- dioxus-native-dom through the `dioxus_document.rs` conflict: stable element ids as `author_id`, the `accessibility_tree` override, the snapshot reading the tree, its text, parts and diff, and `unkeyed_actionable`, decided by `stand_element_ids`, `stand_id_persistence`, `stand_id_edits`, `stand_accessibility_ids` and the snapshot checks; `6ef4b532d` edits its mounted-events path.
- The bridge's boolean attribute handling (`is_bool_attr`, a falsy value removing `disabled`, `hidden` or `inert`), which `a26c6ddeb` and, under a whole merge, `286874cc1` reach from upstream's side.
- blitz-paint's `text.rs` conflict, `stroke_text_as` at its centre, and the painted text the plan records through it: the select list's rows, the `optgroup` header's weight on every build, a password input's bullets.
- The `font-embolden` gate: its witness `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` joins a local gate, so SC 1.4.3's painted reading on the list is read locally, not on `Test (macos)` alone.
- A moved pin (Taffy, Stylo or winit, inside upstream's own coupled set): layout, style or event numbers a stand check asserts may read new values, in both layout modes, and each is re-measured and recorded.
- Upstream's new and edited test files under a take that carries them: `accessibility_label` and `accessibility_description` are upstream's own, and any count the plan records for a test file the take changes moves with it.
- The sink allowlist as the plan records it (obs log format, §3): `blitz*` and `accesskit_xplat` targets printing only their named fields, `escher_*` values redacted, any other target dropped; the taken delta read for new call sites, targets, prints and env reads, the capture checks staying at 0 needles.
- The password masking the snapshot holds (`MASKED_VALUE`, no typed text in a snapshot field, tree node or painted frame), through the `dioxus_document.rs` and `text.rs` conflicts.
- The script runtime's `events.rs` and `bootstrap.rs` conflicts (add/add): what the plan records of script and framework exposure of keyboard and events keeps its meaning under `91ef65e21` and the resolution.

## Standing conventions
- S1 · a11y assertions are Rust tests in blitz-tests over the AccessKit tree, called after `resolve`, one stated table per attribute with its row count asserted · §3 A11y Assertion Harness Contract
- S2 · a stand or state check asserts in both layout modes · §3 A11y Assertion Harness Contract
- S3 · the roles and names a check expects are stated once in the shared module's tables, never a second role or name mapping · .andromeda/a11y-plan-detail/screen-reader-support.md
- S4 · a keyboard check reads the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step · .andromeda/a11y-plan-detail/keyboard-navigation.md
- S5 · a check reading `has_changes()` as what a step changed drains the set first · §2 A11y Strategy
- S6 · an accessibility test file joining the `a11y` leg is named in its list, the leg's count moving with it, and a repeated one also named in `stand-repeat` · .andromeda/a11y-plan-detail/ci-integration.md

## Not needed
- The NOT YET MEASURED lines on must-be-accessible critical paths, a stated POUR strategy, skip links, modal focus traps, route-change focus restoration, live regions, heading hierarchy, timeout extensions, plain language and accessible authentication: the take measures none of them and adds no surface they would cover.
- An a11y violation schema or a screen-reader test pattern: none is defined here, and the chunk writes no such format.
- Building a platform adapter or taking a windowed witness with an active assistive technology: no stand binary builds one, and this chunk adds none.
- §10 SLO Invariants & A11y Budgets, §11 A11y Anti-Patterns and §12 A11y Decisions Log: each holds no recorded intent.
- The observed-absent records in §3's assertion searches, and the fixture lang and fixture text-alternative records: past searches of slice files and fixtures this chunk's named files do not include.
- The recorded absence of `prefers-reduced-motion` and `prefers-color-scheme` rules in the searched fixtures: this chunk adds no motion or colour-scheme handling of ours.
- Accessibility work for the UI graph's coming surfaces: those entries land on the synced engine, after this chunk.
