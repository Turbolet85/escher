@history_item n=1 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host complexity ceiling) — created the record behind decision/complexity-ceiling-host-only: ceiling 15, rust-code-analysis-cli 0.0.25 on the dev host only, test functions counted, no CI leg, "engine functions over it are left as they are". It also recorded the analyser's sum rule: `cognitive.sum` adds nested spaces into their parent, so a closure or a nested `fn` keeps its branches counted in the enclosing function, and only module-level sibling items lower a reading. This is the chunk's main trap. Helpers pulled out of `handle_click`, `execute` or `graph` must be sibling items in the same file, never closures or inner fns. The scope's open question also rests on this entry: whether "left as they are" is a policy the chunk crosses (and so needs a wrap amendment) or only a description of what the gate binds.

@history_item n=2 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the two table tests' private helpers) — precedent for bringing a function under the ceiling with private items alone. No public symbol changed, every assertion and message was kept, helpers were `#[track_caller]` and named only the row and index, and controls went red then green. This is the template for a behaviour-preserving restructure under decision/mutation-controlled-tests.

@history_item n=3 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (line spans into the three edited files) — the added private helpers moved cited `file:line` spans (`lint.rs`, `mcp_tests.rs`, the sweep file), and the wrap re-pointed them. Spans ending above a file's first hunk stayed. Expect the same citation sweep for `packages/escher-driver/src/execute.rs`, `packages/dioxus-native-dom/src/graph.rs` and `packages/blitz-dom/src/events/pointer.rs`. A pattern like `lint.rs:` also matches `stand_lint.rs:`, so anchor the search on the full path.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (coverage reads on CI#41192433140) — under Kept: the complexity readings of new and reworked functions take no row in test-plan. They are dev-host readings with no site in the plan, and the leg's gate prints what binds. So the chunk's before/after readings belong in the chunk's evidence, not in test-plan rows.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the complexity reading before the list's clearing) — once a function had been edited, its 17.0 reading and the over-ceiling count of 85 were qualified as "read before" that edit, with no later reading claimed. This is the precedent for dating any reading this chunk does not re-take (it rests on `met-cnhk7bbe` at `fc2d17afd`; the 89 other functions stay as they are).

@history_item n=6 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — set the mutation-tier recipe the picked rows hold (decision/mutation-runs-under-nextest-profile, mutation-baseline-skipped, mutation-runs-on-dev-host-only): cargo-mutants with `--test-tool nextest` under the `mutants` profile in `.config/nextest.toml`, `--baseline=skip`, and each unit's mutant test command read green first on the unmutated tree. Under plain `cargo test`, hung binaries make mutants read `timeout`. This applies if the plan uses mutants to show the moved arms are held.

@history_item n=7 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure flakiness budget on stand-repeat. Restructuring `execute` and `graph` changes the line counts in two floored crates, so the `Coverage report` must still read 3 of 3.

@history_item n=8 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — trap: a move the chunk forecast as reading-neutral moved escher-driver's coverage reading (98.33 % to 98.07 %), and the entry says a later split should forecast a reading change, not none. Extracting helpers inside `execute.rs` or `graph.rs` changes counted lines in the same way, so forecast a coverage move.

@history_item n=9 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — precedent for the earlier file splits (tests moved out of line with `#[path]` and test paths unchanged). This chunk does the opposite: no file split and no new module, helpers stay in the function's file, and `packages/blitz-dom/src/mutator.rs` and `packages/blitz-dom/src/document.rs` are not edited.

@history_item n=10 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — built escher-driver's executor (`execute.rs`) and its six `stand_act_*` files under `Session::run` in both layout modes, with five mutation controls. It set rules a restructure of `execute` must keep: a driver check's failure message names a layout mode, a task and an index, never an id or screen content. This is the origin of decision/driver-executor-integration-tested.

@history_item n=11 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — added the executor's refusal order `disabled` · `off-screen` · `covered` and the `stale` / `not-found` lookup (contract/driver-session/refusal-order, stale-or-not-found, refusal-cause/*). Nine `stand_act_*` files with nine further mutation controls hold it. `execute`'s per-verb dispatch and `check_target` carry this order, and the boundary forbids changing it.

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — added `stand_act_spans` and the executor's span fields and busy words (`execute.rs` 1 to 3 unit tests). Trap: the same check passed per package and failed in the workspace leg (engine `tracing` features differ by runner), which is the origin of decision/per-package-green-not-workspace-green. The restructured `execute` must read green under the workspace build, not only `-p`.

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `advance` became accepted on every session, moving the clock by exactly `ms` inside one settled step, with `advanced_ms` exactly `ms`. The causes became eight, in order, `fonts-unavailable` last (`the_eight_causes_are_named_in_order`). This covers the picked advance and refusal-cause rows that `execute`'s `advance` arm and refusal path must preserve, guarded by `stand_act_timer`, `host_timer` and `stand_virtual_clock`. The double-click-without-advance row also comes from this entry.

