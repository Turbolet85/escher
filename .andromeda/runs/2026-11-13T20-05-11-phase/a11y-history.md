@history_item n=1 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — (§5's click and range rows cite the named functions) the code audit's earlier complexity split moved `handle_click`'s button and range bodies into private `click_button` / `press_range_input` with no behaviour changed for any document; §5 rows were re-cited to the named functions and the a11y leg passed unchanged. Direct precedent for this chunk's restructure of `handle_click`: extracted helpers move the bodies rows cite, so cited functions/lines must be re-pointed while every claim (focus via `generate_focus_events` / `set_focus_to`, key order, `disabled` presence reads) stays.

@history_item n=2 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (Enter and Space activate a focused control, and a click focuses a button) established keyboard activation dispatching a synthetic Click whose app handlers run first, then `handle_click`; and a pointer click on a button or submit/reset/button input focuses it then submits a submit button's form owner, a `disabled` attribute stopping the walk, any other click clearing focus. Rests rows contract/keyboard-activation/* and decision/click-focuses-buttons, decision/keyboard-activation-enter-space: the restructured `handle_click` button arm must keep focus-then-submit and the handler order (guards `stand_keyboard`, `stand_act_keys`).

@history_item n=3 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (`disabled` by presence, the CRUD list's keys and the keyboard check) disabled-ness keyed one way by attribute presence (founder's «По наличию»), `stand_keyboard` added. Rows decision/disabled-by-attribute-presence and decision/disabled-by-presence: every disabled gate inside `handle_click`'s arms must keep reading presence, never the value.

@history_item n=4 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — disabled was keyed two ways (state/click target by presence, focusability by parsed bool). Trap behind the disabled-presence rows: a restructure that re-derives disabled-ness in a helper must not reintroduce a value parse.

@history_item n=5 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§5, §6: `inert` and `disabled` in focus order, keys and the hit) `disabled` narrowed to form controls via `ElementData::is_disabled`, used by pointer, click, activation and focus gates; a disabled form control stops the click walk while a disabled ancestor `div` does not; a `disabled` on `summary` or `a` gates neither key; `inert` takes focusability, hits and the tree away. Rows decision/disabled-control-stops-click-walk, decision/inert-subtree-excluded, decision/inert-element-takes-no-hit, contract/hit-testing/inert: the click walk's early stop order in `handle_click` must be preserved exactly.

@history_item n=6 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder, Epoch 2 close) ratified the click-walk stop on a disabled form control alone, the activation keys' disabled gate, `inert`, and the one-virtual-clock double click. These rows are ratified engine behaviour for every document; the restructure may not alter them.

@history_item n=7 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — two pointer clicks on one point within 500 ms and 2 px are a double click read on the document's clock (harness `VirtualClock`), each still focusing its target as one click does. Rows decision/double-click-threshold and decision/double-click-on-document-clock (guard `stand_virtual_clock`): click-count logic near `handle_click` must keep the same clock and thresholds.

@history_item n=8 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — a click focuses a range input and moves no value (a label's click too); a main-button pointerdown sets the value by position; each change fires one `input`. Rows contract/range-input/*, decision/range-click-focuses-not-value, decision/range-pointerdown-sets-value: the range arm in `handle_click` must stay focus-only (check `range_input_interaction`).

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`change` on a checkbox and a radio) after `input`, `handle_click`'s checkbox and radio arms dispatch a `Change` with the same value, a checkbox always and a radio only where it was not checked before; the script runtime synthesizes none, so one `change` from the engine. Rows decision/checkable-click-toggles-and-focuses and decision/one-engine-change-per-checkable-click: exactly the arm this chunk moves (checks `checkable_change_event`, `text_range_no_change_event`).

