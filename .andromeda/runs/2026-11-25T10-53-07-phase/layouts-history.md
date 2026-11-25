@history_item n=1 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the stand's nested scroller: added the rule that an into-view scroll moves every holding scroller then the viewport and that `visible_region` reads what an element is seen through (founder-ratified 2026-10-07, every document), with the CRUD row past the list refused `off-screen`. Origin of `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes`; a virtual node read by `layout` and reached by `explore` sits inside its canvas's scrollers, so this rule (and its `off-screen` refusal) is what any virtual-node reach or explanation must agree with.

@history_item n=2 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — what a scrolled `crud-list` reads (same four figures, unnamed in the diff) and a `click` naming the list landing on the row at its centre; Home's box scrolling, Cells hit only once scrolled in. Precedent that bounds and hit readings across scrollers must be measured, not assumed — a standing check went red when the master had not said what was hidden; applies to a virtual node's bounds and reach in the new reads.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list; an open list's rows, the exception to the scrolling boxes rule) — an open select's rows are painted with no layout box yet read their own `bounds` in snapshot and diff; `visible_region` answers the list's scrollport for them and `scroll_into_view` writes the list's `scroll_y` at once — the `except:` clauses of `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes`. Closest precedent for non-DOM-box targets (virtual nodes) entering reads: an exception recorded as the code does it, and a row's own `bounds` read as inside the ratified crossing by the field rule of 2026-10-10 — the same question this chunk faces for virtual ids in graph/actions/layout/explore.

@history_item n=4 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_closed_select` retired for `Entry.in_select_box`) — the lint skips entries in a select box, `clip_region` takes an open row's region from `visible_region` alone; read by element never role (CRUD's `role="option"` rows not taken as a select's). Precedent for how lint admits or excludes painted non-DOM entries; this chunk touches `lint.rs` under F9 A (lint holds no virtual node) and must keep `at_boot` `[0, 0, 0, 0]`.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading, a closed select's options skipped) — moved the stand's boot lint to all zero by skipping closed-select options on F9 (the founder's word); background to the stand-lints-clean rule any lint change in this chunk must keep green.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session` the driver's command line with its four endings: accepted one uncoloured JSON line exit 0, refused exit 1, usage exit 2, session error exit 3. Origin of `contract/escher-session-exit-codes/0` and `/1`; the scope expects no new exit, refusal cause or session error — a virtual id read `not-found` (F10 B) or now answered rides these exits (F3 A).

@history_item n=7 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role; "no ending writes back anything the caller typed" scoped to the commands. Origin of `decision/no-caller-input-echoed`; a virtual id `{owner}///{key}` answered by `layout`/`actions`/`graph` must stay inside that ban and its one `layout` exception.

@history_item n=8 marker=2026-10-12-self-description
**2026-10-12-self-description** — `escher-session help` answers `describe()`'s seven-key line of fixed table text; MCP tool descriptions are each verb's help plus result fields. Origin of `contract/escher-session-help/answer`; the scope sweeps help texts and `describe()` as literal pins when the reads gain virtual nodes.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot [--id]` answering `{"width","height","png"}`, an element shot of no pixel refused `off-screen`, `fonts-unavailable`, the MCP image block and the 1,048,576-byte result bound. Origin of `contract/escher-session-screenshot/answer`, `contract/escher-session-refusals/off-screen`, `contract/escher-session-mcp/screenshot-result` and `/result-bound`; `screenshot --id` on a virtual id reads `not-found` under F10 B, a limit this chunk inherits and must not widen silently.

@history_item n=10 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — the `lint` verb answering `{"findings":[…]}`, kept under `--record`, its MCP tool. Origin of `contract/escher-session-lint/answer`; F9 A holds the lint free of virtual nodes, and its answer carries ids, closed words and numbers only.

@history_item n=11 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answering `{"text"}` or `{"text","part","parts"}`, a part past the last empty text. Origin of `contract/escher-session-snapshot/answer`; the snapshot already carries virtual nodes (prior chunk), and the graph's one-node-per-snapshot-node must agree with these parts.

@history_item n=12 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` keeps a session as one owner-only replayable check (founder-ratified F1 A, F2 B). Origin of `decision/session-recording-replayable-check`; kept checks are what the coverage role replays, where a virtual node's reach is now read.

