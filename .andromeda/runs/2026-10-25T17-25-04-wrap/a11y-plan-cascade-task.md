# Task — a11y-plan · 2026-10-25-cold-agent-bar

## The session
The flight booker's two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`, CSS `.flight-type-row` · `.type-btn` · `.type-btn-active`) are retired; one `select#flight-type` (`aria-label` "Flight type") with options `flight-type-one-way` and `flight-type-return` holds the mode, so the stand states no `aria-pressed` anywhere.
The stand's a11y tables are re-pinned to the select: the lean stand's controls 15 → 14, `stand_keyboard`'s activation table 20 → 16 rows and its tests 14 → 15, `stand_contrast`'s control rows 18 → 17 and `STATE_ROWS` 19 → 18, the a11y leg 84 → 85 tests over the same fourteen files.
The stand gains two served pages beside the lean four, Cells (`boot_cells`) and a load page (`boot_load`, an in-process network stand-in answering after 200 ms), and `escher-session` serves six tasks; neither is a `LeanTask`, so neither has a Tab-order or contrast row.
Measured: the visual lint at boot reads one `overlap` on the flight booker (its two options carry the select's client rect), a driver defect left unresolved and `stand_lint` re-pinned on the founder's word; a click right after a scroll lands on the shown overlay scrollbar.
The cold-agent pipe runs six apps and its verdict gains seven fields and three reasons; the founder's one live run passed, 66 calls, 0 wrong.

## Reached files
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/stand_load_page.rs
- examples/seven_guis/src/tasks/flight_booker.rs
- examples/seven_guis/src/stand.rs
- examples/seven_guis/src/session_host.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/common/mod.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_act_states.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_act_disabled.rs
- tests/blitz-tests/tests/stand_act_ids.rs
- tests/blitz-tests/tests/stand_act_diff.rs
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_session_recording.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_accessibility_ids.rs
- scripts/cold-agent.sh
- .github/scripts/test_cold_agent.py
- escher-0.2.0/verification-matrix.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/scope-record.md

## Touches
- §5 the flight booker's Tab order: `back-btn` → `flight-type` → `flight-start` → `flight-book`, its enabled order adding `flight-return-date` (five stops, reached by Tab and ArrowDown) — every row naming `flight-one-way` or `flight-return` as a Tab stop or control is stale (keyboard-navigation.md:38 · :98).
- §5 `stand_keyboard`'s Enter and Space activation table 20 → 16 rows (`BUTTONS` 10 → 8, the two mode buttons gone), its tests 14 → 15 (keyboard-navigation.md:38 · :97).
- §5 a11y-plan.md:62 cites `stand_keyboard.rs:125-153` and `stand_snapshot_state.rs:329-365` — both files gained lines above those ranges, so the line citations moved.
- §5 every stand control lying in `tab_order` or `ENABLED_TAB_ORDER`: holds with the select alone in the tables, its options taking no focus (keyboard-harness group).
- §6 `stand_contrast`: control rows 18 → 17 (tally 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1), `STATE_ROWS` 19 → 18 (18 · 0 · 1 · 0 → 17 · 0 · 1 · 0), `TEXT_ROWS` 14 unchanged (a11y-plan.md:64, visual-design-verification.md:12).
- §6 the contrast pairs: the `flight-one-way` accent row (white on `#3a5ce5`, 5.47) and the `flight-return` row (`#444444` on `#fafafa`, 9.33) give way to `flight-type` (`#1a1a1a` on `#ffffff`, 17.40, 15px 400); the return-chosen rows read `flight-type` 17.40 and `flight-return-date` 17.40 (visual-design-verification.md:12).
- §6 the line citations into `stand_contrast.rs` in a11y-plan.md:64 (49-132, 163-182, 572-600, 602-630, 632-656) — the file gained lines inside and above them, so they moved.
- §6 the lint at boot: the lean stand no longer reads 0 findings — the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return`), the other three 0 (visual-design-verification.md:14).
- §6 state colour: the pressed mode button's `.type-btn-active` colour state is retired; the chosen mode is the select's value, not a coloured button (visual-state-color group).
- §6 focus ring: the select's `:focus` drops the outline for the `#3a5ce5` border, as the date inputs do (visual-focus-ring group).
- §7 the stand's states by attribute: the flight booker's mode buttons `aria-pressed` are gone — the stand states `aria-selected` (CRUD rows) and `aria-invalid` (date inputs) and no `aria-pressed`; the select states its choice by `selected` (a11y-plan.md:196, screen-reader-support.md:25 · :44).
- §7 the snapshot of the flight booker: `ComboBox "Flight type"` value `one-way` with two `ListBoxOption`s, selected and unselected, each at the select's client rect, in place of two buttons with a pressed reading (sr-snapshot group).
- §7 the stand's snapshot census: 14 controls (was 15); of 77 snapshot nodes a click is accepted on 68 and refused on 9 (was 70 · 7, the two options refused `covered`) — any count of stand controls or named interactive nodes moves.
- §7 the actionable-key rule: `unkeyed_actionable` is empty on six served tasks (the lean four, Cells and the load page); rows saying the stand serves or checks four tasks are stale where they mean the host's tasks (screen-reader-support.md:37; :38 and a11y-plan.md:188 to be re-read — a scope over `LeanTask::ALL` is unchanged).
- §7 the v010-17 flight pressed witness moved to the select's `selected` reading; `aria-pressed` stays pinned only on a fixture by `accessibility_states`.
- §9 the a11y leg: still fourteen files, 84 → 85 tests (ci-integration.md:10; visual-design-verification.md:14); the `stand-repeat` leg takes the two new `stand_*` files by prefix, 3 of 3 rounds green.
- Indirectly through the stand tables in the shared module (`controls`, `tab_order`, `ENABLED_TAB_ORDER`, `rendered`): any row of a11y-plan that counts or lists the stand's controls, roles or names now carries `flight-type` as `Role::ComboBox`.

## New facts
- F1 · keyboard-navigation · The flight booker's flight type is a focusable single-choice select on the stand, and `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows` pins its arrows moving the choice in both layout modes — the engine's select model now exercised on a lean stand task.
- F2 · screen-reader-support · `stand_snapshot::the_flight_type_select_holds_its_two_options` and `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected` pin the select's two options, named `one-way flight` and `return flight`, exactly the chosen one reading selected.
- F3 · visual-design-verification · The driver's visual lint reads a select's options as overlapping, since the snapshot gives each option its select's client rect; the defect is unresolved (its fix a driver change in the lint's overlap reading, owner pinned at the wrap), and `stand_lint` is re-pinned to the measured `overlap` on the founder's word.
- F4 · visual-design-verification · Right after a `scroll`, a main-button pointerdown inside the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing; on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer resting on the thumb keeps it shown (`stand_cells_served::a_click_right_after_the_scroll_lands_on_the_shown_scrollbar`).
- F5 · a11y-scope-summary · Two served pages stand beside the four lean tasks, Cells and the load page; neither is a `LeanTask`, so neither carries a Tab-order, keyboard-activation or contrast row, and their a11y reach is their own checks (keyed, `unkeyed_actionable` empty, the load page's lint 0).
- F6 · screen-reader-support · The load page's two elements are `load-fetch` (Button "Load", white on the stand accent `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`) and `load-done` (Paragraph "Loaded", `display: none` until its sheet lands, so out of the tree until then), pinned by `stand_load_page`.

## Untouched
- The builder's role derivation and HTML-to-AccessKit mapping, its ARIA state reads and the merged upstream reads: no engine crate moved, and the engine's `aria-pressed` read stands as stated.
- The engine's focusability rules, `disabled` by presence on a form control, `inert` and the missing focus fixup: no engine crate moved.
- The engine's single-choice select model (arrow, Home, End and typeahead keys, one `input` then one `change`, no popup): unchanged; the stand only begins to use it.
- Enter and Space keyboard activation and a button click's focus as engine rules: unchanged; only the stand's table of buttons moved.
- `Harness::text_contrast`, `contrast_ratio`, the 4.5 / 3.0 thresholds and the exempt rule where `enabled` reads `Some(false)`: unchanged; only the stand's rows moved.
- The accessibility check files `accessibility_hidden`, `accessibility_roles`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select` and `focusability_updates`: untouched, their counts unchanged.
- The CRUD list's keyboard region `crud-list` (named People), its `option` rows and the unread `aria-activedescendant`: unchanged.
- Hidden-content exclusion and the select-option exception from `display: none`: unchanged.
- The Dioxus bridge's 28 boolean attributes and their falsy-value removal: unchanged.
- The password and file input mask (`MASKED_VALUE`) and blitz-paint's bullets: unchanged.
- The platform adapter, its windowed AT witness (`re-carried:0.2.0`) and that no stand binary builds one: unchanged, measured again (no `accesskit_xplat` or `accesskit_winit` in the stand's tree).
- The flight booker's date inputs' `aria-invalid`, the `invalid` class and the disabled Book button (cognitive error recovery): unchanged.
- Language and direction, text alternatives, IME and the NOT YET MEASURED or NO RECORDED INTENT sections (§10, §11, §12): no session fact reaches them.
- The tree lifecycle and the changed set (`take_changed_nodes`, the shell's rebuild on a non-empty set): unchanged.
