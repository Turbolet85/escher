@history_item n=1 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — stable element ids registered: `element_id` / `element_ids()` computed on demand, no `NodeId`, `ElementId`, `ScopeId` or pointer in an id. Why it matters: the read lists nodes by exactly these ids, the ids the per-node verbs resolve, and an author key versus a positional path is the grammar the actionable-key fold-in has to tell apart.

@history_item n=2 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — snapshot model registered under `accessibility`. Each field is an existing reader's (`enabled` is the `disabled` presence), and no field holds a `NodeId`. Standing rule: the snapshot derives no role or name of its own, so it cannot disagree with the accessibility tree. Why it matters: this is the precedent for the read's "no second reading" rule (refusal causes from the engine's readers, not re-derived).

@history_item n=3 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** (actionable-key check) — registers `DioxusDocument::unkeyed_actionable()` with its three readers (focusable, a closed interactive-role list, `data-dioxus-id`), its seven fields and `remedy()`, with no wire form; adds the anchored-path tier. Trap stated for later chunks: `UnkeyedActionable.node` is a process-local `NodeId`, so the first wire form of the check must not carry it. Why it matters: folding the check into this read is that first wire form.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session socket is a founder-ratified boundary widening. Standing rule: the first request that carries an id, a name, a value, snapshot text or a diff is a new crossing question. Why it matters: this is where the "a new carrier or exit widens the boundary" rule starts, and the new read's answer must clear it.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — builds the one `'static` table: `VERBS`, `VerbSpec`/`ArgSpec`/`FieldSpec`, `verb(name)`, `validate`, a closed `Cause` and a `Refusal` with no `String` field. Standing rule: a later entry adds its verbs to `VERBS`, never a second table. Why it matters: reading per-node verbs from the table with no closed list in code rests on this rule. Any field that says a verb's targets or refusals extends this shape, and that is a contract fork.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` is the one entry point. Nothing public runs a `Command` that did not come from `validate`, nothing resolved outlives a call, and nothing fields an `Outcome` (its `Debug` prints ids, names and values). Why it matters: the new read's `Command` and `Outcome` variants go through `run`, and the new `Outcome` must stay unlogged.

@history_item n=7 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the executor detects the screen-level refusals. A `click` or `type` is refused in the fixed order `disabled` · `off-screen` · `covered` with nothing run. Resolution answers `stale`/`not-found` against the session's `SeenIds` record. Adds `visible_region` and the `scroll` verb, which applies none of the three checks. Standing rule: visibility is read from geometry, never from a hit. Why it matters: this is the shared refusal path (`action_point`) the read must reuse so its causes agree with a real call.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — one hand-built `command` span per call to `Session::run` (no `#[instrument]`) with eight fields; a refused call records `verb` and `cause` only; arguments, `Outcome` and screen content are never fielded. Why it matters: the new read's span records `verb` alone (and `cause` when refused), with no field for its content or size.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the socket carries a call and its answer) — the founder ratified the second widening: ids, names, masked values, snapshot text and diffs cross the socket with no `NodeId`, and the answer is bounded at 1,048,576 bytes. Standing rule: the next surface that carries them asks again. Why it matters: these are the carrier and bound the new read's answer crosses.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (nine verbs in two levels, the written forms, `command_line`) — adds `Level`, the session-level verbs and the hand-written `Outcome::to_json`/`Refusal::to_json`; no command ending writes back caller input; the usage line and argv reader read `VERBS`, with no second list. Why it matters: the new verb reaches the command line and its written form through these hand-written forms.

@history_item n=11 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a box's client rect no longer subtracts its own scroll offset, and the hit stops at the padding box of a box that clips by `overflow`. These are direct engine changes, and the founder's ruling set their reach. Why it matters: the off-screen and covered causes the read predicts depend on these two readers staying as ratified.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (MCP crossing ratified) — `serve_mcp` with `tools/list` over `VERBS`; ratified boundary widening over the `mcp` role's stdio. Standing rule: the next surface that carries a call or a screen's content is a new crossing question. Why it matters: the new verb becomes an MCP tool on this ratified carrier.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` a reader only; the `tool_call` span) — the founder chose `serde_json` as a reader only, with the written forms staying hand-written, and the `tool_call` span carries `tool` and `ending` alone. Standing rule: no span field holds what a call supplied or a screen reads. Why it matters: the new read's answer is hand-written, and its MCP call adds no span field.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help`, `describe`, the tables) — `describe()` writes the self-description from the tables alone (`VERBS`, `NODE_FIELDS`, `CAUSES`, …); `tools/list` descriptions carry each verb's result fields; `Level::Tool`. Why it matters: a new verb moves describe's verb count and its tool description, and every text must be read from the table.

