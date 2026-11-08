@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — made every Dioxus element node carry its stable id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node; dioxus-native-dom's `accessibility` feature gates its own tree override. The read's per-node id is this `author_id`, so the read sits behind the same gate.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — `disabled` was keyed two ways: state and click target read presence, focusability the parsed bool. It also first cleared a falsy Dioxus `disabled`. Trap: a `disabled` reading keyed on the value disagrees with `:disabled`. The read's `disabled` cause must use the presence keying, never the value.

@history_item n=3 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree `accessibility_tree` returns, with no second role or name mapping. This is the origin of the rule that the actionable read takes its node set from that model and adds no mapping of its own.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — founded the actionable-key rule (every element that is focusable, has an interactive role or carries a listener reads an author key) and its check `DioxusDocument::unkeyed_actionable`, which takes the role from the tree. Trap: never satisfy the check by adding a `tabindex` or a role, because that moves the pinned Tab order; key the element instead. A folded key reading in the read must reuse this check.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled` from the presence of `disabled`, and a password reads `MASKED_VALUE`; a falsy Dioxus `hidden` no longer drops a node. The read's `disabled` cause rests on this `enabled`, and its answer must hold no value, masked or not.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — a file input's value is masked like a password's, and `Snapshot::to_text` is one more reader of the model with no second mapping. A precedent: a new reader of the model adds no mapping and leaks no value.

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the shell takes the changed set (`take_changed_nodes`) on a poll and rebuilds the tree when that set is non-empty; `has_changes()` is true while it is non-empty. The read runs no step, so it must neither drain this set nor depend on it.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document's set is non-empty. So the read's no-change check must drain first and then read `has_changes()` false.

@history_item n=9 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's `disabled` cause was worded on the presence reading, and the schema defines no role or name set of its own. Rule: whatever detects `disabled` reads the snapshot's `enabled`, never the attribute value. The read's cause must keep to this rule.

@history_item n=10 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table, and neither stand binary builds a platform adapter. The read in escher-driver reaches the graph, the snapshot and `unkeyed_actionable` through this edge and adds no adapter.

@history_item n=11 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `Session::run` refuses `click` or `type` as `disabled` when `enabled` is `Some(false)`, the first of its three screen-level checks, with nothing dispatched. `crud-delete` and `flight-return-date` are the stand's cases. The read's `disabled` cause must match this check in order and in reading, through the shared refusal path.

@history_item n=12 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the stand's listbox entry) — the CRUD list states `role="listbox"` and each row `role="option"`. The rows are not focusable, and the tree's focus is the list, never the option. The read must list CRUD's role-only options as their own nodes, not as select-owned rows.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result reads the snapshot model and nothing else, over the crossing the founder ratified. A precedent for carrying the read's answer over MCP: it reads the model only, with no second mapping.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — a password typed over MCP reads `MASKED_VALUE`, and the typed sentinel appears 0 times. The node-key help and `differs` are readers that use closed words only. The read's no-value assertion can follow this sentinel-count form.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` reads only the model's bounds, rounded outward and cropped to what can be seen. A shot changes nothing: no focus moves, `has_changes` stays false and the next diff is empty, in both modes. This is the pattern for the read's no-mutation proof and a basis for an off-screen reading from bounds.

@history_item n=16 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — disabled-ness is keyed one way, by presence. The CRUD list takes `tabindex="0"` and becomes a focusable Tab stop while its rows stay unfocusable. Enter and Space activate a focused control. The read's focusable leg and its `disabled` cause follow this keying, and the read must list `crud-list` as its own focusable node.

@history_item n=17 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*` file also runs three times in `stand-repeat` under the zero-failure budget. Trap: a new read check named `stand_*` joins `stand-repeat` by glob, which the binding says changes a11y-plan §9 and test-plan §10.

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable()` returns nothing on the non-lean tasks too (was 2 · 3 · 676). The elements that react to a click and state no role were counted: Circle Drawer 1, Cells 676. A folded key reading must re-assert empty on every task, and the read's set must agree with these counts.

@history_item n=19 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the tree reads `aria-disabled` as disabled, but the snapshot's `enabled` never reads it. So the read must not name `disabled` for `aria-disabled` alone, matching the driver's refusal.

@history_item n=20 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled`) — focusability became: not inert (`Node::is_inert`) and no disabled form control (`ElementData::is_disabled`). A disabled form control stops the click walk; a disabled ancestor `div` does not. An inert subtree leaves the tree and the snapshot, and a hit passes through it. These bound the read's focusable leg, its exclusions and its `covered` cause (an inert element never covers). The founder ratified them at Epoch 2 close.

@history_item n=21 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the lint's `contrast` was first in process only, then (the gated part) the `lint` verb on every carrier, recorded, with findings written as ids, closed words and numbers only. That was the founder's F1 «Новая команда lint». It is the precedent that a new read verb on every carrier is a ratified boundary widening, which makes the read's verb and carrier the founder's fork.

@history_item n=22 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a click focuses a select but an option is not clicked. An option under a `select` stays in the tree under `display: none`; `hidden`, `inert` and `visibility: hidden` still exclude it. `select_interaction` and `stand_act_select` were introduced. These bound which owner-served options the read lists and why they are refused.

@history_item n=23 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the driver's `snapshot` reads a screen over `SNAPSHOT_TEXT_BUDGET` in parts, on the founder's ratified paged answer. A precedent for the read's answer on Cells (676+ nodes) if it exceeds a carrier's budget.

@history_item n=24 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen files on the founder's word; eight also run in `stand-repeat`. The read's new check stays out of the fourteen unless the plan is amended.

@history_item n=25 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach under `## Accessibility reach`, and its pin `.github/scripts/test_capability_statement.py` reads none of those claims. Any change the read makes to the reach must be written into that document by hand.

