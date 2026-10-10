# Codebase Research — 2026-10-10-snapshot-states

## Scope
- **Depth:** deep on the four units the chunk edits, targeted elsewhere · **Reads:** 31 files (16 source, 9 check files, 2 flow scripts, 2 dependency sources, the cookbook, the harness rule) · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, its 1 Session Addition included (a gate entry bounded by `timeout N` never reads green on exit 124); `.claude/rules/testing.md` — 7 Session Additions read, three applied: no deleting key in a stand check, count result lines only under `--no-fail-fast`, search `tests/` and `examples/*/tests/` for a table's name before calling a standing check unaffected. No entry of this chunk is a live leg.
- **Platform issues consulted:** none — no runner-only bullet stands in scope (CI on HEAD read green at P3, below) and no entry outside the operator leg reads CI.
- **External inputs:** `inputs#I1` — who decides which fork (the operator's directive at the take-up) · `inputs#I2` — the founder's ruling of 2026-10-10 as the overseer relayed it, with the option as it was read to him.
- **A spec read, not an input:** WAI-ARIA 1.2, `https://www.w3.org/TR/wai-aria-1.2/`, downloaded whole at P3 (1,410,696 bytes, 2026-10-10) and read locally; what it states is quoted under "The value grammars". The page states no case rule for a state's value.

## Measured first — each state seen absent on HEAD
Taken through the stand's own binary, `target/debug/escher-session` built at HEAD `f0cb9dd7` (`cargo build -p seven_guis --bin escher-session --locked`), one command per run on a hosted session, 2026-10-10. Every snapshot answered exit 0.

| Reading | What the snapshot's text holds | Verdict |
|---|---|---|
| Flight booker, one-way (boot), 1269 bytes | `Button "one-way flight" id="flight-one-way" enabled @240,252 160x32` · `Button "return flight" id="flight-return" enabled @400,252 160x32` | — |
| Flight booker, after `click flight-return`, 1268 bytes | the same two lines, byte for byte | **the active mode reads nowhere**: the two mode-button lines are equal in both modes; the step's diff names `flight-return-date` alone |
| Flight booker, `type flight-start "next week"` | `TextInput "Departure date" id="flight-start" enabled value="next week" focused @239,297 322x36` | — |
| Flight booker, `type flight-start "24.12.2026"` | `TextInput "Departure date" id="flight-start" enabled value="24.12.2026" focused @239,297 322x36` | **invalid reads nowhere on the field**: the two lines differ in the value alone; the invalid step's diff names `flight-start` and `flight-book` (Book turning disabled — the side effect the first trial read) |
| CRUD at boot, 2034 bytes | `GenericContainer "Mustermann, Max" id="crud-person-1" @25,142.796875 367x29` (rows 0 and 2 alike; `crud-list` reads `GenericContainer ""`) | — |
| CRUD, after `click crud-person-1`, 2045 bytes | the three row lines, byte for byte | **the selected row reads nowhere**: the step's diff names `crud-name`, `crud-surname`, `crud-update`, `crud-delete` and no row |

The keys of every changed node in those answers were `bounds · enabled · focused · id · name · parent · role · value` — no key for any of the three states. The two boot sizes equal the figures architecture records (1269, 2034).

## Files inspected
- `packages/blitz-dom/src/accessibility.rs` (full, 281 lines) — `build_accessibility_node` (`:64-109`) sets, on an element node, the role (`role` attribute through `role_from_name`, else the element mapping, `:76-85`), the HTML tag, a label from a trimmed non-empty `aria-label` (`:88-94`) and the hidden flag from `aria-hidden == "true"` (`:96-99`). It writes no state property. The file holds no `cfg` line and no log site, so both builds compile one body. `role_from_name` maps `option` → `ListBoxOption`, `listbox` → `ListBox`, `button`, `radio`, `radiogroup`, `tab`, `tablist`, `row`, `gridcell` (`:127-189`); it holds no `switch`.
- `packages/dioxus-native-dom/src/snapshot.rs` (full, 591 lines) — `NodeState` is four fields (`:46-60`), `Default`-derived; the reader builds it at `:140-147`, the one full literal: `enabled` and `checked` from the element, `focused` from the tree's focus. `tree_node` (an AccessKit `Node`) and `element` are both in hand at that site. 13 unit tests, built by `initial_build` then `resolve(0.0)`; an attribute the bridge never writes is set through `mutate().set_attribute` (`:518-528`).
- `packages/dioxus-native-dom/src/snapshot_text.rs` (lines 1-200, 244-292) — `write_line` (`:48-76`) writes role, name, `id=`, then `enabled|disabled`, `checked|unchecked`, `value=`, `focused`, then bounds; the rustdoc of `to_text` states that order in prose and in an example (`:23-35`). Its tests build `NodeState` with `..NodeState::default()` or `..state()`, so a new defaulted field compiles into them unchanged; `state_tokens_follow_the_model` (`:142` on) pins the tokens one by one.
- `packages/dioxus-native-dom/src/snapshot_diff.rs` (lines 1-150) — a node is `changed` when `was.state != node.state` among five comparisons (`:76-81`); `DiffNode.state` is the whole `NodeState` (`:44`). Its tests build nodes with `NodeState::default()` (`:157`, `:236`).
- `packages/dioxus-native-dom/src/mutation_writer.rs` (`:14-42`, `:413-447`) — `BOOLEAN_ATTRIBUTES` is 27 names, none an `aria-*` name (`grep -n -i aria` over the file: 0 hits); a falsy value of a name outside the list is written by `set_attribute` as its text (`:439-445`). So `aria_pressed: false` reaches the DOM as `aria-pressed="false"`. The `set_attribute` trace site (`:421`) prints the attribute's name and value at debug behind the crate's `tracing` feature.
- `packages/dioxus-native-dom/src/actionable.rs` (`:1-120`) — the interactive role list holds `Button`, `ListBox` and `ListBoxOption`; an element is reported only when its id is a positional path (`:63`), so a keyed row or list that gains an interactive role is still not reported.
- `packages/escher-driver/src/schema.rs` (`:24-36`, `:192-235`, `:565-580`) — `NODE_FIELDS: [&str; 9]` = `id, parent, role, name, enabled, checked, value, focused, bounds` (`:33-35`), pinned word for word by a unit test (`:570-575`). The help of `changed` says "role, name, state, bounds or parent" (`:199-200`) and `snapshot`'s says "id, role, name, state and bounds" (`:221`): neither lists the state fields.
- `packages/escher-driver/src/json.rs` (`:80-180`, `:404-462`) — `NodeParts` (`:94-105`) and `node` (`:109-137`) write a node's keys in `NODE_FIELDS`' order, an optional one left out when absent; `nodes` (`:152-173`) fills the parts from a `DiffNode`. `a_node_is_written_with_the_schema_fields_it_has` (`:412-446`) pins the full line, the bare line and `keys(&full) == NODE_FIELDS`.
- `examples/seven_guis/src/tasks/flight_booker.rs` (`:20-200`) — the two mode buttons (`:67-78`) are `button` elements with `id`, a class switched on `is_return`, and `onclick`; the two date inputs (`:80-94`) carry `aria_label` and a class switched on `start_valid` / `return_valid`. `return_valid` is true whenever the flight is one-way (`:52`). The styles select `.type-btn-active` (`:167`) and `.date-input.invalid` (`:193`).
- `examples/seven_guis/src/tasks/crud.rs` (`:28-142`, `:183-200`) — a row is a `div` with `key: "{person.id}"`, `id: "crud-person-{person.id}"`, a class switched on `selected() == Some(i)` and `onclick` (`:67-79`), inside `div { id: "crud-list", class: "list" }` (`:57`). `selected` is an index into `people`, cleared by a filter edit (`:49`) and by Delete (`:135`). The style selects `.list-item.selected` (`:196`).
- `examples/seven_guis/src/tasks/cells.rs` (`:350-393`, by grep) — a cell's class switches on `is_selected` (`:375`); Cells is not one of the four tasks `seven_guis::stand` boots.
- `tests/blitz-tests/tests/common/mod.rs` (full) — `controls(task)` lists the 15 controls with their roles; the CRUD rows and `crud-list` are in neither `controls` nor `INPUT_NAMES`, and `rendered(Crud)` lists `crud-list` by id only. A role on the rows or the list moves no row of these tables.
- `tests/blitz-tests/tests/stand_act_diff.rs` (`:100-225`) — states each step's exact `[added, removed, changed]` id sets: `click flight-return` → changed `flight-return-date` (`:124`); `click flight-one-way` → the same (`:131-134`); `click crud-person-3` → changed `crud-update`, `crud-delete` (`:197-200`). **Three tables move** once a mode button reads pressed and a row reads selected.
- `tests/blitz-tests/tests/stand_diff.rs` (`:225-345`) — its CRUD and counter steps assert `added` / `removed` sets and `keeps_every_node`, and compare each diff with the file's own reading of the two snapshots; no assertion states the changed set of a flight choice or a row selection.
- `tests/blitz-tests/tests/stand_act_ids.rs` (`:20-270`) — 77 nodes, 70 accepted and 7 refused; of the 15 controls 8 change the screen at boot (`changes_at_boot`, `:36-43`: `flight-return` among them, `flight-one-way` not — "the choice already made"). An attribute alone adds or removes no node, and a click on the already active mode still changes nothing.
- `tests/blitz-tests/tests/stand_snapshot_text.rs` (`:31-38`, `:98-100`, `:150-330`) — ceilings 1024 · 1536 · 1536 · 2304 per task; a token is matched whole (`holds`); `text_start.contains(" value={date:?} ")` (`:252`) needs a space after the value, which bounds or any later token gives.
- `tests/blitz-tests/tests/stand_snapshot_state.rs` (`:1-140`) — the per-control state check v010-05 names: state read through `node(&harness.doc.snapshot(), id).state`, real input through `click` and `type_text`, `for incremental in [false, true]`, an in-file fixture where the lean tasks lack a case.
- `tests/blitz-tests/tests/stand_act_spans.rs` (`:596-626`, by grep) — pins the call count (27) and one `diff.changed.len() == 1` on the counter; no count on a flight or CRUD step that this chunk changes.
- `tests/blitz-tests/tests/stand_flight_booker.rs` (`:1-24`, by grep) — the non-colour invalid cue check reads the `invalid` class of `.date-input` and Book's `disabled`; seven check files select rows by `.list > .list-item`. The three classes stay.
- `examples/seven_guis/tests/flows/flight.sh` (`:29`, `:36`, `:54`) and `counter.sh` (`:36`) — glob patterns over the answer: `id=\"flight-start\" enabled value=\"01.01.2026\"` then `id=\"flight-book\" enabled` in the text; `"id":"flight-start",`…`"value":"next week",`…`"id":"flight-book",`…`"enabled":false,` in the JSON. A token or key written after `value` leaves all four matching; one written between `enabled` and `value` on `flight-start` breaks `:29`.
- `examples/seven_guis/tests/cli_commands.rs` (`:119-258`, by grep) — asserts each answer's top-level keys against the schema's fields, not a node's keys. `examples/seven_guis/tests/host_log.rs` (`:35-62`, by grep) reads its id and name needles from the CRUD screen it boots in process (at least 15 ids).
- `.github/scripts/ci-leg.sh` (`:34`) — the `a11y` leg names three test targets.
- AccessKit 0.25.1 (the locked version, `Cargo.lock:22-23`), `src/lib.rs` in the cargo registry — `Node::is_selected() -> Option<bool>` / `set_selected(bool)` (`:2449`, by `bool_property_methods!`, `:1743-1747`), `toggled() -> Option<Toggled>` / `set_toggled` (`:2467`), `invalid() -> Option<Invalid>` / `set_invalid` (`:2457`); `Toggled { False, True, Mixed }` (`:486-490`), `Invalid { True, Grammar, Spelling }` (`:465-469`). Its own doc: "ARIA uses separate states for checked and pressed controls, while this property is interpreted based on the node's role" (`:2459-2466`) — **one property serves `aria-pressed` and `aria-checked`**. For `selected`: "The absence of this flag (as opposed to a `false` setting) means that the concept of 'selected' doesn't apply" (`:2438-2439`).
- web_atoms 0.3.0 (`Cargo.lock:9319-9320`), `local_names.txt` — `aria-checked` (`:69`), `aria-invalid` (`:89`), `aria-pressed` (`:102`), `aria-selected` (`:112`) are atoms, so `local_name!("aria-pressed")` compiles as `local_name!("aria-label")` does.
- dioxus-html 0.7.10, `src/attribute_groups.rs` — `aria_invalid` (`:1608`), `aria_checked` (`:1625`), `aria_pressed` (`:1652`), `aria_selected` (`:1661`) and `role` (`:338`) are global attributes an `rsx!` element takes.

## The value grammars (WAI-ARIA 1.2, read at P3)
- **`aria-pressed`** — used in role `button`; tristate: `false` "supports being pressed but is not currently pressed", `mixed`, `true`, `undefined` (default) "does not support being pressed". "If the attribute is not present, the button is not a toggle button."
- **`aria-selected`** — used in roles `gridcell`, `option`, `row`, `tab`; inherits into `columnheader`, `rowheader`, `treeitem`; `true` / `false` / `undefined` (default) "The element is not selectable."
- **`aria-invalid`** — used in roles `application`, `checkbox`, `combobox`, `gridcell`, `listbox`, `radiogroup`, `slider`, `spinbutton`, `textbox`, `tree`; inherits into `columnheader`, `rowheader`, `searchbox`, `switch`, `treegrid`; token: `grammar`, `false` (default), `spelling`, `true`. "Any value not recognized in the list of allowed values MUST be treated by user agents as if the value true had been provided. If the attribute is not present, or its value is false, or its value is an empty string, the default value of false applies." The page notes the state "is being deprecated as a global state in ARIA 1.2".
- **`aria-checked`** — used in roles `checkbox`, `menuitemcheckbox`, `option`, `radio`, `switch`; tristate.
- **Not stated by the page:** whether a state's value is compared case-sensitively (its one case sentence is about role names). No browser was measured.
- **What follows for the stand:** a `button` supports `aria-pressed`, a text `input` (role textbox) supports `aria-invalid`, and a `div` with no role supports neither `aria-selected` nor any of the three — a row needs a role from the list above for `aria-selected` to be valid markup.

## Graph impact (from the code-graph query; trace `tree-query-2026-10-10-snapshot-states.json`, 2 queries, rust plane)
- **`build_accessibility_node`** — 1 caller, `build_accessibility_tree @ packages/blitz-dom/src/accessibility.rs:27`. A state written there is on every element node the tree keeps.
- **`build_accessibility_tree`** — 3 production callers: the trait's default `accessibility_tree @ packages/blitz-dom/src/document.rs:159`, `handle_blitz_shell_event @ packages/blitz-shell/src/application.rs:82` and the Dioxus override `accessibility_tree @ packages/dioxus-native-dom/src/dioxus_document.rs:301`; 9 call sites in three check files (`accessibility_hidden`, `accessibility_names`, `accessibility_roles`).
- **`accessibility_tree`** — 3 production callers: the shell's `update_tree @ packages/blitz-shell/src/accessibility.rs:45` (the platform adapter's feed), `unkeyed_actionable @ packages/dioxus-native-dom/src/actionable.rs:57` (reads roles only) and `snapshot @ packages/dioxus-native-dom/src/snapshot.rs:84`; 18 call sites in nine check files (the shared `session_common/mod.rs` among them). **So the builder's new properties reach the platform adapter through `update_tree` and `handle_blitz_shell_event`, and the snapshot through its one call.**
- **`NodeState`** — referenced in four files, all in dioxus-native-dom: `lib.rs` (the re-export), `snapshot.rs` (2), `snapshot_diff.rs` (4), `snapshot_text.rs` (19). No file of escher-driver names the type; `json.rs` reads its fields through `DiffNode.state`.
- **`DiffNode`** — `snapshot_diff.rs` (7), `lib.rs` (1), `packages/escher-driver/src/json.rs` (2), and three check files (`stand_act_diff`, `stand_act_timer`, `stand_diff`, 2 each).
- **`NODE_FIELDS`** — `schema.rs` (1, the definition), `json.rs` (2, the unit test) and `lib.rs` (1, the re-export). **`NodeParts`** — `json.rs` only (5).
- The line column of the trace is 0-indexed; the lines cited here are the editor's.