@history_item n=10 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a native checkbox's or radio's checked state in the tree) a native checkbox or radio on a checkable role writes `toggled` from its checked state; `accessibility_states`' `a_click_and_a_space_move_a_native_control_in_the_tree` moves it by a click. Rows contract/accessibility-tree-states/native-checkbox-or-radio and decision/native-checkable-toggle: the guard that holds the click arm's checked write.

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a `multiple` select's keys and row clicks) `click_select_row` behind `handle_click`'s `option` arm: a disabled select stops it, otherwise focus + clear typeahead/text selection; plain / Ctrl-Meta / Shift row semantics; an option of a closed single select not clicked. Row contract/multiple-select/keys-and-row-clicks: another `handle_click` arm the restructure must keep intact (check `select_interaction`).

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`optgroup`, its role, its name and its options) a disabled group's rows read disabled through `option_disabled`, the one rule readings and form entries share. Row decision/disabled-optgroup-one-rule: helpers extracted from the select arms must keep calling the one rule, not a copy.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a `multiple` select in the tree, the snapshot and the form values) `evt.values()` gives one form entry per chosen option of a `multiple` select on Input and Change, `value` staying single. Row contract/form-event-values/multiple-select: event payloads from the select arms must not change.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the driver on a `multiple` select) the driver's `type` on a `multiple` select focuses it with no click, adding nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. Precedent for row decision/driver-type-focuses-select-no-click inside `execute`'s per-verb dispatch.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`:open` asserted) `open_state` (4 tests) asserts a summary's click and Enter flipping `:open`. Row decision/open-pseudo-class-on-details-dialog: the `summary` arm of `handle_click` is held by `open_state`.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (`:open` matches an open `details` or `dialog`) activating a `details`'s first `summary` toggles `open` through the mutator, flipping `:open`, in every document. Origin of row decision/open-pseudo-class-on-details-dialog; the summary arm's toggle-through-mutator path must stay.

