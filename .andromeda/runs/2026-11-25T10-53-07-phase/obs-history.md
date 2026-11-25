@history_item n=1 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** — 6. Log Coverage: widened `~ decision/declared-node-read-logs-nothing` from the declared-node read alone to the virtual nodes' place in the accessibility tree, the snapshot and a click on one, all with no log site, print, span or field. This chunk is the next widening of the same row: the graph, actions, handler coverage, layout and exploration reading a virtual node must stay inside it, and the row's reader list must name them.

@history_item n=2 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** — 6. Log Coverage: `+ decision/declared-node-read-logs-nothing`. The founding row of the virtual-node line: the read, its validation and its answer have no log site, and a refused declaration holds a closed reason and its index alone. It rests on no fork answer, so it stands unmarked. This chunk extends that register.

@history_item n=3 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — 4. Span / Trace Coverage: `+ decision/canvas-read-opens-no-span`. The canvas read opens no span of its own and runs inside the canvas call's command span. Precedent: a new driver read adds no span by design. Under the CARRY (F10 B), `canvas` reads a virtual id as `not-found`, a refusal recorded only through `cause`.

@history_item n=4 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — 4. Span / Trace Coverage: `~ decision/canvas-read-opens-no-span` widened to cover the lint's canvas label reading (no span, nothing logged, inside the lint call's command span). The lint is one of F9 A's four virtual-node-free readers, so whether it now reads a virtual node lands on this row.

@history_item n=5 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (3. Observability Harness Contract) — `~ decision/sink-host-read-at-trace` gained the canvas needle, and `~ decision/part-answer-never-logged` gained the canvas part. The latter is PROVISIONAL (delegate the escher overseer). Both are picked rows. Virtual ids and names that now ride the graph, actions and layout answers are candidate needles for `host_log` / `mcp_log`; the binary-level reading itself is out of scope.

@history_item n=6 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (4. Span / Trace Coverage, 6. Log Coverage) — `contract/command-span/verb` and `contract/tool-call-span/tool` moved from fifteen to sixteen words, `+ contract/command-span/canvas`, and `contract/command-span/settled` gained canvas. These are literal-count pins. The scope expects no new verb, so the counts must read unmoved. A new verb or cause would be a fork.

@history_item n=7 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — 4. Span / Trace Coverage: `+ decision/layout-read-opens-no-span`, so that the absence is not read as drift. Section 3 also named `stand_layout`'s own recording subscriber as an exception in `decision/checks-silent` and `decision/headless-stand-no-sink`. The layout read is one of this chunk's readers, and its guard `the_read_writes_no_escher_line_at_trace` should cover a virtual id's reading (or its `not-found`).

@history_item n=8 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (4. Span / Trace Coverage) — `~ decision/layout-read-opens-no-span`: the layout read runs inside the layout verb's command span and opens none of its own. `contract/command-span/settled` gained layout. These are the picked row's current wording. An answer for a virtual id goes through that same path.

@history_item n=9 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (6. Log Coverage, 3. Observability Harness Contract, 8. PII) — Added `+ contract/command-span/layout`, the sink-host needle set gaining the layout answer's keys, and `+ contract/layout-answer/content` / `+ contract/layout-answer/logging` (ratified via 2026-11-17-a-component-in-isolation). A layout answer for a virtual id must keep the content class (ids, closed words, numbers) and stay never-logged.

@history_item n=10 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — 8. PII: `~ contract/layout-answer/logging` added the cold-agent transcript as a carrier. A later entry under the same marker ratified the layout and error-field delegate confirms. A virtual node's layout answer crosses to the model provider on a live run, the same carrier set F3 A's exits are read against.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph verb and never-logged answer) — `Outcome::Graphed` is written by `to_json` alone, never logged or fielded, and the graph verb runs under the one command span. This is the base of `decision/graph-answer-never-logged` and `contract/command-answer/graphed`. Graph nodes for virtual snapshot nodes, with their owner, extend this answer.

@history_item n=12 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set) — The graph answer carries ids, component names, state and event names, flag words and booleans, never a value. It is a boundary widening, ratified by 2026-11-15T05-21-42-wrap. A virtual node's id `{owner}///{key}` and its name entering the graph must be read against this carried set and F3 A; a new crossing class would be a fork.

