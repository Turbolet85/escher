# Report — 2026-10-31-engine-debts

**Chunk:** Engine debts — the six recorded debts, each the founder's fork
**Date:** 2026-10-31
**Commits:** (`git log --since=2026-10-27T12:45:06Z --format='%h %s'`) `40ac6ddac` fix(2026-10-31-engine-debts): the free-disk step for stand-repeat · `42eed99a6` chore(2026-10-31-engine-debts): operator pre-CI commit, for the run this chunk's verdict reads · `2109f1c2e` chore(records-door): commit B, the friction family moves into the store · `f5564ef96` chore(records-door): commit A, the .gitignore fragment for the store · `90968642c` chore(records-door): the bookkeeping at chunk 3's close, so the ledger at HEAD is the ledger on disk · `a6c6a37c7` the prior chunk's wrap. The chunk's own base is `2109f1c2e`. `evidence/ci.md`'s second half, the implement outcome, the handoff and this wrap's files are uncommitted and ride this wrap (inputs#I13).

## Changes (structured — detectors read this)

- **Files:** 47 changed against `2109f1c2e` (`gate.py scope`: changed 47 · listed 43 · recorded 4; `cites.py added`: 3,597 lines in 202 ranges). Every coordinate of added text is the section `## New text, by line` below.
  - **Engine (`blitz-dom`):** `src/scrolling.rs` · `src/document.rs` · `src/node/node.rs` · `src/node/select.rs` · `src/mutator.rs` · `src/accessibility.rs` · `src/events/keyboard.rs` · `src/events/pointer.rs` · `src/form.rs` · `assets/default.css`.
  - **Other engine crates:** `blitz-vibey-script/src/runtime/events.rs` · `blitz-vibey-script/tests/dom.rs` · `blitz-shell/src/event.rs` · `dioxus-native-dom/src/{snapshot,dioxus_document,lib}.rs`.
  - **Driver:** `escher-driver/src/{execute,lint,refusal,schema,lib,schema_tests}.rs`.
  - **Stand and records:** `examples/seven_guis/src/tasks/cells.rs` · `docs/capabilities.md` · `.github/scripts/ci-leg.sh` · `.github/scripts/test_ci_workflows.py` · `.github/workflows/ci.yml` · `Cargo.lock`.
  - **New checks (5):** `tests/blitz-tests/tests/{visible_region_transformed,hit_non_invertible_transform,open_state,checkable_change_event,text_range_no_change_event}.rs`.
  - **Edited checks (13):** `transformed_client_rect` · `stand_act_transformed` · `inert_subtree` · `accessibility_states` · `select_interaction` · `stand_act_select` · `stand_lint` · `stand_keyboard` · `stand_contrast` · `stand_load_page` · `stand_cells_served` · `pseudo_element_flow_classification` · `stand_non_lean_tasks`; and `tests/common/mod.rs`.
  - **Chunk records:** `evidence/{counts,readings,census,sink,merge-surface,ci}.md` · `scope-record.md` · `inputs/`.

- **Symbols / APIs (engine changes for every document, each ratified by the founder's own word, 2026-10-31, relayed verbatim by the operator; inputs#I8, inputs#I9):**
  - **F1, «Починить (Рекомендую)».** `BaseDocument::visible_region` keeps its signature and reads each clipping box through its transforms, via a new private `padding_box_in_view` (`scrolling.rs:790-812`, head :793) over `transformed_bounds`, now `pub(crate)` (`document.rs:2574-2578`). With no transform on the chain it reads as before. A mapped box with a non-finite edge answers `None`. Exact under a translate and a scale; under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show. Callers unchanged: `execute.rs` `in_view` and `crop`, `lint.rs` `clip_region`. `physical_unrounded_geometry`, `Node::absolute_position` and `scroll_into_view` are not edited.
  - **F2, «Починить (Рекомендую)».** `Node::hit_inner` (`node/node.rs:1472-1477`): a transform whose determinant is 0 or not finite answers `None` for the whole subtree.
  - **F3, «Снимать фокус без blur (Рекомендую)».** `DocumentMutator::flush` (`mutator.rs:1051-1063`): where `focus_node_id` resolves to a node reading `is_inert()`, it calls `BaseDocument::clear_focus`. No DOM `blur` or `focusout` fires, and no `inert` arm joins the attribute writes, so a subtree moved under an inert box is covered too. `disabled` and `hidden` read as before.
  - **F4, «Откат на окно (Рекомендую)».** `build_accessibility_tree` (`accessibility.rs:104-110`, `:122`): `focus` is written only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window.
  - **F5, «Писать на checkable-ролях (Рекомендую)».** `build_accessibility_node` (`accessibility.rs:199-224`): `is_native_checkable && is_checkable(role)` writes `Toggled::True` or `False` from `checkbox_input_checked()`. Under any other role a native control writes nothing, so `pressed` never begins to read on it. Neither author attribute is read on a native control.
  - **F6, «Построить сейчас».** A `multiple` select, in the keys, pointer, form, tree, sheet, snapshot and bridge.
    - **The rule types and rules (`node/select.rs`):** `SelectCursor` (:56-65, head :58), `RowGesture` (:67-78, head :69), `Gestured` (:80-89, head :82), `first_chosen_option` (:200-205), `last_chosen` (:269-272), `spanned` (:306-315), `gestured` (:317-358); `stepped` takes the row it steps from.
    - **`document.rs`:** public `select_values` (:736-744, head :737) and `clear_select_typeahead` (:752-755, head :753); a `pub(crate)` field `select_cursor`; `select_value` now reads a `multiple` select's first chosen option in tree order; `set_select_choice` takes the chosen set.
    - **`events/keyboard.rs`:** `press_multiple_select_key` (:213-244), `select_rows` (:334-349), `apply_row_gesture` (:351-380), `select_text_stands` (:326-332), and `apply_select_choice`, which takes a set and stays the one write path. ArrowDown · ArrowUp · Home · End alone choose that row alone; with Shift they span from the anchor; with Ctrl or Meta they move the current row and change nothing. A Space with no typed text standing flips the current row, and with text standing joins it. A character with no Ctrl, Alt or Meta chooses its match alone. Disabled rows are never chosen. Ctrl+A is not modelled.
    - **`events/pointer.rs`:** `click_select_row` (:1013-1054, head :1016) behind a new `option` arm of `handle_click`. A disabled select stops it; it focuses the select and clears the typeahead and text selection; plain chooses the row alone, Ctrl or Meta flips it, Shift spans; a disabled row changes nothing.
    - **`mutator.rs`:** a `multiple` write or removal records the select (`record_select_of_multiple_write`, :1262-1271), so removal settles it to one; `set_option_selectedness` leaves a `multiple` select's other options alone.
    - **`accessibility.rs`:** `set_multiselectable` on a `multiple` select.
    - **`dioxus_document.rs`:** `multiple_select_values` (:31-45) fills `values` on `Input` and `Change`, one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set; `value` stays `select_value`.
    - **`snapshot.rs`:** a `multiple` select's rows and groups read their own client rects; `value` reads `select_value`.
  - **F7, «Построить сейчас».** `optgroup`. `options` walks a select's `option` children and its `optgroup` children's, in tree order. `is_select_option` holds under a group. `owner_select` (`node/select.rs:125-129`) and `option_disabled` (:175-190), the one rule `readings` and `form.rs:281` share. Public read-only `BaseDocument::select_of(node) -> Option<NodeId>` (`document.rs:746-750`). `maybe_record_node`'s `optgroup` arm records its select (`mutator.rs:1253-1257`). `accessibility.rs`: `optgroup` → `Role::Group`, its non-empty `label` attribute its name. A group's label is named in the tree and unpainted.
  - **F8, «Давай нарисовать в отдельном чанке» and «Только строки multiple (Рекомендую)».** No popup is built and `packages/blitz-paint/` shows no diff. Enter and Space open nothing on a single select. An option of a closed single select has no click arm and stays refused `covered`, under a group too.
  - **F10, «Шлёт движок (Рекомендую)».** `handle_click`'s checkbox and radio arms (`pointer.rs:793-799`, `:811`, `:827-836`): after `input`, a `DomEventData::Change` with the same value. A checkbox always; a radio only where it was not checked before the click. `blitz-vibey-script/src/runtime/events.rs` loses its `change` synthesis and `target_is_checkbox_or_radio`, so a page's script hears one `change`, from the engine.
  - **Sole-caller facts:** `dioxus_native_dom::closed_select_of` (`snapshot.rs:242-248`) and `is_multiple_select` (`snapshot.rs:242-252`) are public and re-exported (`lib.rs:35-38`). `closed_select_of` has two callers, the snapshot's builder and `lint.rs`; `is_multiple_select` has `closed_select_of` and `execute.rs` `locate`. `clear_select_typeahead`'s one non-test caller is `execute.rs` `focus_multiple_select`.
- **Symbols / APIs (driver and stand):**
  - **F6, `type` on a `multiple` select.** `Target` gains `multiple`. `execute.rs` `focus_multiple_select` (:557-563) focuses with no click, which would land on the row at the centre, and clears the typeahead. A text opening on a space types nothing (:235-239). No verb, argument, key name, result field, cause, session error or exit is added.
  - **F9, «Исключить закрытые option (Рекомендую)», as restated.** `lint.rs`: `Entry.in_closed_select` (`:246-247`, `:337-338`, `:347`) through `closed_select_of`, reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's. `overlaps` skips such an entry on either side (`:364`, `:368`). A `multiple` select's rows are linted as usual.
  - **Fixed `'static` text moved:** `off-screen`'s `meaning()` (`refusal.rs:83-85`, doc `:27-28`), "under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view", with its remedy, the eight causes and the names unmoved. `type`'s help (`schema.rs:239-240`) and `press`'s (`:261-263`, "a set of rows that are not adjacent needs Ctrl, which this verb does not carry"). Crate docs `lib.rs:77-81`.
  - **F13, «Одна Tab-остановка (Рекомендую)».** `cells.rs`: `cells-grid` takes `id`, `tabindex: "0"`, `aria_label: "Cells"` and an `onkeydown`, with no role. Arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`. Enter opens the editor on `current`, focused; its Enter commits, stops propagation and returns focus to the grid. One `onmounted` per data cell (676) holds handles in a map. CSS `.cells-grid:focus .cell.current { outline: 2px solid #3a5ce5; outline-offset: -2px; }` (`:551-554`), the accent's held value. `common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`.
- **Crates / modules:** no crate added or removed. Added `#[cfg(test)]` modules: `blitz-shell/src/event.rs` (`:128-172`), its first unit test. `form.rs`'s test module already stood and gained two tests.
- **Dependencies:** `pulldown-cmark` 0.13.4 → 0.13.5 in `Cargo.lock` alone (`:6505`, `:6507`), RUSTSEC-2026-0414's fix, reached through `rdme`, which never sets `ENABLE_WIKILINKS`, so no reader was exposed (`apps/readme/src/markdown/pulldown_cmark.rs:7-12`, plan step 21). Semver-compatible, no coupled pin, `deny.toml` unmoved, the operator's call and not put to the founder (inputs#I11, inputs#I12). None added. A CI action site is added, not a crate: **Harness / gate surface**.
- **Schema / config:** `describe`'s count row stays `[12, 13, 12, 3, 8, 4, 15]`. `NODE_FIELDS` thirteen, `VERBS` twelve, eight causes, `Defect::ALL` six, fifteen session errors: all unmoved. The default sheet `assets/default.css:161-188` adds: `select:not([multiple]) > optgroup:not([hidden]) { display: contents; }`; `select:not([multiple]) > optgroup > option:checked { display: inline; }`; `select[multiple] > option, select[multiple] > optgroup > option { display: block; padding: 0 2px; white-space: nowrap; }`; `select[multiple] > optgroup { display: block; }`; and the new engine-default chosen-row pair `select[multiple] > option:checked, select[multiple] > optgroup > option:checked { background-color: #c8c8c8; color: #000000; }` (12.56:1, Chromium's inactive selected pair, measured by `select_interaction`). No scrub or redaction shape moved. No log site, print, env read, listener, wall-clock read or panic path on the added lines (entry 4 reads 0, `blitz-shell/src/event.rs` held out, its addition an inline test).
- **Spec-master edits:** none; every master and registry file equals HEAD (`cascade.py sweep --pre-pass` exit 0). The ledger's `v030-04` acceptance and `ref` are concretized and read `implemented`, and the working-route line is stamped: phase's and implement's writes.
- **Retired:**
  - `snapshot.rs`'s private `select_parent`, for `select_of`.
  - `runtime/events.rs`'s `change` synthesis and `target_is_checkbox_or_radio`.
  - The pin name `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`, renamed `…reads_inside_its_visible_region` and inverted (`transformed_client_rect.rs:183-216`).
  - `inert_subtree::removing_inert_restores_focus_hit_and_tree`'s focus-fixup limit lines, the test's name and first assertions standing.
  - `off-screen`'s transformed-box clause; the word `untransformed` leaves `packages/escher-driver` and `docs/capabilities.md` (entry 8).
  - `upstream's both-modes exception`: `pseudo_element_flow_classification`'s three tests now run `for incremental in [false, true]`.
  - No version-census row stands for any of these.
- **Counts / qualifiers moved:** each with its pattern and its master hits. A master row's `standing` figure is the tool's, from `report-patterns.toml` and `report-patterns-2.toml`. "History" names a row recording a measure at a past run, which stands.
  - `blitz-tests` `.rs` files 148 → 153, test targets 147 → 152 (`ls tests/blitz-tests/tests/*.rs | wc -l`, entry 19). `rs-files`: 19 hits · 1 changed (`test-strategy.md:54`) · 18 citation digits.
  - `stand_*.rs` files 50, unmoved (entry 28).
  - `stand-repeat`'s named targets ten → fifteen (`ci-leg.sh:58-59`, `test_ci_workflows.py:365-367`). `ten-named`: 9 hits · 6 changed (`standard-contracts.md:298`, `test-plan.md:377`, `:413`, `ci-integration.md:9`, `:16`, `quality-gates-coverage-targets.md:22`) · 3 history (`test-plan.md:176`, `ci-integration.md:48`, `:71`). `stand-repeat`: 55 hits, the 46 others naming the leg, no count.
  - Workspace tally `209 · 1125 · 0 · 12` → `214 · 1181 · 0 · 12` (entry 17; CI#41192433140 `Test [default features]`). `ws-tally`: 2 hits · 1 changed (`ci-integration.md:71`) · 1 history gaining a row beside it (`:50`).
  - The per-OS table against CI#40487650912 (`evidence/ci.md`, entry 37), each job's own new row: macos and coverage `195 · 1178 · 0 · 12`, windows `195 · 1132 · 0 · 9`, a11y `14 · 112 · 0 · 0`, stand repeat `219 · 1008 · 0 · 21`. `ci-tallies`: 2 hits (`ci-integration.md:49`, `:50`) · both history, the new row added beside.
  - `a11y` leg 14 · 85 → 14 · 112, fourteen files unmoved. `a11y-tally`: 6 hits · 4 changed (`ci-integration.md:8`, `:73`, `a11y-plan-detail/ci-integration.md:10`, `visual-design-verification.md:14`) · 2 history (`ci-integration.md:49`, `:50`).
  - `agent-run.sh run stand` `205 · 0 · 7` → `222 · 0 · 7` over 50 files; `run all` `681 · 0 · 12` (`evidence/counts.md`). `agent-run`: 2 hits · 1 changed (`test-plan.md:190`) · 1 history (`:176`).
  - Coverage reads on CI#41192433140: escher-driver 4950/5049 = 98.04 % (floor 97), dioxus-native-dom 2786/3235 = 86.12 % (floor 83), blitz-test-harness 956/1008 = 94.84 % (floor 90); floors unmoved. `coverage-read`: 11 hits · 1 changed (`quality-gates-coverage-targets.md:16`, its reads) · 10 naming the floor, no read.
  - Unit tests: blitz-dom 106 → 112, blitz-shell 0 → 1, blitz-vibey-script `dom` 30 → 31; escher-driver 100 and dioxus-native-dom 82 unmoved. `unit-counts`: 8 hits · 1 changed (`test-scope-summary.md:71`, the 30) · 2 true (`:59`, `session-lifecycle.md:11`, the 100) · 5 token (a citation or a path).
  - Target counts, each its own fact and pattern, against `evidence/counts.md`: `inert_subtree` 4 → 6 (`n-inert`: 5 hits · 4 changed `ci-integration.md:73`, `test-scope-summary.md:105`, `a11y-plan.md:55`, `a11y-plan-detail/ci-integration.md:11` · 1 history `ci-integration.md:48`); `accessibility_states` 5 → 6 (`n-a11y-states`: 4 hits · 3 changed `ci-integration.md:73`, `test-scope-summary.md:102`, `a11y-plan-detail/ci-integration.md:11` · 1 history); `select_interaction` 7 → 18 (`n-select`: 8 hits · 5 changed `input-validation.md:240`, `ci-integration.md:73`, `test-scope-summary.md:131`, `a11y-plan-detail/ci-integration.md:11`, `keyboard-navigation.md:102` · 3 history `ci-integration.md:47`, `:48`, `:70`); `stand_act_select` 5 → 10 (`n-act-select`: 10 hits · 6 changed `input-validation.md:240`, `ci-integration.md:73`, `test-scope-summary.md:131`, `a11y-plan-detail/ci-integration.md:11`, `keyboard-navigation.md:102`, `session-lifecycle.md:11` · 4 history); `stand_act_transformed` 2 → 3 (`n-act-transf`: 2 hits · 2 changed `test-scope-summary.md:118`, `session-lifecycle.md:11`); `stand_lint` 7 → 8 (`n-lint`: 1 hit · 1 changed `test-scope-summary.md:128`); `stand_keyboard` 15 → 21 (`n-keyboard`: 6 hits · 4 changed `existing-scopes.md:106`, `ci-integration.md:73`, `test-scope-summary.md:121`, `a11y-plan-detail/ci-integration.md:10` · 2 history at 14); `stand_contrast` 5 → 7 (`n-contrast`: 6 hits · 4 changed `existing-scopes.md:113`, `ci-integration.md:73`, `test-scope-summary.md:120`, `visual-design-verification.md:14` · 2 history); `stand_load_page` 2 → 3 (`n-load-page`: 4 hits · 2 changed `existing-scopes.md:135`, `test-scope-summary.md:150` · 2 history); `stand_cells_served` 4 → 5 (`n-cells`: 4 hits · 2 changed, the same two · 2 history). A "`N` tests" form these patterns do not reach: `existing-scopes.md:102` holds `inert_subtree` 4 and `accessibility_states` 5 in that form, read at its offsets below, and the detectors' own sweeps stay the control. The name-token patterns `ct-*` (rows 23 · 31 · 27 · 30 · 17 · 27 · 46 · 33 · 25 · 20) hold those rows and otherwise name a target in a list or a citation, no count: no change, `cites.py` re-pointing the digits.
  - New targets: `visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2. `non-invertible`'s control is unfired (the refusal line: `non-invertible · control UNFIRED over the seven masters (bodies and sets) at 40ac6dda`), so a master holds no statement of F2.
  - Cells' reacting-element count 676 → 677 (`stand_non_lean_tasks`): **Spec claims disproved** below. `cells-676`: 18 hits · 2 changed (`existing-scopes.md:116`, `screen-reader-support.md:38`) · 16 no change (the 676 data cells, a true count, at `standard-contracts.md:259`, `:260`, `layout-templates.md:10`, `test-scope-summary.md:101`; a target name or citation digits at the rest). `.andromeda/residuals.md:23` holds it too (its own command, `grep -n 676`), P5's.
  - `stand_lint`'s `at_boot` `[0, 1, 0, 0]` → `[0, 0, 0, 0]`. `lint-overlap`: 11 hits · 5 changed (`standard-contracts.md:209`, `layout-templates.md:60`, `test-scope-summary.md:129`, `:153`, `visual-design-verification.md:15`) · 6 no change (a closed select's options keep its client rect: `standard-contracts.md:184`, `layout-templates.md:10`, `:15`, `test-scope-summary.md:109`, `screen-reader-support.md:11`, `:45`).
  - Fixture readers 24 → 28 (`evidence/census.md`), `session_common` 31 unmoved. `fixture-readers`: 12 hits · 3 changed (`test-plan.md:151`, `test-strategy.md:53`, `obs-plan.md:92`) · 2 true at 31 (`test-plan.md:149`, `test-strategy.md:52`) · 7 naming the module.
  - Measured unmoved, each named by its reason in `evidence/counts.md`: `PARTS` 5, `FINDINGS_AT_BOOT` 490, `stand_act_ids` 68 and 9, `flight-type` 17.40, `Ran 108 tests`, 2 `info_span!` sites. No master site for any of them moves.
  - Cognitive complexity, no master site (a dev-host reading against 15): `findings` 7 · `press_multiple_select_key` 6 · `click_select_row` 6 · `overlaps` 13 · `visible_region` 11.
  - Text limits the forks moved, each moved fact's pattern and hits: `visible-region` 13 hits · 7 changed (`architecture.md:145`, `standard-contracts.md:46`, `:53`, `:169`, `:171`, `error-handling.md:9`, `test-scope-summary.md:118`) · 6 no change (the reader named: `architecture.md:143`, `standard-contracts.md:44`, `:180`, `:202`, `layout-templates.md:59`, `ci-integration.md:37`). `focus-fixup` 8 hits · 8 changed (`architecture.md:74`, `established-decisions.md:113`, `existing-scopes.md:102`, `input-validation.md:227`, `test-scope-summary.md:105`, `a11y-plan.md:107`, `a11y-scope-summary.md:23`, `keyboard-navigation.md:8`). `tree-focus` 21 hits · 1 changed (`screen-reader-support.md:52`, gaining the window fallback) · 20 no change (focus readings agreeing, true). `native-toggled` 10 hits · 9 changed · 1 no change (`standard-contracts.md:249`, the snapshot's own read). `multiple-select` 16 hits · 12 changed (`architecture.md:62`, `:135`, `established-decisions.md:77`, `standard-contracts.md:17`, `:249`, `:267`, `input-validation.md:240`, `test-scope-summary.md:24`, `a11y-plan.md:123`, `a11y-scope-summary.md:23`, `keyboard-navigation.md:38`, `screen-reader-support.md:11`) · 4 no change (`cross-cutting-patterns.md:40`, `input-validation.md:299`, `a11y-strategy.md:10`, `keyboard-navigation.md:64`). `optgroup` 10 hits · 10 changed (`architecture.md:62`, `established-decisions.md:77`, `:115`, `standard-contracts.md:267`, `input-validation.md:239`, `:240`, `test-scope-summary.md:24`, `a11y-plan.md:123`, `a11y-scope-summary.md:23`, `keyboard-navigation.md:38`). `popup` 7 hits · 6 restated beside their moved limit, "no popup is painted" staying true · 1 token (`aria-patterns-roles.md:39`). `option-click` 5 hits · 4 narrowed to a closed select's option (`architecture.md:62`, `established-decisions.md:77`, `input-validation.md:240`, `keyboard-navigation.md:25`) · 1 true (`test-scope-summary.md:131`). `change-event` 9 hits · 2 changed (`established-decisions.md:153`, `standard-contracts.md:36`) · 7 no change (the harness's input synthesis, `obs-plan.md:95`). `text-range-limit` 2 hits · 1 changed (`standard-contracts.md:36`) · 1 no change. `load-rows` 11 hits · 2 changed (`a11y-plan.md:142`, `test-scope-summary.md:150`) · 9 no change. `cells-grid` 4 hits · 4 gaining `cells-grid` (`standard-contracts.md:260`, `layout-templates.md:10`, `test-scope-summary.md:101`, `screen-reader-support.md:38`). `type-on-select` 8 hits · 5 changed (`established-decisions.md:77`, `standard-contracts.md:171`, `error-handling.md:9`, `input-validation.md:239`, `keyboard-navigation.md:38`) · 3 no change, a single select's reading. `press-ctrl` 9 hits · 9 no change: no site states `press`'s help; the limit lands as a residual line. `new-readers` 7 hits · 1 changed (`standard-contracts.md:53`, "the private `transformed_bounds`") · 6 `is_masked_input`, no change. `default-sheet` 21 hits · all no change in claim, a `default.css` citation past line 160 re-pointed by `cites.py`; the rows and pair are new text. `mounted-hazard` 2 hits · 2 no change (the handle named); the hazard is a new fact.
  - Gaps CARRY 2 closes: `no-assert-gap` 5 hits · 5 changed (`input-validation.md:186`, `keyboard-navigation.md:28`, `visual-design-verification.md:56`, `integration-test-strategy.md:8`, `a11y-plan.md:142`). `open-state` 13 hits · those three and `test-plan.md:19`, `test-scope-summary.md:34` changed · 8 no change (the match itself, or `Connection::open`, `webbrowser::open`). `redraw-route` 15 hits · 2 changed (`established-decisions.md:141`, `test-scope-summary.md:104`) · 13 no change. `flow-class` 12 hits · 5 changed (`conventions.md:53`, `existing-scopes.md:145`, `ci-integration.md:16`, `integration-test-strategy.md:8`, `test-scope-summary.md:32`) · 7 no change (a target named, or a measure at a past run). `remove-attr` 1 hit · 1 changed (`standard-contracts.md:78`).
  - Lines over 2,000 chars, a disposition per (line, fact):
    - `existing-scopes.md:102`: `native-toggled` @c453 changed · `n-a11y-states`' fact @c745 changed ("5 tests") · `focus-fixup` @c1195 changed · `n-inert`'s fact @c1254 changed ("4 tests") · `stand-repeat` @c615, @c1461 no change · `ct-inert` @c1433 no change.
    - `test-plan.md:176`: `stand-repeat` @c849, `ten-named` @c865, `ct-lint` @c1043, `agent-run` @c1382, @c1837, `n-cells` @c1499, `n-load-page` @c1526, `ct-keyboard` @c1619, `flow-class` @c1791, @c2004: history, no change.
    - `ci-integration.md:8`: `a11y-tally` @c1256 changed · the rest (`ct-*` @c779 to @c2527, `stand-repeat` @c1655, @c1958, @c2082, `redraw-route` @c1322) list or leg text, no change.
    - `ci-integration.md:48`: every offset (`stand-repeat`, `ten-named`, `ct-*`, `n-*`) history at CI#40114977936, no change.
    - `session-lifecycle.md:11`: `n-act-transf` @c5744 changed · `ct-act-transf` @c5785 history · `n-act-select` @c6140 changed · `stand-repeat` @c6592 no change · `unit-counts` @c7015 no change (100 unmoved).
    - `layout-templates.md:10`: `lint-overlap` @c2831 ids, no change · `tree-focus` @c5534 no change · `cells-676` @c6674 data cells, no change · `cells-grid` @c6732 changed.
    - `unit-test-strategy.md:34`, `standard-contracts.md:297`, `project-intent.md:9`, `logging-monitoring.md:15`: a list, a commit named or citation digits, no change.
  - Sweep reach: 44 + 12 patterns (the two fires' `total` lines: 967 rows over 74 files; 111 rows over 21 files) · 44 moved facts, 44 with a pattern (this report's own count) · owner pattern control unfired (`owner-title · control UNFIRED over the seven masters (bodies and sets) at 40ac6dda`), `owner-generic` 2 standing, neither naming this chunk (`input-validation.md:203`, `test-scope-summary.md:144`) · by line 5 lines · 15 offsets, and 1 line · 1 offset in the second fire. The trail `cascade-2026-10-31-engine-debts-pre-pass.json` holds all three calls: the refusal (exit 3) and the two fires (exit 0).
- **Dev-tool versions:** none. `rust-code-analysis-cli` read for the complexity figures, no version moved.
- **Harness / gate surface:**
  - `ci-leg.sh` `stand-repeat` names five more targets; `test_stand_repeat_leg_runs_three_rounds` names them too. `BlitzTestsTargetsTest`'s manifest and the `a11y` leg's fourteen files are unmoved. `Ran 108 tests` unmoved.
  - `.github/workflows/ci.yml:311-316`: job `Stand checks repeated` gains `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false`. A widening on the operator's word. `free-disk`'s control is unfired (`free-disk · control UNFIRED over the seven masters (bodies and sets) at 40ac6dda`), so no master states this step or the older one. That older step, once `ci.yml:386` and now `:392` to `:394` by the 6 added lines above it, is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, and the android row runs `ubuntu-latest` (`completed · skipped` on CI#41190771843's `Test (android)`, `evidence/ci.md`). Owner: P5, below.
  - The plan's entry 1 lost `.github/workflows` on the same word; its `baseline` key stands as measured then.
- **Cross-project / external claims:**
  - **CI runs:** CI#41192433140 at `40ac6ddac` `verdict: green`, 17/17, wall 1271 s, the run this chunk's verdict reads. CI#41190771843 at `42eed99a6` ended `failure`, `Stand checks repeated` the one failed job (job 124508189681, linker bus errors, `Free space left: 5 MB`, no test ran). Base tallies at CI#40487650912; the same job's last green at CI#40507800481, `a6c6a37c7`.
  - **Upstream `ebacd68bd`:** every engine change flagged upstreamable and nothing sent (`evidence/merge-surface.md`). `0980cce5f` has no hunk on `visible_region`; its `hit_inner` hunk is a textual conflict with F2 for Epoch 3's sync.
  - **RUSTSEC-2026-0414:** the advisory database, read by `ci-leg.sh audit` on 2026-10-31.
  - **`inputs.py verify`, as printed:** `inputs: 12 entries — unchanged 3 · drifted 2 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 7 · uncited 0 · unparsed 0`, fired before I13 was cited.
  - inputs#I1 · message: the operator, at invocation of /andromeda-phase, 2026-10-31 03:24Z · copy message · n/a.
  - inputs#I2 · `../additional/escher-overseer/relays/adapt-0.3.0-canvas-founder-2026-10-29.md` · copy no-repo · **DRIFT, the source's bytes moved:** the overseer ticked one checklist row (`☐` → `☑ snapped 2026-10-31 (I3, I4); the fixture is written under every option of F1`). No fact moved; re-asserted against the source's current state.
  - inputs#I3 · message: the overseer's reconnaissance of 2026-10-29 · copy message · n/a.
  - inputs#I4 · message: the founder and the overseer, 2026-10-29 · copy message · n/a.
  - inputs#I5 · message: web-fetch agent's quotes of the HTML Standard and HTML-AAM · copy message · n/a.
  - inputs#I6 · `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/accesskit_consumer-0.39.1/src/tree.rs` · copy no-repo · unchanged.
  - inputs#I7 · `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/winit-core-0.31.0-beta.3/src/event_loop/mod.rs` · copy no-repo · unchanged.
  - inputs#I8 · message: the founder, by dialog in the overseer session, 2026-10-31 07:29Z, relayed verbatim by the operator · copy message · n/a.
  - inputs#I9 · `../additional/escher-overseer/relays/engine-debts-founder-2026-10-31.md` · copy no-repo · **DRIFT, the source's bytes moved:** it gained the founder's two P5 words and the statement of the lock bump, all snapped as inputs#I12, which reads unchanged. No fact in I9 moved.
  - inputs#I10 · message: the operator, at the resume invocation, 2026-10-31 07:29Z · copy message · n/a.
  - inputs#I11 · message: the operator, at the P5 review, 2026-10-31, relaying the founder by dialog · copy message · n/a. Its fact names the P5 review card `.andromeda/runs/2026-10-31T03-24-40-phase/p5-card-1.md`, the one card, its `## Beyond the plan` rows verbatim, the word's authority: `word: "yes — apply." — the operator, 2026-10-31`; rows 2 and 4 stand also on the founder's own words, relayed verbatim (inputs#I11, inputs#I12):

    | # | Proposal | Where it lands | By whom |
    |---|---|---|---|
    | 1 | **New working-route entry:** «Select popup — a closed select opens a painted list of its options, chosen by pointer and key; the founder's fork (`{id}`)», markerless, not started | `escher-0.3.0/working-route.md`, directly after the frozen `[2026-10-31-engine-debts]` line: the last of Epoch 2, before `### Epoch 3 — The UI graph` | this chunk's wrap at its P5, as an operator-requested adaptation: phase edits no entry but its own stamp |
    | 2 | **That entry's freight:** `CARRY:` F9 is restated when options read real boxes open; `covered`'s closed-option reading ends there; a group's label and `size` are its to paint or read; the focus and key contract, reduced motion and engine colours come to him as its P4 forks; `CARRY:` playbook Boundary widening, its wrap halting once as his answer card | on that entry | the same wrap |
    | 3 | **New requirement and ledger entry** for the popup: `- {id} · Select popup — …` in `requirements.md`, `matrix.py add` minting the id, never typed (corrected at the word, inputs#I11: `v030-16` is «Records kept true»'s), `planned`, unclaimed | `escher-0.3.0/requirements.md`, `verification-matrix.json` | the same wrap |
    | 4 | **New residual line, target next:** `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver | `.andromeda/residuals.md` | the same wrap |
    | 5 | **Residual lines 31, 32, 33, 39, 40 absorbed** into `v030-04`; **line 34** absorbed but for its popup half, which rows 1 to 3 own | `.andromeda/residuals.md` | the same wrap |

  - inputs#I12 · `../additional/escher-overseer/relays/engine-debts-founder-2026-10-31.md` · copy no-repo · unchanged.
  - inputs#I13 · message: the operator, at invocation of /andromeda-wrap-session, 2026-10-31 10:45Z · copy message · n/a, snapped at this wrap: this report's Commits line and its Outcome rest on it.
- **Reverted / negative API facts:**
  - Step 15's async form for the handles (a task spawned from a key handler, `dioxus_document.rs:743`'s form) was tried and did not ship: it panicked with `RefCell already borrowed` (`events.rs:162`), so Cells calls the handles synchronously (`evidence/readings.md`).
  - `visible_region`'s first form, keeping the existing lines beside the mapped branch, read cognitive complexity 25 and did not ship; `padding_box_in_view` reads 11 with the same arithmetic in the same order for a page with no transform.
  - The plan's rejected forms were not crossed: no DOM `blur`, no `ctrl` on `press`, no `size` read, no `grid` role, `blitz-paint` unmoved.
- **Insufficient fixes (written, kept, not the remedy):** the free-disk step in `Stand checks repeated`. The defect: a runner out of disk (`Free space left: 5 MB`) on CI#41190771843. What the change did: it ran, `completed · success`, its first witness in this workflow, taking `/dev/root` from 86 G free to 109 G free on the green runner. It is not shown to be the cure: that runner had 86 GB free before the step, and the failed run has no `df` line, so a runner of another size or runner variance explains the red at least as well. Owner of the remainder: a `WATCH:` at P5, below.
- **Spec claims disproved by measurement:**
  - **Cells' count, pattern `cells-676`.** The plan's step 16, "`stand_non_lean_tasks`' 676 are read unmoved", and the masters' 676 at `existing-scopes.md:116`, `screen-reader-support.md:38` and `.andromeda/residuals.md:23`. Measured 677: the count is every element with the listener mark that reads `GenericContainer`, and step 15 gives `cells-grid` an `onkeydown` and no role, so it joins the 676 data cells (`evidence/counts.md`; the pin moved with its reason, `stand_non_lean_tasks.rs:102-103`).
  - **The operator pass's first report, pattern `free-disk`, no master site, its control unfired.** It said the free-disk action "already runs on the `matrix_test` job at `ci.yml:386`". The step stands there and does not fire: `completed · skipped` on `Test (android)`, CI#41190771843. So this workflow held no measured run of the action before the fix (`evidence/ci.md`, «A correction to the first report»).
  - **The plan's step 15, pattern `mounted-hazard`, no master states the form.** Its "`dioxus_document.rs:743`'s async form" for focusing an editor, disproved at `evidence/readings.md` §A hazard measured beside step 15.
  - **The plan's step 16, "`press` alone", no master site, a plan text.** The driver's twelve key names hold no character: `press` walks and opens the editor, `type` supplies the characters, `press` commits.
  - **The plan's step 11 and research, "its first tests" of `form.rs`, no master site.** `form.rs` already held a tests module; these joined it.
  - **The plan's step 15, "a pointer-opened editor reads as today", pattern `cells-grid`, no master site.** One path moved: **Deviations** 6a and **Outcome** F13.
  - **The masters' limits F1 to F13 move.** Those limits are stated as limits and the forks fixed them, so they are Changes above and need no disposition here.
- **Expected amendments (from plan):**
  - architecture §Established Decisions → Scrolling and input, DOM semantics; §Standard Contracts (Scrolling, selection, tree; Events; Nodes and widgets; Driver session; Mutation; CI contracts): carried, **Symbols / APIs**, **Counts / qualifiers moved**. Patterns `visible-region`, `focus-fixup`, `native-toggled`, `multiple-select`, `optgroup`, `change-event`, `type-on-select`, `new-readers`, `ten-named`, hits in the bullets above, in `architecture.md` and `architecture-detail/{established-decisions,standard-contracts,existing-scopes,conventions,cross-cutting-patterns}.md`. The eight engine changes ratified by the founder (2026-10-31), flagged upstreamable, their merge surface a line owned by Epoch 3's sync (`evidence/merge-surface.md`).
  - architecture CARRY 3, `remove_attribute` → `clear_attribute` at `standard-contracts.md:78`, with its sidecar entry: carried; `remove-attr`, 1 hit in `architecture-detail/standard-contracts.md`. `mutator.rs` holds `clear_attribute`.
  - architecture, the both-modes exception at `conventions.md:53` ended: carried, **Retired**; `flow-class`, 12 hits, 1 in `conventions.md`.
  - security-plan §Dependency Security, pulldown-cmark 0.13.5: carried, **Dependencies**; `pulldown`, 13 hits (12 in `security-plan-detail/dependency-security.md` and `architecture-detail`, and `ci-integration.md:73`), none naming the advisory.
  - security-plan §Input Validation, the `select`, `inert`, control-states and Layout and Paint rows; each limit's sentence moved: carried; `multiple-select`, `optgroup`, `focus-fixup`, `native-toggled` at `input-validation.md:222`, `:227`, `:239`, `:240`. The Layout and Paint row: `non-invertible` unfired, the fact is new text there, and `input-validation.md` carries no `visible_region` row from `visible-region`.
  - security-plan §Error Handling, `off-screen`'s text: carried, **Symbols / APIs**; `visible-region`, `error-handling.md:9`.
  - security-plan, the `open` row's `no test asserts` retired and the sidecar's `remove_attribute` spelling read: carried; `no-assert-gap`, `input-validation.md:186`. The sidecar is no swept file: the handoff of 2026-10-27 names the entry «… — the `open` attribute read by presence, `:open` matching».
  - design-system §Surface: desktop-native, the default sheet's rows and groups and the chosen-row pair: carried, **Schema / config**; `default-sheet`, 21 hits across the masters, each a citation of `default.css` or a mention of the sheet, 9 of the rows in `design-system-detail/`; the pair is new text.
  - design-system §Depth Strategy, a non-invertible transform takes no hit: carried, **Symbols / APIs** F2; `non-invertible` unfired, new text.
  - layout-templates §Surface: desktop-native, `cells-grid`, its keys and current-cell rule; the flight booker's lint reading; Cells' and the load page's Tab sequences: carried; `cells-grid` (`layout-templates.md:10`), `lint-overlap` (`:60`), `load-rows` (`:14`).
  - test-plan §1, §2, §3, §9, §10, the five targets, the file count, `stand-repeat`'s fifteen named, the counts, the select's limits now asserted, the `:open`, redraw and flow-classification gaps closed, the both-modes exception ended: carried; `rs-files`, `ten-named`, `ws-tally`, `a11y-tally`, `agent-run`, `ci-tallies`, `coverage-read`, `unit-counts`, `n-*`, `no-assert-gap`, `open-state`, `redraw-route`, `flow-class`, hits above in `test-plan.md` and `test-plan-detail/`.
  - obs-plan §3, §6, the census's new checks, `focussed node`'s re-read: carried, `evidence/census.md`, `evidence/sink.md` under **Outcome**; `fixture-readers` (`obs-plan.md:92`), `ct-*` at `obs-plan.md:76`, `:87`.
  - a11y-plan §2, §5, §6, §7, §9, native `toggled`, the fixup, `multiple` and groups' keys, the OPEN gap closed, Cells' row, the leg's tally: carried; `native-toggled`, `focus-fixup`, `multiple-select`, `load-rows`, `a11y-tally`, hits above in `a11y-plan.md` and `a11y-plan-detail/`.
- **Coverage of new surfaces:**
  - `multiple` select's keys and clicks (every document, remote pages included) → validation closed key match and one write path✓ · instrumentation n/a (no log site, entry 4) · PII n/a (the set leaves only as form entries, `input` and `change`, the snapshot's `value`, the options' tree nodes) · tests unit `node::select`, `form`, integ `select_interaction`, `stand_act_select` · a11y SC 2.1.1 keys✓, `ListBox`, `set_multiselectable`; AT `unrunnable-here` · tokens engine default `#c8c8c8`/`#000000`, the engine holding no tokens (hardcoded by design, 12.56:1 measured).
  - `optgroup` → validation closed element-name match✓ · instrumentation n/a · PII a group's `label` a name, inside the ratified name crossing · tests `select_interaction`, `stand_act_select`, `form` · a11y `Role::Group`; label unpainted, stated · tokens n/a.
  - `BaseDocument::select_of` · `select_values` · `clear_select_typeahead`, `dioxus_native_dom::closed_select_of` · `is_multiple_select` → validation `get_node`, a stale id reads none✓ · instrumentation n/a · PII n/a · tests through the checks above · a11y n/a · tokens n/a.
  - `visible_region` through transforms · `hit_inner`'s non-invertible answer → validation a non-finite edge reads `None`, a zero or non-finite determinant no hit✓ · instrumentation n/a · PII n/a · tests `visible_region_transformed`, `hit_non_invertible_transform`, `stand_act_transformed`, `transformed_client_rect` · a11y n/a · tokens n/a.
  - The focus fixup · tree focus · native `toggled` → validation closed values✓ · instrumentation no log site added; `focussed node` re-read, its needles at 0 (`evidence/sink.md`) · PII n/a · tests `inert_subtree`, `accessibility_states` through `&dyn Document` · a11y SC 2.4.3✓, AT `unrunnable-here` · tokens n/a.
  - `change` on a checkbox and a radio → validation n/a · instrumentation n/a · PII the value class `input` already carries · tests `checkable_change_event`, `text_range_no_change_event`, `blitz-vibey-script` `dom` · a11y n/a · tokens n/a.
  - `cells-grid` Tab stop, keys and `.cell.current` outline → validation closed key match✓ · instrumentation n/a · PII n/a · tests `stand_keyboard`, `stand_contrast`, `stand_cells_served`, `stand_non_lean_tasks` · a11y SC 2.1.1 · 1.4.3 · 2.4.3 asserted; role and cell semantics out by residual line 24; a person at a window `unrunnable-here` · tokens stand accent `#3a5ce5`✓.
  - Load page rows → validation n/a · instrumentation n/a · PII n/a · tests `stand_keyboard`, `stand_contrast`, `stand_load_page` · a11y SC 2.1.1 · 1.4.3✓ · tokens read through the shared `read`, no hex copied.
  - `free-disk-space` in `Stand checks repeated` → validation pinned sha✓ · instrumentation n/a · PII n/a · tests CI#41192433140 (`completed · success`) · a11y n/a · tokens n/a.

## Deviations from intent

The plan's own deviations, from the outcome, each with its justification:
1. **`stand_non_lean_tasks`' Cells reads 677, not 676.** A forecast falsified; nothing wrong in the code. The pin moved with its reason, and its doc now says "react to input".
2. **Step 15's async form is not used.** It panicked (**Reverted**). Cells calls the handles synchronously, with `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call. A comment states the lean. Measured headlessly; the windowed path read from the same lines and not run.
3. **Step 16's "`press` alone" cannot reach a value.** `press` walks and opens, `type` supplies, `press` commits.
4. **Three public items the plan does not name:** `BaseDocument::clear_select_typeahead` (without it a second `type` on a `multiple` select joins the first's text), `dioxus_native_dom::closed_select_of` and `is_multiple_select` (escher-driver has no blitz-dom dependency to name `local_name!` with; `is_masked_input` the precedent). No type, verb or field is added.
5. **`visible_region` takes the helper `padding_box_in_view`**, in place of "the existing lines run unchanged": cognitive complexity 25 → 11, same arithmetic in the same order for a page with no transform.
6. **Three small behaviours beyond the plan's letter:**
   a. A click inside a cell's own open editor returns early, so typed text stays where it was reset to the stored text before: a pointer path moves, needed for a keyboard-opened editor to keep its return to the grid.
   b. Shift with no cursor yet spans from the last chosen row (`a_set_the_markup_seeds_is_stepped_and_spanned_from_its_last_chosen_row`).
   c. `type` on a `multiple` select with a text opening on a space types nothing: no label can match, and the Space would flip a row.
7. **`form.rs` already held a tests module.** They joined it.
8. **Step 12's planted case is two overlapping rows** of the `multiple` select, reading exactly `fx-row-a` × `fx-row-b`.
9. **Extra rows not listed:** `cells_a_click_on_another_cell_commits_an_open_editor_to_its_own_cell` and `accessibility_states::a_click_and_a_space_move_a_native_control_in_the_tree`.
10. **The free-disk step,** on the operator's word: first CI run red on `Stand checks repeated`, so `ci.yml` gained the step and entry 1 lost `.github/workflows`.
11. **Entry 36 fired `--wait 1800`** on the operator's word; the plan's text writes `--wait 2400`. The verdict printed inside the bound on both firings.

Scope record, `gate.py scope` summary `scope: clean — changed 47 · listed 43 · recorded 4 (companion 3 · mechanical 0 · in-intent 0 · widening 1) · absorbed 0 · excluded 617`:
- **companion, self:** `packages/escher-driver/src/schema_tests.rs` serves `schema.rs`, pinning `type`'s help · `packages/dioxus-native-dom/src/lib.rs` serves `snapshot.rs`, re-exporting the two readers · `tests/blitz-tests/tests/stand_non_lean_tasks.rs` serves `cells.rs`, the moved pin.
- **widening, on a word:** `.github/workflows/ci.yml` serves step 18 · word: "the free-disk step. Give stand-repeat ci.yml:386's action at its pinned sha, before rust-cache, with android: true, large-packages: false, swap-storage: false. Entry 1 loses .github/workflows on this word" — the operator, 2026-10-31.

## Decisions & corrections

- **The founder's thirteen forks and two further words** (inputs#I8, inputs#I9, inputs#I12), each engine change ratified by his own word, the table in `scope.md` §The founder's word. His two words from the P5 card: «Ограничение и residual (Рекомендую)» on `press`, «В чанк с попапом (Рекомендую)» on `size`. The popup's chunk placed «Сразу после Engine debts (Рекомендую)».
- **The pause holds the next chunk, not this one** (the operator, in session, 2026-10-31, `evidence/ci.md`: "The pause holds the next chunk, not this one: this chunk runs through its push, its CI and its wrap on the founder's plan, so entry 35 pushes."). Only the founder lifts it (inputs#I9). The popup's entry lands at P5 and nothing starts it.
- **The lock bump is the operator's call,** not put to the founder, a semver-compatible fix inside `.claude/rules/security.md` §Dependencies (inputs#I11, inputs#I12).
- **A correction the operator pass made to its own report:** it said the action "already runs" at `ci.yml:386`, and the job log read `completed · skipped` (`evidence/ci.md`). A claim about a CI step read from the YAML alone stood wrong until the job's state was read.
- **An inference weakened by its own next reading:** "this chunk's five targets, near 1 GB, tipped the job over" was stated as inferred at the first firing, and the green runner's `df` (86 GB free before the step) weakens it (`evidence/ci.md` §What the green run does and does not show about the disk). Curation candidate: a red's cause read from one failing run stays unmeasured until the runner's own figures are read on both sides.
- **The operator's stop-on-red held:** "Stop and tell me if any entry is red." Entry 36's first firing stopped there and the word was asked before any fix.
- **A hazard outside this chunk's fixes:** a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in that task panics at `events.rs:162`. Measured headlessly through the harness's `poll(None)`; the windowed shell runs the same lines, read and not run. Any app meets it. No engine line moved. Curation candidate, and it holds no owner: P5.
- **A standing learning recurred:** `harness.center_of` on an inline element reads no box, so a click on a default `label` missed it (`checkable_change_event`, fixed by `display: block` in the fixture).
- **A guard refusal:** a `cat` heredoc with a file target was refused once by the Bash guard and re-issued through the Write tool (implement's outcome).
- **Entry 4's census forbids `.unwrap()` and `.expect(`** on added lines under `packages/`, tests included, so the new unit tests there use neither, unlike their neighbours.
- **The fix loop's own count:** the block read red at entries 15, 16, 18, 27, 29 at 09:25Z, on `stand_non_lean_tasks`' falsified forecast among others, and green at 09:47Z (`gate-2026-10-31-engine-debts.json`). Its normal work, recorded so a reader sees the green was reached rather than first-try.
- **Sweep hazards of this wrap's site sweep:**
  - `\b676\b`, `\b148\b`, `\b1125\b`, `\b681\b` each match citation digits (the tool's warning rows): 18 of `rs-files`' 19 hits are digits.
  - `synthesi[sz]` reads the harness's input synthesis at 7 of `change-event`'s 9 hits.
  - A count pattern `{target}.{0,4}\(?N\b` misses the form "`N` tests in both layout modes" at `existing-scopes.md:102`, read through `cascade.py window`.
  - `stand-repeat` holds 55 rows and only `ten-named` splits its count.
  - The owner title `Engine debts`, `non-invertible` and `free-disk` have unfired controls: three real zeros, each carried with its refusal line.
- **No operator correction arrived during implement's run** (its outcome's own line); the corrections above are the operator pass's.

## Outcome

**Criteria, re-asserted against the diff:**
- **(matrix) `v030-04`:** each debt reached the founder as a fork (`founder-forks.md`, inputs#I8) and ends as his word says, by the table above. Met; the status flip is P7.3's.
- **(F1, arch, design):** met. `visible_region_transformed` 6, `stand_act_transformed` 3, `transformed_client_rect` renamed and inverted, in both layout modes. Red before green at `evidence/readings.md`.
- **(F1, security):** met, `a_box_mapped_to_a_point_leaves_nothing_visible_inside_it`.
- **(F2):** met, `hit_non_invertible_transform` 2.
- **(F3, a11y) and (F4, a11y):** met, `inert_subtree` 6. What an assistive technology reads stays unmeasured, as the criterion states.
- **(F5, a11y):** met, `accessibility_states` 6.
- **(F6, a11y), (F6, the click, driver), (F7):** met, `select_interaction` 18, `stand_act_select` 10, `form.rs` +2.
- **(F8):** met. `git diff` adds no paint module and `packages/blitz-paint/` shows no diff; entry 1 includes it.
- **(F9):** met, `stand_lint` 8, `at_boot` `[0, 0, 0, 0]`.
- **(F10), (F11):** met, `checkable_change_event` 3, `blitz-vibey-script` `dom` 31, `text_range_no_change_event` 2.
- **(F12, a11y):** rows met; the `a11y` leg stays fourteen files. "The OPEN gap closes" is `a11y-plan.md:142`, this wrap's P2 amendment.
- **(F13, a11y):** the keyboard path met. **"A pointer-opened editor reads as it did" is UNMET in one path by the diff:** a click inside a cell's own open editor returns early, so typed text stays where it was reset before (`cells.rs:425-433`; **Deviations** 6a). Unlinked to the matrix text → **a P2 escalation**, its proposal: restate the criterion's reading, the behaviour kept because the keyboard path needs it.
- **(CARRY 2):** met, `open_state` 4, `blitz-shell`'s unit test stating what it cannot read, `pseudo_element_flow_classification` in both modes.
- **(closed sets, arch, obs):** met, against the diff too: `describe`'s count row green and unmoved, 2 `info_span!`, entry 4 at 0, `host_log` and `mcp_log` 0 needles under both builds. The three public items of **Deviations** 4 add no verb, key, cause, defect or session error.
- **(tests):** met, the block below.
- **(CI):** met, CI#41192433140, `stand-repeat: 3 of 3 rounds green`, `coverage floor: 3 of 3 crates at or above their floors`, per-OS tallies explained by name in `evidence/ci.md`.
- **Inputs resting on a drifted source:** F1's pan/zoom proof rests on inputs#I2, re-asserted against its current state: the ticked row names that proof. The founder's words rest on inputs#I9, whose current state adds to it and moves nothing.

**Gates, implement's last full block at 09:47Z (`entries 37 · green 32 · red 0 · recorded 1 · timeout 0 · not-run 4`, trail `.andromeda/runs/2026-10-31T08-53-14-implement/gate-2026-10-31-engine-debts.json`):** each `green`, its `exit` and atom held:
- `git diff --quiet 2109f1c2e… ` (its shorter form re-read green after the operator's word) · the silence census `grep -lE 'println!…'`, exit 1, last line 0 · `grep -rh 'info_span!('`, last line 2 · the added-lines census, exit 1, last line 0 · the clock census, exit 1, last line 0 · `grep -c -E 'DioxusLabs/taffy…'`, exit 1, last line 0 · `test -f …keyboard_compat.rs && grep -c …`, last line 2 · `git grep -l untransformed`, exit 1, last line 0.
- `cargo test -p blitz-dom --locked --lib` (112) · `-p blitz-shell --locked --lib` (1) · `-p escher-driver --locked --lib` (100) · `-p dioxus-native-dom --locked --lib` (82) · `-p blitz-vibey-script --locked --test dom` (31).
- `cargo test -p blitz-tests --locked --test visible_region_transformed …`, the five · `cargo test -p blitz-tests --locked --test transformed_client_rect …`, the forty-one.
- `bash .github/scripts/ci-leg.sh fast` · the workspace tally probe `lines 214 passed 1181 failed 0 ignored 12` · the every-target probe, exit 1, last line 0 · `ls tests/blitz-tests/tests/*.rs | wc -l`, last line 153.
- The CI-scripts tally: **`recorded`**, `Ran 108 tests`, `OK`, forecast unmoved, the leg's own verdict inside `fast`.
- `bash .github/scripts/ci-leg.sh doc` · `… audit`, `advisories ok`, `licenses ok` · the lock probe, exit 1, last line 0 · `… a11y` · the a11y tally `lines 14 passed 112 failed 0`.
- `bash scripts/agent-run.sh boot` · `run stand` `"outcome": "passed"`, `222 · 0 · 7` · `ls tests/blitz-tests/tests/stand_*.rs | wc -l`, last line 50 · `run all` `"outcome": "passed"`, `681 · 0 · 12`.
- `cargo test -p seven_guis --locked` · `cargo tree … accesskit_(xplat|winit)`, exit 1, last line 0 · `cargo build -p seven_guis --bin seven_guis_native --locked`.
- The smoke, `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124`, green, `service.name=seven_guis`. It reads the boot alone.

**Operator entries, `leg = 'operator'`, the results in `evidence/ci.md`, for P7's re-verify:**
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`: `hygiene: clean — read 606…`, green.
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.3.0`: green, twice, `42eed99a` then `40ac6dda`.
- `python -X utf8 … ci.py conclusion --sha HEAD --wait 2400`, fired `--wait 1800` on the operator's word: `verdict: red` on `42eed99a` (runner out of disk), then `40ac6ddac641 verdict: green · checks 17/17 · wall 1271 s · runs CI#41192433140 completed/success`.
- The per-job tally `gh run view <id> …`, `recorded`, exit 0, CI#41192433140: Test [default features] `214 · 1181 · 0 · 12`, Test (macos) `195 · 1178 · 0 · 12`, Coverage report `195 · 1178 · 0 · 12`, Test (windows) `195 · 1132 · 0 · 9`, Accessibility `14 · 112 · 0 · 0`, Stand checks repeated `219 · 1008 · 0 · 21`. Every move explained by name there, whose disposition: this chunk's, green.

**Watches:** none folded (`route.py`'s pins on this entry were three `CARRY:` blocks and no `WATCH`, `scope.md` §Folded freight).

**CARRYs:** CARRY 1, the reading, recorded at `evidence/merge-surface.md` §CARRY 1. CARRY 2, the three checks, above. CARRY 3, P2's amendment (**Expected amendments**).

**Outcome basis:**
- **The operator pass:** Setup 4's commit list, `git log --format='%h %s' 2109f1c2e..HEAD`: `40ac6ddac` fix(2026-10-31-engine-debts): the free-disk step for stand-repeat · `42eed99a6` chore(2026-10-31-engine-debts): operator pre-CI commit, for the run this chunk's verdict reads. The final HEAD's CI run: CI#41192433140 at `40ac6ddac`, entries 34 to 37 in `evidence/ci.md`. The gate verdicts rest on that state, with implement's block the basis for what only it holds.
- **Implement's report:** the persisted `outcome-2026-10-31-engine-debts.md` of `.andromeda/runs/2026-10-31T08-53-14-implement/`, the one implement run of this marker. This window holds no run whole, so its 10 deviations and its census rows are carried from that file, and it is superseded by the operator pass where they differ (its first "Nothing is committed" line, written before the pass).
- **An operator directive between implement and this report:** the free-disk step (**Deviations** 10, 11).
- **Post-implement artifacts:** `evidence/ci.md`, `scope-record.md`'s widening line, the plan's entry 1.

**Process hygiene:** the outcome's table: `seven_guis_native` (entry 33, twice) terminated by `timeout 10`; `escher-session` hosts stopped by `stop`, state directories removed; `escher-session` children of `host_log`, `mcp_log`, `recording` and kin reaped by their guards; `census: none running` at implement's end. Re-measured here, `gate.py census --comm escher-session --comm seven_guis_native --comm seven_guis_nati --args agent-run`: `census: none running`.

**Asks of later phases, this window not carrying them:**
- **P2:** the sampler is forced on this wrap, `detail.py sample --side wrap --force`, the operator's word (inputs#I13), as it was on the phase. Raise F13's unmet clause above as its escalation. The eight engine changes are ratified by the founder's own word (inputs#I8, inputs#I9); the boundary reading the plan left to the wrap (plan.md §Implementation notes, **Boundary reading**) is P2's to read under the playbook's engine-change and ratified-field rules, and to escalate if it reads otherwise.
- **P3:** this report's *Decisions & corrections* is the carrier.
- **P5:** the P5 card's rows 1 to 5 above, an operator-requested adaptation on the operator's word (inputs#I11), applied as those rows read: the popup entry directly after `[2026-10-31-engine-debts]`, its freight, its requirement and `matrix.py add` minting the id (`v030-16` is held), the residual line for `press`, the absorbed lines. `.andromeda/residuals.md:23`'s 676 → 677. Three findings with no owner yet, each needing one: the `MountedData` hazard; the skipped `ci.yml:392` step, its `if:` naming no row that runs; and the free-disk step's unproven cure, observe-only, a `WATCH:` reading its run on `Stand checks repeated`.
- **P6:** no failed command is open.
- **P7:** `matrix.py flip` on `v030-04`; entries 34 to 37 re-verified from `evidence/ci.md`, the final HEAD's run being CI#41192433140; `gate.py scope` re-read after the light gate. The pause holds the next chunk (Next: the wrap names no start).
## Addendum — resumed 2026-10-31
- inputs#I14 · message: the operator, at the resume invocation of /andromeda-wrap-session, 2026-10-31 11:00Z · copy message · n/a — the sampler stays forced; P5 carries the P5 card's rows 1 to 5, owners for the `MountedData` panic and the skipped step at `ci.yml:392`, 676 to 677, the `.gitignore` line for `escher-0.4.0-incubator/`, the `remove_attribute` rename at `standard-contracts.md:78`, and CLAUDE.md's and the rules' old limits; `evidence/ci.md` gains the runner images, read from every job's log of both runs (§The runner images): on CI#41190771843 `Stand checks repeated` alone among the `ubuntu-24.04` jobs ran on image `20261018.96.1`, agent `20261015.610`; the others, and that job on CI#41192433140, read `20261025.353.1`, agent `20261021.618`; the Windows and macOS jobs read images of their own; P5 comes to the operator as a card.
  - bears on **Insufficient fixes (written, kept, not the remedy):** adds to it: the red job stood on an older image than every other `ubuntu-24.04` job. That image's free disk stays unread, so the cure stays unproven.
  - bears on **Asks of later phases, this window not carrying them:** adds to it: the `.gitignore` line for `escher-0.4.0-incubator/` and the `remove_attribute` rename, and the leaves' old limits checked before the card.
- inputs.py verify, re-fired: `inputs: 14 entries — unchanged 3 · drifted 2 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 9 · uncited 1 · unparsed 0`; moved since the report: uncited 1 → 0, I14 cited here. I2 and I9 stay drifted as the report states; no verdict moved.
- inputs#I15 · message: the operator, by dialog at the wrap P2 classifier halt, 2026-10-31 12:53Z · copy message · n/a — classifier `obs-plan-cls-log-coverage.b1` died twice at Haiku · max, its response past the 128000 output token maximum; on this word its answer file is written by hand, all 25 entities `yes · stale`, each clause saying no classifier answered, and read through `fanout.py check` (1 of 1 pass, `detail.py answered` PASS).
  - no fact above rests on it
- inputs#I16 · message: the operator, by dialog at P2 Validate, 2026-10-31 13:13Z · copy message · n/a — the answer card confirmed as the operator's reading of the founder's fork texts, not a new word of his: F6 B, F7 B, F10 A; a group's `label` as a name stands on the P5 lean, not F7's text; the F13 clause restated, the behaviour kept, the criterion the plan's and F13 A saying nothing of it.
  - bears on **Outcome**: adds to it: (F13, a11y)'s unmet clause is settled by this word.
- inputs#I17 · message: the operator, by dialog at the wrap P5, 2026-10-31 13:52Z · copy message · n/a — the P5 route card applied as written: the popup entry and `v030-17`, the `press` residual, the owners on four route lines, the two matrix notes at P7.3.
  - bears on **Asks of later phases, this window not carrying them:** adds to it: residual lines 23, 31 to 34, 39 and 40 already read `absorbed:`, so their corrections are matrix notes; the `.gitignore` line waits for the popup's wrap.

## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 2109f1c2 (the parent of the oldest pre-CI commit 42eed99a) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/ci-leg.sh — added 2 line(s) in 1 range(s)
added: 58-59
### .github/scripts/test_ci_workflows.py — added 3 line(s) in 1 range(s)
added: 365-367
### .github/workflows/ci.yml — added 6 line(s) in 1 range(s)
added: 311-316
### Cargo.lock — added 2 line(s) in 2 range(s)
added: 6505 · 6507
### docs/capabilities.md — added 41 line(s) in 9 range(s)
added: 81-82 · 109-111 · 134-139 · 141-149 · 151-153 · 182-184 · 194-199 · 208-211 · 321-325
- 134-137 «- **a `multiple` `select` model**: its rows have boxes of their own and take a click, plain, with Ctrl or M…»
- 138-139 «- **`optgroup`**: its options are in its select's list, chosen by keys and text, a disabled group's rows sk…»
- 143-144 «- **the tree's focus** is the window when the focused node was not built, so the tree never names a node it…»
- 148-149 «- **a transform** moves what a box clips: `visible_region` reads a clipping box where its transforms show i…»
- 182-184 «- **An element in a rotated box's corner.** Under a rotation a box's visible part reads as the bounding box…»
- 194-195 «- **The focus fixup fires no `blur`.** A node that holds focus when it comes under `inert` gives focus up a…»
- 196-198 «- **The `select` model's limits.** No popup is painted, and Enter and Space open nothing on a single select…»
- 208-211 «- **The `select` model's other limits.** `size` is unread, so a `multiple` select lists every row; a group'…»
- 321-325 «- **`stand-repeat`**: the `stand_*` files, fifteen named blitz-tests targets and seven_guis' eight process …»
### examples/seven_guis/src/tasks/cells.rs — added 83 line(s) in 12 range(s)
added: 3 · 353-357 · 362-393 · 408 · 414-423 · 425-433 · 436-437 · 445-449 · 452-456 · 460-461 · 464-469 · 551-555
- 551-554 «.cells-grid:focus .cell.current {»
### packages/blitz-dom/assets/default.css — added 28 line(s) in 1 range(s)
added: 161-188
- 161-164 @162 «select:not([multiple]) > optgroup:not([hidden]) {»
- 166-168 «select:not([multiple]) > optgroup > option:checked {»
- 173-177 «select[multiple] > optgroup > option {»
- 179-181 «select[multiple] > optgroup {»
- 184-187 «select[multiple] > optgroup > option:checked {»
### packages/blitz-dom/src/accessibility.rs — added 39 line(s) in 8 range(s)
added: 104-110 · 122 · 164-165 · 173-177 · 199-202 · 205-212 · 214-224 · 517
### packages/blitz-dom/src/document.rs — added 46 line(s) in 10 range(s)
added: 340-341 · 558 · 720-721 · 726-730 · 732 · 736-756 · 768-772 · 775-777 · 780 · 2574-2578
  - 736-744 @737 «pub fn select_values(&self, select: NodeId) -> Vec<String> {»
  - 746-750 @748 «pub fn select_of(&self, node: NodeId) -> Option<NodeId> {»
  - 752-755 @753 «pub fn clear_select_typeahead(&mut self) {»
### packages/blitz-dom/src/events/keyboard.rs — added 132 line(s) in 7 range(s)
added: 6 · 114-119 · 184-186 · 193-253 · 326-384 · 388 · 392
- 197-206 @198 «fn select_step(key: &Key) -> Option<SelectStep> {»
  - 199-205 «match key {»
- 208-211 @209 «fn types_into_select(modifiers: Modifiers) -> bool {»
- 213-244 @218 «fn press_multiple_select_key(»
  - 228-240 «let gesture = match (&event.key, select_step(&event.key)) {»
  - 241-243 «if let Some(gesture) = gesture {»
  - 326-332 @327 «fn select_text_stands(&self, select: NodeId) -> bool {»
  - 334-349 @336 «pub(crate) fn select_rows(»
  - 351-380 @353 «pub(crate) fn apply_row_gesture(»
### packages/blitz-dom/src/events/pointer.rs — added 75 line(s) in 7 range(s)
added: 21-24 · 793-799 · 811 · 827-829 · 831-836 · 889-899 · 1013-1055
  - 21-24 «node::{»
- 1013-1054 @1016 «fn click_select_row(»
  - 1023-1029 «if doc»
  - 1032-1038 «generate_focus_events(»
  - 1040-1042 «let Some(index) = options.iter().position(|option| *option == row) else {»
  - 1043-1049 «let gesture = if mods.intersects(Modifiers::CONTROL | Modifiers::META) {»
  - 1051-1053 «if !disabled {»
### packages/blitz-dom/src/form.rs — added 129 line(s) in 2 range(s)
added: 281 · 491-618
  - 494-549 @496 «fn select_entries(attributes: Vec<Attribute>, rows: &[Row<'_>]) -> Vec<(String, String)> {»
  - 551-556 «fn entries_of(values: &[&str]) -> Vec<(String, String)> {»
  - 558-592 @559 «fn a_multiple_select_submits_its_set_less_its_disabled_rows() {»
  - 594-617 @595 «fn a_single_select_submits_its_choice_through_a_group() {»
### packages/blitz-dom/src/mutator.rs — added 43 line(s) in 8 range(s)
added: 325-328 · 489-492 · 1051-1063 · 1253-1257 · 1262-1272 · 1274-1276 · 1282-1283 · 1291
  - 1262-1271 @1264 «fn record_select_of_multiple_write(&mut self, node_id: NodeId) {»
### packages/blitz-dom/src/node/node.rs — added 6 line(s) in 1 range(s)
added: 1472-1477
### packages/blitz-dom/src/node/select.rs — added 341 line(s) in 17 range(s)
added: 1-4 · 56-90 · 96-98 · 101-138 · 140-161 · 163-166 · 175-191 · 200-206 · 213 · 269-281 · 306-359 · 417 · 419
       422-425 · 427-430 · 432-512 · 539-590
- 56-65 @58 «pub(crate) struct SelectCursor {»
- 67-78 @69 «pub(crate) enum RowGesture {»
- 80-89 @82 «pub(crate) struct Gestured {»
- 101-123 @102 «fn select_holding(node: &Node) -> Option<NodeId> {»
  - 104-106 «if !is_option && !node.data.is_element_with_tag_name(&local_name!("optgroup")) {»
  - 108-110 «if parent.data.is_element_with_tag_name(&local_name!("select")) {»
  - 111-117 «if !is_option»
  - 119-122 «group_parent»
- 125-129 @127 «pub(crate) fn owner_select(doc: &BaseDocument, node: NodeId) -> Option<NodeId> {»
- 131-134 @132 «pub(crate) fn is_select_option(node: &Node) -> bool {»
  - 140-142 «let Some(node) = doc.get_node(select) else {»
- 175-190 @177 «pub(crate) fn option_disabled(node: &Node) -> bool {»
  - 178-181 «let carries = |node: &Node| {»
  - 182-189 «carries(node)»
- 200-205 @201 «pub(crate) fn first_chosen_option(doc: &BaseDocument, select: NodeId) -> Option<NodeId> {»
  - 202-204 «options(doc, select)»
- 269-272 @270 «pub(crate) fn last_chosen(options: &[OptionReading]) -> Option<usize> {»
- 306-315 @307 «pub(crate) fn spanned(options: &[OptionReading], from: usize, to: usize) -> Vec<usize> {»
  - 309-314 «options»
- 317-358 @320 «pub(crate) fn gestured(»
  - 326-357 «match gesture {»
  - 438-457 @439 «fn a_step_moves_from_the_row_it_is_given() {»
  - 459-477 @460 «fn a_span_lists_the_enabled_rows_between_two_either_way_round() {»
  - 539-558 @541 «fn select_of_rows(rows: &[(&str, Option<usize>)]) -> (BaseDocument, NodeId, Vec<NodeId>) {»
  - 560-589 @561 «fn the_list_of_options_runs_through_a_group_in_tree_order() {»
### packages/blitz-dom/src/scrolling.rs — added 33 line(s) in 5 range(s)
added: 739-742 · 771 · 773-774 · 777-778 · 790-813
  - 790-812 @793 «fn padding_box_in_view(&self, box_id: NodeId) -> Option<kurbo::Rect> {»
### packages/blitz-shell/src/event.rs — added 46 line(s) in 1 range(s)
added: 127-172
- 128-172 @129 «mod tests {»
  - 138-142 «impl EventLoopProxyProvider for CountedWakes {»
  - 144-171 @149 «fn a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once() {»
### packages/blitz-vibey-script/src/runtime/events.rs — added 2 line(s) in 2 range(s)
added: 4 · 154
### packages/blitz-vibey-script/tests/dom.rs — added 40 line(s) in 1 range(s)
added: 362-401
- 362-370 @363 «fn radio_click(doc: &ScriptDocument) -> Option<DomEvent> {»
  - 366-368 «let click = inner»
- 372-400 @373 «fn radio_click_fires_change_only_where_it_was_not_checked() {»
  - 374-391 «let mut doc = doc_from_html(»
  - 394-399 «for expected in ["input,change", "input,change,input"] {»
### packages/dioxus-native-dom/src/dioxus_document.rs — added 19 line(s) in 4 range(s)
added: 11 · 15 · 31-46 · 396
- 31-45 @33 «fn multiple_select_values(doc: &BaseDocument, target: NodeId) -> Vec<(String, FormValue)> {»
  - 34-36 «let Some(element) = doc.get_node(target).and_then(Node::element_data) else {»
  - 37-39 «if element.name.local != local_name!("select") || !element.has_attr(local_name!("multiple")) {»
  - 41-44 «doc.select_values(target)»
### packages/dioxus-native-dom/src/lib.rs — added 4 line(s) in 1 range(s)
added: 35-38
### packages/dioxus-native-dom/src/snapshot.rs — added 17 line(s) in 4 range(s)
added: 7 · 146-148 · 242-252 · 256-257
- 242-248 @244 «pub fn closed_select_of(doc: &BaseDocument, node: NodeId) -> Option<NodeId> {»
### packages/escher-driver/src/execute.rs — added 28 line(s) in 8 range(s)
added: 10 · 222 · 224-228 · 235-239 · 370-372 · 402-405 · 412 · 557-564
- 557-563 @559 «fn focus_multiple_select(harness: &mut Harness<DioxusDocument>, select: NodeId) {»
### packages/escher-driver/src/lib.rs — added 5 line(s) in 1 range(s)
added: 77-81
### packages/escher-driver/src/lint.rs — added 10 line(s) in 7 range(s)
added: 7 · 202-203 · 246-247 · 337-338 · 347 · 364 · 368
  - 337-338 «let in_closed_select =»
### packages/escher-driver/src/refusal.rs — added 8 line(s) in 3 range(s)
added: 27-28 · 83-85 · 269-271
### packages/escher-driver/src/schema.rs — added 5 line(s) in 2 range(s)
added: 239-240 · 261-263
  - 261-263 «help: "presses one key on the focused element, then settles; on a `multiple` select the \»
### packages/escher-driver/src/schema_tests.rs — added 2 line(s) in 1 range(s)
added: 99-100
### tests/blitz-tests/tests/accessibility_states.rs — added 146 line(s) in 16 range(s)
added: 16-19 · 21 · 28 · 30 · 33 · 36-37 · 274-276 · 278-315 · 318 · 320 · 323-327 · 329 · 332 · 336-339 · 342-346
       352-428
- 352-361 @353 «fn natives_page() -> Element {»
  - 354-360 «rsx! {»
- 363-384 @366 «fn native_readings(»
  - 373-377 «let toggled = tree»
  - 379-382 «let state = &snapshot»
- 386-427 @387 «fn a_click_and_a_space_move_a_native_control_in_the_tree() {»
  - 389-426 «for incremental in [false, true] {»
### tests/blitz-tests/tests/checkable_change_event.rs — new file · 136 line(s)
- 16-44 @17 «fn page() -> Element {»
  - 19-43 «rsx! {»
- 48-58 @49 «fn heard_from(harness: &mut Page, act: impl FnOnce(&mut Page)) -> String {»
  - 53-57 «after»
- 60-66 «fn checked(harness: &Page, id: &str) -> Option<bool> {»
  - 61-65 «harness»
- 68-98 @69 «fn a_click_and_a_space_on_a_checkbox_each_fire_input_then_change() {»
  - 70-97 «for incremental in [false, true] {»
- 100-119 @101 «fn a_radios_change_fires_on_an_actual_change_alone() {»
  - 102-118 «for incremental in [false, true] {»
- 121-136 @122 «fn a_labels_click_reads_as_its_controls() {»
  - 123-135 «for incremental in [false, true] {»
### tests/blitz-tests/tests/common/mod.rs — added 6 line(s) in 1 range(s)
added: 93-98
### tests/blitz-tests/tests/hit_non_invertible_transform.rs — new file · 71 line(s)
- 13-25 @14 «fn page(transform: &str, incremental: bool) -> Harness<HtmlDocument> {»
  - 15-16 «let html = format!(»
  - 17-21 «<div id="clip" style="width:600px; height:400px; overflow:hidden">»
- 33-56 @34 «fn a_box_under_a_non_invertible_transform_takes_no_hit() {»
  - 36-55 «for incremental in [false, true] {»
- 58-71 @59 «fn an_invertible_twin_is_hit_where_it_shows() {»
  - 60-70 «for incremental in [false, true] {»
### tests/blitz-tests/tests/inert_subtree.rs — added 187 line(s) in 9 range(s)
added: 5-10 · 13 · 17 · 21-24 · 182-183 · 185-209 · 211-333 · 335-357 · 359-360
- 185-201 @187 «fn focus_page() -> Element {»
  - 189-200 «rsx! {»
- 220-227 @223 «enum MadeInert {»
- 229-310 @230 «fn a_node_made_inert_loses_focus_and_takes_no_key() {»
  - 231-237 «const ROWS: [(&str, MadeInert); 5] = [»
  - 240-309 «for incremental in [false, true] {»
- 312-319 @314 «const LEFT_OUT: &str = r#"<html><body>»
### tests/blitz-tests/tests/open_state.rs — new file · 143 line(s)
- 14-24 @16 «const PAGE: &str = r#"<html><head><style>»
- 26-34 «fn boot(incremental: bool) -> Harness<HtmlDocument> {»
  - 27-33 «Harness::from_html_with(»
- 36-38 «fn open_name() -> QualName {»
- 40-52 @42 «fn matches_open(harness: &Harness<HtmlDocument>, selector: &str, mode: &str) -> bool {»
  - 47-50 «assert!(»
- 54-73 @55 «fn at_construction_open_is_read_from_the_attribute() {»
  - 56-61 «const ROWS: [(&str, bool); 4] = [»
  - 63-72 «for incremental in [false, true] {»
- 75-104 @76 «fn a_write_and_a_removal_through_the_mutator_move_it() {»
  - 79-103 «for incremental in [false, true] {»
- 106-123 @107 «fn a_summary_click_flips_its_details() {»
  - 108-122 «for incremental in [false, true] {»
- 125-143 @126 «fn a_summary_enter_flips_its_details() {»
  - 127-142 «for incremental in [false, true] {»
### tests/blitz-tests/tests/pseudo_element_flow_classification.rs — added 49 line(s) in 7 range(s)
added: 4-5 · 12 · 18 · 44-54 · 61-71 · 78-81 · 88-106
  - 44-54 «for incremental in [false, true] {»
  - 61-71 «for incremental in [false, true] {»
### tests/blitz-tests/tests/select_interaction.rs — added 553 line(s) in 3 range(s)
added: 10-18 · 25-27 · 354-894
- 355-380 @356 «fn enter_and_space_open_nothing_on_a_single_select() {»
  - 357-379 «for incremental in [false, true] {»
- 382-423 @384 «fn rows_page() -> Element {»
  - 389-422 «rsx! {»
- 425-428 @426 «fn values(harness: &Page, selector: &str) -> Vec<String> {»
- 430-445 @431 «fn focused_rows(incremental: bool) -> Page {»
  - 435-439 «assert_eq!(»
  - 440-443 «assert!(»
- 447-476 @448 «fn a_multiple_select_lays_out_its_rows_with_boxes_of_their_own() {»
  - 451-475 «for incremental in [false, true] {»
- 478-521 @479 «fn row_keys() -> [(Key, Modifiers, &'static [&'static str], u32); 17] {»
  - 480-485 «let (none, shift, ctrl, meta) = (»
  - 486-489 «let (down, up) = (»
  - 492-520 «[»
- 523-556 @524 «fn the_keys_of_a_multiple_select_choose_span_and_flip_its_rows() {»
  - 527-555 «for incremental in [false, true] {»
- 558-588 @559 «fn typed_text_chooses_its_row_alone_and_a_space_joins_it_while_it_stands() {»
  - 560-587 «for incremental in [false, true] {»
- 590-599 @591 «fn click_row(harness: &mut Page, selector: &str, mods: Modifiers) {»
- 601-640 @602 «fn a_click_on_a_row_chooses_flips_or_spans_it() {»
  - 603-614 «let rows: [(&str, Modifiers, &[&str], u32); 10] = [»
  - 616-639 «for incremental in [false, true] {»
- 642-664 @643 «fn a_disabled_multiple_select_takes_no_click_and_no_key() {»
  - 644-663 «for incremental in [false, true] {»
- 666-689 @667 «fn multiple_removed_settles_the_set_to_one_option() {»
  - 668-688 «for incremental in [false, true] {»
- 691-712 @692 «fn a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair() {»
  - 693-711 «for incremental in [false, true] {»
- 714-723 @715 «fn seeded_page() -> Element {»
  - 716-722 «rsx! {»
- 725-743 @726 «fn a_set_the_markup_seeds_is_stepped_and_spanned_from_its_last_chosen_row() {»
  - 727-742 «for incremental in [false, true] {»
- 745-779 @747 «fn groups_page() -> Element {»
  - 750-778 «rsx! {»
- 781-855 @782 «fn a_groups_options_are_chosen_by_a_single_selects_keys_and_shown_alone() {»
  - 783-791 «let keys = [»
  - 793-854 «for incremental in [false, true] {»
- 857-894 @858 «fn a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen() {»
  - 859-893 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_select.rs — added 204 line(s) in 3 range(s)
added: 9-15 · 24 · 262-457
- 263-280 @264 «fn rows_page() -> Element {»
  - 265-279 «rsx! {»
- 282-284 «fn hold_rows(incremental: bool) -> Session {»
- 286-295 @287 «fn differs(acted: &Acted, id: &str) -> Vec<Reading> {»
  - 288-294 «acted»
- 297-309 @298 «fn chosen_rows(session: &Session) -> Vec<String> {»
  - 300-308 «["fx-one", "fx-two", "fx-three"]»
- 311-352 @312 «fn a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds() {»
  - 313-351 «for incremental in [false, true] {»
- 354-379 @355 «fn a_click_on_a_row_is_accepted_and_chooses_it() {»
  - 356-378 «for incremental in [false, true] {»
- 381-408 @382 «fn a_press_with_shift_spans_the_rows() {»
  - 383-407 «for incremental in [false, true] {»
- 410-446 @411 «fn type_on_a_multiple_select_chooses_its_match_alone_with_no_stray_click() {»
  - 412-445 «for incremental in [false, true] {»
- 448-457 @449 «fn an_option_under_a_closed_selects_group_is_refused_covered() {»
  - 450-456 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_transformed.rs — added 81 line(s) in 4 range(s)
added: 8-13 · 16-17 · 21 · 106-177
- 106-136 @108 «fn zoomed_world_fixture() -> Element {»
  - 110-135 «rsx! {»
- 138-176 @139 «fn a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows() {»
  - 140-175 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_cells_served.rs — added 44 line(s) in 3 range(s)
added: 10-14 · 22-23 · 179-215
- 179-214 @180 «fn the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part() {»
  - 183-213 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_contrast.rs — added 75 line(s) in 5 range(s)
added: 13-15 · 19-20 · 23 · 25 · 638-705
- 638-645 @640 «fn assert_meets_its_ratio(harness: &Harness<DioxusDocument>, selector: &str, at: &str) {»
  - 641-644 «assert!(»
- 647-677 @648 «fn the_load_page_meets_its_ratios() {»
  - 651-676 «for incremental in [false, true] {»
- 679-704 @680 «fn cells_meets_its_ratios() {»
  - 681-687 «const ROWS: [&str; 5] = [»
  - 689-703 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_keyboard.rs — added 218 line(s) in 5 range(s)
added: 25-31 · 38 · 41 · 45-48 · 132-336
- 45-48 «use common::{»
- 132-155 @135 «fn assert_walk(harness: &mut Harness<DioxusDocument>, sequence: &[&str], mode: &str) {»
  - 137-140 «assert!(»
  - 141-147 «for index in 0..=n {»
  - 148-154 «for index in 1..=n + 1 {»
- 157-177 @158 «fn the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space() {»
  - 159-176 «for incremental in [false, true] {»
- 179-184 @180 «fn cells_on_grid(incremental: bool, mode: &str) -> Harness<DioxusDocument> {»
- 186-191 @187 «fn is_current(harness: &Harness<DioxusDocument>, id: &str) -> bool {»
  - 188-190 «harness»
- 193-200 @194 «fn cells_walks_its_stated_sequence() {»
  - 195-199 «for incremental in [false, true] {»
- 202-249 @203 «fn cells_arrows_and_enter_open_the_moved_to_cell_and_its_value_commits_by_keyboard() {»
  - 204-248 «for incremental in [false, true] {»
- 251-268 @252 «fn cells_tab_leaving_an_open_editor_commits_it() {»
  - 253-267 «for incremental in [false, true] {»
- 270-291 @273 «fn cells_a_click_on_another_cell_commits_an_open_editor_to_its_own_cell() {»
  - 274-290 «for incremental in [false, true] {»
- 293-335 @294 «fn cells_arrows_clamp_at_the_grids_ends_and_bring_a_cell_into_view() {»
  - 295-334 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_lint.rs — added 54 line(s) in 3 range(s)
added: 278-279 · 282-332 · 338
- 286-317 @288 «fn selects_page() -> Element {»
  - 289-316 «rsx! {»
### tests/blitz-tests/tests/stand_load_page.rs — added 48 line(s) in 3 range(s)
added: 5-9 · 15 · 55-96
### tests/blitz-tests/tests/stand_non_lean_tasks.rs — added 4 line(s) in 3 range(s)
added: 2 · 38 · 102-103
### tests/blitz-tests/tests/text_range_no_change_event.rs — new file · 97 line(s)
- 15-40 @16 «fn page() -> Element {»
  - 19-39 «rsx! {»
- 44-57 @45 «fn fired(harness: &mut Page, act: impl FnOnce(&mut Page)) -> (u32, u32) {»
  - 46-52 «let read = |harness: &Page, selector: &str| {»
- 59-76 @60 «fn typing_in_a_text_input_fires_input_and_no_change() {»
  - 61-75 «for incremental in [false, true] {»
- 78-97 @79 «fn a_ranges_key_and_pointerdown_fire_input_and_no_change() {»
  - 80-96 «for incremental in [false, true] {»
### tests/blitz-tests/tests/transformed_client_rect.rs — added 10 line(s) in 5 range(s)
added: 10-13 · 183 · 185 · 211-212 · 215-216
### tests/blitz-tests/tests/visible_region_transformed.rs — new file · 276 line(s)
- 21-23 «fn page(html: &str, incremental: bool) -> Harness<HtmlDocument> {»
- 25-38 @27 «fn world(transform: &str) -> String {»
  - 28-29 «format!(»
  - 30-35 «<div id="viewport" style="position:absolute; left:50px; top:30px; width:500px; height:400px; overflow:hidde…»
- 40-56 @42 «fn node_body(transform: &str) -> String {»
  - 43-44 «format!(»
  - 45-53 «<div id="viewport" style="position:absolute; left:50px; top:30px; width:500px; height:400px; overflow:hidde…»
- 58-68 @59 «fn scrolling_box(style: &str) -> String {»
  - 60-61 «format!(»
  - 62-65 «<div id="box" style="position:absolute; left:100px; top:100px; width:200px; height:100px; border:2px solid …»
- 70-76 «fn rect(harness: &Harness<HtmlDocument>, selector: &str) -> Rect {»
  - 71-74 «let seen = harness»
- 78-83 «fn region(harness: &Harness<HtmlDocument>, selector: &str) -> Option<Rect> {»
  - 79-82 «harness»
- 85-87 «fn centre((x, y, width, height): Rect) -> (f64, f64) {»
- 89-91 «fn inside(point: (f64, f64), (x, y, width, height): Rect) -> bool {»
- 93-98 @94 «fn hits(harness: &Harness<HtmlDocument>, point: (f64, f64), selector: &str) -> bool {»
  - 95-97 «harness»
- 100-109 @102 «fn assert_shown(harness: &Harness<HtmlDocument>, selector: &str, mode: &str) {»
  - 104-107 «assert!(»
- 111-123 @113 «fn assert_panned_out(harness: &Harness<HtmlDocument>, selector: &str, mode: &str) {»
  - 115-118 «assert!(»
  - 119-122 «assert!(»
- 125-130 @126 «const WORLD_ROWS: [&str; 3] = [»
- 132-150 @133 «fn a_world_under_an_untransformed_viewport_shows_a_node_and_pans_one_out() {»
  - 135-149 «for incremental in [false, true] {»
- 152-157 @154 «const NODE_BODY_ROWS: [&str; 2] = [»
- 159-175 @160 «fn a_target_in_a_node_body_under_a_transformed_world_reads_where_it_shows() {»
  - 162-174 «for incremental in [false, true] {»
- 177-199 @178 «fn a_page_with_no_transform_reads_its_padding_box_in_the_client_rects_frame() {»
  - 179-198 «for incremental in [false, true] {»
- 201-216 @202 «fn a_translate_moves_the_region_by_the_translation() {»
  - 203-215 «for incremental in [false, true] {»
- 221-244 @222 «fn a_box_mapped_to_a_point_leaves_nothing_visible_inside_it() {»
  - 224-243 «for incremental in [false, true] {»
- 246-276 @249 «fn under_a_rotation_the_region_reads_the_rotated_boxs_bounding_box() {»
  - 250-275 «for incremental in [false, true] {»