@history_item n=17 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a click focuses a select, an option is not clicked; a label's click reaches its bound labelable element; a select's changed choice marks the changed set; `input` then `change` per change. Origin of the select and label arms of `handle_click` and rows contract/single-select/change-events, decision/label-names-its-labelable-element.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list as a scope entity) the select list is an engine overlay on every document that a closed single select opens, answered first by hit testing and painted after the root. Row decision/single-select-engine-overlay-list; the list's opening lives in the click path.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a click, the pointer and focus on an open list) a click on a single select opens or closes its list, each focusing; a row of an open list takes a click through `handle_click`'s `option` arm behind `click_lands_on_row`; focus stays on the select while open; a label bound to a single select focuses it, opens nothing and re-dispatches nothing. Rows decision/focus-stays-on-select-with-list-open, decision/label-click-focuses-select-only: the most intricate `handle_click` arms this chunk moves (pins in `select_popup`).

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list's keys, choice and close) every choice goes through the one write path `apply_select_choice` (`choose_current_and_close`, `click_list_row`): one `input` then one `change`, the chosen option firing nothing; Escape, a second click, a press outside and a page scroll close unchanged with no event; a close clears typed text. Rows decision/select-list-choice-one-write-path, contract/select-open-list/*: helpers must keep the single write path.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what opens a single select's list) Enter and a Space with no typed text open the list; a disabled, inert, hidden or empty select opens nothing. Bears on the select arms and the keyboard-dispatched Click path the restructure must keep.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a single select's `expanded` in the tree; the changed set where a list stands) a single select writes `expanded` from its list's open state, reading no author attribute; its list opening or closing marks the changed set, a scroll closing it included. Rows decision/single-select-expanded-from-list, contract/aria-property-reads/aria-expanded, contract/accessibility-tree/single-select, contract/accessibility-tree/select-expanded, decision/changed-set-marks: changed-set marking on open/close in the click path must be kept.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a group's label painted as a header) a header row chooses nothing. Row contract/select-open-list/optgroup-header: a header click in the row-click arm must stay a no-op.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list against `:open`, clips and `opacity`) an open list is no `:open` state; a driver `click` on an element under the list is refused `covered`. Rows decision/open-select-list-not-open-state, contract/driver-click-refusals/covered: the `covered` refusal in `execute`'s check order must stay.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the driver's `type` on any select) `type` on any select focuses it via `focus_select` with no DOM focus events and closes any list (`Target.multiple` → `Target.select`). Row decision/driver-type-focuses-select-no-click: a branch of `execute`'s per-verb dispatch the restructure moves.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the list's checks and the legs they run in) `select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12 run in the workspace `test` leg and `stand-repeat`; the list's keyboard and focus coverage is headless. Row decision/select-list-coverage-headless: these are the existing checks that must stay green over the moved arms.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (how an open list's labels leave) a shot of a row crops to it and leaves the list open; labels leave only as pixels and the snapshot's name. Rows contract/driver-screenshot/row-shot, decision/option-labels-leave-only-as-pixels: the `screenshot` branch of `execute` must keep this.

@history_item n=28 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` is a reader of bounds only: an element shot by stable id is its bounds rounded outward and cropped, and a shot changes nothing (no focus, `has_changes` false, next diff empty). Rows contract/driver-screenshot/element-shot, decision/screenshot-changes-nothing: preserved through `execute`'s restructure.

@history_item n=29 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal is detected on the snapshot's `enabled` being `Some(false)`, the first of its screen-level checks, nothing dispatched, focus as it was. Rows contract/driver-actions/disabled, contract/driver-refusal/disabled: `execute`'s refusal order (`check_target`) must keep `disabled` first.

@history_item n=30 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the `disabled` cause is worded on the attribute's presence; rule: the detector reads the snapshot's `enabled`, never the attribute's value. Constraint on any refusal helper extracted from `execute`.

@history_item n=31 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot's `enabled` reads the presence of `disabled` from the element. Rows contract/snapshot/enabled, decision/enabled-by-disabled-presence: the reading the driver refusal rests on.

@history_item n=32 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on `dioxus-native-dom` unconditionally, no `[features]` table, because its executor reads the snapshot. Row decision/driver-requires-accessibility: the restructure must add no feature table or cfg split in `execute.rs`.

@history_item n=33 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the driver's closed spans form the obs sink's second record class; `execute.rs` citations re-pointed. The scope forbids changing span fields: helpers extracted from `execute` must keep the spans and their fields as they are.

@history_item n=34 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle (and the driver session's `act`) neither reads nor drains the changed set; a freshly booted document holds a non-empty set. Row decision/harness-settle-leaves-changed-set (guard `stand_settle`): `execute`'s act path must not start draining.

@history_item n=35 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the changed set's marking sites (mutation, focus or checked change, text input) and the refresh on change, ratified at 2026-10-07-driver-session. Origin of row decision/changed-set-marks: click-path marks must stay where they are.

@history_item n=36 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (2. A11y Strategy) `~ decision/changed-set-marks`: a scroll that closes a list marks its select; any other scroll does not. The current wording of the row this chunk's click path must keep.

@history_item n=37 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (5. Keyboard Navigation) `+ decision/hidden-checkable-label-toggle`: a native checkbox or radio keeps its checked state with no box, so a label click toggles a hidden control (guard `clicking_label_toggles_hidden_checkbox`). Bears on `handle_click`'s label and checkable arms.

@history_item n=38 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the served pages, their open gap, the lint's option overlap and the scrollbar's band) right after a `scroll` a pointerdown in the shown overlay scrollbar's band is a thumb drag and dispatches nothing, so a driver `click` needs an `advance` past the fade. Row decision/driver-click-in-shown-scrollbar-drags: the driver click path in `execute` must keep this outcome.

@history_item n=39 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — a `<label>` names its bound element (`for`-target else first nested), before or after it; widened to every labelable element at 2026-10-20-select-by-id. Row contract/accessible-name/label-element; the label-click forwarding arm shares that binding.

@history_item n=40 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule: every actionable element reads an author key; trap: never satisfy the check by adding `tabindex` or a role. Row decision/actionable-key-rule (guard `unkeyed_actionable`).

@history_item n=41 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the read-only UI graph reader DioxusDocument::graph) `DioxusDocument::graph`, behind `accessibility`, a `UiGraph` of one `GraphNode` per snapshot node in pre-order (id, owner, component, handlers with declared writes, states read) plus per-state declared/confirmed/flagged writes and readers; names and closed words only; `stand_graph` asserts graph ⊆ snapshot, no `///` id, a read changes nothing. Rows decision/ui-graph-one-node-per-snapshot-node, decision/ui-graph-carries-no-values, decision/agent-readers-behind-accessibility, contract/node-id/reserved-separator: the exact content `graph`'s restructure must preserve.

@history_item n=42 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (graph state registration and the bridge dispatch's write record) `use_graph_state` with `DeclaredWrite`s; writes flagged `undeclared` / `unconfirmed` by the stated rules; a page with no registered state reaches no registry. Rows contract/ui-graph-state/*: the flagging logic `graph` assembles must not shift.

@history_item n=43 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the «reacts to a click» snapshot field built as NodeState.clickable) the vdom walk's current listeners (`element_listeners`) feed `clickable` and the graph's handlers, so neither over-reports as the listener mark does. Row decision/listeners-from-vdom-walk: `graph`'s handler list must keep reading the walk, never the mark.

@history_item n=44 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the listener mark's over-report and the actionable-key check) a listener removed on a re-render leaves its event attribute and `data-dioxus-id` mark standing. The trap behind decision/listeners-from-vdom-walk for any helper extracted from `graph`.

@history_item n=45 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the one actionable rule and the check's listener leg) `unkeyed_actionable` and `actionable_nodes` share the crate-private `Readings`, listener leg the walk's exact current listeners holding an event other than `mounted`; the mark serves dispatch alone. Row decision/one-actionable-rule: no second rule may appear when `graph` is split.

@history_item n=46 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actionable-node reader) `actionable_nodes` lists the UI graph's nodes in snapshot pre-order that the rule holds for or whose `owner` is set, each `id`, `owner`, `keyed`. Row contract/actionable-nodes/listing: depends on `graph`'s node order and owner field.

