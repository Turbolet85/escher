@history_item n=1 marker=2026-11-25-circle-drawer-virtual-nodes
**2026-11-25-circle-drawer-virtual-nodes** — `~ contract/coverage-line/content`: a canvas virtual node's click is listed under its `{owner}///{key}` id, owned by its canvas, and a canvas click inside a node counts for the canvas alone (ratified, «Подтвердить все 10»). This chunk changes what a click on a circle does (B1: it selects and adds none) and re-reads `stand_handler_coverage`'s `GRAPH_DRIVES`, so the coverage line's content row must still hold. Any keyboard-reached circle action that is recorded must stay inside this carried set.

@history_item n=2 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** — `~ decision/declared-node-read-logs-nothing`: widened from the read alone to the virtual nodes' place in the accessibility tree, the snapshot and a click on one, each with no log site, print, span or field. This is the precedent this chunk extends. The new `selected` field, the virtual node's focus in the tree and snapshot, and `press` on a focused canvas must each add no log site, span or field, or the row widens again.

@history_item n=3 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** — `+ decision/declared-node-read-logs-nothing`: the declared-node read, its validation and its answer have no log site, print, span or field, and a refused declaration holds only a closed reason and its index. `VirtualNodeDeclaration` gains `selected` here, so the declaration read and its refusal reasons stay under this rule.

@history_item n=4 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (§4 · §6 · §3) — The verb table and the tool listing went from fifteen to sixteen words (`contract/command-span/verb`, `contract/tool-call-span/tool`). `contract/command-span/settled` gained `canvas` among the verb-alone reads, and `+ contract/command-span/canvas` was added. `decision/sink-host-read-at-trace` gained canvas needles, and `decision/part-answer-never-logged` gained the canvas part (PROVISIONAL, delegate). Also `+ decision/canvas-read-opens-no-span`. This chunk adds no verb: `press` on a focused canvas is an existing acting verb, so the sixteen-word count, the eight fields and the canvas line contract must not move.

@history_item n=5 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `~ decision/canvas-read-opens-no-span`: the lint's canvas label reading opens no span and logs nothing, inside the lint call's command span. Any new canvas-side reading this chunk adds (focus, selection, the `covered` reading of a circle) follows the same rule: no span of its own, running inside the call's command span.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — The command span as built: INFO, target `escher_driver`, entered before `validate`, with eight fields each recorded only where it applies. It never fields an argument, id, screen text, `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`. This is the origin of `contract/command-span/*` and `decision/command-span-no-call-content`. A `press` that moves the current circle, or a click that selects, records only `verb`, `settled`, `busy`, `passes`, `added`, `removed` and `changed`, never a circle key or id.

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — The closed span's line was ratified, and with it the founder's ruling that a diff's three sizes may reach a log while its content never does. That ruling is what `added`, `removed` and `changed` stand on. A circle's `differs: [selected]` diff may count into `changed`, but the word `selected` and the node id never reach the span.

@history_item n=8 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — The span's `cause` field takes a `Cause` name only. A `Command`, `Call` or `ArgValue` is never fielded with `Debug`, and an argument is never fielded by its schema name; only `text` is in the scrub set. This is a standing rule behind `decision/debug-types-never-logged` and the refusal-cause rows: the stand's `covered`, `off-screen` and `disabled` readings on a circle are recorded by `Cause::name` alone.