## Patterns detected
- **An ARIA attribute is read by value beside its spec URL** (`packages/blitz-dom/src/accessibility.rs:88-99`): `aria-label` is trimmed and an empty one names nothing; `aria-hidden` is the exact text `"true"`. Each carries its spec link in a comment above it.
- **A snapshot field has one reader, and the tree-read ones equal the tree** (`packages/dioxus-native-dom/src/snapshot.rs:138-147`): role and name come from `tree_node`, `focused` from the tree's focus, `enabled` and `checked` from the element because the tree carries neither.
- **A state check names exactly the controls whose reading moved** (`tests/blitz-tests/tests/stand_snapshot_state.rs:55-75`): the snapshot's reading is asserted equal to the accessibility tree's where the tree carries it.
- **A driver step is held against its stated id sets and against the other layout mode** (`tests/blitz-tests/tests/stand_act_diff.rs:119-144`): `step(&call, [added, removed, changed])`, then `assert_modes_agree`.
- **An engine accessibility check is a parsed HTML page and a match on the node's property** (`tests/blitz-tests/tests/accessibility_hidden.rs`, `accessibility_roles.rs`; `build_accessibility_tree` after `resolve`).

## Conventions to follow
- **Both layout modes and both builds:** every scenario runs `for incremental in [false, true]`; every new or restated check is read under `cargo test -p blitz-tests --locked --test {name}` and under `cargo test --workspace --locked --test {name}` (the workspace build compiles blitz-dom with more features and its tracing call sites in).
- **One test target per file** under `tests/blitz-tests/tests/`, auto-discovered; the manifest gains no `[[test]]` table (`BlitzTestsTargetsTest` pins it). A `stand_` prefix enters `run stand` with no script change.
- **A driver assertion's message carries a layout mode, a task and a row index** — never an id a call supplied or what a screen read (`tests/blitz-tests/tests/stand_act_ids.rs:93-97`).
- **A state change is proven with clicks and typed characters**, never a deleting key (`.claude/rules/testing.md`, Session Additions).
- **Each new test is read red once** — on the unchanged tree, or under a named mutation of the reader it holds — and the reading goes to the chunk's `evidence/`.
- **Rustdoc under `-D warnings`**: every new public field carries a doc line; escher-driver denies missing docs.

