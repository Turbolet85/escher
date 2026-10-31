# Apply — a11y-plan · part 4 of 5

## Verdicts
- fanout-a11y-plan--cascade-g4.md 1 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:13+#1
- fanout-a11y-plan--cascade-g4.md 3 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:14#1
- fanout-a11y-plan--cascade-g4.md 4 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:14#2
- fanout-a11y-plan--cascade-g4.md 5 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:15#1
- fanout-a11y-plan--cascade-g4.md 6 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:16#1, .andromeda/a11y-plan-detail/visual-design-verification.md:16#2
- fanout-a11y-plan--cascade-g4.md 7 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:16#3
- fanout-a11y-plan--cascade-g4.md 8 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:16+#1
- fanout-a11y-plan--cascade-g4.md 9 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:32+#1
- fanout-a11y-plan--cascade-g4.md 10 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:56#1
- fanout-a11y-plan--cascade-g4.md 11 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:77#1
- fanout-a11y-plan--cascade-g4.md 12 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:78+#1
- fanout-a11y-plan--cascade-g5.md 1 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:11#1, .andromeda/a11y-plan-detail/screen-reader-support.md:11#2, .andromeda/a11y-plan-detail/screen-reader-support.md:11#3
- fanout-a11y-plan--cascade-g5.md 2 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:11+#1
- fanout-a11y-plan--cascade-g5.md 3 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:11+#2
- fanout-a11y-plan--cascade-g5.md 4 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:38#1, .andromeda/a11y-plan-detail/screen-reader-support.md:38#2, .andromeda/a11y-plan-detail/screen-reader-support.md:38#3
- fanout-a11y-plan--cascade-g5.md 5 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:38+#1
- fanout-a11y-plan--cascade-g5.md 6 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:39#1
- fanout-a11y-plan--cascade-g5.md 7 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:52#1
- cascade-compare-a11y-plan.md 4 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:31+#1

