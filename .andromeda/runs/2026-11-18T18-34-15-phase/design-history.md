@history_item n=1 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — (1 of 2, ~ contract/escher-session-tasks/cells · counter · crud · flight-booker · load · timer) every served-task row moved from «six» to «seven» tasks with the order page. The live run gives the agent these seven apps (scripts/cold-agent.sh:20-31); the README/capability statement and v030-15's concretized acceptance must say seven apps, not 0.2.0's six.

@history_item n=2 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — (2 of 2, + contract/escher-session-tasks/order) order added as the seventh served task, on the founder's F5 «В 0.3.0, до прогона» relayed by the overseer. It is the app whose controls panic, return an error and log; the CARRY needs it met and `caused_by_kind` at least 1 per kind, and its authored error text in `caused` is the ratified carriage (F5).

@history_item n=3 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — + contract/escher-session-commands/layout-answer and layout-arguments: `layout --id` on every carrier, one uncoloured JSON line, a missing id refused `malformed` (status 1), not a usage ending. `layout` is one of the capabilities the verdict requires used (`capability-unused` at 0), and a refused `layout` counts toward wrong calls.

@history_item n=4 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the `actions` answer line) added `escher-session actions`: no argument (an argument refused `malformed`), one uncoloured line `{"nodes":[{id,owner,keyed,verbs:[{verb,cause}]}]}` on every carrier including mcp; verbs decided by the same `validate` and `check_target`, so a listed cause is the cause the call returns; `stale` for an unresolved node. Its crossing to the provider is what F4 ratifies; the pipe requires `actions` used.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the unpaged `actions` answer, a stated limit) `actions` is unpaged; Cells at boot answers 106,088 bytes for 678 nodes, within the 1,048,576-byte bound, owned by residual `res-7cc5bi4k`. A large tool result on Cells weighs on the run's 600 s timeout and $3 budget; the reach statement should keep the limit.

@history_item n=6 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the cli surface's graph answer line) added `escher-session graph`: no argument, one uncoloured `{"nodes","states"}` line, over mcp the text block of that line; carries ids, names, closed flags and booleans only, never a value or typed text. Rows contract/escher-session-commands/graph-answer, graph-arguments and contract/ui-graph-line/content come from it; it is the carriage F4 ratifies, and the pipe requires `graph` used.

@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the unpaged graph, a pending limit) `graph` is unpaged; Cells at boot answers 293,869 bytes, the stated limit «An unpaged graph» in docs/capabilities.md, residual `res-7cc5bi4k`. A trap for the live run: a `graph` call on Cells is a large mcp result against the 600 s timeout and $3 cap, and the reach statement keeps this limit.

@history_item n=8 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — (ratified — the graph answer line; ratified — the coverage role) the graph line and the coverage role's records moved from PROVISIONAL (delegate the overseer) to ratified by the founder's own word. The `graph` answer rows this run relies on rest on the founder's ratification, and F4 extends it to carrying that answer to the model provider.

@history_item n=9 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage role's answer line, endings and refusals; the three stated limits) added `escher-session coverage <app> <check>...` with no new exit status, and its limits (canvas handlers uncounted, unreached screens unlisted, `stopPropagation` unread). The capability statement on a pass names handler coverage as where the run's unreached handlers are proven, so its limits bound that reach claim.

@history_item n=10 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the 0.2.0 cold-agent chunk set `escher-session` to six served tasks (counter, flight-booker, timer, crud, cells, load) in the usage line's `apps:` and the MCP `start` enum, and recorded two traps for an agent: a driver `click` on a Cells cell in the shown overlay scrollbar's band right after a scroll is taken as a thumb drag, needing an `advance` past the fade first; and the load page answers its stylesheet after `LOAD_ANSWER_DELAY` (200 ms), so a `click` on `load-fetch` waits it out. This is the 66-call, 0-wrong baseline the 5 % bar is set from, and these traps can still turn into wrong calls in the live run.

