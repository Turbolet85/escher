# Task — a11y · 2026-10-31-select-popup

## The chunk
A closed single `select` opens a painted list of its options in every document and renderer, the list a new engine overlay in paint, hit testing and keys (capability `v030-17`).
A person or an agent chooses an option from the open list by pointer and by key, the select then reading that choice through the one write path `apply_select_choice`; an option of the open list can be acted on through the driver, and the list closes without a choice.
It restates the lint's `overlaps` exclusion and the `covered` refusal of a closed single select's option with its schema text, since an open list's options read boxes of their own, and it paints or reads a group's label and `size`.
The focus and key contract, reduced motion and the engine's colours (a new default, the engine holding no tokens) go to the founder as P4 forks, under SC 2.1.1, 1.4.3 and 2.4.3.
The stand's tables that read `select#flight-type` move with any new row, and `.gitignore` gains `escher-0.4.0-incubator/`.

## Reached files
- packages/blitz-paint
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/events/keyboard.rs · edits a select's keys with a list open
- packages/blitz-dom/src/events/pointer.rs · edits a click on a select and on an option
- packages/blitz-dom/assets/default.css
- packages/blitz-dom/src/accessibility.rs
- packages/dioxus-native-dom/src/snapshot.rs · edits the options' `bounds`
- packages/escher-driver/src/execute.rs · edits the `covered` refusal's reading of an option
- packages/escher-driver/src/lint.rs · edits the `overlaps` exclusion
- packages/escher-driver/src/schema.rs · edits the schema help and the `covered` text
- packages/escher-driver/src/schema/spec.rs · edits the schema help and the `covered` text
- tests/blitz-tests/tests/stand_contrast.rs · edits its tables, with any new row for `select#flight-type`
- tests/blitz-tests/tests/stand_keyboard.rs · edits its tables, with any new row for `select#flight-type`
- tests/blitz-tests/tests/stand_act_select.rs · edits its tables, with any new row for `select#flight-type`
- tests/blitz-tests/tests/stand_lint.rs · edits its tables, with any new row for `select#flight-type`
- .gitignore · edits the incubator lines

