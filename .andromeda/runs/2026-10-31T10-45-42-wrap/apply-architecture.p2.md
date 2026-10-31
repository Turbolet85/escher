# Apply — architecture · part 2 of 5

## Verdicts
- fanout-architecture--cascade-g2.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:64#1
- fanout-architecture--cascade-g2.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:68#1
- fanout-architecture--cascade-g2.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:75+#1
- fanout-architecture--cascade-g2.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:77#1
- fanout-architecture--cascade-g2.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:77#2
- fanout-architecture--cascade-g2.md 7 · applied · .andromeda/architecture-detail/established-decisions.md:78+#1
- fanout-architecture--cascade-g2.md 8 · applied · .andromeda/architecture-detail/established-decisions.md:78+#2
- fanout-architecture--cascade-g2.md 9 · applied · .andromeda/architecture-detail/established-decisions.md:77#3
- fanout-architecture--cascade-g2.md 11 · applied · .andromeda/architecture-detail/established-decisions.md:99#1
- fanout-architecture--cascade-g2.md 12 · applied · .andromeda/architecture-detail/established-decisions.md:113#1
- fanout-architecture--cascade-g2.md 13 · applied · .andromeda/architecture-detail/established-decisions.md:113#2
- fanout-architecture--cascade-g2.md 14 · applied · .andromeda/architecture-detail/established-decisions.md:113+#1
- fanout-architecture--cascade-g2.md 16 · applied · .andromeda/architecture-detail/established-decisions.md:115#1
- fanout-architecture--cascade-g2.md 17 · applied · .andromeda/architecture-detail/established-decisions.md:115#2
- fanout-architecture--cascade-g2.md 18 · applied · .andromeda/architecture-detail/established-decisions.md:116#1
- fanout-architecture--cascade-g2.md 19 · applied · .andromeda/architecture-detail/established-decisions.md:116#2
- fanout-architecture--cascade-g2.md 20 · applied · .andromeda/architecture-detail/established-decisions.md:116+#1
- fanout-architecture--cascade-g2.md 21 · applied · .andromeda/architecture-detail/established-decisions.md:116+#2
- fanout-architecture--cascade-g2.md 23 · applied · .andromeda/architecture-detail/established-decisions.md:123#1
- fanout-architecture--cascade-g2.md 24 · applied · .andromeda/architecture-detail/established-decisions.md:141#1
- fanout-architecture--cascade-g2.md 25 · applied · .andromeda/architecture-detail/established-decisions.md:153#1
- fanout-architecture--cascade-g2.md 26 · applied · .andromeda/architecture-detail/established-decisions.md:161+#1

## Edits
- line: .andromeda/architecture-detail/established-decisions.md:64
  old: «and from neither on a native checkbox or radio (ratified by the founder, 2026-10-18)»
  new: «and from neither attribute on a native checkbox or radio (ratified by the founder, 2026-10-18), which writes `Toggled::True` or `False` from its own `checkbox_input_checked()` under a checkable role and nothing under any other»
- line: .andromeda/architecture-detail/established-decisions.md:68
  old: «a click on a `select` focuses it through its own arm of the walk (below), and a click on any other element still clears focus»
  new: «a click on a `select` focuses it through its own arm of the walk (below), a click on a `multiple` select's row focuses that select through the walk's `option` arm (below), and a click on any other element still clears focus»