@history_item n=14 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the one-virtual-clock marks (advance exact, eight causes in order, the clock never moved by `screenshot`, `text_contrast` or `settle`) and `password_caret_click`'s double click selecting the whole value. These are the ratified rows the restructure must keep unchanged.

@history_item n=15 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting verbs wait out a load in flight up to `LOAD_WAIT` (5 s wall, settle every 10 ms, no time moved); `snapshot` and `screenshot` wait on nothing. Guarded by `stand_load_wait` and session.rs's four unit tests. `execute`'s acting and reading split must keep this (decision/load-wait-bound).

@history_item n=16 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — `CountedHandler` forwards `NetHandler::error`, so a failed load ends the harness and driver settle wait (decision/failed-load-ends-settle-wait, contract/harness-settle/failed-load). It is pinned by `settle.rs`'s `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it`, and the driver's load wait runs through the same settle.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`type` focuses any select with no click) — `type` on any select focuses it with no click and fires no DOM focus event (`Target.select`, `focus_select`; `schema_tests` pins the help). This is one of `execute`'s per-verb branches the restructure moves (contract/driver-session/type-replaces).

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause and the restated help) — `covered`'s fixed meaning adds a closed select's option, and `schema_tests`, `refusal` and `json` pin the help texts with `VERBS` at 12 and causes at 8. The refusal texts are `'static` and must not move in a restructure (contract/refusal-cause/covered).

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (stand_actions) — `Session::actions` and `check_target` are covered by `stand_actions` (8), not by a unit test. `check_target` reads `can_paint_text` after the crop and before painting (the `fonts-unavailable` ordering), and `harness.click` waits on nothing while a driver `click` waits out `LOAD_WAIT` (decision/harness-click-waits-on-nothing). Any move of `check_target` out of `execute` is held only by this integration check, and the actions answer holds no value (decision/actions-answer-holds-no-value).

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions call in the span and log checks; the verb table at fourteen) — `stand_act_spans` drives all ten instance verbs (38 calls per mode), each `command` line recording `verb` alone. `host_log` and `mcp_log` read 0 needles at `trace`. `VERBS` is at 14, and the complexity-ceiling record's test name was renamed without re-reading its value of 3. These hold the span and log rows (contract/command-span/*, decision/driver-log-fixed-words-only) that `execute`'s dispatch must leave unchanged.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit scope and its one test-only shim) — added `graph.rs` with nine `graph::tests` under the `accessibility` feature. The bridge tests gained no constructor or `pub`, and their only `cfg(test)` shim is `DioxusDocument::dispatch_to_vdom` (decision/bridge-tests-without-widening). A `graph` restructure must not widen visibility for tests, and the feature gate matters for which build compiles its tests.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit-test groups for the graph) — states what `graph::tests` pins: each node's component, current listeners (`mounted` included, a removed one gone), declared writes confirmed after dispatch, readers, an option owned by its select, prefix declarations, a drifting declaration reading both flags, and a gone element's write not handed to its ancestor. Trap: `stand_graph` disproved the plan's resolution rule, which was fixed by stopping at a gone node. A restructure that reorders the resolution walk risks bringing that bug back.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb held in session and over MCP; the unconfirmed flag's limit; GRAPH_DRIVES; Cells' answer) — `stand_graph` (6), in both layout modes, holds the `graph` verb equal to `DioxusDocument::graph` on every read, no served id in the `{owner}///{key}` form, and the Cells answer (293,869 B) within the answer bound. `unconfirmed` fires once a handler's event passed through its element and no run wrote the state. `GRAPH_DRIVES` drives each conditional write once. These are the rows (decision/graph-answer-holds-no-value, unrun-declared-write-reads-unconfirmed, graph-drives-confirm-every-write, reserved-graph-id-form, graph-signal-registration) whose content must not change.

@history_item n=24 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (one actionable rule for two readers) — both actionable readers read one rule from the current listeners, never from the `data-dioxus-id` mark (decision/one-actionable-rule). It is guarded by `a_listener_removed_on_a_re_render_is_read_by_neither_reader`, the in-tree witness that the mark outlives a removed listener. `graph`'s listener reading shares this source.

@history_item n=25 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (listeners and a held signal's readers) — a node's current listeners must be read from the vdom walk's dynamic attributes (`mounted` included), not from the bridge's listener attributes, which over-report (decision/listeners-read-from-vdom-walk, guard `link_2_a_removed_listeners_attribute_stays`). `graph` relies on this, so a restructure must keep the walk as the source.

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage, its tables and limits; the reached record) — `stand_handler_coverage` (8) replays kept checks and reads the UI graph at boot and after each step against `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES`. A refused step exercises nothing, a keyboard activation exercises its click, and a bubbling dispatch counts every ancestor. The canvas, unreached-screen and `stopPropagation` limits are stated. This is the second check the scope names as holding `graph`, and its tables pin `graph`'s handler listing exactly.

