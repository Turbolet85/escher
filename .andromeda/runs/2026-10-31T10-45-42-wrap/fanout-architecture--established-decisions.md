proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 64
    section: "## Established Decisions"
    change: "After `and from neither on a native checkbox or radio (ratified by the founder, 2026-10-18)` add: `, which under a checkable role writes it `True` or `False` from `checkbox_input_checked()` and under any other role writes nothing, so `pressed` never begins to read on it (ratified by the founder, 2026-10-31, «Писать на checkable-ролях (Рекомендую)»; packages/blitz-dom/src/accessibility.rs:199-224)`; the clause keeps that neither author attribute is read on a native control."
    sidecar: "Established Decisions, the builder's toggled: a native checkbox or radio now writes toggled from its checked state on a checkable role and nothing under any other role (F5, 2026-10-31-engine-debts)."
    rationale: "The report's F5 and its native-toggled sweep: the tree now states a native control's checked state, so a toggled read from neither attribute no longer tells the whole of it (report, Symbols / APIs, F5)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:23
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 77
    section: "## Established Decisions"
    change: "The limit sentence now reads: `Its limits: no popup is painted (the founder, 2026-10-31, «Давай нарисовать в отдельном чанке» and «Только строки multiple (Рекомендую)»), an option of a closed single select is not clicked (a driver `click` on one is refused `covered`, under a group too), and a group's label is unpainted. A `multiple` select and an `optgroup`'s options are modelled, each in a row of its own.` The sentence `Its choice model is [DOM semantics]'.` stays."
    sidecar: "Established Decisions, the select model's limits: multiple and optgroup are modelled, the option-click limit narrowed to a closed single select, no popup standing (F6, F7, F8, 2026-10-31-engine-debts)."
    rationale: "The report's multiple-select, optgroup, option-click and type-on-select sweeps each name this line as changed, and F6, F7 and F8 move its stated limits (report, Symbols / APIs; Counts / qualifiers moved, text limits)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:34
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 68
    section: "## Established Decisions"
    change: "Replace `a click on a `select` focuses it through its own arm of the walk (below)` with `a click on a `select` focuses it through its own arm of the walk, and a click on a `multiple` select's row through an `option` arm (below)`."
    sidecar: "Established Decisions, the walk's click arms: a multiple select's row takes a new option arm that focuses its select (F6, 2026-10-31-engine-debts)."
    rationale: "The sweep of the retired no-option-click limit: `click_select_row` sits behind a new `option` arm of `handle_click` and focuses the select, so that row is no longer among the elements whose click clears focus (report, F6, events/pointer.rs)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:28
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 115
    section: "## Established Decisions"
    change: "Replace `a select's options are its `option` children in tree order (an `optgroup`'s are not listed)` with `a select's options are its `option` children and its `optgroup` children's, in tree order`."
    sidecar: "Established Decisions, the select's rules: its options now run through its optgroup children (F7, 2026-10-31-engine-debts)."
    rationale: "The report's optgroup sweep names this line as changed: `options` walks a select's `option` children and its `optgroup` children's, in tree order (report, F7)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:33
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 115
    section: "## Established Decisions"
    change: "Replace `a `selected` write sets that option chosen and its select's other options not` with `a `selected` write sets that option chosen and a single-choice select's other options not, a `multiple` select's left alone`."
    sidecar: "Established Decisions, a selected write: a multiple select's other options are left alone (F6, 2026-10-31-engine-debts)."
    rationale: "The sweep of the retired no-multiple-model limit: `set_option_selectedness` leaves a `multiple` select's other options alone (report, F6, mutator.rs)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:29
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 116
    section: "## Established Decisions"
    change: "Replace `keeps an option whose parent is a `select` although it computes `display: none`` with `keeps an option of a `select`, under an `optgroup` too, although it computes `display: none``."
    sidecar: "Established Decisions, the builder and options: an option under a group is kept as one whose parent is the select (F7, 2026-10-31-engine-debts)."
    rationale: "The sweep of the retired optgroup limit: `is_select_option` holds under a group, so the parent-is-a-select wording no longer bounds what the builder keeps (report, F7)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:33
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 116
    section: "## Established Decisions"
    change: "Replace `so a select lays out as a box holding its chosen option` with `so a single-choice select lays out as a box holding its chosen option`."
    sidecar: "Established Decisions, the sheet: the box holding one chosen option is a single-choice select's, a multiple select's rows having boxes of their own (F6, 2026-10-31-engine-debts)."
    rationale: "The sweep of the retired no-multiple-model limit: the default sheet now gives a `multiple` select's rows `display: block` (report, Schema / config)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:44
    dependent-of: D-arch-decisions
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 99
    section: "## Established Decisions"
    change: "Replace `its test file not asserting both layout modes` with `its test file asserting both layout modes since 2026-10-31-engine-debts, upstream's exception ended`."
    sidecar: "Established Decisions, the flow vote: its test file runs both layout modes, upstream's both-modes exception retired (2026-10-31-engine-debts)."
    rationale: "The report's Retired bullet: upstream's both-modes exception, `pseudo_element_flow_classification`'s three tests now run `for incremental in [false, true]`; this line still describes the exception as standing, one the flow-class pattern's list does not name."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:52
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 113
    section: "## Established Decisions"
    change: "Replace the sentence from `A known limit: HTML's focus fixup is not built` to `(target: next).` with: `The focus fixup is built (ratified by the founder, 2026-10-31, «Снимать фокус без blur (Рекомендую)»): where `focus_node_id` resolves to a node reading `is_inert()`, `DocumentMutator::flush` calls `BaseDocument::clear_focus`, no DOM `blur` or `focusout` firing and no `inert` arm joining the attribute writes, so a subtree moved under an inert box is covered too; `disabled` and `hidden` read as before (packages/blitz-dom/src/mutator.rs:1051-1063; tests/blitz-tests/tests/inert_subtree.rs:229-310).`"
    sidecar: "Established Decisions, inert: the focus-fixup limit and its pin retired, the fixup stated with no blur fired (F3, 2026-10-31-engine-debts)."
    rationale: "The report's Retired bullet names `inert_subtree::removing_inert_restores_focus_hit_and_tree`'s focus-fixup limit lines, F3 builds the fixup, and the focus-fixup sweep names this line as changed."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:50
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 153
    section: "## Established Decisions"
    change: "Replace `change` synthesised after `input` on checkbox/radio` with `a page's script hears one `change` on a checkbox or radio, the engine's ([Scrolling and input]), the runtime no longer synthesising it`."
    sidecar: "Established Decisions, JavaScript: the runtime's change synthesis retired, a page's script hearing the engine's one change (F10, 2026-10-31-engine-debts)."
    rationale: "The report's Retired bullet names `runtime/events.rs`'s `change` synthesis and `target_is_checkbox_or_radio`, and the change-event sweep names this line as changed (report, F10)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:48
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 123
    section: "## Established Decisions"
    change: "Replace `with no test of its own, `details_element` and the a11y leg green` with `held since 2026-10-31-engine-debts by `open_state`, four tests of the attribute at construction, a mutator write and removal and a `summary` click and Enter (tests/blitz-tests/tests/open_state.rs:54-143), `details_element` and the a11y leg green`."
    sidecar: "Established Decisions, :open: the no-test gap closed by open_state (CARRY 2, 2026-10-31-engine-debts)."
    rationale: "CARRY 2 closes the gap: `open_state` 4 is a new target, and this line's no-test statement stands in a phrasing the report's listed changed sites do not name (report, Outcome, CARRY 2; Counts / qualifiers moved, new targets)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:65
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 141
    section: "## Established Decisions"
    change: "Replace `held by the windowed `seven_guis_native` smoke boot and `focus_redraw`` with `held by the windowed `seven_guis_native` smoke boot, `focus_redraw` and, since 2026-10-31-engine-debts, blitz-shell's first unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read (packages/blitz-shell/src/event.rs:144-171)`."
    sidecar: "Established Decisions, the shell's redraw route: blitz-shell's first unit test holds it too (CARRY 2, 2026-10-31-engine-debts)."
    rationale: "The report's redraw-route sweep names this line as changed, and Crates / modules adds the unit test's module (report, Counts / qualifiers moved, Gaps CARRY 2 closes; Outcome, CARRY 2)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: "New row: - Eight direct engine changes for every document stand beside those, each ratified by the founder's own word (2026-10-31, by dialog in the overseer session, relayed verbatim by the operator): `visible_region` read through transforms, the non-invertible hit, the focus fixup, the tree's focus, native `toggled`, the `multiple` select, `optgroup` and `change` on a checkbox and a radio. Each is flagged upstreamable at upstream `main` `ebacd68bd` with nothing sent; their merge surface is a line owned by Epoch 3's sync, where `0980cce5f` has no hunk on `visible_region` and its `hit_inner` hunk is a textual conflict with the non-invertible hit (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md)."
    sidecar: "Established Decisions, the eight engine changes of 2026-10-31 recorded with their ratification, upstreamable flag and merge-surface owner (2026-10-31-engine-debts)."
    rationale: "The report's Expected amendments name §Established Decisions, the eight changes ratified, flagged upstreamable and their merge surface a line owned by Epoch 3's sync; the Cross-project claims hold the upstream readings."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:128
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - `BaseDocument::visible_region` reads each clipping box through its transforms, its signature kept (ratified by the founder, 2026-10-31, «Починить (Рекомендую)»): the private `padding_box_in_view` reads over `transformed_bounds`, now `pub(crate)`; with no transform on the chain it reads as before, and a mapped box with a non-finite edge answers `None`. It is exact under a translate and a scale; under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show. Its callers are unchanged, and `physical_unrounded_geometry`, `Node::absolute_position` and `scroll_into_view` are not edited (packages/blitz-dom/src/scrolling.rs:790-812; packages/blitz-dom/src/document.rs:2574-2578; §Standard Contracts → Scrolling, selection, tree)."
    sidecar: "Established Decisions, visible_region through transforms, its rotation reading stated (F1, 2026-10-31-engine-debts)."
    rationale: "F1 is a founder-ratified engine change for every document, and this file records each such change under [Scrolling and input] (report, Symbols / APIs, F1)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - A transform whose determinant is 0 or not finite takes no hit: `Node::hit_inner` answers `None` for the whole subtree there (ratified by the founder, 2026-10-31, «Починить (Рекомендую)»; packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56)."
    sidecar: "Established Decisions, the non-invertible hit: no hit for a subtree under a zero or non-finite determinant (F2, 2026-10-31-engine-debts)."
    rationale: "F2 is a founder-ratified engine change for every document, and the report states that no master holds it yet: `non-invertible`'s control is unfired (report, Counts / qualifiers moved, new targets)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - The engine fires `change` on a checkbox and a radio (ratified by the founder, 2026-10-31, «Шлёт движок (Рекомендую)»): `handle_click`'s checkbox and radio arms dispatch, after `input`, a `DomEventData::Change` with the same value, a checkbox always and a radio only where it was not checked before the click, so a page's script hears one `change`, from the engine (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836)."
    sidecar: "Established Decisions, change on a checkbox and a radio is the engine's (F10, 2026-10-31-engine-debts)."
    rationale: "F10 is a founder-ratified engine change for every document, and it replaces the synthesis [JavaScript] held (report, Symbols / APIs, F10)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:35
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - A `multiple` select takes keys and clicks of its own (the founder, 2026-10-31, «Построить сейчас»): focused, ArrowDown · ArrowUp · Home · End alone choose that row alone, with Shift they span from the anchor, with no cursor yet from the last chosen row, and with Ctrl or Meta they move the current row and change nothing; a Space with no typed text standing flips the current row, and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen, and Ctrl+A is not modelled (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:326-332; packages/blitz-dom/src/events/keyboard.rs:334-349; packages/blitz-dom/src/events/keyboard.rs:351-380; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358). `click_select_row`, behind a new `option` arm of `handle_click`: a disabled select stops it; it focuses the select and clears the typeahead and text selection; plain chooses the row alone, Ctrl or Meta flips it, Shift spans; a disabled row changes nothing (packages/blitz-dom/src/events/pointer.rs:1013-1054). `apply_select_choice` takes a set and stays the one write path."
    sidecar: "Established Decisions, a multiple select's keys and clicks (F6, 2026-10-31-engine-debts)."
    rationale: "F6's keys and pointer are the model the old limit at this category's select row excluded; [DOM semantics] sends a select's keys and click here (report, Symbols / APIs, F6)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - A measured hazard of a key handler's spawned task, no engine change: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in that task panics `RefCell already borrowed` (`events.rs:162`), so Cells calls its handles synchronously from the handler, `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call. Measured headlessly through the harness's `poll(None)`; the windowed shell runs the same lines, read and not run (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md)."
    sidecar: "Established Decisions, a measured MountedData hazard in a task spawned from a key handler, beside the overlay scrollbar's (2026-10-31-engine-debts)."
    rationale: "The report states the hazard as a new fact any app meets, with no master holding it (report, Decisions & corrections; Reverted / negative API facts; `mounted-hazard`); this group's overlay-scrollbar row is the form's precedent, and its owner stays P5's to name."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:182
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 113
    section: "## Established Decisions"
    change: "New row: - The tree's focus falls back to the window (ratified by the founder, 2026-10-31, «Откат на окно (Рекомендую)»): `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window (packages/blitz-dom/src/accessibility.rs:104-110; packages/blitz-dom/src/accessibility.rs:122)."
    sidecar: "Established Decisions, the tree's focus falls back to the window where the focused node was not built (F4, 2026-10-31-engine-debts)."
    rationale: "F4 is a founder-ratified engine change for every document, beside the focus fixup it pairs with (report, Symbols / APIs, F4)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 123
    section: "## Established Decisions"
    change: "New row: - A `multiple` select holds a set (the founder, 2026-10-31, «Построить сейчас»): `set_select_choice` and `apply_select_choice` take the chosen set, and a `multiple` write or removal records the select (`record_select_of_multiple_write`), so removal settles it to one (packages/blitz-dom/src/mutator.rs:1262-1271). `select_value` reads its first chosen option in tree order; the public `BaseDocument::select_values` reads the set and `clear_select_typeahead` clears the buffer (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:752-755). The builder writes `set_multiselectable` on it; the snapshot reads its rows' and groups' own client rects and its `value` from `select_value`; the bridge's `multiple_select_values` fills `values` on `Input` and `Change`, one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set and `value` stays `select_value` (packages/dioxus-native-dom/src/dioxus_document.rs:31-45). `size` is unread. A driver `type` on one focuses it with no click, which would land on the row at the centre, and clears the typeahead, and a text opening on a space types nothing, with no verb, argument, key name, result field, cause, session error or exit added (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239)."
    sidecar: "Established Decisions, a multiple select's choice model, its readers and its driver type (F6, 2026-10-31-engine-debts)."
    rationale: "The select row's choice model is [DOM semantics]', and F6 moves it across document, mutator, tree, snapshot and bridge; the type-on-select sweep names the select row as changed (report, Symbols / APIs, F6)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 123
    section: "## Established Decisions"
    change: "New row: - An `optgroup`'s options are its select's (the founder, 2026-10-31, «Построить сейчас»): `is_select_option` holds under a group, `owner_select` finds a row's select, and `option_disabled` is the one rule `readings` and `form.rs` share (packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/form.rs:281). The public read-only `BaseDocument::select_of(node) -> Option<NodeId>` answers it outside the crate (packages/blitz-dom/src/document.rs:746-750), and `maybe_record_node`'s `optgroup` arm records its select (packages/blitz-dom/src/mutator.rs:1253-1257). The builder takes an `optgroup` as `Role::Group`, its non-empty `label` attribute its name, so a group's label is named in the tree and unpainted."
    sidecar: "Established Decisions, optgroup modelled: its rows, its disabled rule, select_of and Role::Group (F7, 2026-10-31-engine-debts)."
    rationale: "F7 is a founder-ratified engine change for every document, and the select's rules live in [DOM semantics]; the optgroup sweep names this category's two sites (report, Symbols / APIs, F7)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 123
    section: "## Established Decisions"
    change: "New row: - The user-agent sheet gives a group and a `multiple` select's rows their layout: `select:not([multiple]) > optgroup:not([hidden]) { display: contents; }`, `select:not([multiple]) > optgroup > option:checked { display: inline; }`, `select[multiple] > option, select[multiple] > optgroup > option { display: block; padding: 0 2px; white-space: nowrap; }`, `select[multiple] > optgroup { display: block; }` and the engine-default chosen-row pair `select[multiple] > option:checked, select[multiple] > optgroup > option:checked { background-color: #c8c8c8; color: #000000; }`, 12.56:1, Chromium's inactive selected pair, measured by `select_interaction` (packages/blitz-dom/assets/default.css:161-188)."
    sidecar: "Established Decisions, the default sheet's group and multiple-row rules and the engine-default chosen-row pair (F6, F7, 2026-10-31-engine-debts)."
    rationale: "The report's Schema / config states the rows and the pair as new text, and this category already holds the sheet's select rules (report, Schema / config; Counts / qualifiers moved, `default-sheet`)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:44
note: "D-arch-resources and the stack half of D-arch-decisions found nothing in this file (the lock bump and the CI action touch no row here); .andromeda/architecture.md:60, :62 and :74 state the same moved limits in the body and are outside this file; :99 and :123 are not on the report's changed lists, read by this sweep."