- after: .andromeda/architecture-detail/established-decisions.md:75
  anchor: «- Seven direct engine changes for every document stand beside those eight, each ratified by the founder (2026-10-18): `inert` honoured and `disabled` read on a form control alone ([DOM semantics]); the builder's toggle read per role, the hit's stop at a `contain: paint` box and `Node::absolute_position` less its containing blocks' scroll alone (above); a click in a password `input` placing the caret by its bullets — `ClickTarget::TextInput` carries a `password` flag (an `input` whose `type` reads `password`, ASCII case-insensitive), and the private `click_password_input` lays out one U+2022 per character through `BaseDocument::layout_as_text_input`, scrolls that layout as paint scrolls it, finds the bullet under the point with parley's `Cursor::from_point` and moves the real editor to that character's byte offset — a click `move_to_byte`, a shift-click `extend_selection_to_byte`, two or more clicks `select_all` — every other text input keeping `move_to_point` on the editor's own layout (packages/blitz-dom/src/events/pointer.rs:585-651); and the client rect read where an element's transforms move it (§Standard Contracts → Scrolling, selection, tree). `aria-activedescendant` stays unread and a box with no area stays hit at its centre, stated in `covered`'s meaning (§Standard Contracts → Driver session) — neither an engine change. Each of the seven is flagged upstreamable against upstream `main` at `23b9b26a9` with no upstream pull request open; their merge surface, by file (escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md), is owned by a `.andromeda/residuals.md` line, re-read row by row with its owner quoted (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/readings.md), its owner Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
  add: «- Eight more ratified direct engine changes for every document stand beside those seven: `visible_region` reading a clipping box through its transforms and a non-invertible transform taking no hit (§Standard Contracts → Scrolling, selection, tree); the focus fixup ([DOM semantics]); the tree's focus falling back to the window and a native control's `toggled` on a checkable role (§Standard Contracts → Mutation, query and CSSOM); the `multiple` select (below) and `optgroup` ([DOM semantics]); and the engine's `change` on a checkbox and a radio (§Standard Contracts → Events). Each is flagged upstreamable and nothing is sent; upstream `0980cce5f` has no hunk on `visible_region`, and its `hit_inner` hunk is a textual conflict with the non-invertible answer for Epoch 3's sync, their merge surface a line owned by Epoch 3's «Upstream sync ahead of the UI graph» (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md).»
- line: .andromeda/architecture-detail/established-decisions.md:77
  old: « an option is not clicked (a driver `click` on one is refused `covered`), and a `multiple` select or an `optgroup`'s options are not modelled.»
  new: « and an option of a closed single select is not clicked (a driver `click` on one is refused `covered`, under a group too).»
- line: .andromeda/architecture-detail/established-decisions.md:77
  old: «Every choice that changes the chosen option goes through the one path `apply_select_choice`»
  new: «Every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, which takes the chosen set»
- after: .andromeda/architecture-detail/established-decisions.md:78
  anchor: « Flagged upstreamable, with no upstream pull request open; its merge surface, by file — blitz-dom `node/select.rs`, `document.rs`, `events/{keyboard,pointer,mod}.rs`, `mutator.rs`, `accessibility.rs`, `form.rs`, `node/element.rs` and `assets/default.css`, blitz-traits `events.rs` and blitz-vibey-script `runtime/events.rs` — is owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
  add: «- A `multiple` select's keys, through `press_multiple_select_key`: `ArrowDown`, `ArrowUp`, `Home` and `End` alone choose that row alone, with Shift span from the anchor, and with Ctrl or Meta move the current row and change nothing; a Space flips the current row with no typed text standing and joins the text otherwise; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen; Ctrl+A is not modelled (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:326-332; packages/blitz-dom/src/events/keyboard.rs:334-349; packages/blitz-dom/src/events/keyboard.rs:351-380).»
- after: .andromeda/architecture-detail/established-decisions.md:78
  anchor: « Flagged upstreamable, with no upstream pull request open; its merge surface, by file — blitz-dom `node/select.rs`, `document.rs`, `events/{keyboard,pointer,mod}.rs`, `mutator.rs`, `accessibility.rs`, `form.rs`, `node/element.rs` and `assets/default.css`, blitz-traits `events.rs` and blitz-vibey-script `runtime/events.rs` — is owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
  add: «- A `multiple` select's pointer: `click_select_row`, behind an `option` arm of `handle_click`, stops at a disabled select, focuses the select and clears the typeahead and the text selection; a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, Shift with no cursor yet spans from the last chosen row, and a disabled row changes nothing (packages/blitz-dom/src/events/pointer.rs:1013-1054).»
- line: .andromeda/architecture-detail/established-decisions.md:77
  old: «Its limits: no popup is painted,»
  new: «Its limits: no popup is painted (owner: the route entry «Select popup»), `size` is unread,»
- line: .andromeda/architecture-detail/established-decisions.md:99
  old: «its test file not asserting both layout modes (§Conventions → Tests)»
  new: «its test file asserting both layout modes, every test running `for incremental in [false, true]` (§Conventions → Tests)»