@history_item n=15 marker=2026-10-12-self-description
**2026-10-12-self-description** (fixed program text on the ratified carriers) — the program's own fixed `'static` table text may ride the ratified carriers; text from a page, a call or a screen, or a new carrier or exit, still widens. Why it matters: the new verb's help and cause meanings are fixed table text inside the carriers, while its per-node answer is screen-derived and needs a founder fork.

@history_item n=16 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the `screenshot` verb: with an `id` it resolves as `click` does, and a crop of no pixel is refused `off-screen`; adds `fonts-unavailable`; `Outcome::Shot` dispatches nothing. Why it matters: if the read lists `screenshot` as a per-node verb, its refusals are its own (crop `off-screen`, `fonts-unavailable`), not click's order.

@history_item n=17 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (`covered`'s second limit) — a driver `click` on a target beside a box clipped on one axis is refused `covered` though it shows, and `covered`'s meaning states it. The hit's own-box readings are measured (img, iframe, filled input). Why it matters: the read must report this off reading as a real call does, and `stand_act_obstructed` is its witness.

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed) — `unkeyed_actionable()` returns nothing on all seven stand tasks, with the author ids named. Why it matters: on the stand the actionable-key answer folded into the read is "every node an author key", so a positional-path case needs a minimal fixture.

@history_item n=19 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe runs on the escher server, and `passed` needs `isolated` on exactly the server's `mcp__escher__{verb}` tools. Why it matters: a new verb changes the tool count that the isolation check and `test_cold_agent.py` assert.

@history_item n=20 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — adds the `stand-repeat` leg (three rounds, zero-failure budget, no retry) and the coverage floors `coverage_floor.py` (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %). Why it matters: the new stand checks must pass three rounds green, and the new driver and bridge code must keep those crates at their floors.

@history_item n=21 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — `schema`'s six shape types (`VerbSpec` etc.) moved to the private child `spec`, re-exported, with tests in `schema_tests.rs`. `mcp` has three `pub(super)` children (framing, listing, result), with tests in `mcp_tests.rs`. Why it matters: a `VerbSpec` field and the new MCP tool land in these children.

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (seven engine changes: `inert`, `disabled` on form controls) — `ElementData::is_disabled` (`can_be_disabled()` and the attribute present) is the one reading for focusability, the pointer and click gates and Enter/Space. `inert` is honoured by presence (not focusable, no hit, out of the tree and the snapshot). The founder ratified both on 2026-10-18. Why it matters: the read's disabled cause must come from this one reading, and its actionable set must leave out inert elements.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the geometry readers; the toggle per role and the driver's texts) — the hit stops at a `contain: paint` box, and the client rect is read under transforms. `covered`'s meaning gains the no-area reading (`task-header-spacer`, hit at its centre). Why it matters: the read inherits these two off readings of `covered` and must state them as a real call returns them.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs) — `time-unavailable` retired, `Cause` closed at eight (`CAUSES: [Cause; 8]`); `advance` moves the one clock on every session. Founder-ratified 2026-10-18; a cause leaving narrows the refusal surface. Why it matters: the closed eight is the cause set the read's per-verb refusals must stay within, and adding one is the founder's fork.

@history_item n=25 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check: a launcher-named, owner-only recording directory; a record per answered call; the strict `Check::from_bytes` over the instance verbs. A founder-ratified boundary widening (call content at rest). Why it matters: a new instance verb must be kept as a recorded step and admitted by the strict reader.

@history_item n=26 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer is held whole at rest (`kept_line`), and `Check::replay` reruns through `Session::run`, comparing by the verb's result fields in table order. The two committed Counter checks are compared byte for byte by `record_checks.sh`. Why it matters: the new read's kept form and its `VerbSpec` fields decide its replay, and the committed checks must stay byte-equal.

@history_item n=27 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (in-process lint) — `Session::lint` records each id as `snapshot` does, runs no step and changes nothing. Its clip region is `visible_region` cut by `contain: paint` ancestors (its own walk). Why it matters: precedent for a read-only screen reader in the driver that feeds the record of ids, and for reading visibility through `visible_region`.

@history_item n=28 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` the twelfth verb) — `lint` joined `VERBS` on every carrier and as a recorded step: no argument, one refused `malformed`, `Command::Lint`/`Outcome::Linted`, a hand-written `{"findings":[…]}`, `verb` alone on the span, describe and `tools/list` counts moved. A founder-ratified boundary widening (F1 «Новая команда lint»), superseding the delegate's "in process only". Why it matters: this is the direct precedent for a new read-only verb and for the fork it needs.