## New files to create
- `tests/blitz-tests/tests/accessibility_states.rs` — the engine's reading of the three attributes on parsed HTML: each value of each grammar, an absent attribute, an element whose role does not support the attribute.
- `tests/blitz-tests/tests/stand_act_states.rs` — through `Session::run`, in both layout modes: the step that changes each of the three states returns that node in its diff.

## Files to modify
- `packages/blitz-dom/src/accessibility.rs` — the builder reads the three state attributes and writes them on the element's accessibility node, each beside its spec URL
- `packages/dioxus-native-dom/src/snapshot.rs` — three fields join `NodeState`, read at the one construction site; unit tests per field
- `packages/dioxus-native-dom/src/snapshot_text.rs` — the three states' tokens in the fixed order, the rustdoc of `to_text` restated, the token pins extended
- `packages/dioxus-native-dom/src/snapshot_diff.rs` — a unit test that a state change alone names its node `changed`; the doc of what "state" holds
- `packages/escher-driver/src/schema.rs` — `NODE_FIELDS` gains the three keys, its pin restated with the new length; the help of `text` and of `changed` restated with their word-for-word pins
- `packages/escher-driver/src/json.rs` — `NodeParts`, `node` and `nodes` write the three keys in the table's order; the node test restated
- `packages/escher-driver/src/lib.rs` — the crate docs' paragraph on what an element reads gains the three states and the two limits, beside the limit it already states
- `examples/seven_guis/src/tasks/flight_booker.rs` — the two mode buttons and the two date inputs state their state by attribute
- `examples/seven_guis/src/tasks/crud.rs` — the rows take `role="option"` and state their selection by attribute, the list takes `role="listbox"`
- `tests/blitz-tests/tests/stand_snapshot_state.rs` — three checks through real input: pressed, selected and invalid read on exactly the control that holds each, in both layout modes
- `tests/blitz-tests/tests/stand_act_diff.rs` — the three stated id sets that move, restated with the node each step now returns