- line: .andromeda/architecture-detail/established-decisions.md:113
  old: «tests/blitz-tests/tests/inert_subtree.rs:77-363). A known limit: HTML's focus fixup is not built — a node that holds focus when it becomes inert keeps it until focus moves (pinned by `inert_subtree::removing_inert_restores_focus_hit_and_tree`), its fix owned by a `.andromeda/residuals.md` line (target: next).»
  new: «tests/blitz-tests/tests/inert_subtree.rs:77-363).»
- line: .andromeda/architecture-detail/established-decisions.md:113
  old: «and no mutator hook is added»
  new: «and no `inert` arm joins the attribute writes»
- after: .andromeda/architecture-detail/established-decisions.md:113
  anchor: «- `inert` is honoured by its presence on the element or an ancestor element (ratified by the founder, 2026-10-18): `Node::is_focussable` also reads false for an inert node through the crate-private `Node::is_inert`, an ancestor walk, so the cached `ElementData::is_focussable` stays per element and no mutator hook is added, and the Tab walk `focus_next_node` / `focus_prev_node`, autofocus (`process_added_subtree`) and `DioxusDocument::unkeyed_actionable` skip it; `hit_inner` answers no hit at an element carrying `inert`, before any other test, so a hit passes through it; the accessibility builder's `is_hidden_from_accessibility_tree` reads true for it, so the subtree leaves the tree and the snapshot (packages/blitz-dom/src/node/node.rs:563-580; packages/blitz-dom/src/node/node.rs:1437-1446; packages/blitz-dom/src/accessibility.rs:279; packages/blitz-dom/src/accessibility.rs:283-296; tests/blitz-tests/tests/inert_subtree.rs:77-363). A known limit: HTML's focus fixup is not built — a node that holds focus when it becomes inert keeps it until focus moves (pinned by `inert_subtree::removing_inert_restores_focus_hit_and_tree`), its fix owned by a `.andromeda/residuals.md` line (target: next).»
  add: «- HTML's focus fixup stands without its events: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, firing no DOM `blur` or `focusout`, so a subtree moved under an inert box is covered too, and `disabled` and `hidden` read as before (packages/blitz-dom/src/mutator.rs:1051-1063), held by `inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key`.»
- line: .andromeda/architecture-detail/established-decisions.md:115
  old: «a select's options are its `option` children in tree order (an `optgroup`'s are not listed)»
  new: «a select's options are its `option` children and its `optgroup` children's, in tree order»
- line: .andromeda/architecture-detail/established-decisions.md:115
  old: «a `selected` write sets that option chosen and its select's other options not»
  new: «a `selected` write sets that option chosen and, on a single-choice select, its select's other options not, a `multiple` select's others left alone»
- line: .andromeda/architecture-detail/established-decisions.md:116
  old: «keeps an option whose parent is a `select` although it computes `display: none`»
  new: «keeps an option whose parent is a `select`, or an `optgroup` under one, although it computes `display: none`»
- line: .andromeda/architecture-detail/established-decisions.md:116
  old: «so a select lays out as a box holding its chosen option»
  new: «so a single-choice select lays out as a box holding its chosen option»
- after: .andromeda/architecture-detail/established-decisions.md:116
  anchor: « The accessibility builder keeps an option whose parent is a `select` although it computes `display: none` — `hidden`, `inert` and `visibility: hidden` still exclude it —, writes its `selected` from its chosen state with no `aria-selected` read, and takes its non-empty `label` attribute as its `label` where it has no `aria-label` (packages/blitz-dom/src/accessibility.rs:164-170; packages/blitz-dom/src/accessibility.rs:184-194; packages/blitz-dom/src/accessibility.rs:280-281; packages/blitz-dom/src/accessibility.rs:283-296). The user-agent stylesheet gives `select` the border, padding and background of `input, textarea` and `display: inline-block`, and shows a single-choice select's chosen option by `select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`, so a select lays out as a box holding its chosen option (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159). Its keys and its click are [Scrolling and input]'s.»
  add: «- `optgroup`: `is_select_option` holds under a group, `option_disabled` is the one rule `readings` and `form.rs` share, `maybe_record_node`'s `optgroup` arm records its select, and the tree gives a group `Role::Group` named by its non-empty `label`, which stays unpainted (packages/blitz-dom/src/node/select.rs:131-134; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/form.rs:281; packages/blitz-dom/src/mutator.rs:1253-1257).»