@history_item n=9 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Outcome` joins the `Debug`-deriving types that nothing prints, logs or fields. An `Outcome` fielded with `Debug` would print an id, an accessible name or a control's value. This is the base of `decision/debug-types-never-logged`, and the trap applies to any new outcome or virtual-node type carrying `selected` or focus.

@history_item n=10 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — The refusal-cause domain is returned by code, eight of eight. This is where the `covered`, `disabled` and `off-screen` cause words in `contract/refusal-cause/*` come from. This chunk makes a circle refused by these words on the stand, with no new cause name.

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — A shot cropped to no pixel is refused `off-screen`, which is the extra clause in `contract/refusal-cause/off-screen`. It matters if the stand's off-screen reading of a circle is made through an element screenshot.

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `cause` became one of eight names, with `time-unavailable` retired (ratified in 2026-10-18-engine-debts). The closed cause set the refusal-cause rows read is fixed at eight; a circle's refusals must use the existing words.

@history_item n=13 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `busy` is driven by `stand_act_spans`' `press tab` on a re-keying fixture. The line carries `settled=false` and `busy="render"`, with no `passes` and no `cause`. This is a precedent for how a `press` that repaints a canvas each render is read in the span (`contract/command-span/busy`, `passes`, `settled`).

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — `click`'s, `type`'s and `press`'s help text and `covered`'s meaning are the program's own `'static` text. They hold no call text and add no log site or span field, and the `command` span keeps its eight fields (`stand_act_spans`). If this chunk rewords `press`'s help or `covered`'s meaning for canvas focus or a covering sibling (F4), that stays fixed text under `contract/command-span/field-count`.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open row's bounds) — An open list's row crosses the snapshot and the diff by its own `bounds`, and `differs` names `bounds` within its closed set, adding no field, key name or carrier (main's read: outside the widening, under the field rule of 2026-10-10). This is a precedent for a circle's `differs: [selected]` riding the snapshot's existing `selected` without a new carrier field.

@history_item n=16 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — `~ decision/select-chosen-set-never-logged`: a chosen row crosses only in the snapshot, the diff and the actions answer, with no log site, span or field. This is the nearest precedent for a selected state carried by `selected`. A selected circle must likewise leave only on the answer carriers, never on a log line or span.

@history_item n=17 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (snapshot `clickable`) — The snapshot's `clickable` reading (`contract/snapshot-node/clickable`, `contract/snapshot-text/clickable`, the token after `focused`) is a closed bool carrying no page text. `selected` and the snapshot's `focused` naming a virtual node sit in the same snapshot-state set, so they must stay closed bools or ids inside the ratified crossing.

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph verb, bridge dispatch) — `Outcome::Graphed` is written by `to_json` alone and never logged or fielded (`decision/graph-answer-never-logged`). The bridge dispatch sets the `SignalRegistry` only where a registry context stands, so a page with no registered state reaches no registry (`decision/no-state-page-skips-registry`). `stand_graph` is in this chunk's sweep. A new canvas key handler or selection state in Circle Drawer passes through the same dispatch, and a registered selection would show in the graph as a state name only, never its value.

@history_item n=19 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (§3 · §6 · §8) — The graph answer gained its part form (`contract/command-answer/graphed`, `contract/graph-answer/shape`, `contract/graph-answer/content`), and `decision/part-answer-never-logged` names the graph part. `contract/command-span/graph` covers a part read. These are the rows this chunk rests on if `stand_graph` pins on Circle Drawer move.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — The actions answer carries ids, owner ids, the table's verb words, closed cause words and booleans only (`contract/actions-answer/content`, `cause`, `logging`; ratified). `Session::actions` with `check_target` / `target_of` opens no span, inside the call's command span. A circle's keyboard reach, or a `covered` cause from F4, shows up in the actions answer only as these closed words.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record) — The dioxus-native-dom `reached` record holds stable ids and event names only, with no `NodeId`, and its `Debug` prints its count alone (`decision/reached-record-ids-and-events-only`). `Coverage::to_json` writes the coverage line by hand and never logs it (`decision/coverage-line-never-logged`, `contract/coverage-line/shape`, `owner`). A canvas key handler (`keydown`) becomes a new handler event, listed by id and event name only.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage line's carried fields) — The coverage line carries stable ids, owner ids, event names and two counts, and a typed sentinel occurs 0 times in it. This was ratified in 2026-11-15T05-21-42-wrap. `contract/coverage-role-stdout/coverage-line` keeps it on stdout as an answer. Rereading `GRAPH_DRIVES` under B1 must leave this line's content class unchanged.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (host_log's coverage run) — `host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` reads 0 id, name, coverage, sentinel, path and recorded needles at `trace` under both builds. It is the guard of `decision/coverage-line-never-logged`, and its needles follow the CRUD screen only. Circle Drawer is served by no role here (a boundary), so this guard does not reach the new canvas handlers.

@history_item n=24 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/answers-echo-nothing-typed` except: a caused item's message, chain and kept fields hold what their author wrote; a hidden-set field reads `[redacted]`, and an `error` field is shown as written, cut at the text bound. This was ratified in 2026-11-17-a-component-in-isolation. This is the row's current exception: a canvas `press` answer echoes no key the caller typed.

@history_item n=25 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — An acting step, `press` and `click` included, runs under escher-telemetry's scoped capture, which prints nothing (`decision/headless-stand-no-sink`, `decision/driver-library-installs-no-subscriber`, `decision/driver-emits-no-event`). A `press` on a focused canvas and a selecting click are acting steps, so they inherit this. Any engine warning the new focus or key path raises lands in `caused`, not on a log.

@history_item n=26 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `stand_keyboard` joins the silent census: it installs no subscriber, reads no env var, prints nothing and binds no listener. The shared module, whose `tab_order` and `focused` it reads, also probes 0. This chunk extends `stand_keyboard` and the shared module's tab-order tables with Circle Drawer, and both must stay silent.

@history_item n=27 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` (`tab_order_is_unchanged`, reading the tree through `&dyn blitz_dom::Document`) are recorded silent, the probe 0 over them and the shared module. `stand_accessibility_ids` is in this chunk's sweep.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand_keyboard`'s reach; Cells' Tab stop) — `stand_keyboard`'s stated reach was widened to the load page and Cells. Cells' `cells-grid` Tab stop and keys are instrumentation n/a, and the tree's focus fallback and the inert focus fixup add no log site. This is the direct precedent for making `circle-canvas` a Tab stop with a key handler. A stated reach that names its apps goes stale when Circle Drawer joins.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read) — The engine's `focussed node` INFO line fires once per focus move under the workspace build, holding a node number and no id or name, and the focus fixup adds no log site. When the canvas takes focus in the Tab walk, this line fires. If the tree's or snapshot's focus names a virtual node, the line must still carry no virtual node id or name (`decision/session-logs-carry-no-page-content`).

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `focussed node` count) — Counting the `focussed node` line through the sink on the workspace build reads 0 lines and has no base reading, so neither moved nor unmoved. This trap applies if this chunk counts focus lines for the canvas: a reading with no base settles nothing.

@history_item n=31 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — The workspace-built host printed the `blitz_dom::document` `focussed node` INFO line twice, a cause recorded but not established. This trap applies to any host-log line count after this chunk's focus changes.

@history_item n=32 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — Engine focus changes (the not-in-document focus refusal, the focus-move redraw) were recorded as adding no log site, span, field or env read, and the `Focussed node` line stands after the new gate. A canvas-focus engine change or a virtual-node tree focus in `accessibility.rs` follows this precedent.

@history_item n=33 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/checks-silent` except now names the order page's log site beside the sink, recorder, bench and re-run-child exceptions. The stand checks this chunk touches (`stand_circle_nodes`, `stand_circle_drawer_canvas`, `stand_canvas_*`, `stand_keyboard`, `stand_graph`, `stand_handler_coverage`) fall under this row. A new exception must be named here.

@history_item n=34 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — The origin of `decision/ci-log-artifact-on-failure`: each leg's merged stdout and stderr goes to its own log, uploaded only on failure as `ci-log-{job id}` for 7 days, as unscrubbed build output. CARRY `cry-sdshdd4m`'s windows warnings are witnessed in the CI windows job's own log. That is the step log, read only after `Compiling`, not a failure artifact. The operator's direction keeps raw logs with temp or home paths out of evidence.