@history_item n=11 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the flight-type select overrides and the load page's group leave design-system) the load page and select facts moved to layout-templates; the founder kept the cli item naming the served tasks and the driver's load-wait items in motion as they stand («Оставить как есть»). The served-task list stays a design-system row, which is why served-error-page amended it to seven.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the exit-status rows: one JSON line per command on stdout, keyed by the schema's words, with no colour, no ESC byte and no terminal test, and exit 0 accepted · 1 refused · 2 usage · 3 session error, on the founder's answers. The pipe reads refusals (status 1) by cause from this form, and a readiness check that drives the apps locally should read the same lines.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: stdout holds only newline-delimited JSON-RPC 2.0 lines, one reply per request, no ESC byte, and an `Err` ending's message goes to stderr with status 3. The live run's `claude` client talks to `escher-session mcp --record` over exactly this stdout (rows contract/escher-session-mcp/*, contract/escher-session-stdout/mcp-line), and an `Err` ending reads as a `client-exit` or `no-result` failure.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — the exit statuses are read from the schema's `EXIT_STATUSES` table; `help` answers the self-description as one line without reaching a session; a session error carries its fixed `remedy`; over mcp, `help`'s result is that line. These are the rows decision/exit-statuses-from-schema-table, contract/escher-session-commands/help and contract/escher-session-session-error/remedy, and the self-description is what a cold agent reads first.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a screenshot whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; `screenshot [--id]` answers `{"width","height","png"}`, and over mcp it answers an image block then a text block. This is the source of the typography screenshot rows and the mcp screenshot-result row. The run must use `screenshot` (capability-unused), and the stand's bundled font is what keeps that call from being refused.

@history_item n=16 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand registers the bundled DejaVu Sans for every generic with system fonts off on native, the reason a stand screenshot is never refused `fonts-unavailable`. The reach statement names «the bundled fonts» as where that side is proven.

@history_item n=17 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's font registration leaves for architecture's Headless stand) the bundled-font registration is now held in architecture §Standard Contracts → Headless stand, not design-system. Where the run's reach statement cites the bundled fonts, cite architecture.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording's `record-dir-not-private` and `recording-failed` are session errors with exit 3 and no exit of their own (row contract/escher-session-exit-status/3). The live run uses `mcp --record`, and its verdict fails on `recording-not-kept`, so these session errors are the ways the recording side ends the run.

@history_item n=19 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part <part>]` answers `{"text"}` whole or `{"text","part","parts"}` in parts, and over mcp a paged result is that JSON line, on the founder's F1. The pipe requires a part read (capability-unused), and this is the form of rows contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result.

@history_item n=20 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every member of the animating set the same way, none holding it open, and reads animation time only through the harness clock without advancing it. This is the origin of decision/settle-does-not-wait-on-animation: an agent's step returns settled beside a running animation.

@history_item n=21 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves the app's own time (whole Timer ticks on the stand), and `click`, `type` and `press` leave the clock where it was; later widened by one-virtual-clock. The run requires `advance` used, and the Timer goal hangs on it.

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock`. The driver's `advance` moves it by exactly its `ms` together with the app's own time, and click, type and press leave it alone; the scrollbar fade reads true for 700 ms of that clock after a `scroll`. This is the source of decision/animation-driven-by-virtual-clock, contract/driver-verbs-and-virtual-clock/advance and decision/driver-advance-moves-one-clock. It was PROVISIONAL here and ratified in the 2026-10-18-engine-debts entry (n=23).

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder, Epoch 2 close) ratified one-virtual-clock's `advance` moving the clock by exactly its `ms` and the app's time together, along with the paint-clip hit stop. The motion rows the run's `advance` calls rest on are therefore founder-ratified.

@history_item n=24 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait moves no clock: while a step waits on a load for at most `LOAD_WAIT` (5 s wall time), the `VirtualClock` stands still. This is the source of decision/load-wait-moves-no-clock. The load page's click waits on it, so in the live run it costs wall time against the 600 s timeout.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list answers the hit before the page's walk) the open list answers first in `hit_with_scrollbar`, so `hit`, hover, pointerdown and the driver's `covered` read it, and a button under the list is refused `covered`. This is the source of decision/open-select-list-hit-first, and the cry-32fid2ry CARRY says this `covered` clause can move `wrong_calls`; the report states `refused_by_cause` as read.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list paints last, whole past clips and opacity) the list paints last in `paint_scene`, whole past an `opacity` ancestor and a clipping box. This is the source of decision/select-list-painted-after-root: a screenshot of an open list in the live run reads it.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open list's wheel and key, and no motion) the select list has no motion, so a screen read right after a step reads the new state without an `advance`, on the founder's F5 (reduced motion). This is the source of decision/select-list-opens-without-motion, and the list opening the pipe requires needs no `advance`.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list's row cap, placement and row height) at most `MAX_ROWS` = 20 rows; `place` keeps the list inside the viewport, between its select's width and the viewport's, with every edge finite. This is the source of decision/select-list-placed-in-viewport.

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a single select's list is painted, an engine overlay) a click, Enter, or a Space with no typed text standing opens a single select's list; it is an engine overlay through `paint_scene`, not an OS window, and the windowed list was read but not run. This is the source of decision/single-select-open-gestures. It replaced 2026-10-31-engine-debts' «no select popup is painted» and is the list opening the pipe counts.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (how a single select's list opens, moves, chooses and closes) the list opens on its chosen option with no `input`/`change`; Enter, Space, Tab or a row click chooses with one `input` then one `change`; Escape, a second click, a click outside, a wheel elsewhere or a page scroll closes it; a disabled, inert, hidden, empty or `multiple` select opens nothing. This is the source of contract/select-list-events/* and decision/selects-that-open-no-list, and an agent's call against a select that opens nothing is a possible wrong call.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the tree, the snapshot and a shot read of an open list) the highlighted row is in no snapshot or tree; an open select reads `ComboBox` and `expanded`, and its chosen option reads `selected`; a row carries its own `bounds`, and a screenshot of a row crops to it and leaves the list open. This is the source of contract/select-accessibility/*, contract/snapshot/select-list-row-bounds and contract/screenshot/select-list-row: what the agent sees of a list opening.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`mix-blend-mode` and `isolation` in the depth rules) the open list's paint-last row now also paints past the blend layers. This is the current wording of decision/select-list-painted-after-root.

@history_item n=33 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (364 records into 137 rows), each checked against the code, with fates approved by the founder. This explains the handles the picked rows carry, and the cli surface's form, motion and select rows are stated as rows only from here on.