@history_item n=26 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select) — `select#flight-type`, named `Flight type`, is the stand's select, with options `flight-type-one-way` and `flight-type-return`. A driver `click` on a flight-type option is refused `covered`. This is the stand's owner-served refusal case for the read.

@history_item n=27 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, the lint's option overlap, the scrollbar band) — `unkeyed_actionable` is empty on all six served tasks, the load page included; `load-done` is out of the tree while hidden. Right after a `scroll`, a click in the shown scrollbar band is a thumb drag that dispatches nothing until an `advance` passes the fade. The read's "acceptable now" answer must agree with that click.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' rows) — the load page and Cells got Tab, keyboard and contrast rows (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`), closing the served-pages gap. The read's stand proof must cover these two served tasks beside the lean four.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`multiple` select tree and snapshot; `optgroup`) — a `multiple` select writes `set_multiselectable`, and its rows and groups read their own client rects while a closed select's options read the select's rect. `optgroup` became `Role::Group` named by `label`; a disabled group's rows read disabled through `option_disabled`. Together these give the owner-served rows' bounds and the read's `disabled` and off-screen causes for them.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys and row clicks; the driver on a `multiple` select) — `click_select_row` sits behind `handle_click`'s `option` arm; a disabled row changes nothing; an option of a closed single select, under a group too, is not clicked and stays refused `covered`. The driver's `type` on a `multiple` select focuses with no click, and the closed sets hold no verb, cause or field for it. The read's verbs for owner-served rows must agree with this.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard) — `cells-grid` carries `onkeydown` with no role, so the reacting elements on Cells are 677: 676 cells plus the grid. The read must give the grid a key verb but no `click`.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended; a non-invertible transform) — `overlaps` finds a closed select's options through `closed_select_of` and never the role, so CRUD's `role="option"` rows are not taken as a select's. A non-invertible transform gives its subtree no hit. Owner-served detection follows the select, not the role, and `covered` and off-screen follow this hit rule.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list scope entity; what opens a list; a click, the pointer and focus on an open list) — `select_list` is an engine overlay that `hit_with_scrollbar` answers first. A row of an open list takes a click through `click_lands_on_row`; a group header chooses nothing; a disabled, inert or hidden select opens nothing. A closed single select's option is still refused `covered` (F9). The read's `click` and `covered` verdicts for owner-served nodes follow the list's open state.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded`; what the snapshot reads of an open list) — a single select writes `expanded` from engine state, not from `aria-expanded`. There is no `expanded` node key (`NODE_FIELDS`), the highlighted row is in no snapshot, and an open row's `bounds` come from `select_list_bounds`. The open state is not a snapshot reading, so the read must take it elsewhere or bring a new field to the founder.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against clips and `opacity`; the lint over an open list; a long list; the driver's `type` on any select) — a driver `click` on an element under an open list is refused `covered`. `Entry.in_select_box` finds rows through `closed_select_of` and `select_list_bounds`. The list's thumb takes no hit. `type` on any select focuses with no click and closes any list. The read's `covered` cause and verb table must agree with each of these.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands; the capability statement's named limits) — opening or closing a list marks the changed set, and a scroll or wheel with a list standing marks its select. The published limits now read: the fixup firing no `blur`, a single select's `size` unread, `aria-activedescendant` unread. The read must open or close no list, and the reach text it edits by hand keeps these limits.

@history_item n=37 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — measured that an element reacts to a click (the link-2 reading) and that the listener mark `data-dioxus-id` over-reports after a re-render, so `unkeyed_actionable` over-includes, conservatively. `dioxus_runtime_probe` runs in the workspace `test` leg, outside the `a11y` and `stand-repeat` legs. The read's actionable set must use current listeners, not the mark (the measured fork), and its check follows the probe's leg placement.

@history_item n=38 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`NodeState.clickable`) — built `clickable` from the vdom walk's current `click` listeners on no disabled form control, with `NODE_FIELDS` 13 → 14. `stand_non_lean_tasks` gained a `CLICKABLE` table (Circle Drawer 1, Cells 676, the others 0) beside the mark's 677. `unkeyed_actionable` still reads the mark. The read's clickable leg and its stand counts rest on this.

@history_item n=39 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader `DioxusDocument::graph`) — `UiGraph` holds one `GraphNode` per snapshot node, with id, `owner` (an option's or optgroup's select), handlers and states; its answer is ids, names, closed flag words and booleans, never a value. It is gated behind `accessibility`. `stand_graph` asserts the graph is a subset of the snapshot, no `///`, no mutation and no typed text. The read's node model and its proof shape are this graph's.

@history_item n=40 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration and the bridge dispatch's write record) — `use_graph_state` and `DeclaredWrite` exist, and the bridge sets the registry's dispatch around `runtime.handle_event`. The read dispatches nothing, so it must never set or reach the registry's dispatch, and it leaves `graph.rs`'s recording unchanged.
