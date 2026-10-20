# Codebase Research — 2026-10-20-select-by-id

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 12
- **Harness rules consulted:** `.claude/rules/testing.md` — 9 additions applied (auto-loaded on the probe file; the
  key names, the both-mode loop and the fixture-page placement rule read from it) · `.claude/rules/a11y.md` — 1 addition
  read, none applies (no windowed boot)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's directive: boundary-widening and direction forks go into one
  `founder-forks.md` in the run dir, the most conservative option marked, then a dialog to the operator; technical forks
  by dialog

## Measured (the residual's ask: a `select` under the driver's verbs first)
A throwaway probe held a fixture page — `button#fx-before` · `label[for=fx-pick] "Fruit"` · `select#fx-pick` (Apple /
Banana `selected` / Cherry) · `select#fx-named` `aria-label="Colour"` (Red / Green) · `select#fx-off` `disabled`
`aria-label="Off"` · `button#fx-after` — in a driver session (`fixtures::hold`) and on a booted harness, in both
layout modes, and was removed after the run. Source and readings: `evidence/select-probe.rs.txt`,
`evidence/select-probe-readings.txt`. Every reading below is identical in both modes.
- **A `select` lays out as nothing.** Its computed display is `inline`, its layout `0×0`, and
  `BaseDocument::get_client_bounding_rect` returns `None` for it. Each `option` computes `display: none` (UA rule
  `option { display: none; }`, packages/blitz-dom/assets/default.css:146-148). With every child out of flow the inline
  box holds no fragment, so the select has no rect (the `label` beside it, holding text, reads 35.48×23).
- **The snapshot leaves it out.** `Builder::visit` keeps an element only when its client rect resolves
  (packages/dioxus-native-dom/src/snapshot.rs:142-147), so neither select is a snapshot node: the boot snapshot lists
  `fx-before`, the label and `fx-after` alone. Every id-addressed verb on a select is refused `not-found` with nothing
  run — `click fx-pick`, `type fx-pick …`, `click fx-named`, `click fx-off`, `screenshot --id fx-pick`.
- **The accessibility tree holds it, empty.** Each select is a `ComboBox` node with 0 children (the options are
  excluded as `display: none`, accessibility.rs:244-257); `fx-named` and `fx-off` read their `aria-label`; `fx-pick`
  reads no label — a `<label>` binds an `input` alone (`label_bound_input_element`, document.rs:678-703, matching
  `local_name!("input")` on both arms); no node reads a value.
- **It takes focus and no key.** Both enabled selects are focusable and the disabled one is not
  (`is_focussable`): Tab walks `fx-before` → `fx-pick` → `fx-named` → `fx-after` → `fx-before`, the two select stops
  showing in the diff only as `fx-before` losing focus (tab 2) and an empty diff (tab 3). On a focused select
  `press arrow-down`, `home`, `enter` and `space` each settle with an empty diff.
- **What an option change could reach.** blitz-traits has no `change` event kind (`DomEventKind`,
  packages/blitz-traits/src/events.rs:140-158 — `Input` the one form event); the Dioxus bridge hands `Input` to the app
  as `NativeFormData { value }` (packages/dioxus-native-dom/src/dioxus_document.rs:377-380), so an `oninput` handler is
  the one a select change can reach, as the range model's does. Form submission skips a select (the WHATWG step is a
  `TODO`, packages/blitz-dom/src/form.rs:270-274).

## Files inspected
- `packages/blitz-dom/src/accessibility.rs` (10-90, 200-280, 320-345, 465-480) — the role mapping (`select` →
  `ComboBox`, `multiple` → `ListBox`, `option` → `ListBoxOption`), the label loop, the hidden-node exclusion
- `packages/blitz-dom/src/document.rs` (678-710) — `label_bound_input_element`, `input` alone
- `packages/blitz-dom/src/node/element.rs` (480-490, 715-745) — `can_be_disabled` and default focusability both
  name `select`
- `packages/blitz-dom/src/node/range.rs` (1-60) — the range model: `RangeInputData` held apart from the attribute
- `packages/blitz-dom/assets/default.css` (75-100, 140-150) — `select` only in the font rule; `option` display none
- `packages/blitz-dom/src/form.rs` (255-285) — the select entry `TODO`
- `packages/dioxus-native-dom/src/snapshot.rs` (20-262) — the node model, the bounds gate, the `value` reader
- `packages/dioxus-native-dom/src/dioxus_document.rs` (355-395) — events handed to Dioxus
- `packages/blitz-traits/src/events.rs` (140-160, 240-260) — the event kinds
- `packages/escher-driver/src/execute.rs` (100-360) — `click` / `type` resolve through the snapshot; `type` clicks,
  selects all where the snapshot reads a value, then types
