- line: .andromeda/architecture.md:85
old: «(packages/blitz-dom/src/stylo.rs:477-479; PROVISIONAL, [DOM semantics])»
new: «(packages/blitz-dom/src/stylo.rs:477-479; [DOM semantics])»
- line: .andromeda/architecture.md:86
old: «from `aria-selected`, `aria-pressed` and `aria-invalid`, read by value —»
new: «from `aria-selected`, `aria-pressed` and `aria-invalid`, read by value (a select's option excepted, its `selected` its chosen state, [DOM semantics]) —»
- line: .andromeda/architecture.md:86
old: «a `disabled` attribute on any other element not, and a click on any other element still clears focus»
new: «a `disabled` attribute on any other element not, a click on a `select` focuses it through its own arm of the walk (below), and a click on any other element still clears focus»
- line: .andromeda/architecture.md:103
old: «Focus keeps `:focus-within` on its ancestors and asks for a frame (PROVISIONAL):»
new: «Focus keeps `:focus-within` on its ancestors and asks for a frame:»
- line: .andromeda/architecture.md:137
old: «the fourteen driver-action checks `stand_act_*`»
new: «the fifteen driver-action checks `stand_act_*`»
- line: .andromeda/architecture.md:159
old: «naming a node from four sources»
new: «naming a node from five sources»
- line: .andromeda/architecture.md:159
old: «a `<label>` bound through `label_bound_input_element` (its `for`-target `<input>` by id, else its first nested `<input>`), pushed onto the bound input's `labelled_by` after the visit when both have built nodes (packages/blitz-dom/src/accessibility.rs:36-61; packages/blitz-dom/src/accessibility.rs:151-163)»
new: «a `<label>` bound through `label_bound_input_element` (its `for`-target labelable element by id, else its first nested labelable element — HTML's labelable elements, `button`, an `input` whose `type` is not `hidden` (ASCII case-insensitive), `meter`, `output`, `progress`, `select` and `textarea`, matched by the private `is_labelable`), pushed onto the bound element's `labelled_by` after the visit when both have built nodes, a select's option's non-empty `label` attribute where it has no `aria-label` (packages/blitz-dom/src/accessibility.rs:36-61; packages/blitz-dom/src/accessibility.rs:151-163; packages/blitz-dom/src/document.rs:677-715; packages/blitz-dom/src/document.rs:3126-3140)»
- line: .andromeda/architecture.md:159
old: «also reads state attributes on every element, the toggle by role,»
new: «also reads state attributes on every element — a select's option excepted, whose `selected` is its chosen state, true or false, with no `aria-selected` read (packages/blitz-dom/src/accessibility.rs:171-181) —, the toggle by role,»
- line: .andromeda/architecture.md:160
old: «`DomEventKind::from_str` strips `on` and maps `composition` to `Ime` (packages/blitz-traits/src/events.rs:165-212)»
new: «`DomEventKind::from_str` strips `on`, maps `composition` to `Ime` and `change` to `Change` (packages/blitz-traits/src/events.rs:165-212); `DomEventKind::Change`, after `Input`, and `DomEventData::Change(BlitzInputEvent)` — named `change`, not cancelable, bubbling, with no UI event mapped and no default action — are dispatched by the engine only after `input` on a select's choice, a checkbox, radio or text input firing no engine `change` ([Scrolling and input]) (packages/blitz-traits/src/events.rs:150; packages/blitz-traits/src/events.rs:250-251; packages/blitz-dom/src/events/mod.rs:93; packages/blitz-dom/src/events/mod.rs:246-248)»
- line: .andromeda/architecture.md:192
old: «`Reason` (PROVISIONAL) from private modules»
new: «`Reason` from private modules»
- line: .andromeda/architecture.md:192
old: «`type`'s help still says it replaces what the element holds and that an empty text clears it;»
new: «`type`'s help still says it replaces what the element holds and that an empty text clears it, and says that on a select the text chooses the first option whose label starts with it (packages/escher-driver/src/schema.rs:208-209);»
- line: .andromeda/architecture.md:198
old: «- The visual lint, in process only (PROVISIONAL):»
new: «- The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part):»
- line: .andromeda/architecture.md:205
old: «`value` the editor text, else — for a range»
new: «`value` a `select`'s chosen option's value (`BaseDocument::select_value`, below), else the editor text, else — for a range»
- line: .andromeda/architecture.md:205
old: «writes from `aria-selected`, `aria-pressed` (or `aria-checked` on a checkable role) and `aria-invalid` (§Standard Contracts → Mutation, query and CSSOM)»
new: «writes from `aria-selected`, `aria-pressed` (or `aria-checked` on a checkable role) and `aria-invalid` (§Standard Contracts → Mutation, query and CSSOM) — a select's option's `selected` its chosen state, so a choice names the old and the new option with `selected` in `differs`»
- line: .andromeda/architecture.md:287
old: «packages/blitz-dom/src/document.rs:1911-1915; PROVISIONAL)»
new: «packages/blitz-dom/src/document.rs:1911-1915)»
- line: .andromeda/architecture.md:289
old: «nothing for a move that changes nothing (packages/blitz-dom/src/events/keyboard.rs:333-354)»
new: «nothing for a move that changes nothing (packages/blitz-dom/src/events/keyboard.rs:333-354), and `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead changed its chosen option and nothing for a choice of the chosen option (packages/blitz-dom/src/events/keyboard.rs:271-297)»
- line: .andromeda/architecture.md:297
old: «the first partial take ratified, the second PROVISIONAL»
new: «both partial takes ratified»
- line: .andromeda/architecture.md:349
old: «the in-process visual lint, PROVISIONAL)»
new: «the in-process visual lint)»
- line: .andromeda/architecture.md:354
old: «2 tests in both layout modes (tests/blitz-tests/tests/stand_act_transformed.rs:76-116), `stand_screenshot_repeat`»
new: «2 tests in both layout modes (tests/blitz-tests/tests/stand_act_transformed.rs:76-116), a fifteenth, `stand_act_select`, a select driven by id — at boot its node, `ComboBox` with its name and value and its options as its children, `type` with a label choosing that option, a click then a key, the Tab walk skipping a disabled select, a disabled select and an unchosen option refused — 5 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:79-256), the engine check `select_interaction`, no `stand_*` file — the select's box, its seed, keys and typeahead over one second of the document's clock, a disabled select, a re-seed — 7 tests in both layout modes (tests/blitz-tests/tests/select_interaction.rs:113-343), `stand_screenshot_repeat`»
- line: .andromeda/architecture.md:354
old: «by `stand_settle`, by the fourteen `stand_act_*`»
new: «by `stand_settle`, by the fifteen `stand_act_*`»
- line: .andromeda/architecture.md:354
old: «and by `stand_replay`, twenty-five readers, and holding»
new: «and by `stand_replay` — the readers counted below —, and holding»
- line: .andromeda/architecture.md:363
old: «`tests/session_common/mod.rs`, by 27, and `tests/fixtures/mod.rs`»
new: «`tests/session_common/mod.rs`, by 28, and `tests/fixtures/mod.rs`»
- line: .andromeda/architecture.md:363
old: «by 21 (20 `stand_*` files and `font_face_sniffed_source`)»
new: «by 23 (21 `stand_*` files, `font_face_sniffed_source` and `select_interaction`)»
- line: .andromeda/architecture.md:363
old: «(tests/blitz-tests/tests/fixtures/mod.rs:17-107) — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
new: «(tests/blitz-tests/tests/fixtures/mod.rs:17-107) — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
