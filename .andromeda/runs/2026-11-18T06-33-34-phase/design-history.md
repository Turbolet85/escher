@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the driver command line's output form: one line of JSON on stdout per command (answer, refusal or session error's object keyed by the schema's words), usage error one fixed stderr line, no colour, no ESC byte, no terminal test, exit 0 accepted · 1 refused · 2 usage · 3 session error. It is the origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the exit-status/exit-code rows. The cold-agent verdict reads these lines and statuses, so it must take refused (1) and error endings from this fixed form and add none.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role: stdout holds newline-delimited JSON-RPC 2.0 lines only, one reply per request, a tool result's text is the screen text for `snapshot` and the JSON line otherwise, an `Err` ending's message on stderr with status 3. It is the source of contract/escher-session-mcp/* and contract/escher-session-stdout/mcp-line. The cold agent reaches 0.3.0's reads only over this carrier, so the widened task and the fake `escher-session` in `.github/scripts/test_cold_agent.py` must answer in this shape.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses are read from the schema's `EXIT_STATUSES` table that `help` serves, a session error's object carries its fixed `remedy`, and `help` answers one uncoloured line. It is the source of decision/exit-statuses-from-schema-table and contract/escher-session-session-error/remedy. The pipe's verdict should count endings by these table classes, and its cause counts must use closed words only, never remedy text.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the `screenshot` answer line `{"width","height","png"}` and the MCP image block, with a shot refused `fonts-unavailable` when no sans-serif resolves. `screenshot` is one of the three verbs `COUNTED` already counts. It is the precedent for how a counted verb's answer reaches the agent, and the fake server's answer shape for it stands.

@history_item n=5 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time, while `click`, `type` and `press` leave the clock where they found it. `advance` is a `COUNTED` verb. This is the first form of contract/driver-verbs-and-virtual-clock/click, later superseded by the one-virtual-clock entry (n=6).

@history_item n=6 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock`. `advance` moves it by exactly its ms together with the app's own time, and `click`, `type` and `press` leave it as it was. This is the source of decision/driver-advance-moves-one-clock and contract/driver-verbs-and-virtual-clock/click (ratified by n=7). A cold agent on Timer or Cells has to use `advance`, and the advance count reads it.

@history_item n=7 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — the founder ratified the one-virtual-clock clause, among others, so decision/driver-advance-moves-one-clock stands as the founder's word, not as provisional.

@history_item n=8 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `record-dir-not-private` and `recording-failed` are named inside exit class 3 with no new exit. This is the wording of contract/escher-session-exit-status/3. The verdict's error counts must fold these into session errors, and the chunk's boundary adds no session error.

@history_item n=9 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the paged `snapshot` answer `{"text":…,"part":n,"parts":m}`, with a paged MCP result being that JSON line. This is the source of contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result. A cold agent reading Cells meets the paged form.

@history_item n=10 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait moves no clock: an acting step waits at most `LOAD_WAIT` (5 s wall time) with the `VirtualClock` standing still. On the served `load` task a `click` on `load-fetch` waits the stylesheet out. This bears on what the widened task can ask of the load app without a wrong call.

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the 0.2.0 cold-agent chunk, this entry's direct precedent. The flight booker's mode became one `select#flight-type`. `escher-session` serves six tasks (counter · flight-booker · timer · crud · cells · load), listed in the usage line's `apps:` and the MCP `start` enum, and an unknown task is a usage ending (the source of contract/escher-session-tasks/*). In the scrollbar band, a click on a Cells cell needs an `advance` past the fade first. Its run is 0.2.0's baseline (six apps, 66 calls, 0 wrong), and its traps (scrollbar band, load wait) still shape the wrong-call count.