## Edits
- after: .andromeda/a11y-plan-detail/visual-design-verification.md:13
  anchor: « `text_contrast` reads the declared `:hover` background after `move_mouse_to` and the `:active` one after `mouse_down_at`, the same in both layout modes; a button with no `:active` rule keeps its hover background while pressed, so the active rows of CRUD's buttons and `back-btn` read the hover pair. Three tests assert the three tables in both layout modes, so SC 1.4.3 is met on every measured non-exempt stand row; the check runs in the workspace `test` leg, per package and in the `a11y` leg (§9) (packages/blitz-test-harness/src/contrast.rs:13-23; packages/blitz-test-harness/src/contrast.rs:25-41; packages/blitz-test-harness/src/contrast.rs:56-91; packages/blitz-test-harness/src/contrast.rs:121-147; tests/blitz-tests/tests/stand_contrast.rs:55-138; tests/blitz-tests/tests/stand_contrast.rs:482-513; tests/blitz-tests/tests/stand_contrast.rs:515-565; tests/blitz-tests/tests/stand_contrast.rs:169-188; tests/blitz-tests/tests/stand_contrast.rs:207-280; tests/blitz-tests/tests/stand_contrast.rs:300-412; tests/blitz-tests/tests/stand_contrast.rs:578-606; tests/blitz-tests/tests/stand_contrast.rs:608-636; tests/blitz-tests/tests/stand_contrast.rs:706-730 — as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md and its evidence/pairs.md, the re-colour, the two tables and the three tests at escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md and its evidence/hover-active.md)»
  add: «- `stand_contrast` holds two tests beside its three tables, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` (five rows), each asserting its rows' required ratios in both layout modes, so SC 1.4.3 is asserted on the load page and on Cells as well as the lean four; the load page's colours are read through the shared `read`, no hex copied (tests/blitz-tests/tests/stand_contrast.rs:647-677; tests/blitz-tests/tests/stand_contrast.rs:679-704 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:14
  old: «its one finding at boot the flight booker's `overlap` of its select's two options (below)»
  new: «and no finding of any kind at boot, `at_boot` reading `[0, 0, 0, 0]` (below; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:14
  old: «the `a11y` leg reads 85 tests over 14 targets, `stand_contrast`'s 5 among them (§9; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  new: «the `a11y` leg reads 112 tests over 14 targets, `stand_contrast`'s 7 among them (§9; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:15
  old: «The driver's visual lint reads a select's options as overlapping, the snapshot giving each `ListBoxOption` its select's client rect, so the stand's flight booker at boot reads one `overlap` (`flight-type-one-way` × `flight-type-return`) in both layout modes, `stand_lint`'s `the_stand_at_boot_reads_its_measured_findings` reading `[0, 1, 0, 0]`; the defect is open, its fix a driver change for every document in the lint's overlap reading (`packages/escher-driver/src/lint.rs`, `overlaps`), owned by a `.andromeda/residuals.md` line (tests/blitz-tests/tests/stand_lint.rs:277-363 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  new: «The driver's visual lint leaves a closed select's options out of its overlap reading in every document, the snapshot giving each such `ListBoxOption` its select's client rect: `overlaps` skips such an entry on either side, `Entry.in_closed_select` reading the element through `closed_select_of` and never the role, so CRUD's `role="option"` rows are not read as a select's and a `multiple` select's rows are linted as usual; the stand's flight booker at boot reads no `overlap` in both layout modes, `stand_lint`'s `the_stand_at_boot_reads_its_measured_findings` reading `[0, 0, 0, 0]` (packages/escher-driver/src/lint.rs:246-247; packages/escher-driver/src/lint.rs:337-338; packages/escher-driver/src/lint.rs:347; packages/escher-driver/src/lint.rs:364; packages/escher-driver/src/lint.rs:368; tests/blitz-tests/tests/stand_lint.rs:277-363 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:16
  old: «The user-agent `select` lays out as an `inline-block` box holding its chosen option»
  new: «The user-agent single `select` lays out as an `inline-block` box holding its chosen option, a group not `hidden` reading `display: contents` and the chosen option under it `display: inline`»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:16
  old: «packages/blitz-dom/assets/default.css:155-159);»
  new: «packages/blitz-dom/assets/default.css:155-159); a `multiple` select lays out its groups and rows as blocks, each row (`padding: 0 2px`, `white-space: nowrap`) with a box of its own (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:447-476);»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:16
  old: «that user-agent pair is not measured»
  new: «that user-agent pair is not measured on a single select, while `select_interaction` measures a `multiple` select's plain and chosen rows at the user agent's pair (tests/blitz-tests/tests/select_interaction.rs:691-712)»
- after: .andromeda/a11y-plan-detail/visual-design-verification.md:16
  anchor: «- The user-agent `select` lays out as an `inline-block` box holding its chosen option, with the `input, textarea` border (`1px solid #999`), padding (`2px`) and background (`white`) (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159); that user-agent pair is not measured; the stand's one select, `select#flight-type`, replaces it by author CSS (15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10), and `stand_contrast` reads its pair at 17.40, 15px 400, against 4.5, as a control row and as a return-chosen `STATE_ROWS` row (examples/seven_guis/src/tasks/flight_booker.rs:160-172)»
  add: «- The default sheet holds an engine-default chosen-row pair: `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` read `#000000` on `#c8c8c8`, Chromium's inactive selected pair, hardcoded by design as the engine holds no tokens, 12.56:1 measured by `select_interaction` in both layout modes (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:691-712 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- after: .andromeda/a11y-plan-detail/visual-design-verification.md:32
  anchor: «- seven_guis inputs replace the outline with a border color or box-shadow on focus, the flight booker's `select#flight-type` dropping it for the `#3a5ce5` border as the date inputs do (examples/seven_guis/src/tasks/temp_converter.rs:77-81; examples/seven_guis/src/tasks/flight_booker.rs:183-186; examples/seven_guis/src/tasks/flight_booker.rs:169-172)»
  add: «- Cells shows grid focus on its current cell: `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5` at `outline-offset: -2px`, the accent's held value, within the coverage of `stand_keyboard`, `stand_contrast`, `stand_cells_served` and `stand_non_lean_tasks` (examples/seven_guis/src/tasks/cells.rs:551-554 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:56
  old: «unasserted — no test asserts `:open` (packages/blitz-dom/src/stylo.rs:510) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  new: «`open_state`'s 4 tests, in both layout modes, read the `open` attribute by presence at construction, move it through a mutator write and removal and a summary's click and Enter, and read `:open` matching (packages/blitz-dom/src/stylo.rs:510; tests/blitz-tests/tests/open_state.rs:54-143) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/visual-design-verification.md:77
  old: «so a select's options are its children in the tree and the snapshot»
  new: «so a select's options are under it in the tree and the snapshot, directly or through an `optgroup`, the exception holding under a group (packages/blitz-dom/src/node/select.rs:131-134)»