- after: .andromeda/architecture-detail/established-decisions.md:116
  anchor: « The accessibility builder keeps an option whose parent is a `select` although it computes `display: none` — `hidden`, `inert` and `visibility: hidden` still exclude it —, writes its `selected` from its chosen state with no `aria-selected` read, and takes its non-empty `label` attribute as its `label` where it has no `aria-label` (packages/blitz-dom/src/accessibility.rs:164-170; packages/blitz-dom/src/accessibility.rs:184-194; packages/blitz-dom/src/accessibility.rs:280-281; packages/blitz-dom/src/accessibility.rs:283-296). The user-agent stylesheet gives `select` the border, padding and background of `input, textarea` and `display: inline-block`, and shows a single-choice select's chosen option by `select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`, so a select lays out as a box holding its chosen option (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159). Its keys and its click are [Scrolling and input]'s.»
  add: «- The default sheet lays out `optgroup` and `multiple` rows — `select:not([multiple]) > optgroup:not([hidden]) { display: contents; }`, `select:not([multiple]) > optgroup > option:checked { display: inline; }`, `select[multiple] > option, select[multiple] > optgroup > option { display: block; padding: 0 2px; white-space: nowrap; }` and `select[multiple] > optgroup { display: block; }` — and holds the engine-default chosen-row pair `select[multiple] > option:checked, select[multiple] > optgroup > option:checked { background-color: #c8c8c8; color: #000000; }`, 12.56:1 as `select_interaction` measures it, Chromium's inactive selected pair, hardcoded since the engine holds no tokens (packages/blitz-dom/assets/default.css:161-188).»
- line: .andromeda/architecture-detail/established-decisions.md:123
  old: «with no test of its own,»
  new: «held by its own `open_state` (the attribute at construction, a write and a removal through the mutator, a `summary`'s click and its Enter),»
- line: .andromeda/architecture-detail/established-decisions.md:141
  old: «held by the windowed `seven_guis_native` smoke boot and `focus_redraw`»
  new: «held by the windowed `seven_guis_native` smoke boot, `focus_redraw` and blitz-shell's unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read (packages/blitz-shell/src/event.rs:144-171)»
- line: .andromeda/architecture-detail/established-decisions.md:153
  old: «`change` synthesised after `input` on checkbox/radio (packages/blitz-vibey-script/src/runtime/events.rs:171-188)»
  new: «a checkbox's or radio's `change` comes from the engine, the runtime synthesising none, so a page's script hears one `change` (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836)»
- after: .andromeda/architecture-detail/established-decisions.md:161
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-dioxus-integration -->»
  add: «»
  add: «- A measured hazard of a task spawned from a key handler, no engine change: such a task that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`), so Cells calls its handles synchronously. Any app meets it; measured headlessly through the harness's `poll(None)`, the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md).»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = "from neither on a native checkbox or radio"

[[pattern]]
id = "arch2-02"
fixed = "an option is not clicked"

[[pattern]]
id = "arch2-03"
regex = "options are not modelled"

[[pattern]]
id = "arch2-04"
fixed = "not asserting both layout modes"

[[pattern]]
id = "arch2-05"
fixed = "focus fixup is not built"

[[pattern]]
id = "arch2-06"
fixed = "no mutator hook is added"

[[pattern]]
id = "arch2-07"
fixed = "an `optgroup`'s are not listed"

[[pattern]]
id = "arch2-08"
fixed = "chosen and its select's other options not"

[[pattern]]
id = "arch2-09"
fixed = "whose parent is a `select` although"

[[pattern]]
id = "arch2-10"
fixed = "so a select lays out as a box"

[[pattern]]
id = "arch2-11"
fixed = "with no test of its own"

[[pattern]]
id = "arch2-12"
fixed = "`change` synthesised after `input`"
