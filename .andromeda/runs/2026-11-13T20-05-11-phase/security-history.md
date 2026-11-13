@history_item n=1 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — (the range and keyboard-activation rows cite the named functions) This is the direct precedent for this chunk. The code audit's complexity split moved bodies into private functions (`take_sign`/`take_digits`/`take_exponent`, `press_range_input_key`, `press_range_input`, `click_button`) with no behaviour changed. The rows kept their claims and were re-pointed to cite where the code now stands, and the added lines carried no `unwrap`, `expect`, panic, print, `tracing` site or env read, so §Error Handling's panic-path list gained nothing. Bar for this chunk: every helper extracted from `handle_click`, `execute` and `graph` must meet the same no-panic, no-log rule, and any row that names a moved function gets a citation re-point.

@history_item n=2 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Added the keyboard-activation row: Enter and Space dispatch a Click that runs the app's handlers and then `handle_click`, so those keys reach form submission, link navigation and `summary` toggling. It also added that a button click focuses the button before a submit button submits its form owner (`click_button`), and made `disabled` read by presence. A ratified boundary widening and engine change. `handle_click`'s button, submit, link and summary arms hold decision/button-click-focuses, decision/enter-activates-focused-control and decision/space-keyup-activates-focused-control. Splitting `handle_click` must keep their order.

@history_item n=3 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — The `disabled` row: presence alone (`disabled="false"` included) makes an element ignore pointer selection and click default actions. This is the origin of decision/disabled-read-by-presence, which `handle_click`'s gates still read.

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (`disabled` on form controls; the password caret) `disabled` is read by presence on a form control only, through `ElementData::is_disabled`. Readers include `ClickTarget::Disabled`, `handle_click`'s walk gate and the activation arms, and a disabled ancestor stops no click walk. The password caret is placed by the bullets (`click_password_input`). These gates and the password click path sit in `pointer.rs`'s click handling, and the restructure must keep the exact gate and the walk-stop semantics (decision/disabled-read-by-presence, decision/button-click-focuses).

@history_item n=5 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the `inert` read and the toggle per role) Added the `inert` row: no focus and no hit (`hit_inner`), and the subtree leaves the tree and the snapshot. Also added the toggle read per role (`aria-checked` only on checkable roles; a native checkbox or radio reads neither attribute). This is the origin of decision/inert-subtree and decision/toggle-read-per-role, which hold for every document that `handle_click` acts on.

@history_item n=6 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder, Epoch 2 close) The founder ratified the presence reading of `disabled`, the toggle per role, `inert`, the activation keys' disabled gate, the password caret, the eight causes and `advance` reporting exactly `ms`. These are the ratifications behind several picked rows that the restructured functions must keep unchanged.

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the engine's `change` on a checkbox and a radio) `handle_click`'s checkbox and radio arms send `change` after `input` with the same value: a checkbox always, a radio only where it was not checked before. The script runtime synthesizes none. Pinned by `checkable_change_event` 3, `text_range_no_change_event` 2 and `radio_click_fires_change_only_where_it_was_not_checked`. These are the guard and arms of decision/engine-change-event-checkable that the restructure moves. The event order must be kept exactly.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a `multiple` select's keys, clicks and chosen set) `click_select_row` sits behind `handle_click`'s `option` arm. A disabled select stops it; it focuses the select and clears the typeahead and text selection. A plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing. Every choice goes through `apply_select_choice`, the one write path. Covers decision/multiple-select-keys-and-clicks, decision/select-choice-one-write-path and decision/multiple-select-set-exits (guards `select_interaction`, `stand_act_select`). The `option` arm's branches must survive the split unchanged.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `select` row's rules, limits and pins; `optgroup` and its `label`) Settling by the selectedness algorithm, options through `optgroup` in tree order, the one option-disabled rule shared by the readings and form submission, and the limits (an option of a closed single select has no click arm and stays refused `covered`). Pins: `select_interaction` 18, `stand_act_select` 10. These are the history of decision/single-select-selectedness, decision/optgroup-options-and-label and decision/option-disabled-one-rule, which the select arms of `handle_click` carry.