@history_item n=29 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the bounded load wait applies to the five acting verbs; `snapshot` and `screenshot`, which run no step, wait on nothing, and `session.rs` is the one clock reader. Why it matters: the new read runs no step and so waits on nothing, answering on the load page while its fetch is in flight.

@history_item n=30 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model, an engine change for every document: an option's `bounds` are its select's, `type` chooses an option by its label, and an option is not clicked. Why it matters: this is where the owner-served select case begins, and a closed select's option reads refused `covered`.

@history_item n=31 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot` takes `part`, and a screen over the budget crosses in parts with no new verb, carrier, exit, cause, session error, span field or dependency; a ratified widening. Why it matters: precedent for an answer's size on Cells, and a paging option if the new read's whole answer nears the 1,048,576-byte bound.

@history_item n=32 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` and its pin `CapabilityStatementTest`: every verb and cause a code span, the five counts in words, and README.md's «the {word} driver commands as tools». Why it matters: a new verb has to move `docs/capabilities.md`, README.md and the pin together.

@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand serves Cells and a load page) — `seven_guis::stand` serves six tasks (the four lean ones, Cells and the load page with `LoadNet`'s delayed fetch), and `escher-session` serves the six. Why it matters: these are the served screens the read must be proven on, the load page with its fetch in flight.

@history_item n=34 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (stand checks re-pinned to the flight booker's select) — the click census: a driver `click` is accepted on 68 of 77 lean-task nodes at boot and refused on 9 (three `disabled`, four `task-header-spacer` `covered`, two flight-type options `covered`). Why it matters: the read's acceptance on the stand must agree with these figures in both layout modes.

@history_item n=35 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served-page checks, the scrollbar hazard) — `stand_cells_served` and `stand_load_page`, `unkeyed_actionable` empty on all six served tasks. A measured hazard: right after a `scroll`, a pointerdown in the shown overlay-scrollbar band is a thumb drag that dispatches nothing. Why it matters: a check comparing the read with a real call inside a just-scrolled box must `advance` past the fade first, and an accepted answer there is no dispatch.

@history_item n=36 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (row helpers) — the MCP listing test asserts each row through private `#[track_caller]` helpers that name the row and index only. Why it matters: the new tool's row goes through this row walk.

@history_item n=37 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms; a non-invertible transform takes no hit) — `visible_region` reads clipping boxes through their transforms. The limit that stands is the rotation's, stated in `off-screen`'s meaning. `hit_inner` answers nothing under a determinant of 0 or a non-finite one. Engine changes ratified on F1 and F2. Why it matters: the off-screen and covered causes the read predicts read these readers and their stated limits.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`multiple` select, `optgroup`, the public select readers) — models `click_select_row` and `optgroup`. `option_disabled` is the one rule for an option (its own attribute or its group's), and `select_of`/`select_values`/`closed_select_of` are public read-only. `owner_select` and `option_disabled` stay crate-private. Why it matters: owner-served nodes and an option's disabled cause must go through the public readers; widening a private one is an engine change and the founder's fork.

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the select limits; `type` and `press` on a `multiple` select; the lint's overlap) — an option of a closed single select, under a group too, stays refused `covered`. `type` focuses a `multiple` select without a click, and `press` carries no Ctrl. The lint reads closed-select options through `closed_select_of` by the element, never the role. Why it matters: these fix the per-verb answers for select options and rows, and the element-not-role rule for the select test.

@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's exceptions to scrolling and the hit) — `hit_with_scrollbar` answers `select_list_hit` first, so the driver's `covered` reads the list. `visible_region` for an open row answers the list's scrollport. `scroll_into_view` on a row writes the list alone. Why it matters: the read's covered and off-screen answers for rows, and for targets under an open list, must come through these same readers.

@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded) — Enter, a Space with no typed text, or a click opens a single select's list; a click on a row chooses. The engine change was ratified on F1–F10, and the closed-option limit was kept. Why it matters: a fixture proving an owner-served option accepted must open the list first.

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers; the list's state; the bridge's select tests; the driver's verbs on a select) — the list crosses only as row `bounds` and pixels; the snapshot has no `expanded` key and `NODE_FIELDS` is unchanged. `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout` are public read-only, and `close_select_list` is the writer. `closed_select_of` answers `None` while the list is open, and `is_select` is called by `locate` alone. `Target.select` and `focus_select` exist. `covered`'s meaning gains the "option of a closed select" clause. Why it matters: the read must use only the read-only list readers, never `close_select_list` or `focus_select`, and must not widen `NODE_FIELDS`.