@history_item n=47 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions verb as a read-only reader) the `actions` verb answers each verb with the cause `check_target` would refuse it with, `stale` for no element; changes nothing; ids and closed words only. Rows decision/actions-verb-changes-nothing, contract/actions-answer/*: `check_target` and its refusal order in `execute` are shared with `actions`, so the restructure must keep both answers identical.

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the keyed reading, provisional on the carriers) `keyed` on the `actions` answer, true where the id holds no `/`; ratified at 2026-11-13T07-43-19-wrap. Row decision/keyed-wire-form.

@history_item n=49 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (line citations the chunk's added lines moved) citations into escher-driver's `execute.rs` re-pointed after added lines. Precedent: moving code inside `execute.rs` / `graph.rs` / `pointer.rs` obliges a citation re-point at wrap.

@history_item n=50 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (cited source lines re-pointed after the chunk's edits) rows citing `execute.rs` and the dioxus-native-dom files re-pointed. Same citation-sweep precedent for this chunk's moved functions.

@history_item n=51 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the ungated reached record on handle_event's path) `record_reached` / `reached_handlers` ungated, written before `runtime.handle_event`, first-reached order, deduplicated; `element_listener_ids` ungated while `element_facts` stays gated. Rows contract/dioxus-document/reached-handlers, contract/dioxus-document/record-reached, decision/agent-readers-behind-accessibility: gating must not move if graph helpers are shared.

@history_item n=52 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (handler_coverage, a further reader of the UI graph; stand_handler_coverage's keyboard-activation check) `handler_coverage` reads `graph` at boot and after each step, `mounted` left out; `stand_handler_coverage` asserts a refused step exercises nothing and a keyboard activation exercises its control's click. Row contract/handler-coverage/answer: a check that holds `graph`, `execute`'s refusals and `handle_click`'s activation together.

@history_item n=53 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen files (`stand_keyboard`, `accessibility_states`, `inert_subtree`, `select_interaction`, `stand_act_select`, …), several also in `stand-repeat`. The leg set that must stay green over the moved arms; no leg list is to change.

@history_item n=54 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the a11y master's text became rows (241 rows), each verified against the code at `edd0cfaea7f0`. The basis of every picked row; any row whose guard or citation names a moved function is a lead for the wrap's re-point.