@history_item n=10 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`type` on a `multiple` select and the public select readers; `visible_region` through transforms) Added `focus_multiple_select` in the driver: focus with no click, then clear the typeahead. The select readers go through `get_node`, so a stale id reads none. `visible_region`'s callers are `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region`, pinned by `visible_region_transformed`, `stand_act_transformed` and `transformed_client_rect`. These are executor paths that `execute`'s restructure moves (decision/driver-type-on-select, decision/visible-region-through-transforms, decision/select-readers-resolve-node-closed-select-none).

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a non-invertible transform takes no hit; the focus an inert subtree loses and the tree's `focus`; a native checkable's checked state) `hit_inner` answers `None` for the subtree of a non-invertible transform. A focused node that comes under `inert` gives up focus with no `blur` or `focusout`. The tree's focus names only a built node, else the window. These are the origins of decision/non-invertible-transform-no-hit, decision/inert-focus-clear-silent, decision/accessibility-focus-built-nodes-only and decision/toggle-read-per-role, all of which must hold for every document after the restructure.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `open` row's test gap closed) `open_state`'s 4 tests assert `:open` in both layout modes, including when it is flipped by a summary's click and its Enter. The `summary` arm of `handle_click` is therefore held by a test (decision/open-pseudo-class-by-open-attribute).

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the `open` attribute read by presence) `flush_open_state` sets `ElementState::OPEN` from `open` on a `details` or a `dialog`. This is the origin of decision/open-pseudo-class-by-open-attribute. A summary click toggles the attribute through `handle_click`.

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the script fetch's `file:` refusal by document, a narrowing) `document_may_load` lets a `file:` URL load only for a `file:` document. JS `fetch()`, module `import` and `<script src>` each refuse it otherwise with a `FetchError`. Guard: `remote_document_cannot_load_file_urls`. This is the history of decision/file-url-only-for-file-documents, decision/default-script-fetcher-local-only and decision/file-urls-only-from-file-documents. None of the three functions touches it, so it is untouched by this chunk.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the `select` row: Enter, Space and a click open the list) A click on a single select focuses it and opens its list, or closes one standing. A click on an open list's row reaches `click_list_row` through the `option` arm. This is a ratified boundary widening and engine change for every document, and these are `handle_click`'s select arms. The open, close and choose branches and their order must survive the split (decision/select-list-keys-and-painted-labels, decision/select-choice-one-write-path).

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list's state, keys, pointer and close) The single `select_list` field is read through `live_select_list` everywhere except `reconcile_select_list`, the removal arm and `hover_list_row`. In the list, a press starts no text drag and the release chooses the row under it. A press elsewhere closes the list through `close_list_outside`. A page scroll or a box move closes it, and `fresh` keeps a list a click just opened. Guards include `a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends` and `escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event`. Covers decision/select-list-live-reader, decision/select-list-pointer-release-chooses and decision/select-list-closing, which are the pointer paths in `pointer.rs`.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (which select opens a list, what a row takes, and hostile markup) `select_list_allowed`: a disabled, inert or hidden select, one with no rows and one under a non-invertible transform open no list. A disabled row changes nothing and leaves the list open, and a group header chooses nothing. Guards are named in decision/select-list-open-gate and decision/select-list-row-choice. These are click-path gates the restructure must keep.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a label's click on a bound single select) A click on a label re-dispatches to its select, except for a bound single select, whose label's click focuses it and opens nothing. Asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing`. This is `handle_click`'s label arm, which the scope names among the moved arms.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a single select's `expanded`, and an author's `open` and `aria-expanded`) Neither steers a select. A single select's expanded state comes from engine state only, and `open` on a select does not match `:open`. This is the origin of decision/select-expanded-from-engine-state, with its named guard.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the driver's `type` on any select; the select readers and the list's public surface) `focus_select` (in the executor) focuses any select with no click and no DOM focus events, clears the typeahead and closes any list. `is_select`'s one caller is `execute.rs` `locate`. `closed_select_of` answers none for an open list. `close_select_list` is the one public writer, and `SelectListRow.label` is never logged. These are `execute`'s select handling and the surfaces behind decision/driver-type-on-select, decision/select-readers-resolve-node-closed-select-none and decision/select-list-label-never-logged.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list's readers and its close add no panic; what else of the list crosses the carriers; an open list's visible region) The readers go through `get_node` with no panic path, and a select made unfit closes its list with no panic. The list crosses carriers only as a row's own `bounds` and as pixels. `visible_region` answers an open row's region from the list's scrollport. This is the history of decision/select-list-closes-when-select-unfit, decision/select-row-layout-none-without-styles, decision/select-list-crossing and decision/visible-region-open-list-row.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list in the log; a page's markup reaches the select list; `optgroup`'s label painted) The list holds no log site, span or field, and `packages/escher-driver/src` holds two `info_span!`. Markup reaches the engine-painted list on every document. A group's label is painted as a header. This is the history of decision/select-list-label-never-logged and decision/select-list-keys-and-painted-labels. The restructure must add no log site.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`covered`'s closed-option clause and the select verbs' help) `covered`'s meaning gained the closed-option clause, and the help of `click`, `press` and `type` was restated. Each is a fixed literal of the verb table holding nothing a call supplies. Bears on contract/refusal-causes/covered. The refusal texts must not move in the `execute` split.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (citations moved with their lines) Line citations into `pointer.rs`, `execute.rs` and the other edited files were re-read to where the lines stand. Trap: this chunk's restructure of `pointer.rs` and `execute.rs` will shift the same citations again, and they are owed a re-point at wrap.