## Needs
- The open list as an engine overlay in every document and renderer: `dioxus-native`'s windowed path shows the same list from the engine, the shell holding no second one and no separate OS window assumed.
- A focused single select's key rule as it stands: arrows, Home, End and typeahead (1 s of the document's clock) change the choice with no list open, every other key does nothing, each change fires one `input` then one `change`, and a key that changes nothing fires none; what these keys do with a list open, and whether a closed select's arrows keep moving the choice, is the P4 fork.
- Enter and Space activation: neither key's list holds a select today, so they open nothing, and the fork that changes this must act on those lists and on the disabled gate, which reads the presence of `disabled`.
- Focus with a list open under SC 2.4.3: a select is focusable and an option is not today, the engine's focus, the snapshot's `focused` and the tree's focus agree at each step, focus fixup applies when a node comes under `inert`, and `accessibility_active_descendant_unread` pins the builder reading no `aria-activedescendant`, which bounds any tree focus the fork gives an option.
- The pointer rule the list changes: a click on a select focuses it, an option of a closed single select takes no click, a disabled form control stops the walk, a `multiple` select's row click stays, and a double click still focuses as one click does.
- `apply_select_choice` as the one write path: a choice from the open list goes through it and fires the same one `input` and one `change`, no second write path.
- The engine's changed set: a select's changed choice marks it while hover, active, scroll and layout do not, and the tree refreshes only on a non-empty set, so an open or a close that the tree must report has to reach it.
- The tree saying whether the select is open: `expanded` is a state the builder reads from `aria-expanded` alone (true, false, nothing else), one read per attribute, its table in `accessibility_merged_states`, and a native select's open state holds no such attribute today.
- The hidden-content carve-out: an `option` under a `select` is excepted from `display: none` in the tree, `visibility: hidden` and `inert` still exclude, and an option reads its chosen state as `selected`; the list must not change which options the tree holds.
- The snapshot's option reading: a closed select's options read `bounds` equal to its client rect and an open list's read real boxes, while role, name, focus and `selected` still come from the tree, `enabled` and `value` from the element, with no second role or name mapping.
- The driver over an open list: an option can be acted on, the `covered` refusal and its schema text are restated, the `disabled` refusal holds where `enabled` reads `Some(false)` with nothing dispatched, and no verb, node key, cause, session error, exit status, carrier, listener, port, env var or workspace crate is added unless a fork names it.
- The actionable-key rule reaching the options: an element an agent can act on reads an author key and `unkeyed_actionable` returns nothing on every stand task, so options that become actionable through the driver fall under it on `select#flight-type`.
- The lint over an open list: the `overlaps` exclusion is restated for the options of a closed single select, those under an `optgroup` included, and the `contrast` reading keeps `Harness::text_contrast` and `stand_contrast`'s exempt rule.
- The list's colours under SC 1.4.3: each new engine default pair owes 4.5, or 3.0 for large text, read by `Harness::text_contrast`, which reads computed styles and returns `None` under opacity below 1, a filter or a background image, so colours painted outside computed styles reach no reader; the colours themselves are the P4 fork.
- `stand_contrast`'s three tables and counts (17 control rows, 14 `TEXT_ROWS`, 18 `STATE_ROWS`), the painted-pixel witness of a measured background, and the exempt rule, moving with any new row for the flight booker's popup.
- `stand_keyboard`'s stated orders (`tab_order`, `ENABLED_TAB_ORDER`) on the flight booker, which hold the select's Tab stop and move with any new row or key step.
- `stand_act_select` and `stand_lint`'s readings of `select#flight-type`, the flight booker's mode, whose options state its choice by `selected`; they move with any new row.
- A group's label: an `optgroup` reads `Role::Group`, its non-empty `label` its name, named in the tree and unpainted, `content: attr()` unbuilt, and its options are the select's own in tree order; this chunk paints or reads it.
- `size` and `multiple`: `size` is unread today and this chunk's to read; a `multiple` select's `set_multiselectable`, rows reading their own client rects, `value` reading `select_value`, keys and row clicks stay as built.
- Reduced motion: the engine holds no `prefers-reduced-motion` handling, so any motion of the list and its answer are the P4 fork.
- The option's visual states: the focus ring and not-colour-alone rules over the list's current and chosen option, which the focus contract and colour forks must answer.
- Hit testing under the overlay: the list's options take hits where they paint, including beyond the select's own box, a hit still passes through an element carrying `inert`, and removing `inert` restores the hit.

## Standing conventions
- S1 · A new accessibility or stand test file joins the `a11y` leg's named list and, where it is a stand or select check, the `stand-repeat` list, the stated count following · §9 CI Integration, .andromeda/a11y-plan-detail/ci-integration.md
- S2 · A stand or tree check runs in both layout modes · §3 A11y Assertion Harness Contract
- S3 · A state or contrast check is a stated table read by one shared reader with its row count asserted, roles and names stated once in the shared module · §3 A11y Assertion Harness Contract, §7 Screen Reader Support
- S4 · A keyboard check drives keys through the harness's `press` and `press_with` and reads the focused node back per step · §5 Keyboard Navigation, Test harness pattern
- S5 · An accessibility assertion reads the built tree after `resolve`, with the existing matchers and helpers · §3 A11y Assertion Harness Contract
- S6 · A check reading `has_changes()` as what a step changed drains the set first · §2 A11y Strategy
- S7 · A lint finding is written as ids, closed words and numbers only · §6 Visual Design Verification

## Not needed
- The AccessKit platform adapters, `accesskit_xplat`, adapter creation order and the windowed assistive-technology witness: no stand binary builds an adapter and the chunk adds none.
- The Dioxus bridge's list of boolean attributes: the chunk writes no new attribute path through that bridge.
- Language and direction, text alternatives and hidden decoration in fixtures, and IME: the list adds no document language, image or composition surface.
- Cognitive accessibility, error recovery and orientation cues, the flight booker's date inputs included: the chunk changes no error cue.
- Skip links, route-change focus restoration, live regions and heading hierarchy: the chunk adds no route, region or heading.
- The application and fixture markup inventories under `examples/assets`: the chunk edits none of that markup.
- Upstream sync work: the next epoch's.