@history_item n=27 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/handler-coverage-reads-every-replayed-screen`: state moved from provisional to standing on the founder's ratification. That row reads `graph` at boot and after every replayed step, so `graph`'s output is part of a ratified contract.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`change` on a checkbox and a radio, from the engine) — the engine sends `change` after `input`, and a radio sends it only where it was not checked before the click. A label's click reads as its control's, and the script runtime synthesizes none (decision/change-event-from-engine-only). Pinned by `checkable_change_event` (3) and `radio_click_fires_change_only_where_it_was_not_checked`. These are `handle_click`'s checkbox, radio and label arms, whose event order the boundary freezes.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the select model's limits asserted; what `select_interaction` pins; the `:open` gap closed) — `handle_click`'s select arms (single and `multiple`, `optgroup`, row clicks, Shift span, a disabled select taking no click) are pinned by `select_interaction` (18) and `stand_act_select` (10). The `summary` click toggling `:open` is pinned by `open_state` (4) (decision/open-pseudo-class-follows-attribute). These are the existing checks the restructure leans on for those arms.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (select_popup, the select limits retired, the standing select checks, stand_act_popup) — a click, Enter or Space opens a single select's list. A click chooses a row (one `input` then one `change`), and a second click, or a click outside, closes the list. A label's click focuses the select and opens nothing, with no re-dispatch, while a `multiple` select's label click is re-dispatched and asserted by no check. These are held by `select_popup` (25), `stand_act_popup` (12), `stand_keyboard` (23) and `select_interaction`'s `KEYS`. The `multiple` label re-dispatch arm is an arm no check reaches, which the scope's witness clause requires the plan to name.

@history_item n=31 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — the founder ratified that a click focuses a button and that `disabled` works by presence. `stand_keyboard` holds Enter and Space as a click on the stand rows, and `stand_act_obstructed` holds the clicked button reading `focused`. These are `handle_click`'s button-focus arm and its pins.

@history_item n=32 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — `range_input_interaction` (now 8), in both layout modes, and `stand_act_range` hold the slider moved by click and key. These are among the checks the scope names for `handle_click`'s range path.

@history_item n=33 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§1 coverage) — `password_caret_click` (2) pins one caret index for two values of one length and a double click selecting the whole value. The entry also records that the password click's shift-click and triple-click branches were claimed but not searched. These are text-input click arms that `handle_click` may route through and that may be unreached.

@history_item n=34 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a raw click on an element outside the padding box of a box that clips it no longer lands on it (contract/harness-input/clipped-click). Trap: a check that leaned on the old hit holds none of the words a sweep would search for, and is found only by running the stand gates.

@history_item n=35 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — trap: a headless key press is not the same on every platform. The editor's Backspace arm is compiled out on macOS, so a check passed on Linux and failed on the macOS CI leg. Any check added before a move must prove state changes with clicks and typed characters, not deleting keys.

@history_item n=36 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the bridge tests sit in a `cfg(test)` child module of the Dioxus document module because `DioxusEventHandler`'s fields are private (decision/bridge-tests-without-widening). Trap: a sed or grep address ending at a function's name also selects every item whose name starts with it, and a control built with the same address cannot catch that. Relevant when extracting helpers named after `execute`, `graph` or `handle_click`.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` reads fmt → clippy → test → ci-scripts → embolden. `embolden` is the local font-embolden witness, pinned last by `test_fast_leg_ends_on_the_font_embolden_witness` (decision/local-pre-push-gate, embolden-local-witness). The scope requires `fast` green, which includes this step.

@history_item n=38 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three engine checks) — the workspace build and the per-package build compile different bodies of the bounds reader (`writing-mode`). A fix or move in one body reads green under the other build, so an engine change in blitz-dom needs both builds read (decision/per-package-green-not-workspace-green).

@history_item n=39 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Integration Test Strategy) — `~ decision/layout-tests-in-both-layout-modes`: an except clause was added saying upstream targets taken in a sync that run one layout mode stand as named exceptions. The rule otherwise still binds: any check added before a move runs in both layout modes, guarded by `incremental_oracle`.

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand-repeat` names fifteen targets) and **2026-10-31-select-popup** (seventeen named words) — `checkable_change_event`, `open_state`, `select_interaction`, `select_popup` and `select_popup_painted` are named `--test` words in `stand-repeat`, pinned by `test_stand_repeat_leg_runs_three_rounds`. These are the legs that already rerun the click-arm checks under the zero-failure flakiness budget.