@history_item n=25 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — Added the single-choice `select` row: the choice is held as `CHECKED`, settled by the selectedness algorithm, with closed keys and a 1 s private typeahead. A click on a `select` focuses it. A click on a select's label re-dispatches to it, and `label_bound_input_element` binds labelables. This is the origin of `handle_click`'s select and label arms (decision/single-select-selectedness, decision/select-typeahead-private, decision/select-choice-exits).

@history_item n=26 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — Added the range input row: a main-button pointerdown sets the value by position, while a click focuses and changes no value. These act only with no `disabled` attribute, with one `input` per change and no panic path. This is the origin of decision/range-pointer-actions. The scope names `range_input_interaction` among the checks holding `handle_click`'s arms.

@history_item n=27 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — The executor arrived: `Session::run` validates first, and nothing public runs a `Command` that did not come from `validate`. An id is looked up anew on every call through `element_ids()` and `get_node`, never by index, and an unknown id is refused with nothing run. There is no panic path on call-supplied input. These are the invariants of `execute` (decision/id-lookup-anew-each-call, decision/no-panic-on-call-input, decision/one-validation-path-for-every-carrier).

@history_item n=28 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — After `validate`, the executor refuses with nothing run: first `stale` or `not-found` for an id, then `disabled`, `off-screen`, `covered` in that order for `click` and `type`. `scroll` says `in_view` instead of refusing. The snapshot and both focus readings are equal after each refusal. The session keeps a private record of at most 4096 ids. This is the fixed refusal order that `execute`'s restructure must keep (decision/target-refusal-order, decision/unknown-id-refused-stale-or-not-found, decision/session-id-record-bounded, contract/driver-id-refusal/*).