- `packages/escher-driver/src/schema.rs` (18-31) — the twelve `KEY_NAMES`
- `tests/blitz-tests/tests/fixtures/mod.rs` (1-60) — `boot`, `hold`, the shared pages
- `tests/blitz-tests/tests/session_common/mod.rs` (1-110) — the call builders
- `.andromeda/residuals.md` (line 11) — the absorbed residual

## Graph impact (from the code-graph query)
- **label_bound_input_element** — 2 callers: `build_accessibility_tree` @ packages/blitz-dom/src/accessibility.rs:53
  and `handle_click` @ packages/blitz-dom/src/events/pointer.rs:860 — widening it to a `select` also moves what a click
  on its `<label>` does (focus moves to the select), in every document.
- **value** (snapshot.rs) — 1 caller, `Builder::visit` @ packages/dioxus-native-dom/src/snapshot.rs:170 — a select
  branch in the reader reaches every snapshot through that one site.
- **apply_range_move** — 2 callers, `press_range_input_key` @ events/keyboard.rs:153 and `press_range_input` @
  events/pointer.rs:672 — the one write · mark · `input` · redraw path a select change would copy.
- **flush_range_input** — 1 caller, `collect_layout_children_with_wrap` @ layout/construct.rs:436 — where a control's
  special data is seeded at box construction.
- Trace: `.andromeda/runs/2026-10-20T03-46-58-phase/tree-query-2026-10-20-select-by-id.json` (rust plane rebuilt,
  8647 nodes / 52389 edges; every row count > 0).

## Patterns detected
- **The range interaction model** (node/range.rs:1-60, events/keyboard.rs:99-153, events/pointer.rs:449-672,
  layout/construct.rs:932-944, mutator.rs:357-507): a `SpecialElementData` variant holding the current value apart from
  the attribute, seeded at box construction, re-seeded on a `value` write, moved by key and pointer arms gated on
  `is_disabled`, every change through one write · mark · `input` path.