@history_item n=12 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flight-type select overrides and the load page's group leave design-system) — the stand's select and load-page values moved to layout-templates. Kept, on the founder's word «Оставить как есть», is surface's cli item naming the six served tasks, along with the driver's load-wait items. The task rows this chunk's prompt targets stay design-system's, and the stand's visual values are not this master's.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (no select popup is painted, and who owns it) — Enter and Space opened nothing on a single select, and the working route's Select popup entry owned it. Superseded by n=15. It is listed so the plan reads the opening gestures from the select-popup entries and not from this older state.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk) — the open list answers first in `hit_with_scrollbar`, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and a button under the list is refused `covered`. This is the source of decision/open-select-list-hit-first and the CARRY that `covered`'s added clause and the select list can move `wrong_calls`. The chunk's by-cause refusal counts read exactly this `covered`.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay; how it opens, moves, chooses and closes) — a click, Enter or a Space with no typed text standing opens a single select's list. It opens on its chosen option with no input/change, and Escape, a second click, a click outside, a wheel elsewhere or a page scroll closes it. This is the source of decision/single-select-open-gestures and contract/select-list-events/open. The widened task's flight-booker step (a click on `select#flight-type`) rests on these rules. The windowed list stays out of scope.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's wheel and key, and no motion) — the select list has no motion and needs no `advance`, so a screen read after a step reads the new state. This is the source of decision/select-list-opens-without-motion. A cold agent that advances before reading the open list is doing an unneeded call, not a required one.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's row cap, placement and row height) — `place` keeps the list inside the viewport, at least its select's width and at most the viewport's, with every edge finite, and at most `MAX_ROWS` = 20 rows. This is the source of decision/select-list-placed-in-viewport. It bears on what `layout`/`snapshot` report for the open list in the agent's run.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — the highlighted row is in no snapshot or tree. The tree reads the open select ComboBox and expanded with its chosen option selected, an author's `open` steers nothing, and a row carries its own `bounds` in the snapshot (a shot of a row crops to it). This is the source of contract/select-accessibility/* and contract/snapshot/select-list-row-bounds, which together give what a cold agent can observe of the open list.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — `graph` takes no argument (an argument is refused `malformed`) and answers one uncoloured JSON line `{"nodes":[…],"states":[…]}`, the same over socket, CLI, MCP and a kept check. It carries names, closed flag words and booleans only. This is the source of contract/escher-session-commands/graph-* and contract/ui-graph-line/content. `graph` is one of the reads the widened task asks for, and its content set is the crossing the live run would carry to the model provider (F3).

@history_item n=20 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the cli surface's graph answer line; ratified — the coverage role) — the founder's own word replaced the overseer's delegate confirm on the graph line and the coverage role. The graph rows this chunk counts are settled, not provisional.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unpaged graph, a pending limit) — `graph` is unpaged, and Cells at boot answers 293,869 bytes (residual `res-7cc5bi4k`, the stated limit «An unpaged graph»). A cold agent asking `graph` on Cells receives that whole line, which affects the run's cost and context, and the task should weigh which app it asks `graph` on.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line on the cli surface) — `actions` takes no argument and answers `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}` on every carrier. It lists actionable nodes in snapshot pre-order, with causes decided by the same `validate` and `check_target` the verbs meet, and a node that resolves to nothing reads `stale`. This is the source of decision/actions-share-the-verbs-checks and contract/actions-line/* and contract/escher-session-commands/actions-*. Because a listed cause equals the call's refusal cause, the cause words the verdict counts (e.g. `covered`) share one closed vocabulary with `actions`.

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged `actions` answer, a stated limit) — `actions` is unpaged, and Cells at boot answers 106,088 bytes for 678 nodes (owner `res-7cc5bi4k`). Like the graph's, this size bears on what the widened task asks a cold agent to read.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's answer line, endings and refusals; its three stated limits) — `coverage <app> <check>...` is a command-line role with no MCP tool, adds no exit status, and has its own stated limits. This is why the scope names handler coverage outside a cold agent's reach, with its headless witness standing.

@history_item n=25 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added contract/escher-session-commands/layout-answer (one line `{id, bounds, laid_out, placed, shifts, width, height}` on every carrier) and layout-arguments (one required `--id`; a missing id, an id past the bound or a second argument is refused `malformed`, status 1). `layout` is the fifteenth verb the pipe lists. The widened task asks for it, and a malformed `layout` call is a refusal the wrong-call count would read.

@history_item n=26 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (137 rows, each verified against the code), the form the picked rows take. Later amendments name what they change by row handle, so this chunk's expected amendments, if any, land as `+`/`~` rows in the surface/surface-cli areas, not as prose.