@history_item n=43 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the bridge's listener attributes over-report) — the vdom walk reads current listeners exactly, while the listener mark (`data-dioxus-id`) over-reports a removed listener, so `unkeyed_actionable` over-includes by its listener reader. Why it matters: the read's actionable set and the actionable-key fold-in must state which reading they take; `clickable`/graph handlers read the exact one.

@history_item n=44 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (a node's component through public API) — a node's owning component and a registered signal's readers read through Dioxus 0.7.10's public API with no fork change, while hook-local state is unreachable. Why it matters: this is the node model the UI graph builds on and the read lists from.

@history_item n=45 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers) — a ratified boundary widening: `graph` crosses the socket `call`, the command line and MCP and rests in a kept check, holding ids, owner ids, names, closed flag words and booleans, never a value, typed text or `NodeId`. No argument, one refused `malformed`; no new carrier, exit, cause, session error, span field or dependency. Ratified provisionally, pending the founder's own word in his batch at the epoch boundary. Why it matters: this is the nearest precedent and the boundary shape the new read's crossing will be judged against.

@history_item n=46 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the `graph` verb in the Driver session contract) — `VERBS` goes 12 → 13 (instance verbs 8 → 9), `FieldKind` 10 → 12, describe to 13 verbs and 14 node keys. Adds `Command::Graph`/`Outcome::Graphed` (dispatches nothing), and its node ids feed the record of ids. `verb` alone on the span, `tools/list` thirteen, a hand-written `{"nodes":[…],"states":[…]}`. Why it matters: these are the counts and registries the new verb extends again, and the pattern it follows.

@history_item n=47 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — `{owner}///{key}`, spelled by `virtual_node_id` and `VIRTUAL_KEY_SEPARATOR`, written by no walk; a stable-id contract change on the founder's P4 fork 2. Why it matters: the read's optional owner must stay open to this form for 0.4.0's virtual nodes.

@history_item n=48 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph reader) — `DioxusDocument::graph`, read-only under `accessibility`: one `GraphNode { id, owner, component, handlers, reads }` per snapshot node in pre-order, `owner` the stable id of `select_of`. Why it matters: this is the node model the read takes, with owner-served options named by their select.

@history_item n=49 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's `clickable` reading) — `NodeState.clickable` is true where the exact listeners hold `click` on no `is_disabled` form control. `NODE_FIELDS` is fourteen. `element_listeners` is the exact reading, never the mark. `CLICKABLE` on the stand: 0 on five tasks, 1 on Circle Drawer, 676 on Cells, against the mark's 677. Why it matters: the actionable set and the disabled cause must agree with `clickable`, measured against these counts.

@history_item n=50 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch; feature gating) — the `graph` module is gated by `accessibility` and `signal_registry` is ungated. The dispatch change on every Dioxus document's event path is inert without a registry. Why it matters: any reader added to `graph.rs`, `snapshot.rs` or `actionable.rs` stays under the `accessibility` gate.

@history_item n=51 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`stand_graph` and the shared modules) — `stand_graph` (6 tests in both modes) checks that a graph read changes nothing, holds no typed text, puts no `///` in a served id and keeps Cells within the bound. `session_common` gains the `graph` builder and `graphed` reader, now read by 33 checks; `common` by 22. Why it matters: these are the shape and reader counts the new read's stand check extends.

@history_item n=52 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the log checks read the graph's names) — `graph_needles` sits in seven_guis' `tests/common/mod.rs`. A `graph` call's `command` span records `verb` alone, parented by `tool_call`, and `host_log`/`mcp_log` read 0 needles at `trace`. Why it matters: the new read's call needs the same log-check witness and a needle set in the same module.

@history_item n=53 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (thirteen verbs where a surface counts them) — the README count, the recording's instance verbs and `Check::from_bytes`, the lint and load-wait rows, the served command line, and the cold-agent isolation count of thirteen `mcp__escher__{verb}` tools. Why it matters: this is the full list of count sites a fourteenth verb must move together.

@history_item n=54 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unpaged graph and its Cells size) — `graph` answers the whole screen, and Cells reads 293,869 bytes under the 1,048,576-byte bound after two narrowings. The plan's forecast was disproved, and the stated limit «An unpaged graph» is owned by `res-7cc5bi4k`. Why it matters: the new read's whole-screen answer on Cells must be measured against the same bound, and a forecast must be re-measured.