- **A test-only control on a fixture page** (`fixtures::password_named`, fixtures/mod.rs:38-45; the textarea on
  `stand_snapshot_state`'s page): named by `aria-label`, because a `<label>` binds an `input` alone.
- **Driver resolution through the snapshot** (execute.rs:333-357): `click` and `type` find their target as a snapshot
  node; an element the snapshot does not list cannot be acted on, whatever the engine holds.

## Conventions to follow
- **Both layout modes**: every engine and fixture check runs `for incremental in [false, true]` (arch §Conventions).
- **Handler arms as private named functions**: `handle_click` / `handle_key_or_input_event` keep their signatures;
  a new arm is a private function (`press_range_input_key` precedent, events/keyboard.rs:140).
- **Key names**: the driver's are `arrow-up` · `arrow-down` · `home` · `end` (schema.rs:18-31) — `arrowdown` is
  refused `malformed` (measured).

## The answered forks (inputs#I2) and where each lands
The operator answered every fork with its recommendation, as delegate, PROVISIONAL for the founder's word: F1 keys and
typeahead through the verbs there are; F2 the engine shows the chosen option's label; F3 options read as the select's
children; F4 a `change` event, and a `<label>` names HTML's labelable elements. The sites below are read at HEAD:
- **The chosen option as `:checked`** — the radio group already writes a choice as the `CHECKED` element state through
  `snapshot_node_and` (document.rs:735, `toggle_radio`), which restyles `:checked` (stylo.rs:470 reads the state) and
  marks the changed set (document.rs:1660-1662); `ElementData` seeds `CHECKED` from a markup attribute at creation
  (element.rs:443-452, a checkbox's `checked`). HTML says a chosen `option` matches `:checked`, so the select's choice
  can be that state on its options — and a user-agent rule `select:not([multiple]) > option:checked { display: inline }`
  lays the chosen option's own text out as the select's content, with no special layout code (F2).
- **Seeding and re-seeding** — the mutator's attribute hooks re-seed a range input on a `value` write
  (mutator.rs:367-378 `set_attribute`, :490-500 `clear_attribute`); its tree entry points are `append_children`
  (:707), `insert_nodes_before` (:713), `insert_nodes_after` (:841), `replace_node_with` (:857), `remove_node` (:590)
  and `remove_and_drop_node` (:614) — where an option's `selected` write and an option's insertion or removal re-run
  HTML's selectedness setting algorithm on its select.
- **Keys and typeahead** — `handle_key_or_input_event`'s arms (events/keyboard.rs:80-116): the range arm is gated on
  `range_input_data` and no `disabled` (:99-104) and calls `press_range_input_key` (:140-154), which hands a move to
  `apply_range_move` (:238). A select arm sits beside it. `Harness::type_text` presses one `Key::Character` per
  character (blitz-test-harness/src/input.rs:222-226), and the driver's `type` first presses select-all (`Ctrl`/`Cmd`
  + `a`, execute.rs:497-499) on a control the snapshot reads a value for — a modified key the select arm must ignore.
  So `type --id <select> --text <label>` reaches typeahead with no driver code change.
- **A click focuses it** — `handle_click`'s walk (events/pointer.rs:840-900) has a `label` arm that re-dispatches the
  click to `label_bound_input_element`'s target (:858-866) and a button arm that focuses (`click_button`); a select arm
  beside them focuses the select and starts typeahead afresh.
- **`change`** — `DomEventKind` (blitz-traits/src/events.rs:140-158, `from_str` :190-210) and `DomEventData`
  (:240-260, `name` :268-310, `kind` :340-350) gain `Change`; the Dioxus bridge counts listeners per kind index in a
  64-slot array (mutation_writer.rs:23, :349-358) and hands `Input` to the app as `NativeFormData`
  (dioxus_document.rs:377-380) — `Change` takes the same form. blitz-vibey-script synthesizes its own script-side
  `change` for a checkbox or radio after an engine `Input` (runtime.rs:1671-1687); a real `Change` from a select must
  not be doubled there.
- **The label** — `label_bound_input_element` (document.rs:678-703) matches `input` on both arms; its two callers are
  the tree's label loop (accessibility.rs:53) and `handle_click`'s label arm (pointer.rs:860).

## New files to create
- `packages/blitz-dom/src/node/select.rs` — a select's options, its chosen option, HTML's selectedness algorithm, its value and the typeahead match
- `tests/blitz-tests/tests/select_interaction.rs` — the engine check of the select model, both layout modes
- `tests/blitz-tests/tests/stand_act_select.rs` — the driver check on a fixture page holding a select

## Files to modify
- `packages/blitz-dom/src/node/mod.rs` — declare the select module and re-export what dioxus-native-dom reads
- `packages/blitz-dom/src/node/element.rs` — seed `CHECKED` on an `option` from its `selected` attribute at creation
- `packages/blitz-dom/src/document.rs` — the typeahead state, the select choice write, the select value reader and `label_bound_input_element` widened
- `packages/blitz-dom/src/events/keyboard.rs` — the select key and typeahead arm and its change path
- `packages/blitz-dom/src/events/pointer.rs` — the select arm of `handle_click`
- `packages/blitz-dom/src/mutator.rs` — re-run the selectedness algorithm on an option's `selected` write and on option insertion or removal
- `packages/blitz-dom/src/accessibility.rs` — keep a select's options in the tree, each `selected` by its own selectedness
- `packages/blitz-dom/src/form.rs` — the select's entry in form submission
- `packages/blitz-dom/assets/default.css` — a select's inline-block box and its chosen option shown
- `packages/blitz-traits/src/events.rs` — the `Change` event kind and data
- `packages/blitz-vibey-script/src/runtime.rs` — a real `Change` not doubled by its own synthesized one
- `packages/dioxus-native-dom/src/dioxus_document.rs` — `Change` handed to the app as form data
- `packages/dioxus-native-dom/src/snapshot.rs` — the select's value and an option's bounds, with unit tests
- `packages/escher-driver/src/schema.rs` — `type`'s help sentence for a select
- `packages/escher-driver/src/schema_tests.rs` — the help pin of `type` (schema_tests.rs:75-78) moved with it
- `tests/blitz-tests/tests/accessibility_label.rs` — a `<label>` naming a `select` and a `textarea`

## Sweeps
- `select` over packages/blitz-dom/src and assets (git grep, case-insensitive, `selection|selector|select_` excluded):
  8 hits · 0 changed yet · 8 no-change at research (accessibility.rs:472 role map, element.rs:487 and :730 the
  disabled and focusable lists, mutator.rs:1072 the form-associated list, node.rs:824/836 comments, form.rs:270 the
  TODO, default.css:84 the font rule) — re-derived by `git grep -n -i select -- packages/blitz-dom/src
  packages/blitz-dom/assets`
- `"change"` over packages/ (git grep): 3 hits, all in blitz-vibey-script (its own script event) · 0 in blitz-traits
  or the Dioxus bridge — the absence of a `change` kind derived from that grep and from events.rs:140-158 read whole

- `select|option|label_bound_input_element|ComboBox|ListBoxOption|event_handler_counts` over tests/, examples/, apps/,
  wpt/ and the four crates' sources (git grep, case-insensitive): 2 standing checks read a `select` — 
  `accessibility_roles` (two empty selects, roles only: no-change, no option to choose) and the bridge's unit test at
  mutation_writer.rs:523-543 (`selected: off` clears the attribute: no-change, the attribute is untouched by the
  choice); 0 stand tasks hold a select; `stand_accessibility_ids` and `accessibility_states` name the roles on `div`s
  (no-change)

## Open questions
- none — the four forks are answered (inputs#I2); implementation-scope: the `CHECKED`-state mechanism for the chosen
  option is measured at implement's first step (a `:checked` rule's restyle in both layout modes) before the rest
  builds on it
