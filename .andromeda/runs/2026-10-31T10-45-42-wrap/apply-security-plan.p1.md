# Apply — security-plan · part 1 of 2

## Verdicts
- fanout-security-plan--cascade-g2.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:186#1
- fanout-security-plan--cascade-g2.md 4 · applied · .andromeda/security-plan-detail/input-validation.md:200#1
- fanout-security-plan--cascade-g3.md 1 · applied · .andromeda/security-plan-detail/input-validation.md:222#1
- fanout-security-plan--cascade-g3.md 2 · applied · .andromeda/security-plan-detail/input-validation.md:223#1
- fanout-security-plan--cascade-g3.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:224+#1
- fanout-security-plan--cascade-g3.md 4 · applied · .andromeda/security-plan-detail/input-validation.md:227#1
- fanout-security-plan--cascade-g3.md 5 · applied · .andromeda/security-plan-detail/input-validation.md:227+#1
- fanout-security-plan--cascade-g3.md 6 · applied · .andromeda/security-plan-detail/input-validation.md:236#1
- fanout-security-plan--cascade-g3.md 7 · applied · .andromeda/security-plan-detail/input-validation.md:236+#1
- fanout-security-plan--cascade-g3.md 8 · applied · .andromeda/security-plan-detail/input-validation.md:239#1
- fanout-security-plan--cascade-g3.md 9 · applied · .andromeda/security-plan-detail/input-validation.md:239#2
- fanout-security-plan--cascade-g3.md 10 · applied · .andromeda/security-plan-detail/input-validation.md:240#1
- fanout-security-plan--cascade-g3.md 11 · applied · .andromeda/security-plan-detail/input-validation.md:240#2
- fanout-security-plan--cascade-g3.md 12 · applied · .andromeda/security-plan-detail/input-validation.md:240#3
- fanout-security-plan--cascade-g3.md 13 · applied · .andromeda/security-plan-detail/input-validation.md:241+#1
- fanout-security-plan--cascade-g3.md 14 · applied · .andromeda/security-plan-detail/input-validation.md:241+#2
- fanout-security-plan--cascade-g3.md 15 · applied · .andromeda/security-plan-detail/input-validation.md:241+#3
- fanout-security-plan--cascade-g3.md 16 · applied · .andromeda/security-plan-detail/input-validation.md:241+#4
- fanout-security-plan--cascade-g3.md 17 · applied · .andromeda/security-plan-detail/input-validation.md:241+#5
- fanout-security-plan--cascade-g3.md 18 · applied · .andromeda/security-plan-detail/input-validation.md:257+#1
- fanout-security-plan--cascade-g3.md 19 · applied · .andromeda/security-plan-detail/input-validation.md:260+#1
- fanout-security-plan--cascade-g3.md 20 · applied · .andromeda/security-plan-detail/input-validation.md:380+#1

## Edits
- line: .andromeda/security-plan-detail/input-validation.md:186
  old: «no test asserts `:open`, while `details_element` and the a11y leg read green»
  new: «`open_state`'s 4 tests assert `:open` in both layout modes: read from the attribute at construction, moved by a write and a removal through the mutator, and flipped by a summary's click and its Enter (tests/blitz-tests/tests/open_state.rs:54-73; tests/blitz-tests/tests/open_state.rs:75-104; tests/blitz-tests/tests/open_state.rs:106-123; tests/blitz-tests/tests/open_state.rs:125-143), as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/security-plan-detail/input-validation.md:200
  old: «on each element node whose markup states them, closed enums and booleans, never an attribute's value as typed (`toggled` from `aria-checked` on a checkable role and from `aria-pressed` on every other;»
  new: «on each element node whose markup or native checked state states them, closed enums and booleans, never an attribute's value as typed (`toggled` from `aria-checked` on a checkable role and from `aria-pressed` on every other, and on a native checkable from `checkbox_input_checked()` on a checkable role and nothing under any other role, neither author attribute read on it, so `pressed` never begins to read on one;»
- line: .andromeda/security-plan-detail/input-validation.md:222
  old: «and its native checked state reaches no tree node either — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next) —»
  new: «and its native checked state reaches the tree node under a checkable role and under no other (the native checked-state row below) —»
