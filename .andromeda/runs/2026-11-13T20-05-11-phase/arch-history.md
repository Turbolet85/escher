@history_item n=1 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the dev-host cognitive-complexity reading) recorded `rust-code-analysis-cli` 0.0.25 on the dev host alone, never in CI, reading each function against a ceiling of 15 (the source of decision/cognitive-complexity-ceiling); the operator kept the ceiling as no architecture convention, with 85 functions left over it. This chunk reads its before/after figures by the same tool and adds no CI gate; the "functions over it are left as they are" stance is the clause the scope asks P3 to read.

@history_item n=2 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (row helpers) brought two table-asserting tests under the ceiling of 15 by keeping the outer walk in the test and moving per-row assertions into private `#[track_caller]` helpers, with no public symbol, test name or message changed. This is the house precedent for a behaviour-preserving split into private helpers in the same file.

@history_item n=3 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — (range and button arms as private named functions) kept `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` signatures, moved arm bodies into private functions (`click_button`, `press_range_input`, `range_track_fraction`, `press_range_input_key`), and **deliberately left the parents above the ceiling at their upstream floor so the next upstream sync's diff stays out of arms escher does not own**. The gates (`disabled` by presence), the event order and the range-focus block inside `handle_click`'s walk were kept in place. This is the direct precedent and trap for restructuring `handle_click` (97): a deeper split widens the upstream merge surface.

@history_item n=4 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — brought the id walk (`VdomWalk::walk`) under the complexity ceiling by splitting out private helpers (`record_roots`, `walk_component`, `child_chain`) with no behaviour change, killing mutants with tests placed inside the module. This is an earlier precedent for a ceiling-driven split in dioxus-native-dom, the crate that holds `graph`.

@history_item n=5 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — split escher-driver's `mcp` and `schema` into child modules and moved test modules out through `#[path]`, following the seams the code has. This chunk takes the opposite option: no file split and no new module, with helpers kept in the file that holds the function today.

@history_item n=6 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — turned `handle_click`'s submit arm into a button arm that focuses through `generate_focus_events`/`set_focus_to`, submits a submit button's form owner and ends the walk matched. A `disabled` attribute still stops the walk first, and a click on any other element clears focus (decision/click-focuses-button). It also routed keyboard activation through a queued Click (`dispatch_activation_click`), with `events/pointer.rs` on its merge surface. The restructure must keep the walk order, the match-ending and the unmatched clear of focus.

@history_item n=7 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (seven engine changes) made `handle_click`'s walk gate, `ClickTarget::Disabled` and the activation keys read `ElementData::is_disabled` (a form control with the attribute present), so a disabled ancestor `div` stops no walk (decision/disabled-read-on-form-control-only, decision/one-disabled-reading). It also counted the direct engine changes and their merge surface. Any moved gate must still read that one reading.

@history_item n=8 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified, Epoch 2 close) ratified the seven engine changes, including the click gates' `is_disabled` reading and the document's clock (double-click window). These are the standing rows a behaviour-preserving restructure of `handle_click` must leave unchanged.

@history_item n=9 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — made a range input's click focus it through `generate_focus_events` and change no value, while the pointerdown moves the value (decision/range-pointer). This is the range-focus block inside `handle_click`'s walk that the 2026-10-16 split kept in place.

@history_item n=10 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — made a click focus a `select`, with a disabled one stopped first, and routed every choice through `apply_select_choice` (`input` then `change`). This added the select arm to `handle_click`'s click-focus clause (decision/select-choice-single-path).

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a multiple select's chosen set, keys and pointer) added an `option` arm to `handle_click` (`click_select_row`): it stops at a disabled select, focuses the select and clears typeahead and the text selection; a plain click, Ctrl/Meta and Shift choose, flip and span (decision/multiple-select-row-clicks). The click walk gained that arm, which must survive the restructure.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (change on a checkbox and a radio) added `DomEventData::Change` after `input` in `handle_click`'s checkbox and radio arms: a checkbox always, a radio only where it was not checked before, with the runtime synthesising none (decision/checkbox-change-from-engine). The scope names these arms and their check `checkable_change_event`.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the list's default actions) gave `handle_click` its `select` arm (open or close a single select's list, focusing through private `focus_select`), its `option` arm behind `click_lands_on_row`, an `optgroup` arm, and a `label` arm that focuses a bound single select and opens nothing, with no re-dispatch. It also added `handle_pointerdown`'s list return. These are the select arms the scope's "select arms" refers to, held by `select_popup` and `select_interaction`.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list recorded) stated how the list opens, closes with no event and chooses, and that its merge surface (sixteen files) is owned by an upstream-sync line (decision/select-list-closes-without-event, decision/single-select-list-opening). These are behaviours `handle_click`'s select arm must keep.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the merge surfaces' reading and owner at `e42ee1f25`) read that upstream's take left `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched. It moved the merge surfaces' owner to the CARRY on «Upstream sync ahead of determinism and exploration». Restructuring `handle_click` in `pointer.rs` grows escher's own diff in an upstream-owned file, which every later sync re-reads.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the merge surfaces read at `ebacd68bd`) found keyboard activation's taken commits leaving `events/pointer.rs` untouched with no conflict. This is the earlier reading of the same pointer.rs merge surface.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the eight engine changes of this chunk) recorded eight more ratified direct engine changes, each flagged upstreamable with nothing sent, and their merge surface owned by an upstream-sync line (decision/hit-focus-form-engine-changes, decision/direct-engine-changes-flagged-upstreamable). Several live in `handle_click` (the multiple select, `optgroup`, checkbox/radio `change`).