@history_item n=13 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check keeps per call its verb, arguments, acceptance, answer digest and masked answer, replayed by `Check::replay` (founder F3 B). A virtual-id `click` kept in a check and replayed is the path through which coverage reads a virtual node's reach; any answer gaining virtual ids lands in the record masked as on every carrier.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — `graph` answering `{"nodes","states"}`, ids, component names, handler event names, state names, closed flag words and booleans only, never a value. Origin of `contract/escher-session-graph/answer`; this chunk adds one graph node per virtual snapshot node with its owner, lifting F9 A's exclusion, inside this carrier's closed field set.

@history_item n=15 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — escher-session gains the graph verb; ratified — escher-session's coverage role) — the founder's own word replaced the overseer's delegate confirms of the graph verb and the coverage role. The `graph` and `coverage` rows this chunk extends stand ratified, so a change in what they carry is read against founder-ratified text, not a delegate's.

@history_item n=16 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/escher-session-graph/answer`: graph takes `--part`, part 1 alone carrying the states. Virtual graph nodes must page within this part form.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — `actions` answering `{"nodes":[{id,owner,keyed,verbs:[{verb,cause}]}]}`, unpaged, never a value, typed text or `NodeId`, founder-ratified on every carrier. Origin of `contract/escher-session-actions/answer`; this chunk lists a virtual node there with its verbs and refusal causes (`owner` already a field; no new cause word expected).

@history_item n=18 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role; the coverage role a further consumer of the TaskShell mount) — `coverage <app> <check>...`, checks read bounded at 1,048,576 + 1 bytes and refused `malformed` holding nothing of path or file, answering `{"unexercised":[{id,owner,event}],"checks","held"}`, a failed boot exit 3; each check replayed on a fresh boot. Origin of `contract/escher-session-coverage/answer` and `/check-bound`; this chunk reads a virtual node's reach in handler coverage within that answer form.

@history_item n=19 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-layout/answer`, `~ decision/no-caller-input-echoed` (except: the layout answer's id field names the call's id), verbs fourteen to fifteen. `layout` currently reads a virtual id `not-found` (F10 B); answering one makes `{owner}///{key}` echo through that exception and requires bounds/placed/width/height for a region with no layout box.

@history_item n=20 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirm under «Глагол `layout --id`», so the layout answer row and the no-echo exception this chunk extends stand on his own word.

@history_item n=21 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the `escher-session` host as a cli entry and consumer of the TaskShell mount; with 2026-10-10-driver-cli the root of `contract/escher-session-command/form`, whose verb count (sixteen) and `VERBS.len()` are literal pins this chunk sweeps without adding a verb.

@history_item n=22 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — `~ contract/escher-session-command/form` and `~ contract/escher-session-mcp/tools` to sixteen verbs (canvas after layout), `~ contract/escher-session-refusals/fonts-unavailable` (canvas added), `+ contract/escher-session-mcp/canvas-tool` (PROVISIONAL, delegate), and the circles crossing as shot pixels and canvas draw commands. The current verb-count pins and the canvas carrier on which `canvas --id` reads a virtual id `not-found` (F10 B).

@history_item n=23 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — `+ decision/circle-drawer-circles-painted-on-canvas`: circles painted by a canvas widget from `data-circles`, never DOM nodes, reaching no snapshot, tree or graph. Circle Drawer's circles as virtual nodes is out of this chunk's scope; the row's «no snapshot, tree or graph» must stay true while the graph gains virtual nodes only from declared fixtures.

@history_item n=24 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** (Surface: desktop-native: 1 row(s); ratified — Circle Drawer circles painted on a canvas) — the founder ratified `decision/circle-drawer-circles-painted-on-canvas`, state standing; its wording is founder-held, so a graph that reads virtual nodes must leave the Circle Drawer clause untouched until its own chunk.

@history_item n=25 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `~ decision/circle-drawer-circles-painted-on-canvas` (lint reads label ink boxes, F1 A) and `~ decision/stand-lints-clean-at-boot` (Circle Drawer no canvas finding after a click, at 12 and 100 circles; PROVISIONAL, delegate). The most recent rewording of the canvas row; precedent of a canvas read entering one driver read by restating the row's reach — the pattern for this chunk's graph/actions/explore/layout reaching virtual nodes, while lint stays without them under F9 A.

@history_item n=26 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence), each verified against the code; the row handles this chunk's amendments must name (`~ {handle}` / `+ {handle}`) come from here.