- line: .andromeda/security-plan-detail/input-validation.md:223
  old: «each attribute is read once and nothing else writes `toggled`»
  new: «each attribute is read once and nothing else but a native checkable's own state, under a checkable role, writes `toggled`»
- after: .andromeda/security-plan-detail/input-validation.md:224
  anchor: « `False` and `Mixed` read `Some(false)`), and `.invalid`, a `bool` (a grammar or spelling error reads invalid), written by `Snapshot::to_text` as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid` and by `Outcome::to_json` as booleans, crossing the session socket and the client's stdout inside the ratified carriers (the Driver session socket row) and, since 2026-10-10-mcp-surface, MCP tool results on the `mcp` role's stdout — a whole-screen `snapshot` result is `to_text`'s output, a paged one its JSON line `{"text","part","parts"}` holding the same lines, and an acting call's result its JSON line, so the states cross there as the same closed words and booleans, never the attribute's text; the password and file mask is untouched (packages/blitz-dom/src/accessibility.rs:182-236; packages/blitz-dom/src/accessibility.rs:370; packages/blitz-dom/src/accessibility.rs:418-438; packages/dioxus-native-dom/src/snapshot.rs:63-74; packages/dioxus-native-dom/src/snapshot.rs:170-183; packages/dioxus-native-dom/src/snapshot.rs:220-232; packages/dioxus-native-dom/src/snapshot_text.rs:112-124; packages/escher-driver/src/json.rs:156-162; tests/blitz-tests/tests/accessibility_states.rs:141-444) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** a native checkbox's or radio's checked state in the tree»
  add: «  - **How (stack-specific):** In every document, remote pages included: `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never begins to read on it, and neither author attribute is read on a native control; a closed value carrying no attribute text, the added lines holding no log site, print, env read or panic path; pinned by `accessibility_states` 6, what an assistive technology reads unmeasured — an engine change for every document, ratified (packages/blitz-dom/src/accessibility.rs:199-224) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/security-plan-detail/input-validation.md:227
  old: «a known limit: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next) (packages/blitz-dom/src/node/node.rs:563-580;»
  new: «a node that holds focus when it comes under `inert` gives it up: `DocumentMutator::flush`, where `focus_node_id` resolves to a node reading `is_inert()`, calls `BaseDocument::clear_focus`, no `inert` arm joining the attribute writes, so a subtree moved under an inert box is covered too, `disabled` and `hidden` unmoved — an engine change for every document, ratified (packages/blitz-dom/src/mutator.rs:1051-1063; packages/blitz-dom/src/node/node.rs:563-580;»
- after: .andromeda/security-plan-detail/input-validation.md:227
  anchor: «  - **How (stack-specific):** Read by its presence from parsed content — remote pages included — on the element or an ancestor element, through the crate-private `Node::is_inert`, an ancestor walk (ratified by the founder, 2026-10-18): an inert node is not focusable, so the Tab walk, autofocus (`process_added_subtree`) and `DioxusDocument::unkeyed_actionable` skip it; `hit_inner` answers no hit at an element carrying `inert`, before any other test; and the accessibility builder's `is_hidden_from_accessibility_tree` reads true for it, so its subtree leaves the tree the shell hands the adapter and the snapshot; a presence check only — no value read, no text carried out, no print, log site, env read or panic path, and no carrier, verb, key name, field, log field or span added; a known limit: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next) (packages/blitz-dom/src/node/node.rs:563-580; packages/blitz-dom/src/node/node.rs:1437-1446; packages/blitz-dom/src/accessibility.rs:279; packages/blitz-dom/src/accessibility.rs:283-296; tests/blitz-tests/tests/inert_subtree.rs:77-363)»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** the focus an inert subtree loses, and the tree's `focus`»
  add: «  - **How (stack-specific):** In every document, remote pages included: `build_accessibility_tree` writes the tree's `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build; the inert focus clear fires no DOM `blur` or `focusout`; closed values, the added lines holding no log site; pinned by `inert_subtree` 6, what an assistive technology reads unmeasured — an engine change for every document, ratified (packages/blitz-dom/src/accessibility.rs:104-110; packages/blitz-dom/src/accessibility.rs:122; packages/blitz-dom/src/mutator.rs:1051-1063) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/security-plan-detail/input-validation.md:236
  old: «a click on a `select` focusing it (the `select` row) and a click on any other element unchanged»
  new: «a click on a `select` focusing it (the `select` row), a click on a `multiple` select's row reaching `click_select_row` through an `option` arm (the `select` with `multiple` row), a checkbox's and a radio's arms sending `change` after `input` (the engine's `change` row) and a click on any other element unchanged»
- after: .andromeda/security-plan-detail/input-validation.md:236
  anchor: « `handle_click`'s submit arm became a button arm: a `button` of any type or an `input` of type `submit`, `reset` or `button` is focused through `generate_focus_events` / `set_focus_to`, its form owner submitted when it is a submit button, the walk ended matched, a disabled form control still stopping the walk first, a `disabled` attribute on any other element not, a click on a `select` focusing it (the `select` row) and a click on any other element unchanged (packages/blitz-dom/src/events/pointer.rs:938-950; the focus and submit in the private `click_button` since 2026-10-16-handler-splits-and-readme, packages/blitz-dom/src/events/pointer.rs:1056-1073); the gates are a closed element-name and type match and `ElementData::is_disabled`, the presence check on a form control (packages/blitz-dom/src/events/keyboard.rs:260; packages/blitz-dom/src/events/keyboard.rs:277; packages/blitz-dom/src/events/pointer.rs:760), with no print, log site, env read, `unwrap`, `expect` or panic in the added engine lines (the census probes read 0); no carrier, verb, key name (`KEY_NAMES` unchanged), field, log field or span is added — an engine change for every document and a boundary widening, ratified by the founder: his answers to the chunk's forks of 2026-10-13 «Клавиши нажимают (Рекомендую)» and «Клик даёт фокус (Рекомендую)», and his confirm of the wrap's answer card on 2026-10-14 «Подтвердить (Рекомендую)» — each by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label, the English wording the overseer's — as measured at escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** the engine's `change` on a checkbox and a radio»
  add: «  - **How (stack-specific):** In every document, remote pages included: `handle_click`'s checkbox and radio arms send, after `input`, a `DomEventData::Change` with the same value, the value class `input` carries — a checkbox always, a radio only where it was not checked before the click; `blitz-vibey-script`'s runtime synthesizes no `change`, so a page's script hears one `change`, from the engine; no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added; pinned by `checkable_change_event` 3 (a click and a Space on a checkbox each fire `input` then `change`), `text_range_no_change_event` 2 and `blitz-vibey-script`'s `radio_click_fires_change_only_where_it_was_not_checked` — an engine change for every document and a boundary widening, ratified (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/security-plan-detail/input-validation.md:239
  old: «read the `option` children in tree order (an `optgroup`'s not listed)»
  new: «read the select's `option` children and those of its `optgroup` children, in tree order (the `optgroup` row)»
- line: .andromeda/security-plan-detail/input-validation.md:239
  old: «the last chosen, else the first option not disabled (packages/blitz-dom/src/node/element.rs:472-478;»
  new: «the last chosen, else the first option not disabled; a `multiple` write or removal records the select, so removal settles it to one, and `set_option_selectedness` leaves a `multiple` select's other options alone (packages/blitz-dom/src/mutator.rs:1262-1271; packages/blitz-dom/src/node/element.rs:472-478;»
- line: .andromeda/security-plan-detail/input-validation.md:240
  old: «stated limits: a `multiple` select and an `optgroup`'s options are not modelled, no popup is painted and an option is not clicked (refused `covered`)»
  new: «stated limits: no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select has no click arm and stays refused `covered`, under a group too, and `size` is not read»
- line: .andromeda/security-plan-detail/input-validation.md:240
  old: «the rules module's 4 unit tests, `select_interaction` 7 and `stand_act_select` 5, both layout modes»
  new: «the rules module's 8 unit tests, `select_interaction` 18 and `stand_act_select` 10, both layout modes, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/security-plan-detail/input-validation.md:240
  old: «the driver's `type` on a select is the same typeahead»
  new: «the driver's `type` on a single select is the same typeahead (a `multiple` select's, its own row below)»
- after: .andromeda/security-plan-detail/input-validation.md:241
  anchor: « tests/blitz-tests/tests/stand_act_select.rs:84-261) — an engine change for every document and a boundary widening, ratified — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** `select` with `multiple` — its keys and clicks»
  add: «  - **How (stack-specific):** In every document, remote pages included: a closed key match, `press_multiple_select_key`: ArrowDown · ArrowUp · Home · End alone choose that row alone, with Shift span from the anchor (with no cursor yet, from the last chosen row), with Ctrl or Meta move the current row and change nothing; a Space with no typed text standing flips the current row and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; Ctrl+A is not modelled; a click on a row, `click_select_row` behind `handle_click`'s `option` arm: a disabled select stops it, it focuses the select and clears the typeahead and text selection, plain chooses the row alone, Ctrl or Meta flips it, Shift spans, a disabled row changes nothing; disabled rows are never chosen and every choice passes `apply_select_choice`, the one write path, taking a set; the added lines hold no log site, print, env read or panic path, and no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span is added; pinned by `select_interaction` and `stand_act_select`, counted at the `select` row — an engine change for every document and a boundary widening, ratified (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:351-380; packages/blitz-dom/src/events/pointer.rs:1013-1054) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:241
  anchor: « tests/blitz-tests/tests/stand_act_select.rs:84-261) — an engine change for every document and a boundary widening, ratified — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** `optgroup`, its `label`»
  add: «  - **How (stack-specific):** In every document, remote pages included: read by a closed element-name match: `options` walks a select's `option` children and its `optgroup` children's, in tree order, `is_select_option` holding under a group; `owner_select` and `option_disabled`, the one rule `readings` and form submission share (packages/blitz-dom/src/form.rs:281); `maybe_record_node`'s `optgroup` arm records its select; a group takes `Role::Group` and its non-empty `label` is its name in the tree, unpainted, inside the ratified name crossing; pinned by `select_interaction`, `stand_act_select` and `form`'s tests — an engine change for every document and a boundary widening, ratified as to the list through a group and not as to the `label` as a name (packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/mutator.rs:1253-1257) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:241
  anchor: « tests/blitz-tests/tests/stand_act_select.rs:84-261) — an engine change for every document and a boundary widening, ratified — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** a `multiple` select's chosen set, where it leaves»
  add: «  - **How (stack-specific):** The set leaves only as form entries, as `input` and `change` values — `multiple_select_values` fills `values` with one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set, `value` staying `select_value` — as the snapshot's `value`, `select_value`'s first chosen option in tree order, and as the options' tree nodes, the select taking `set_multiselectable`; no carrier, verb, field, log field or span added — an engine change for every document and a boundary widening, ratified (packages/dioxus-native-dom/src/dioxus_document.rs:31-45) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:241
  anchor: « tests/blitz-tests/tests/stand_act_select.rs:84-261) — an engine change for every document and a boundary widening, ratified — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** the driver's `type` on a `multiple` select»
  add: «  - **How (stack-specific):** `focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead; a text opening on a space types nothing, since no label can match and the Space would flip a row; `Target` holds `multiple`, and no verb, argument, key name, result field, cause, session error or exit is added; `type`'s help is fixed text — a boundary widening, ratified, with the `select` with `multiple` row (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239; packages/escher-driver/src/schema.rs:239-240) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:241
  anchor: « tests/blitz-tests/tests/stand_act_select.rs:84-261) — an engine change for every document and a boundary widening, ratified — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** the public select readers `BaseDocument::select_of` · `select_values` · `clear_select_typeahead`, `dioxus_native_dom::closed_select_of` · `is_multiple_select`»
  add: «  - **How (stack-specific):** Each reader taking a node goes through `get_node`, a stale id reading none, and `clear_select_typeahead` takes no node; `closed_select_of`'s callers are the snapshot's builder and `lint.rs`, `is_multiple_select`'s `closed_select_of` and `execute.rs` `locate`, and `clear_select_typeahead`'s one non-test caller `execute.rs` `focus_multiple_select`; no type, verb or field is added (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/document.rs:752-755; packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:257
  anchor: «  - **How (stack-specific):** Since 2026-10-19-visual-lint a read-only reader on every document, remote pages included: an id that does not resolve, a node that is no inline root or one with no inline layout data reads `None` through `get_node`, never an index; it mutates nothing and has no log site, print, env read or panic path; it returns geometry only, and its one non-test caller is `Session::lint`, in process and through the `lint` verb's arm (packages/blitz-dom/src/text_extent.rs:18-54)»
  add: «- **Boundary:** Layout values»
  add: «  - **What to validate:** a clipping box's visible region (`BaseDocument::visible_region`)»
  add: «  - **How (stack-specific):** On every document, remote pages included: `visible_region` reads each clipping box through its transforms, by the private `padding_box_in_view` over `transformed_bounds`; with no transform on the chain it reads the padding box in the client rect's frame, and a mapped box with a non-finite edge answers `None`; exact under a translate and a scale, under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show; its callers are `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region`; the added lines hold no log site, print, env read or panic path; pinned by `visible_region_transformed` 6, `stand_act_transformed` 3 and `transformed_client_rect` — an engine change for every document, ratified (packages/blitz-dom/src/scrolling.rs:790-812; packages/blitz-dom/src/document.rs:2574-2578) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:260
  anchor: «  - **How (stack-specific):** Geometry whose transformed bounds are non-finite or beyond f32::MAX is culled before painting (packages/blitz-paint/src/render.rs:404-419)»
  add: «- **Boundary:** Paint values»
  add: «  - **What to validate:** a non-invertible transform under a hit test»
  add: «  - **How (stack-specific):** On every document, remote pages included: `Node::hit_inner` answers `None` for the whole subtree of a transform whose determinant is 0 or not finite, so it takes no hit; the added lines hold no log site, print, env read or panic path; pinned by `hit_non_invertible_transform` 2 — an engine change for every document, ratified (packages/blitz-dom/src/node/node.rs:1472-1477) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:380
  anchor: «  - **How (stack-specific):** Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59); cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:29-31; examples/seven_guis/src/tasks/cells.rs:147; examples/seven_guis/src/tasks/cells.rs:230-233); circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:111-114); the flight booker's mode `select#flight-type` writes its `flight_type` signal by a closed match on the `input` value — `return` a return flight, any other value one-way — over its two options (`one-way`, `return`) (examples/seven_guis/src/tasks/flight_booker.rs:66-89)»
  add: «- **Boundary:** Example apps»
  add: «  - **What to validate:** 7GUIs Cells, the `cells-grid` keys»
  add: «  - **How (stack-specific):** `cells-grid` takes `tabindex` 0 and an `onkeydown` with a closed key match and no role: arrows move `current`, clamped, Enter opens the editor on `current`, focused, and the editor's Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early (examples/seven_guis/src/tasks/cells.rs:425-433), ratified as to the grid — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»

## Patterns
[[pattern]]
id = "sec1-01"
fixed = 'no test asserts `:open`'

[[pattern]]
id = "sec1-02"
fixed = 'whose markup states them'

[[pattern]]
id = "sec1-03"
fixed = 'reaches no tree node'

[[pattern]]
id = "sec1-04"
fixed = 'nothing else writes `toggled`'

[[pattern]]
id = "sec1-05"
fixed = 'focus fixup is not built'

[[pattern]]
id = "sec1-06"
fixed = 'the `select` row) and a click on any other element unchanged'

[[pattern]]
id = "sec1-07"
fixed = 'read the `option` children in tree order'

[[pattern]]
id = "sec1-08"
fixed = 'are not modelled'

[[pattern]]
id = "sec1-09"
fixed = 'an option is not clicked'

[[pattern]]
id = "sec1-10"
fixed = '`select_interaction` 7'

[[pattern]]
id = "sec1-11"
fixed = '`stand_act_select` 5'

[[pattern]]
id = "sec1-12"
fixed = '4 unit tests'

[[pattern]]
id = "sec1-13"
fixed = 'on a select is the same typeahead'