@history_item n=18 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (the document's clock) made the double-click window and other pointer timing read the document's clock, with `events/pointer.rs` on its merge surface (decision/document-clock). Any timing read moved out of `handle_click` must keep going through the document's clock.

@history_item n=19 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Established Decisions: 4 rows) added decision/unresolvable-url-skipped: URL resolution is fallible and every caller skips, never a panic. It also re-stated decision/upstream-take-engine-changes at 29 picks at `563d3db73`. `handle_click`'s link arm resolves an href, so the skip must survive the move. This is the latest take, sitting directly under this chunk.

@history_item n=20 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Conventions: 1 row) gave decision/both-layout-modes its exception for upstream-taken targets running one mode, while escher's own targets run both. The restructure's witnesses (`select_interaction`, `checkable_change_event`, `stand_*`, `incremental_oracle`) run both modes.

@history_item n=21 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — created the executor (`execute` module, the crate's `cfg` gate) behind `Session::run`: validate first; `click`/`type` resolve ids per call; each acting verb is one settled step returning the diff. It set the rules "nothing public runs a `Command` that did not come from `validate`" and "nothing resolved outlives a call" (decision/validated-commands-only). These are the invariants `execute`'s restructure must hold.

@history_item n=22 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — made `run` open the `command` span and moved `run`'s former body into the private `Session::execute`. It fixed the eight span fields and what each verb records (`snapshot` records `verb` alone; a refused call records `verb` and `cause`), with the never-fielded list. The restructure of `execute` must keep exactly which fields are recorded where (contract/command-span/*).

@history_item n=23 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — gave the executor the screen-level refusals and the order `disabled` · `off-screen` · `covered` for `click`/`type`, `stale` vs `not-found` by the session's record, the `scroll` verb, and the click point at the snapshot bounds' centre. A refused call runs nothing. This is the refusal order the scope says must not move (contract/target-check/click-type).

@history_item n=24 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (`type` replaces) made `type` press select-all then type (an empty text clears), and gave the executor a second `cfg` gate for the select-all modifier beside the macOS backward-delete arm. These `cfg` arms in `execute` must survive the split on every platform (decision/type-replaces-control-content).

@history_item n=25 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `screenshot` arm to the executor: it resolves an id as `click` does, crops to the visible part (private `crop`, an empty crop `off-screen`), refuses `fonts-unavailable` before painting, and dispatches nothing (contract/target-check/screenshot). This is one of the per-verb arms `execute` dispatches.

@history_item n=26 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Driver session and CLIs) made `advance` move the session's clock by exactly `ms` on every session inside one settled step, with `advanced_ms` = `ms`, and retired `time-unavailable` (eight causes). This is the `advance` arm of `execute` (decision/advance-moves-session-clock, decision/advanced-ms-is-the-clock-move).

@history_item n=27 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — made the executor's settled step settle through the crate-private `settle_waiting` (the bounded load wait), so the five acting verbs wait on every carrier while snapshot, screenshot, lint, graph and actions wait on nothing (contract/load-wait/verbs). The restructure must keep which arms wait.

@history_item n=28 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — gave `snapshot` its optional `part` and `Outcome::Part`. The executor answers whole within budget, else part n of `text_parts`, a part past the last being the empty text, and a part read records the whole screen's ids (contract/snapshot-verb/part). This is another branch inside `execute`'s snapshot arm.

@history_item n=29 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: `lint` the twelfth verb) added `Command::Lint` / `Outcome::Linted`, whose arm calls `Session::lint`: no step, no settle, refused `fonts-unavailable` before reading, and the span recording `verb` alone. This is one of the per-verb arms of `execute`.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the driver's verbs on a select) changed `Target` to carry `select` and `focus_multiple_select` to `focus_select` (which also closes any list). It made `type` focus any select with no click and no DOM focus events (contract/dom-events/driver-type-on-select). These are executor arms and helpers the restructure moves among.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (type and press on a multiple select) added the executor's no-click focus for a multiple select, with the typeahead cleared, and the rule that a text opening on a space types nothing. `press` carries no Ctrl (contract/driver-verbs/press, contract/driver-verbs/type).