## Sweep records
- `NodeState {` (struct-literal sites; `grep -rn 'NodeState {' --include=*.rs packages tests examples`): 13 hits · 2 changed (the definition and the one full literal, `snapshot.rs:46`, `:140`) · 11 no-change (`snapshot_text.rs`, each closed by `..state()` or `..NodeState::default()`).
- `"enabled"|"checked"|"focused"` in `packages/escher-driver/src` (the node-key sites): `schema.rs:34`, `:573` and `json.rs:100-126`, `:162-164`, `:412-457` — all changed with the table; `execute.rs` and `lib.rs` hold no node key.
- `flight-return"|flight-one-way|crud-person` over six stand check files (`stand_diff`, `stand_act_diff`, `stand_act_ids`, `stand_act_spans`, `stand_snapshot_text`, `stand_snapshot`; the exact-set pins): 18 hits · 3 changed (`stand_act_diff.rs:124`, `:132`, `:198`) · 15 no-change (added / removed sets, row lookups, the boot table of `stand_act_ids`).
- `GenericContainer` over the stand checks, both shared modules and seven_guis' tests and flows: 0 hits — no check pins a row's or the list's role by name.
- `invalid|type-btn-active|list-item` over `tests/blitz-tests/tests/stand_*.rs`: the class selectors and the `invalid` class read of `stand_flight_booker` — 0 changed; the classes stay as the style hooks.

