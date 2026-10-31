proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: '## Standard Contracts'
    change: 'The `select_value` clause reads: HTML `select.value` — the chosen option value, on a `multiple` select its first chosen option in tree order since 2026-10-31-engine-debts, `None` with none chosen or for a node that is no select; the writers clause adds that `set_select_choice` takes the chosen set and that the `pub(crate)` field `select_cursor` stands beside the private `select_typeahead`.'
    sidecar: 'select_value reads the first chosen option of a multiple select; set_select_choice takes a set; select_cursor registered.'
    rationale: 'Report Changes, Symbols / APIs, F6 under `document.rs`; Counts / qualifiers moved, pattern `multiple-select`, changed at `standard-contracts.md:17`.'
    basis: packages/blitz-dom/src/document.rs:736-744
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: '## Standard Contracts'
    change: 'New item after this line: - `BaseDocument::select_values(&self, select: NodeId) -> Vec<String>`, public and read-only since 2026-10-31-engine-debts: the values of the chosen options, a stale id reading none; the Dioxus bridge reads it for a `multiple` select (packages/blitz-dom/src/document.rs:736-744; packages/dioxus-native-dom/src/dioxus_document.rs:31-45).'
    sidecar: 'Registered the public BaseDocument::select_values.'
    rationale: 'A new public API of the report, Symbols / APIs F6 and Coverage of new surfaces, unregistered in this contract.'
    basis: packages/blitz-dom/src/document.rs:736-744
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: '## Standard Contracts'
    change: 'New item after this line: - `BaseDocument::select_of(&self, node: NodeId) -> Option<NodeId>`, public and read-only since 2026-10-31-engine-debts: the select holding an option or an `optgroup`, an option under a group included, a stale id reading none; it takes the place of the snapshot private `select_parent` (packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/node/select.rs:125-129).'
    sidecar: 'Registered the public BaseDocument::select_of.'
    rationale: 'A new public API of the report, Symbols / APIs F7 and Retired, unregistered in this contract.'
    basis: packages/blitz-dom/src/document.rs:746-750
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: '## Standard Contracts'
    change: 'New item after this line: - `BaseDocument::clear_select_typeahead(&mut self)`, public since 2026-10-31-engine-debts: without it a second `type` on a `multiple` select joins the text of the first; its one non-test caller is the `focus_multiple_select` of the escher-driver executor (packages/blitz-dom/src/document.rs:752-755; packages/escher-driver/src/execute.rs:557-563).'
    sidecar: 'Registered the public BaseDocument::clear_select_typeahead and its sole caller.'
    rationale: 'A new public API of the report, Deviations 4 and Sole-caller facts, unregistered in this contract.'
    basis: packages/blitz-dom/src/document.rs:752-755
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 26
    section: '## Standard Contracts'
    change: 'Replace from "and the builder writes no `toggled`" through "(target: next)" with: and since 2026-10-31-engine-debts (F5 «Писать на checkable-ролях (Рекомендую)», the founder, 2026-10-31) the builder writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never begins to read on it (packages/blitz-dom/src/accessibility.rs:199-224); and the closing "nothing else in the file writes `toggled`" reads "nothing else in the file but that native arm writes `toggled`".'
    sidecar: 'The tree now states native checkbox and radio checkedness on checkable roles; the measured absence is ended.'
    rationale: 'Report Symbols / APIs F5; pattern `native-toggled`, whose one no-change site in this file is `standard-contracts.md:249`.'
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 27
    section: '## Standard Contracts'
    change: 'After "`selected`, `toggled` and `invalid` on each element node whose markup states them" add: and, since 2026-10-31-engine-debts, `toggled` on a native checkbox or radio under a checkable role, read from its checkedness, a closed value.'
    sidecar: 'The states class handed to the adapter also carries the native toggled write.'
    rationale: 'Sweep of the F5 claim: this clause limits the states class to what markup states, which report F5 ends for a native control.'
    dependent-of: D-arch-resources
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: '## Standard Contracts'
    change: 'New item after this line: - The tree focus, since 2026-10-31-engine-debts (F4 «Откат на окно (Рекомендую)», the founder, 2026-10-31): `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window (packages/blitz-dom/src/accessibility.rs:104-110; packages/blitz-dom/src/accessibility.rs:122).'
    sidecar: 'New detail, the tree focus falls back to the window.'
    rationale: 'Report Symbols / APIs F4, an engine change for every document with no statement in this contract.'
    basis: packages/blitz-dom/src/accessibility.rs:104-110
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: '## Standard Contracts'
    change: 'New item after this line: - A select in the tree, since 2026-10-31-engine-debts (F6 and F7 «Построить сейчас», the founder, 2026-10-31): `set_multiselectable` on a `multiple` select; an `optgroup` builds `Role::Group`, its non-empty `label` attribute its name, a group label named in the tree and unpainted.'
    sidecar: 'New detail, multiselectable and optgroup in the accessibility tree.'
    rationale: 'Report Symbols / APIs F6 under `accessibility.rs` and F7.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: '## Standard Contracts'
    change: 'New item after this line: - The focus fixup, since 2026-10-31-engine-debts (F3 «Снимать фокус без blur (Рекомендую)», the founder, 2026-10-31): `DocumentMutator::flush`, where `focus_node_id` resolves to a node reading `is_inert()`, calls `BaseDocument::clear_focus`; no DOM `blur` or `focusout` fires and no `inert` arm joins the attribute writes, so a subtree moved under an inert box is covered too; `disabled` and `hidden` read as before (packages/blitz-dom/src/mutator.rs:1051-1063).'
    sidecar: 'New detail, the mutator flush clears focus from a node made inert.'
    rationale: 'Report Symbols / APIs F3; Expected amendments name the Mutation contract.'
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: '## Standard Contracts'
    change: 'New item after this line: - The mutator and a select, since 2026-10-31-engine-debts: a `multiple` write or removal records the select (`record_select_of_multiple_write`), so a removal settles it to one; `set_option_selectedness` leaves the other options of a `multiple` select alone; the `optgroup` arm of `maybe_record_node` records its select (packages/blitz-dom/src/mutator.rs:1262-1271; packages/blitz-dom/src/mutator.rs:1253-1257).'
    sidecar: 'New detail, the mutator records a select on a multiple or optgroup write.'
    rationale: 'Report Symbols / APIs F6 under `mutator.rs` and F7.'
    basis: packages/blitz-dom/src/mutator.rs:1262-1271
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 36
    section: '## Standard Contracts'
    change: 'Replace from "are dispatched by the engine only after `input`" through "firing no engine `change`" with: are dispatched by the engine only after `input`, on a select choice and, since 2026-10-31-engine-debts (F10 «Шлёт движок (Рекомендую)», the founder, 2026-10-31), in the checkbox and radio arms of `handle_click` with the same value, a checkbox always and a radio only where it was not checked before the click, so a page script hears one `change`, from the engine; a text or range input fires no engine `change`, pinned by `text_range_no_change_event` (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836).'
    sidecar: 'The engine dispatches change on a checkbox and a radio; a text or range input still fires none.'
    rationale: 'Report Symbols / APIs F10 and Retired; patterns `change-event` and `text-range-limit`, each changed at `standard-contracts.md:36`.'
    basis: packages/blitz-dom/src/events/pointer.rs:827-836
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 37
    section: '## Standard Contracts'
    change: 'New item after this line: - The keys of a `multiple` select, since 2026-10-31-engine-debts (F6 «Построить сейчас», the founder, 2026-10-31): private `press_multiple_select_key` and `select_text_stands`, crate-internal `select_rows` and `apply_row_gesture`, and `apply_select_choice`, which takes a set and stays the one write path. ArrowDown · ArrowUp · Home · End alone choose that row alone; with Shift they span from the anchor; with Ctrl or Meta they move the current row and change nothing; a Space with no typed text standing flips the current row, and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; Shift with no cursor yet spans from the last chosen row; disabled rows are never chosen; Ctrl+A is not modelled (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:326-332; packages/blitz-dom/src/events/keyboard.rs:334-349; packages/blitz-dom/src/events/keyboard.rs:351-380).'
    sidecar: 'New detail, the key contract of a multiple select.'
    rationale: 'Report Symbols / APIs F6 under `events/keyboard.rs`, and Deviations 6b.'
    basis: packages/blitz-dom/src/events/keyboard.rs:213-244
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 37
    section: '## Standard Contracts'
    change: 'New item after this line: - A click on a row, since 2026-10-31-engine-debts (F6): a new `option` arm of `handle_click` calls the private `click_select_row`; a disabled select stops it; it focuses the select and clears the typeahead and the text selection; a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans; a disabled row changes nothing (packages/blitz-dom/src/events/pointer.rs:1013-1054).'
    sidecar: 'New detail, the option arm of handle_click.'
    rationale: 'Report Symbols / APIs F6 under `events/pointer.rs`.'
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 37
    section: '## Standard Contracts'
    change: 'New item after this line: - No popup, on the founder (2026-10-31, F8 «Давай нарисовать в отдельном чанке» and «Только строки multiple (Рекомендую)»): Enter and Space open nothing on a single select, and an option of a closed single select has no click arm and stays refused `covered`, under a group too; no popup is built.'
    sidecar: 'New detail, the single select keeps no popup and no click arm on a closed option.'
    rationale: 'Report Symbols / APIs F8.'
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 44
    section: '## Standard Contracts'
    change: 'The `None` clause reads: `None` for a node that does not resolve, when nothing is left and, since 2026-10-31-engine-debts, when a mapped box has a non-finite edge; and after "each clipping box is placed where the bounds reader puts it" add: on a chain with a transform, where its transforms show it (the client rect under transforms, below).'
    sidecar: 'visible_region contract: the non-finite None and the placement under a transform.'
    rationale: 'Report Symbols / APIs F1. The report disposes `standard-contracts.md:44` as no change for the reader named, while this line lists when the reader answers `None` and where it places a box, both of which F1 moves; this sweep is the control.'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 46
    section: '## Standard Contracts'
    change: 'Before "`BaseDocument::get_client_bounding_rect` reads that box where its transform moves it" add: `visible_region` reads that box through its transforms since 2026-10-31-engine-debts (the client rect under transforms, below);'
    sidecar: 'The transformed-and-scrolled reading names the visible region beside the hit and the client rect.'
    rationale: 'Report pattern `visible-region`, changed at `standard-contracts.md:46`.'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 53
    section: '## Standard Contracts'
    change: 'Where it reads "the private `transformed_bounds`" say `transformed_bounds`, `pub(crate)` since 2026-10-31-engine-debts; and drop `visible_region` from the list of readers not edited, so it reads: `physical_unrounded_geometry`, `offset_rect` (CSSOM `offsetTop`, untransformed) and `inline_fragment_rects` are not edited.'
    sidecar: 'transformed_bounds is pub(crate); visible_region leaves the not-edited list.'
    rationale: 'Report Symbols / APIs F1; pattern `new-readers`, changed at `standard-contracts.md:53`.'
    basis: packages/blitz-dom/src/document.rs:2574-2578
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 53
    section: '## Standard Contracts'
    change: 'Replace the sentence from "A measured limit: `visible_region` places" through "(target: next)." with: Since 2026-10-31-engine-debts (F1 «Починить (Рекомендую)», the founder, 2026-10-31) `visible_region` keeps its signature and reads each clipping box through its transforms, by the private `padding_box_in_view` over `transformed_bounds`; with no transform on the chain it reads as before. It is exact under a translate and a scale; under a rotation it reads the bounding box of the rotated padding box, so it holds corners the box does not show, a limit stated in the meaning of `off-screen` (§Driver session). Its callers, the `in_view` and `crop` of the executor and the `clip_region` of the lint, are unchanged (packages/blitz-dom/src/scrolling.rs:790-812; tests/blitz-tests/tests/transformed_client_rect.rs:183-216).'
    sidecar: 'The untransformed visible_region limit is retired; the rotation limit stands in its place.'
    rationale: 'Report Retired, the transformed-box clause of `off-screen` and the inverted pin of `transformed_client_rect`; Symbols / APIs F1.'
    basis: packages/blitz-dom/src/scrolling.rs:790-812
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 55
    section: '## Standard Contracts'
    change: 'New item after this line: - The hit under a non-invertible transform, since 2026-10-31-engine-debts (F2 «Починить (Рекомендую)», the founder, 2026-10-31): in `Node::hit_inner` a transform whose determinant is 0 or not finite answers `None` for the whole subtree, in every document; pinned by `hit_non_invertible_transform` in both layout modes (packages/blitz-dom/src/node/node.rs:1472-1477).'
    sidecar: 'New detail, a non-invertible transform takes no hit.'
    rationale: 'Report Symbols / APIs F2; Counts / qualifiers moved records that `non-invertible` is unfired, so no master states F2.'
    basis: packages/blitz-dom/src/node/node.rs:1472-1477
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 76
    section: '## Standard Contracts'
    change: 'The list of types adds `SelectCursor`, `RowGesture` and `Gestured`, and the list of readers adds `first_chosen_option`, `last_chosen`, `spanned`, `gestured`, `owner_select` and `option_disabled`, the last the one rule `readings` and `form.rs` share; since 2026-10-31-engine-debts `options` walks the `option` children of a select and those of its `optgroup` children, in tree order, `is_select_option` holds under a group, and `stepped` takes the row it steps from; still no public type (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89; packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/node/select.rs:200-205; packages/blitz-dom/src/node/select.rs:269-272; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358).'
    sidecar: 'node::select registry gains the multiple and optgroup rules.'
    rationale: 'Report Symbols / APIs F6 under `node/select.rs` and F7.'
    basis: packages/blitz-dom/src/node/select.rs:56-65
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 78
    section: '## Standard Contracts'
    change: 'Where it reads "and by `remove_attribute`" say "and by `clear_attribute`".'
    sidecar: 'The classes entry spells clear_attribute, the mutator method; earlier entry spelling remove_attribute read.'
    rationale: 'Report Expected amendments, architecture CARRY 3; pattern `remove-attr`, 1 hit at `standard-contracts.md:78`.'
    basis: .andromeda/architecture-detail/standard-contracts.md:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 140
    section: '## Standard Contracts'
    change: 'New item after this line: - The grid of Cells, since 2026-10-31-engine-debts (F13 «Одна Tab-остановка (Рекомендую)», the founder): `cells-grid` takes `id`, `tabindex: "0"`, `aria_label: "Cells"` and an `onkeydown`, with no role; arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a click inside the open editor of a cell returns early, so typed text stays; one `onmounted` per data cell (676) holds handles in a map; `.cells-grid:focus .cell.current` outlines the current cell `2px solid #3a5ce5` at `outline-offset: -2px` (examples/seven_guis/src/tasks/cells.rs:551-554; examples/seven_guis/src/tasks/cells.rs:425-433); `tests/common/mod.rs` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` (tests/blitz-tests/tests/common/mod.rs:93-98).'
    sidecar: 'New detail, the Tab stop, keys and current-cell outline of the Cells grid.'
    rationale: 'Report Symbols / APIs F13, and Deviations 6a.'
    basis: examples/seven_guis/src/tasks/cells.rs:551-554
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 169
    section: '## Standard Contracts'
    change: 'The second clause of the `off-screen` meaning, from "a scrolling box under a transform is read where it stands untransformed" through "though it shows", reads: under a rotation a box''s visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view; and "its second a measured limit" adds: restated at 2026-10-31-engine-debts, its remedy, the eight causes and their names unmoved (packages/escher-driver/src/refusal.rs:83-85).'
    sidecar: 'off-screen meaning: the transformed-box clause is retired for the rotation clause.'
    rationale: 'Report Retired, the transformed-box clause of `off-screen`; Fixed text moved quotes the new clause; pattern `visible-region`, changed at `standard-contracts.md:169`.'
    basis: packages/escher-driver/src/refusal.rs:83-85
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 171
    section: '## Standard Contracts'
    change: 'In the crate docs list, the third reading, from "a scrolling box under a transform read where it stands untransformed" through "though it shows", reads, since 2026-10-31-engine-debts: under a rotation a box''s visible part read as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view (packages/escher-driver/src/lib.rs:77-81).'
    sidecar: 'Crate docs reading: the transformed-box clause is retired for the rotation clause.'
    rationale: 'Report Retired, the word `untransformed` leaves `packages/escher-driver`; Fixed text moved names crate docs `lib.rs:77-81`; pattern `visible-region`, changed at `standard-contracts.md:171`.'
    basis: packages/escher-driver/src/lib.rs:77-81
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 171
    section: '## Standard Contracts'
    change: 'After "says that on a select the text chooses the first option whose label starts with it" add: and, since 2026-10-31-engine-debts, states the reading on a `multiple` select, where the text chooses its match alone (packages/escher-driver/src/schema.rs:239-240).'
    sidecar: 'The help of type states the multiple select reading.'
    rationale: 'Report Symbols / APIs F6 for `type` and Fixed text moved; pattern `type-on-select`, changed at `standard-contracts.md:171`.'
    basis: packages/escher-driver/src/schema.rs:239-240
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 181
    section: '## Standard Contracts'
    change: 'After "`type` clicks it the same way, which focuses a text input," add: a `multiple` select excepted since 2026-10-31-engine-debts, focused with no click (the `multiple` select through the driver, below),'
    sidecar: 'The type path names the multiple select exception.'
    rationale: 'Sweep of the `type` claim: report Symbols / APIs F6 for `type` states `focus_multiple_select` focuses with no click, which this line states of every target.'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 209
    section: '## Standard Contracts'
    change: 'Replace the text before its closing citation with: The `overlap` reading of select options: the snapshot gives each option of a closed select the client rect of that select, and since 2026-10-31-engine-debts (F9 «Исключить закрытые option (Рекомендую)», as restated) `overlaps` skips such an entry on either side through `Entry.in_closed_select`, read by `closed_select_of` from the element and never the role, so the `role="option"` rows of CRUD are not read as a select option; at boot the four lean tasks read `[0, 0, 0, 0]`, which `stand_lint` pins in both layout modes; the rows of a `multiple` select are linted as usual, a planted pair reading `fx-row-a` × `fx-row-b` (packages/escher-driver/src/lint.rs:246-247; packages/escher-driver/src/lint.rs:337-338; packages/escher-driver/src/lint.rs:347; packages/escher-driver/src/lint.rs:364; packages/escher-driver/src/lint.rs:368).'
    sidecar: 'The open overlap defect on a closed select is fixed; at_boot reads all zero.'
    rationale: 'Report Symbols / APIs F9; pattern `lint-overlap`, changed at `standard-contracts.md:209`; Deviations 8.'
    basis: packages/escher-driver/src/lint.rs:364
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 210
    section: '## Standard Contracts'
    change: 'New item after this line: - The `multiple` select through the driver, since 2026-10-31-engine-debts (F6): `Target` gains `multiple`; on `type` the private `focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead; a text opening on a space types nothing, since no label can match and the Space would flip a row; no verb, argument, key name, result field, cause, session error or exit is added (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239).'
    sidecar: 'New detail, type on a multiple select.'
    rationale: 'Report Symbols / APIs, driver and stand, F6; Deviations 6c.'
    basis: packages/escher-driver/src/execute.rs:557-563
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 210
    section: '## Standard Contracts'
    change: 'New item after this line: - The help of `press`, since 2026-10-31-engine-debts, states its limit on a `multiple` select: "a set of rows that are not adjacent needs Ctrl, which this verb does not carry" (packages/escher-driver/src/schema.rs:261-263).'
    sidecar: 'New detail, the help of press states the Ctrl limit.'
    rationale: 'Report Fixed text moved quotes it; pattern `press-ctrl` records that no site states the help of `press`.'
    basis: packages/escher-driver/src/schema.rs:261-263
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 249
    section: '## Standard Contracts'
    change: 'Where it reads "`value` a `select`" and "chosen option" say: `value` the `BaseDocument::select_value` reading of a `select`, the chosen option value and, on a `multiple` select, its first chosen option in tree order.'
    sidecar: 'The snapshot value of a multiple select names its first chosen option.'
    rationale: 'Sweep of the `multiple-select` claim, changed at `standard-contracts.md:249`; report F6 under `snapshot.rs`.'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 260
    section: '## Standard Contracts'
    change: 'After "`col_label(c)` and `cell-input`" add: and, since 2026-10-31-engine-debts, `cells-grid`, the keyed Tab stop of the grid (the 676 data cells stay a true count; the listener count of `stand_non_lean_tasks` reads 677 with the grid).'
    sidecar: 'The Cells ids gain cells-grid.'
    rationale: 'Report pattern `cells-grid`, gaining at `standard-contracts.md:260`; Spec claims disproved, the count of Cells; pattern `cells-676` keeps the 676 data cells here.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: '## Standard Contracts'
    change: 'Replace the last clause, from "and an option whose parent is a `select`" through "named by their labels", with: and an option of a closed select, under a group too, reads the client rect of that select as its `bounds` (`closed_select_of`; the private `select_parent` retired for `BaseDocument::select_of` at 2026-10-31-engine-debts), while the rows and groups of a `multiple` select read their own client rects; the options of a select are its children in the snapshot, named by their labels.'
    sidecar: 'select_parent is retired; rows and groups of a multiple select read their own client rects.'
    rationale: 'Report Retired, the private `select_parent` of `snapshot.rs`; F6 under `snapshot.rs`; patterns `multiple-select` and `optgroup`, changed at `standard-contracts.md:267`.'
    basis: packages/dioxus-native-dom/src/snapshot.rs:242-248
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: '## Standard Contracts'
    change: 'New item after this line: - `dioxus_native_dom::closed_select_of(doc: &BaseDocument, node: NodeId) -> Option<NodeId>` and `is_multiple_select`, public and re-exported since 2026-10-31-engine-debts, since escher-driver has no blitz-dom dependency to name `local_name!` with (`is_masked_input` the precedent); `closed_select_of` has two callers, the snapshot builder and the lint, and `is_multiple_select` two, `closed_select_of` and the `locate` of the executor (packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38).'
    sidecar: 'Registered the public closed_select_of and is_multiple_select.'
    rationale: 'Two new public items of the report, Sole-caller facts and Deviations 4, unregistered in this contract.'
    basis: packages/dioxus-native-dom/src/snapshot.rs:242-248
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: '## Standard Contracts'
    change: 'New item after this line: - The set in the bridge, since 2026-10-31-engine-debts (F6): the private `multiple_select_values` fills `values` on `Input` and `Change`, one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set; `value` stays `select_value` (packages/dioxus-native-dom/src/dioxus_document.rs:31-45).'
    sidecar: 'New detail, the chosen set crosses the bridge as form entries.'
    rationale: 'Report Symbols / APIs F6 under `dioxus_document.rs`.'
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:31-45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: '## Standard Contracts'
    change: 'New item after this line: - A measured hazard (2026-10-31-engine-debts): a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document `RefCell`, and a `MountedData` call in that task panics with `RefCell already borrowed` (`events.rs:162`); measured headlessly through the `poll(None)` of the harness, the windowed shell running the same lines, read and not run; no engine line moved, and Cells calls its handles synchronously.'
    sidecar: 'New detail, the MountedData hazard of a task spawned from a key handler.'
    rationale: 'Report Reverted, Decisions & corrections and pattern `mounted-hazard`, which calls it a new fact; its owner is asked of P5.'
    basis: packages/dioxus-native-dom/src/events.rs:162
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 298
    section: '## Standard Contracts'
    change: 'Where it reads "ten named blitz-tests targets" say "fifteen named blitz-tests targets".'
    sidecar: 'stand-repeat names fifteen blitz-tests targets.'
    rationale: 'Report Counts / qualifiers moved, pattern `ten-named`, changed at `standard-contracts.md:298`; Harness / gate surface.'
    basis: .github/scripts/ci-leg.sh:58-59
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 311
    section: '## Standard Contracts'
    change: 'New item after this line: - The free-disk step, since 2026-10-31-engine-debts on the operator word: the `Stand checks repeated` job runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false` (.github/workflows/ci.yml:311-316), `completed · success` on CI#41192433140; it is not shown to be the cure of the out-of-disk red on CI#41190771843. The older step of that action, under the matrix job, is skipped on every row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest` (.github/workflows/ci.yml:392-394).'
    sidecar: 'New detail, the free-disk action site in stand-repeat and the skipped older step.'
    rationale: 'Report Harness / gate surface and Insufficient fixes; `free-disk` is unfired, so no master states either step; the owners of the skipped step and the unproven cure are asked of P5.'
    basis: .github/workflows/ci.yml:311-316
note: 'The body broad lines at .andromeda/architecture.md:135, :139, :141 and :145 state limits these proposals move and lie outside this file; D-arch-decisions found nothing here, the one dependency move being a lock-only pulldown-cmark patch this file does not name.'