- after: .andromeda/a11y-plan-detail/visual-design-verification.md:78
  anchor: «- Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1448-1456)»
  add: «- A transform whose determinant is 0 or not finite answers `None` from `Node::hit_inner` for the whole subtree, so that subtree takes no hit, an invertible twin hit where it shows, asserted by `hit_non_invertible_transform` in both layout modes (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  old: «keeps its `option` children in the tree»
  new: «keeps its `option` children, and those under an `optgroup`, in the tree»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  old: «(the chosen option's value, nothing with none chosen)»
  new: «(the chosen option's value, for a `multiple` select its first chosen option's in tree order, nothing with none chosen)»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  old: «and its options as its children, each with its select's client rect as its `bounds`»
  new: «and a closed select's options under it, each with its select's client rect as its `bounds`»
- after: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  anchor: «- A `select` (role `ComboBox`, `ListBox` for `multiple`) keeps its `option` children in the tree as `ListBoxOption` nodes, under `display: none` too, each `selected` true or false from its chosen state and named by its non-empty `label` attribute, else its text; the snapshot reads a `select`'s `value` as `BaseDocument::select_value` (the chosen option's value, nothing with none chosen) and its options as its children, each with its select's client rect as its `bounds`; on `stand_act_select`'s page, in both layout modes, `fx-pick` reads `ComboBox`, name `Fruit` (by its `<label>`), value `b`, its three options Apple · Banana · Cherry under it with Banana selected, and `type --id fx-pick --text Cherry` names `fx-pick` with `value` in `differs` and `fx-banana` / `fx-cherry` with `selected`; on the stand the flight booker's `select#flight-type`, named `Flight type` by `aria-label`, reads `ComboBox` with value `one-way` and its two options `flight-type-one-way` and `flight-type-return` as `ListBoxOption` children named `one-way flight` and `return flight`, exactly the chosen one selected (`stand_snapshot::the_flight_type_select_holds_its_two_options`, `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected`) (examples/seven_guis/src/tasks/flight_booker.rs:66-89; tests/blitz-tests/tests/stand_snapshot.rs:118-170; packages/blitz-dom/src/accessibility.rs:164-170; packages/blitz-dom/src/accessibility.rs:184-194; packages/blitz-dom/src/document.rs:720-734; packages/dioxus-native-dom/src/snapshot.rs:144-153; packages/dioxus-native-dom/src/snapshot.rs:242-248; packages/dioxus-native-dom/src/snapshot.rs:256-261; tests/blitz-tests/tests/stand_act_select.rs:84-137; tests/blitz-tests/tests/stand_act_select.rs:139-172) — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  add: «- A `multiple` select's rows and groups read their own client rects as their `bounds` in the snapshot, unlike a closed select's options, in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:311-352) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  anchor: «- A `select` (role `ComboBox`, `ListBox` for `multiple`) keeps its `option` children in the tree as `ListBoxOption` nodes, under `display: none` too, each `selected` true or false from its chosen state and named by its non-empty `label` attribute, else its text; the snapshot reads a `select`'s `value` as `BaseDocument::select_value` (the chosen option's value, nothing with none chosen) and its options as its children, each with its select's client rect as its `bounds`; on `stand_act_select`'s page, in both layout modes, `fx-pick` reads `ComboBox`, name `Fruit` (by its `<label>`), value `b`, its three options Apple · Banana · Cherry under it with Banana selected, and `type --id fx-pick --text Cherry` names `fx-pick` with `value` in `differs` and `fx-banana` / `fx-cherry` with `selected`; on the stand the flight booker's `select#flight-type`, named `Flight type` by `aria-label`, reads `ComboBox` with value `one-way` and its two options `flight-type-one-way` and `flight-type-return` as `ListBoxOption` children named `one-way flight` and `return flight`, exactly the chosen one selected (`stand_snapshot::the_flight_type_select_holds_its_two_options`, `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected`) (examples/seven_guis/src/tasks/flight_booker.rs:66-89; tests/blitz-tests/tests/stand_snapshot.rs:118-170; packages/blitz-dom/src/accessibility.rs:164-170; packages/blitz-dom/src/accessibility.rs:184-194; packages/blitz-dom/src/document.rs:720-734; packages/dioxus-native-dom/src/snapshot.rs:144-153; packages/dioxus-native-dom/src/snapshot.rs:242-248; packages/dioxus-native-dom/src/snapshot.rs:256-261; tests/blitz-tests/tests/stand_act_select.rs:84-137; tests/blitz-tests/tests/stand_act_select.rs:139-172) — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  add: «- `evt.values()` reads a `multiple` select's set through `multiple_select_values`, one `(name, FormValue::Text(value))` form entry per chosen option on `Input` and `Change`, while `value` stays `select_value` (packages/dioxus-native-dom/src/dioxus_document.rs:31-45) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/a11y-plan-detail/screen-reader-support.md:31
  anchor: « — the input a test-only element of the shared password page `fixtures::password_named`, no stand task or app gaining one with it (tests/blitz-tests/tests/fixtures/mod.rs:37-44; tests/blitz-tests/tests/stand_mcp_masked.rs:67-143); and since that chunk two further readers of the same model, closed words only: the node keys' help, which `help` serves — each key's reading stated by the tool, the boolean model's two limits among them (a mixed toggle reads `pressed` false, a grammar or spelling error reads `invalid` true) — and `DiffNode::differs`, a changed node naming the readings that differ from before the step in node-key order, the Flight Booker's Book naming eight nodes, in both layout modes — seven that only moved, `differs` = `[bounds]`, and since 2026-10-13-stand-keyboard-harness `flight-book` itself, which the click now focuses, `differs` = `[focused, bounds]` (packages/dioxus-native-dom/src/snapshot_diff.rs:174-209; tests/blitz-tests/tests/stand_diff.rs:488-530; tests/blitz-tests/tests/stand_diff.rs:522-525) — as measured at escher-0.1.0/chunks/2026-10-12-self-description/report.md and, for `flight-book`, as measured at escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/report.md; since 2026-10-12-headless-screenshot the driver's `screenshot` is one more reader of the model's bounds and of nothing else in it — an element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen — and a shot changes nothing: no focus moves, `has_changes` stays false and the next diff is empty, in both layout modes (packages/escher-driver/src/execute.rs:473-494; tests/blitz-tests/tests/stand_act_screenshot.rs:125-150; tests/blitz-tests/tests/stand_act_screenshot.rs:269-302) — as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md»
  add: «- The tree states a native checkbox's or radio's checked state: on a checkable role `toggled` reads `True` or `False` from `checkbox_input_checked()`, neither author attribute read on a native control; under any other role a native control writes nothing, so `pressed` does not read on it; the snapshot's `checked` reads the native state from the element, and what an assistive technology reads of it is unmeasured (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:38
  old: «and the elements that react to a click and state no interactive role are counted»
  new: «and the elements that react to input and state no interactive role are counted»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:38
  old: «Circle Drawer 1, Cells 676»
  new: «Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid`»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:38
  old: «and, for the three non-lean tasks and the count, at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md»
  new: «and, for the three non-lean tasks and the count, at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md, Cells' count at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/a11y-plan-detail/screen-reader-support.md:38
  anchor: « examples/seven_guis/src/tasks/crud.rs:92 — as measured at escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/report.md) —, and, since 2026-10-14-stand-requirement-sweep, returns nothing on the three non-lean tasks too — Temperature Converter · Circle Drawer · Cells 0 · 0 · 0, in both layout modes, and 0 with Circle Drawer's dialog open, its controls reading their keys — through nine author ids added by `id:` lines alone (`temp-celsius`, `temp-fahrenheit`; `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter` and `circle-close`, the last two only while the dialog is open; `cell-{A..Z}{0..25}` on the 676 data cells and `cell-input`), no element, class, style, wrapper or order moving and no CSS selecting by them; until then it read 2 · 3 · 676; and the elements that react to a click and state no interactive role are counted on the stand, in both layout modes, and pinned as measured — 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676 (`stand_non_lean_tasks::elements_that_react_to_a_click_and_state_no_role_are_counted`); no snapshot field says that an element reacts to a click (`NODE_FIELDS` thirteen) — that reading owned by a `.andromeda/residuals.md` line (target: next) on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)» (2026-10-14), this count its starting reading (tests/blitz-tests/tests/stand_actionable_keys.rs:142-144; tests/blitz-tests/tests/stand_actionable_keys.rs:153-155; tests/blitz-tests/tests/stand_non_lean_tasks.rs:93-149; packages/dioxus-native-dom/src/actionable.rs:55-89; packages/dioxus-native-dom/src/actionable.rs:91-122; tests/blitz-tests/tests/stand_actionable_keys.rs:1-4) — as measured at escher-0.1.0/chunks/2026-10-06-id-stability-across-code-edits/report.md and, for the three non-lean tasks and the count, at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md»
  add: «- Cells' `cells-grid` takes `aria_label` `Cells` and no role; role and cell semantics stay out by `.andromeda/residuals.md` line 24, and what a person reads at a window is `unrunnable-here` — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:39
  old: «out of the tree and the snapshot while it is hidden, and no Tab-order, keyboard-activation or contrast row (§5) (examples/seven_guis/src/stand.rs:185-200; tests/blitz-tests/tests/stand_load_page.rs:35-127)»
  new: «out of the tree and the snapshot while it is hidden; it holds Tab-order, keyboard-activation and contrast rows (§5), in both layout modes: `stand_keyboard::the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` walks `LOAD_TAB_ORDER` and takes Enter and Space on `load-fetch`, and `stand_contrast::the_load_page_meets_its_ratios` reads its ratios (examples/seven_guis/src/stand.rs:185-200; tests/blitz-tests/tests/stand_load_page.rs:35-127; tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_contrast.rs:647-677) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/screen-reader-support.md:52
  old: «and the focused DOM node is reported as tree focus (packages/blitz-dom/src/accessibility.rs:13;»
  new: «and the focused DOM node is reported as tree focus, `build_accessibility_tree` writing `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build (packages/blitz-dom/src/accessibility.rs:13; packages/blitz-dom/src/accessibility.rs:104-110;»

## Patterns
[[pattern]]
id = "a11y4-01"
fixed = 'its one finding at boot'

[[pattern]]
id = "a11y4-02"
fixed = '85 tests over 14 targets'

[[pattern]]
id = "a11y4-03"
fixed = 'options as overlapping'

[[pattern]]
id = "a11y4-04"
regex = 'user-agent `select` lays out as an `inline-block`'

[[pattern]]
id = "a11y4-05"
regex = 'user-agent pair is not measured(?! on a single)'

[[pattern]]
id = "a11y4-06"
fixed = 'no test asserts `:open`'

[[pattern]]
id = "a11y4-07"
fixed = 'options are its children in the tree'

[[pattern]]
id = "a11y4-08"
fixed = 'keeps its `option` children in the tree'

[[pattern]]
id = "a11y4-09"
fixed = 'value, nothing with none chosen'

[[pattern]]
id = "a11y4-10"
fixed = 'its options as its children, each with'

[[pattern]]
id = "a11y4-11"
fixed = 'react to a click and state no interactive role'

[[pattern]]
id = "a11y4-12"
regex = 'Cells 676\b'

[[pattern]]
id = "a11y4-13"
fixed = 'no Tab-order, keyboard-activation or contrast row'