## CI on HEAD, read again
`f0cb9dd7` — green · checks 16/16 · wall 719 s · CI#38066501931 (`ci.py conclusion`, at P3; in progress at Setup).

## Not measured
- The platform adapter's reading of any state: neither stand binary builds one, and the shell's refresh on change has no windowed witness on this host (a11y-plan §2, as the extract states it).
- Whether `set_attribute` on an `aria-*` attribute marks the engine's changed set as any attribute write does — stated by architecture, not re-read; no criterion of this chunk rests on the set (the checks read the returned diff).
- The by-level line counts and byte sizes of `host_log` after the markup change — the gate's reading, taken at implement under both builds.
- A browser's comparison of a state's value by case.

## Open questions
- none — the three carried to P4 (how the engine reads the states, which roles the stand takes, which fields, tokens and keys carry them) were answered by the operator there (`inputs#I3`, `inputs#I4`; scope.md §Decisions owed), and the two lists above were settled to those answers: the model gains no public type of its own, so the crate root's re-export line is not edited, and the tokens are written after `value=`, so neither shell flow is.

## Read at P4, after the answers
- `packages/escher-driver/src/schema.rs:409-440` — the unit test `every_verb_states_its_help_its_arguments_and_a_result` holds three help strings word for word, among them the help of `snapshot`'s `text` (`:421`) and of `changed` (`:425`), and asserts that the five acting verbs share one `changed` help. Its other held copy of each text is the table row itself (`:227`, `:199-200`); `grep -rn 'one line per element\|whose role, name, state' packages examples tests` finds no further copy of either help (two rustdoc lines use the same words: `snapshot_diff.rs:20`, `execute.rs:21`).
- `tests/blitz-tests/tests/stand_actionable_keys.rs:41-70` — the check's own reader counts an element actionable by focusability, a listener, or one of `Button`, `TextInput`, `Slider`; it pins no count against the library's. `crud-list` gains a role the library's list holds (`ListBox`) and stays keyed, so `unkeyed_actionable` still returns nothing on the CRUD — the gate's reading decides.