@history_item n=13 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (command and tool_call lines) — A graph's line is `verb="graph"` and holds no node, state, id or name. `host_log` / `mcp_log` read 0 graph needles at trace under both builds, and `stand_act_spans` makes one call per mode. Virtual graph nodes must stay off this line.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph modules) — dioxus-native-dom's `graph` and `signal_registry` modules and the bridge dispatch add no log site, and a page with no registered state skips the registry (`decision/no-state-page-skips-registry`). The graph module is where F9 A's exclusion lives, and its silence census must still read 0 after the change.

@history_item n=15 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (clickable) — The snapshot's `clickable` is a closed bool: true where an element's current listeners hold click and it is no disabled control, carrying no page text. This is the source of `contract/snapshot-node/clickable` and `contract/snapshot-text/clickable`. Under F12 A a virtual node's `clickable` reads false; changing it touches these rows.

@history_item n=16 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (3. Observability Harness Contract, 6. Log Coverage, 8. PII) — The graph gained parts. Rows touched: `decision/part-answer-never-logged`, `contract/command-answer/graphed`, `contract/graph-answer/shape` (states in part 1 alone), `contract/command-span/graph` (part 1..1000, no part number fielded) and `contract/graph-answer/content` (part number and count). Virtual graph nodes must split across parts under the same shape and span rules.

@history_item n=17 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — Ratified the graph answer's carried set and the coverage role's sink reach, endings, command-span reach and coverage line fields, all formerly PROVISIONAL. Widening any of these carriers with virtual ids rests on these ratifications.

@history_item n=18 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set) — The actions answer carries ids, owner ids, verb words, closed cause words and booleans only, never a value, typed text, mask or `NodeId`. It is a boundary widening, ratified. This is the source of `contract/actions-answer/*`. A virtual node listed with its verbs and causes must fit this shape, with `owner` the canvas.

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (command and tool_call spans) — Brought in `contract/command-span/actions`: an actions line is `verb="actions"` and holds no node, id, owner, verb word or cause word of its answer. The verb went to fourteen words, ten of them instance verbs. A virtual node's would-be causes must not leak into the span's `cause`.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read) — `Session::actions`, `check_target` and `target_of` open no span. The read changes nothing in the document (`decision/actions-read-changes-nothing`, guarded by `stand_actions`). Listing virtual nodes goes through `check_target` (the F10 B arm) and must keep that no-change guard green.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (needles and silence) — Added `actions_needles` for `host_log` / `mcp_log`, put `stand_act_spans` at 38 calls, and recorded `stand_actions` and `actionable_nodes` as silent. These are the literal counts and guards to sweep; `stand_actions.rs` asserts no `///`.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage module, reached record and their check) — The `coverage` module and dioxus-native-dom's `reached` record log nothing. The record holds stable ids and event names only, never a `NodeId`, and its Debug prints its count alone (`decision/reached-record-ids-and-events-only`). A virtual node's reach goes into this record (`reached.rs` is in scope) and must keep it id-and-event only.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage line's carried fields) — The coverage line carries stable ids, owner ids, event names and two counts only. It is a command-line answer, never logged, and the widening is ratified. These are the `contract/coverage-line/*` rows. Virtual ids in `unexercised` sit inside this carried set.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (command span reach, endings, sink reach) — Each replayed step of a kept check writes its own command span, and the role adds none. The coverage line is written on stdout, never as a log line. `host_log`'s coverage run reads 0 needles at trace (`decision/coverage-line-never-logged` guard). A replayed click on a virtual id leaves only its command span.

@history_item n=25 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (4. Span / Trace Coverage) — `+ decision/exploration-adds-no-span`: each call an exploration drives runs through the session's run and leaves its own command span. A virtual node as an exploration candidate adds calls but no span.

@history_item n=26 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (7. Error Capture & Reporting) — `+ decision/exploration-catches-panics`: a panicking explored call is reported as the closed ending Panicked, with no payload. Exploring a virtual node keeps that ending closed.

@history_item n=27 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — Made `Cause::name` the domain of the span's cause field. Rule: never field a `Command`, `Call` or `ArgValue` with Debug, nor an argument by its schema name. The F10 B `not-found` for a virtual id must be a `Cause` name; an id fielded would print a virtual id.