@history_item n=32 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions read and the shared per-verb target check) made the crate-private `check_target` (with `Cleared`) the one per-verb target check, called by the four id-taking arms of the executor, which keep no check of their own, and by `Session::actions`. `Target` became `pub(crate)` and `locate` ends in `target_of`. This is the current shape of `execute`'s dispatch and the scope's named `check_target`: the restructure must keep one shared check.

@history_item n=33 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (fourteen verbs) added `Command::Actions` / `Outcome::Actions` as the tenth instance verb: no step, recorded with `verb` alone. Together with `graph` this is the latest arm added to `execute`'s per-verb match, and it raised that function's reading toward the entry's 32.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the `graph` verb in the Driver session contract) added `Command::Graph` / `Outcome::Graphed`, read by the read-only `DioxusDocument::graph`: it dispatches nothing, runs no step, records `verb` alone and feeds the record of ids from the graph's node ids. This is an `execute` arm, and the link between `execute` and `graph`.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the UI graph reader) introduced `DioxusDocument::graph(&self) -> UiGraph` in the crate-private `graph` module and its readings. Nodes are one per snapshot node in pre-order. `owner` comes from `select_of`, a repeated registration reads `{name}:{k}`, `readers` names each component once, and `writers` lists only confirmed or flagged writes. A write resolves to the first chain element listening for its event, or to nothing past a gone node, and `unconfirmed` fires only after a dispatch passed through. This is the exact behaviour the `graph` (50) restructure must preserve (contract/ui-graph/*).

@history_item n=36 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph registry and the bridge's dispatch) records that `graph` is gated by `accessibility` and `signal_registry` is ungated. It states that `use_graph_state` registers state, the registry's dispatch is set around `runtime.handle_event`, and registered state is reachable only through the registry (decision/graph-state-by-registration, decision/accessibility-feature-gating). Helpers extracted from `graph` stay under the same gate.

@history_item n=37 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the snapshot's `clickable` reading) made the graph's handlers and `clickable` read the crate-private `element_listeners`, never the listener mark (decision/listeners-read-from-vdom). This is the reader `graph` depends on.

@history_item n=38 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (`stand_graph`, the unpaged graph) registered `stand_graph` (6 tests, both modes, in `stand-repeat`) and Cells' graph answer at 293,869 bytes under the answer bound. The `writers` narrowing that cut it from 522,692 means any change to which writes the graph lists moves that size. `stand_graph` is the scope's named witness for `graph`.

@history_item n=39 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (async handler writes land inside their dispatch) measured that an async handler's spawned task runs inside its dispatch, so its writes are recorded against that event. `stand_graph`'s first red, on Circle Drawer, is a trap for any change to how `graph` matches writes to dispatches.

@history_item n=40 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (a node's component and a held signal's readers through Dioxus public API) established reading a node's owning component through Dioxus 0.7.10's public API, a deleted row resolving to none, and reading a registered signal's readers through `subscribers()` (decision/owning-component-via-public-api). These are the walks `graph` relies on.

@history_item n=41 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the bridge's listener attributes over-report) measured that the bridge's listener attributes and mark over-report, while the vdom walk reads current listeners exactly. This is the origin of decision/listeners-read-from-vdom, which `graph`'s handler reading must keep.

@history_item n=42 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the one actionable rule and actionable_nodes) made `DioxusDocument::actionable_nodes` read over the UI graph's nodes, keeping each node the rule holds for or whose graph `owner` is set (contract/actionable-nodes/actionable-nodes). A restructure of `graph` changes the input of this reader and of `actions`.

@history_item n=43 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage reader handler_coverage) reads the UI graph at boot and after each replayed step, with `mounted` left out and deduplication by id and event (contract/handler-coverage/reached). `stand_handler_coverage`, the scope's second witness for `graph`, depends on the graph's handler list being unchanged.

@history_item n=44 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the reached record on the bridge's dispatch) made the reached record read the crate-private ungated `element_listener_ids`, not the `accessibility`-gated `element_facts`, so it compiles on the wasm build. This trap applies to any helper `graph`'s split shares with ungated code.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` step of `fast`) made `fast` run fmt → clippy → test → ci-scripts → embolden (decision/local-pre-push-gate, contract/ci-leg-legs/fast). The scope requires that whole leg to be green after the restructure.

@history_item n=46 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added the `stand-repeat` zero-failure leg, the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %) and the upstream blitz-dom feature-subset clippy line (decision/stand-repeat-zero-failure-budget, decision/per-crate-coverage-floors, decision/blitz-dom-feature-subset-clippy). Helpers extracted from `execute` and `graph` must keep those crates' line coverage over their floors, and pointer.rs must stay clean under the subset clippy.

@history_item n=47 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — recorded the host-run mutation tier (cargo-mutants with nextest, the `mutants` profile) beside the complexity reading. This is a dev-host instrument the scope's witness ("a test that could fail") can lean on for the moved arms.