@history_item n=29 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — Added `validate` over a closed table and bounded argument kinds, giving one answer per call in a fixed order, with no `unwrap`, `expect`, `panic!`, `unreachable!` or index on call input and no clock read. Added the closed refusal cause set with fixed texts and no `String` field. This is the origin of decision/closed-driver-verb-schema, decision/command-schema-no-clock, decision/refusal-holds-no-call-input and contract/driver-argument-kinds/*.

@history_item n=30 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — One INFO `command` span per call to `Session::run` (target `escher_driver`), entered before validation, with fixed fields: `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`. Only a refusal's cause name reaches a log, recorded at one site for every refusal. This is the origin of decision/one-command-span-per-call, decision/refusal-log-cause-name-only and contract/driver-command-span/*. The scope forbids any span-field change, so the span's site and fields in `execute.rs` must not move semantically.

@history_item n=31 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — Ratified the sink's per-span record class, and that a diff's three sizes may reach a log while its content never does. Standing rule: a field holding a node, id, name or value of a diff is a new crossing. This is the history of decision/diff-counts-only-in-logs and decision/closed-span-lines.

@history_item n=32 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the sink's second record class and the diff's lengths, PROVISIONAL; typed text read in process) A sink capture of the six verbs and of a refused call per cause holds 0 supplied texts, ids or names. This is the precedent for decision/no-screen-content-in-logs and the `stand_act_spans` reading, which must stay green after `execute` moves.

@history_item n=33 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — A harness settle reports a load in flight at once as `NotSettled(Loads)`, never waits on one, reads no clock, and its pass loop is bounded at 64. The outcome names a class only. This is the origin of decision/settle-never-waits-on-loads and contract/driver-answer/settled.

@history_item n=34 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — The driver's acting step (the executor's settled step for `click`, `type`, `press`, `advance` and `scroll`) re-settles every 10 ms for at most `LOAD_WAIT` (5 s of wall time), moving no time on the held instance, then returns `settled: false` with the busy class. The session reads the wall clock for this alone. This is the history of decision/load-wait-bound and decision/bounded-load-wait, which are executor paths `execute`'s split must keep.

@history_item n=35 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — Eight causes, `time-unavailable` retired, two returned by `validate` and six by the executor. `advance` is accepted on every session and moves the virtual clock by exactly `ms` inside one settled step. Time reads in `events/pointer.rs` go through the document's `Clock` with no epoch `unwrap`. Bears on decision/advance-moves-virtual-clock-exactly, decision/time-moves-only-on-advance and decision/closed-refusal-cause-set, and on `pointer.rs`'s clock reads, which must stay on `Clock`.

@history_item n=36 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the screen's pixels in band; the engine's password mask, nine causes) `screenshot` takes an optional `id` looked up like a `click`'s. `fonts-unavailable` refuses before anything is painted, a crop of no pixel is `off-screen`, and a refused shot leaves the instance unchanged. No image reaches a log. These are executor arms in `execute` (contract/refusal-causes/fonts-unavailable, decision/password-painted-masked).

@history_item n=37 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: the `lint` verb's findings on every carrier) `lint` takes no argument (one refused `malformed`), runs no step, and `fonts-unavailable` also refuses it. Its findings and their count are never fielded, so a `lint` records `verb` and, when refused, `cause`. This is an executor arm of `execute`.

@history_item n=38 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot` takes an optional `part` (1 to 1000). A part past the last answers the empty text, and a part's number and count are never fielded. This is an executor arm of `execute` and the history of contract/driver-argument-kinds/part.

@history_item n=39 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — The cause set is stated as eight, with the executor returning six (`fonts-unavailable` among them). Lint findings and diffs were reconciled onto the carried sets. Bears on decision/closed-refusal-cause-set and the executor's refusal arms.

@history_item n=40 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph verb on the validated surface; the thirteenth verb) `schema::GRAPH` is read as `Command::Graph`, and any argument is refused `malformed` with nothing run on every carrier. Fields are `nodes` and `states`. It runs no step, feeds the session's record of ids and is written by `to_json` alone. The answer holds names, the flag words `undeclared`/`unconfirmed` and booleans, never a value, typed text or `NodeId`. `DioxusDocument::graph` is a read-only reader of the id holding no `NodeId`. This is the origin of decision/graph-answer-crossing, contract/driver-verbs/graph and contract/graph-answer/nodes-states, which `graph`'s restructure must leave byte-for-byte unchanged in content.

@history_item n=41 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the UI graph joins what the carriers take out; a recorded graph answer at rest) The graph crosses the socket, the command line, MCP and a kept check. It was a boundary widening, then ratified. This is the history of decision/graph-answer-crossing and decision/kept-check-graph-answer.

@history_item n=42 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph answer's size under the answer bound) `graph` is unpaged. Cells' answer reads 293,869 bytes, held under 1,048,576 by `stand_graph`, and the remainder is owned by `res-7cc5bi4k`. This is decision/graph-answer-unpaged, and a `graph` restructure must not change the answer's size or content.

@history_item n=43 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the Dioxus bridge's dispatch and the signal registry) The dispatch is set only where `runtime.has_context::<SignalRegistry>(ROOT)` holds and is inert without a registry. `use_graph_state` holds no value of a state and has no log site, print, env read or `unwrap`. `DeclaredWrite` holds a stable id or a `*` prefix and an event without `on`. This is the history of decision/dispatch-recorded-only-with-signal-registry, decision/signal-registry-holds-no-state-value and contract/declared-write/*, which `graph` reads.

@history_item n=44 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph verb's silence in the logs) `graph` records `verb` alone on its `command` span. dioxus-native-dom's `graph` and `signal_registry` modules hold no log site. `host_log` and `mcp_log` read 0 id, name and graph needles at `trace`. Helpers extracted from `graph` must add no log site.

@history_item n=45 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the cited lines the chunk's edits moved) Citations into `execute.rs`, `dioxus_document.rs` and other files were re-read to new lines. Trap: this chunk's edits to `execute.rs` and `graph.rs` owe the same re-point.

@history_item n=46 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the shared target check and the actions read) The crate-private `check_target` answers `Cleared` or a `Refusal`. `click` and `type` meet `action_point`, `scroll` none, and `screenshot` with an id `in_view`, an empty crop, then `fonts-unavailable`. It is called by the four id-taking arms of `Session::execute` and by `Session::actions`, and no arm keeps a check of its own. `target_of` ends `locate`. This is the origin of decision/actions-read-shares-target-check and decision/actions-read-changes-nothing (guards in `stand_actions`). A split of `execute` must keep every arm on the one `check_target`.

@history_item n=47 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions verb's validation and answer lines; the verb table at fourteen; the unpaged answer's size; its silence in the logs) `actions` takes no argument (refused `malformed`), runs no step and answers `{"nodes":[…]}` through `json.rs`'s `action_node`. There are ten instance verbs. Cells' answer is 106,088 bytes under the bound. The verb records `verb` alone. This is the history of contract/driver-verbs/actions, contract/actions-answer/*, decision/actions-answer-unpaged and decision/no-screen-content-in-logs, all of which `execute`'s dispatch serves.

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions answer in the threat model's carried sets and on the socket and in kept checks) The actions answer crosses the socket, the command line, MCP and kept checks, never a value, typed text or `NodeId`. This is the history of decision/actions-answer-crossing and decision/kept-check-actions-answer.

@history_item n=49 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the one actionable rule and actionable_nodes) `Readings` is the one crate-private actionable rule (focusable, an interactive role, or current listeners other than `mounted`), read by `unkeyed_actionable` and `actionable_nodes`, with no `NodeId`. This is the origin of decision/one-actionable-rule.

@history_item n=50 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the line locators the chunk's edits moved) Locators into `execute.rs` and others were re-read. `stand_act_spans` drives 38 calls. Trap: the restructure of `execute.rs` owes the same re-point.

@history_item n=51 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — (Input Validation: 2 rows; Data Protection: 1 row) `~ decision/actions-keyed-field`, `~ contract/actions-answer/keyed` and `~ decision/kept-check-actions-answer` went from provisional to standing on the founder's ratification of the `keyed` field. These are picked rows that the `execute` restructure must keep unchanged.

@history_item n=52 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the reached record and handler_coverage as readers of the stable id) Where `record_reached` switches it on, `handle_event` writes each reached handler's stable id and event, deduplicated, with no log site and no `NodeId`. A handler whose run removes its element still counts. `handler_coverage` returns ids, owner ids and event names. This is the origin of decision/reached-handler-record-without-log and decision/reached-record-deduplicated-survives-removal. The scope names `stand_handler_coverage` as holding `graph`.

@history_item n=53 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage line's content and silence; the kept actions step's replay) The coverage line holds ids, owner ids, event names and counts only, guarded by `a_typed_sentinel_never_reaches_the_coverage_line`. A kept `actions` step's replay is exercised by `stand_replay`. This is the history of decision/coverage-line-no-user-data and the guard of decision/kept-check-actions-answer.

@history_item n=54 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the key-press panic retired) `deallocate_anonymous_block` clears the layout parent of every child still naming the block, so a key press on a control whose box was dropped does not panic and still acts. Guard: `key_press_after_container_lost_its_box`. This is the origin of decision/dropped-anonymous-block-clears-parent, an engine path near the pointer and keyboard dispatch this chunk restructures.

@history_item n=55 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed load reaches the handler and ends the wait) `CountedHandler` forwards `NetHandler::error`, so a failed stylesheet ends a settle's wait. Trap recorded: a trait's do-nothing default can hide a fix through a wrapper. This is the origin of decision/failed-load-ends-wait, which bears on the executor's settled step.

@history_item n=56 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `MountedData` panic and the free-disk step are unrelated; the citations the chunk's edits moved) Citations were re-anchored after edits to blitz-dom, dioxus-native-dom and escher-driver. This is the same re-point duty this chunk owes on `pointer.rs`, `execute.rs` and `graph.rs`.

@history_item n=57 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — Added the `coverage_floor.py` row (`CRATE_DIR=PERCENT` ≤ 100) and the gates of the whole-workspace audit. This is the history of decision/per-crate-coverage-floors (escher-driver 97, dioxus-native-dom 83). Extracted helpers in `execute.rs` and `graph.rs` must stay covered so the floors hold.

@history_item n=58 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session socket carries calls and answers; the command line is an external input surface; ids, names and values leave in an answer) The socket and the command line reach the schema through `validate` first. No ending echoes caller input. A `Refusal` is written by `Refusal::to_json` from its cause's fixed strings. This is the history of decision/session-socket-carries-calls, decision/one-validation-path-for-every-carrier, decision/no-echo-of-caller-input and contract/refusal-json/*.

@history_item n=59 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the MCP server's input and its crossing, ratified) MCP `tools/call` became a third surface reaching `validate`, with masked values on every carrier. This is the history of decision/mcp-result-crossing and contract/mcp-result/masked-control-values.

@history_item n=60 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (wire v3; the kept check at rest) The socket grammar is `call v3`, and a masked `type` is kept as `MASKED_STAND_IN`. This is the history of contract/session-socket-request/call, contract/session-socket-reply/ok-refused and contract/masked-input-value/masked-stand-in, which `execute`'s outcomes feed.

@history_item n=61 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — Each kept answer is held whole at rest, masked as on every carrier. `Check::replay` runs each kept step through `Session::run` alone. The executor's answers are replayed byte for byte (`stand_replay`), so any output drift from the restructure fails there.

@history_item n=62 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (three state readings and the closed-field rule; the adapter crossing's third class) Control states leave only as closed words and booleans. A new closed-value field inside a ratified carrier is not a widening, but authored text or a new carrier is. This is the history of decision/control-states-closed-values-only, decision/aria-state-lowercase-exact-match, contract/snapshot-text/state-words and contract/driver-session-reply/state-readings.

@history_item n=63 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the merged ARIA reads) `aria-describedby` and `aria-description` reach only the platform adapter. The snapshot reads no description. This is the origin of decision/descriptions-adapter-only.

@history_item n=64 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — A password's or file input's non-empty value reads `MASKED_VALUE`. The DOM and form submission keep the value. This is the history of decision/masked-input-values, decision/password-file-value-masked and contract/snapshot-value/masked-value.

@history_item n=65 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the engine's password mask) blitz-paint paints a password's value as bullets in every document, so a typed password never leaves as pixels. This is the origin of decision/password-painted-masked and decision/mask-holds-on-every-carrier (guards `password_painted_masked`, `stand_act_screenshot`).

@history_item n=66 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — Defined the anchored `{key}//{segment}` tier, the three-way uniqueness argument, and that no `NodeId`, `ElementId`, `ScopeId` or pointer crosses with the id. This is the origin of decision/anchored-id-tier, contract/stable-element-id/anchored-path, contract/stable-element-id/no-collision and decision/no-process-handle-crosses, which `graph`'s ids must keep.

@history_item n=67 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — The HTML `id` is an author key only when non-empty, `/`-free and first in document pre-order. A non-element, stale or detached node reads no id, and no path panics. This is the origin of contract/stable-element-id/author-key, contract/stable-element-id/derivation and contract/stable-element-id/unresolved-node.

@history_item n=68 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — A Dioxus key that is empty or holds `/` is refused, and the row reads the positional segment. Each case has a named witness. This is the history of contract/stable-element-id/keyed-list-segment.

@history_item n=69 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — A row's key is a model-assigned `u64`, never an index, pointer, hash or clock, so it is the same across re-render, remount and process. This is the origin of contract/stable-element-id/stability.

@history_item n=70 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — The sink drops records from outside its allowlist whole, at every level, so no id and no name is logged at any level. This is the history of decision/stable-id-not-logged and decision/accessible-names-not-logged (guard `mcp_log`).

@history_item n=71 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (what a sink-installing host logs) Trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. `host_log` and `mcp_log` are the real checks. This applies to any silence claim this chunk re-reads after the restructure.

@history_item n=72 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the snapshot's clickable reading) `NodeState.clickable` is true where current listeners hold `click` and the element is not a disabled form control. It is a further reader of the presence reading of `disabled`. This bears on decision/disabled-read-by-presence and the listener reads shared with `graph`.

@history_item n=73 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — The master's text became rows (decision, contract, absence), each verified against the code at `fc2d17afd`. This is the base the picked rows rest on, read at the same sha as the metric record `met-cnhk7bbe` the scope cites.