@history_item n=28 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — The executor stays silent. `Outcome` joins the Debug-deriving types nothing prints. Rule kept: field a cause by `Cause::name`, never an `Outcome` with Debug. This is the base of `decision/debug-types-never-logged`, which `execute.rs`'s resolve/locate/check_target changes must keep.

@history_item n=29 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — Set the command span's cause to one of eight names, ratified at 2026-10-18-engine-debts. Covers the picked rows `contract/command-span/cause` and `contract/refusal-cause/*`. A new refusal cause for virtual nodes would break the ratified eight and is a fork.

@history_item n=30 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — An empty crop is refused `off-screen` (`contract/refusal-cause/off-screen`), and image bytes are never fielded. Under F10 B, `screenshot` with a virtual id reads `not-found` rather than a crop; any change to that arm touches this cause row.

@history_item n=31 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built) — Eight fields, never the screen text, any argument, or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`. This is the origin of `decision/driver-spans-carry-no-content`, `decision/command-span-no-call-content` (guard `stand_act_spans`) and `contract/command-span/field-count`. The scope adds no span field.

@history_item n=32 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — Ratified the closed span's line. Diff sizes (`added`, `removed`, `changed`) may reach a log, but diff content never does. A click on a virtual node changes the diff sizes on the command span, never its content.

@history_item n=33 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line writes answers, not logs) — Standing rule: the next surface adds no span or log field that holds what a call supplied or a screen reads. This is the base of `contract/escher-session-stdout/command-answer` and the exit-code rows 0 and 1.

@history_item n=34 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — Introduced the `tool_call` span (`tool`, `ending`), ratified. This is the base of `contract/tool-call-span/line`. A virtual-id call over MCP leaves `tool_call` with its `command` child and adds no field.

@history_item n=35 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined both spans' closed sets, and the `help` call leaves `tool_call` with no `command` child (`contract/tool-call-span/help`, `contract/escher-session-commands/help`). The help texts and `describe()` are fixed text; the literal pins in them must be swept if the wording changes.

@history_item n=36 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of them, so changes are made by hand (`decision/capability-statement-updated-by-hand`). The scope sweeps that file's counts and its pin; its «Pinned by a check» lists the F9 A / F10 B / F12 A limits this chunk lifts.

@history_item n=37 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — Brought in `lint` inside the command span, its findings never fielded (`contract/command-span/lint`, `contract/command-answer/linted`), with no span field (the founder's F2). The lint is in F9 A's exclusion, so lint findings over a virtual node keep this shape.

@history_item n=38 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — A part read records verb alone and holds no part number or count (`contract/command-span/snapshot-part`). Virtual nodes already in the snapshot also ride its parts.

@history_item n=39 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — 3. Observability Harness Contract: `~ decision/answers-echo-nothing-typed` except-clause in its current form. It was ratified in 2026-11-17-a-component-in-isolation. An answer naming a virtual node must still echo nothing the caller typed.

@history_item n=40 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — Rewrote `decision/driver-library-installs-no-subscriber` and `decision/driver-emits-no-event` around escher-telemetry's scoped capture during an acting step. A click on a virtual node is an acting step under that capture, with no event added.

@history_item n=41 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — Reached the current wording of `decision/driver-library-installs-no-subscriber` (capture plus chaining panic hook) and `decision/driver-emits-no-event` (keeps records and panics in process). These are the picked rows' wording that the new readers must leave true.

@history_item n=42 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** — Graph, actions and layout became counted verbs, reversing `decision/graph-actions-layout-not-counted-verbs`. `+ contract/cold-agent-verdict/refused-by-cause` counts the eight cause words from the command span's cause field and ignores any other word. A virtual id's F10 B `not-found` counts as `not-found`; a new cause word would go uncounted.

@history_item n=43 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — `~ decision/select-chosen-set-never-logged`: a list box's chosen row and labels cross in the snapshot, the diff and the actions answer with no log site, span or field. This is a precedent for widening what the actions answer lists without a new log surface or field.

@history_item n=44 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — Trap: a zero read from a host that installs no sink says nothing about one that does. The in-process fixture-canvas proof of virtual nodes in the graph, actions and layout says nothing about the sink-installing host (`decision/sink-host-read-at-trace`, `decision/session-logs-carry-no-page-content`). That reading is out of scope, owned by «Circle Drawer's circles as virtual nodes».
