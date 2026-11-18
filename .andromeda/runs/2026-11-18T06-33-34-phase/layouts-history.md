@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand skips Home and mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with DejaVu Sans; this is the origin of decision/headless-stand-pinned-viewport, the mount every served task a cold agent boots goes through.

@history_item n=2 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added `scripts/cold-agent.sh` (`<verb> [task]`, verbs run · status · cleanup · logs, usage error on stderr with empty stdout and exit 2 before any precondition, stdout JSON lines only, transcript kept in `target/cold-agent/transcript.jsonl`, `cold-agent.ps1` a pass-through); the origin of contract/cold-agent-cli/usage, stdout, exit-2 and windows-pass-through that a widened prompt or a second task in `TASKS` must keep.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author ids (counter, flight, timer, crud), attributes only with no class, style, wrapper or order; the origin of the contract/stand-element-ids rows, which the fixed prompt must not name (decision/cold-agent-prompt-names-no-id).

@history_item n=4 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows keyed by `{person.id}`, not index, so an id an agent holds follows its person under filter, Create and Delete; behind contract/stand-element-ids/crud-person-id that a cold agent's CRUD steps lean on.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the six stand inputs named by attributes alone (`for`, `aria-label` "Departure date" / "Return date"), no element or order changed; origin of decision/stand-a11y-by-attributes-alone and the named-input id rows the agent reads by name.

@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows read the author id `crud-person-{person.id}` beside the key, and Home's task cards `task-card-{slug}`; the founder's rule that every actionable element reads an author key, behind decision/every-stand-control-keyed.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the `escher-session` host as a third consumer of the TaskShell mount, adding no markup; a held instance reads as a fresh boot and the mount and viewport hold after commands; stderr lines carry `service.name=seven_guis` — origin of decision/held-instance-reads-as-fresh-boot and contract/escher-session-host/log-service-name, the log the verdict would read for causes.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the CRUD list is the stand's nested scroller; after twelve Creates the last row is refused `off-screen` until a driver `scroll` brings it in; a refusal a cold agent can hit, counted as a wrong call, behind contract/escher-session-refusals/off-screen.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session <verb> [arguments] --session <dir>` with four endings (accepted exit 0, refused exit 1, usage exit 2, session error exit 3) and the host role `serve`; origin of contract/escher-session-exit-codes/0 and /1, and the endings the verdict's `refused`/`error` counts read.

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — what a scrolled `crud-list` reads and where a click naming it lands; Home's column scrolls with Cells' card below the fold; the stand readings a cold agent's CRUD steps meet.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the stand states roles and states by attributes alone (`aria-invalid`, `role="listbox"`/`option`, `aria-selected`, then `aria-pressed` on the flight mode buttons, later retired for the select); behind decision/stand-a11y-by-attributes-alone.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — `escher-session mcp`, an MCP server over stdio, local to the user, no listener or auth, protocol lines only on stdout, echoing an admitted id; the no-echo rule scoped to commands; origin of decision/mcp-server-local-stdio, contract/escher-session-mcp/stdio and the carrier decision/cold-agent-uses-driver-mcp drives; its crossing is the founder's ratified one (2026-10-10).

@history_item n=13 marker=2026-10-12-self-description
**2026-10-12-self-description** — `escher-session help`'s seven-key line reaching no session, the MCP tool descriptions as help plus result fields, `help` last; origin of contract/escher-session-help/answer and the shape of contract/escher-session-mcp/tools a cold agent reads.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the `screenshot` verb; refusals `fonts-unavailable` and `off-screen`; over MCP an image block then a text block, a refused shot one text block with `isError`; origin of contract/escher-session-mcp/screenshot-result and the two refusal rows; `screenshot` is one of the `COUNTED` verbs the chunk widens beside.

@history_item n=15 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` one Tab stop with ArrowDown/ArrowUp/Home/End selecting rows and filling the name fields, focus on the list never a row, no `aria-activedescendant`; origin of decision/crud-list-keyboard-region, on the operator's F5.

@history_item n=16 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named "People" by `aria-label` alone; behind contract/stand-element-ids/crud-list.

@history_item n=17 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the non-lean tasks keyed: Cells `cell-{A..Z}{0..25}` and `cell-input` among them, attributes only, `unkeyed_actionable()` 0 on all seven tasks; origin of contract/stand-element-ids/cell-column-row and cell-input and of decision/every-stand-control-keyed.

@history_item n=18 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the pipe's one task became `stand` under `STAND_PROMPT` naming no element id or app slug; `run` needs `claude`, `python3` and a built `target/debug/escher-session` (exit 3, builds nothing); the client is given the driver's own MCP server; the check kept in `target/cold-agent/check.json`, never printed — origin of decision/cold-agent-pipe-builds-nothing, decision/cold-agent-prompt-names-no-id, decision/cold-agent-uses-driver-mcp and contract/cold-agent-cli/exit-3, on the founder's P4 answers; the constraints a widened prompt or a second task must keep.

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` on `start`, the host and `mcp` roles: owner-only `check-{n}.json` in a `0700` directory, session errors `record-dir-not-private` and `recording-failed`; the founder-ratified widening behind contract/cold-agent-cli/record-dir, which the live run's server uses.

@history_item n=20 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check holds per call its verb, arguments, acceptance, digest and masked answer, replayed by `Check::replay`; what the pipe's replays and the record directory hold, and why checks are never printed.

@history_item n=21 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, `escher-session lint`) — `lint` the eighth instance verb on every carrier, twelve verbs and twelve MCP tools, a `lint` step kept under `--record`; the twelve-verb baseline the 0.2.0 run used and `lint` one of the `COUNTED` verbs; the earlier in-process entry pins the stand's clean boot reading and `crud-person-14`'s `clipped` after twelve Creates.

@history_item n=22 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — each acting MCP tool's `settled` field states a load in flight is waited up to 5 s, fixed help text in a ratified carrier; bears on the load page a cold agent drives and on contract/escher-session-mcp/tools' descriptions.

@history_item n=23 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part`, paged answers and a `--part` out of bound refused `malformed`; a refusal a cold agent reading Cells can incur, read by the cause counts.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — 0.2.0's cold-agent bar: six served tasks (counter · flight-booker · timer · crud · cells · load) on the host and `mcp` roles and the `start` tool's enum; the flight mode became `select#flight-type` with `flight-type-one-way` · `flight-type-return`; the load page with `load-fetch` / `load-done` and `LoadNet` after 200 ms; the pipe's `stand` task names six apps and its live server runs `mcp --record` into `target/cold-agent/record` (owner-only, removed by `cleanup`, replays without `--record`) — the direct precedent and baseline this chunk widens; origin of the contract/escher-session-served-tasks rows, the flight-type id rows, decision/load-page-only-network and contract/cold-agent-cli/record-dir.

@history_item n=25 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines) — the load page's served bullet as measured at 2026-10-25-cold-agent-bar: `LoadNet` answers from a thread of its own, so a driver `click` on `load-fetch` waits it out; the select's 15px/400 styling and its options showing the chosen one alone; behind decision/load-page-only-network and decision/flight-type-select-boot-reading.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (load Tab sequence and keys; the Cells grid; the flight booker's boot lint) — `load-fetch` takes Enter and Space under `LOAD_TAB_ORDER`; `cells-grid` one Tab stop and keyboard region named "Cells", no role, arrows moving `current`, Enter opening the editor (founder's F13); `overlaps` skips a closed select's options so the flight booker reads 0 at boot (founder's F9) — origin of contract/stand-element-ids/load-fetch and cells-grid, and the lint reading a cold agent's `lint` calls meet.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — a click, Enter or Space opens `select#flight-type`'s engine-painted list; Escape, a second click, a click outside or scroll closes it; focus stays on the select; while open each row reads its own `bounds` in the snapshot and the diff, after a close back at the select's client rect (founder's F1–F4, F7) — origin of decision/open-list-rows-own-bounds; the v030-17 capability this chunk's prompt asks a cold agent to reach, and the `covered` clause whose wrong calls the run must now read by cause.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the scrolling-boxes exception; the lint's `Entry.in_select_box`) — an open list's row is seen through the list's scrollport and `scroll_into_view` on it moves no box; the lint skips `overlap` for an open list's rows (founder's F10) and the snapshot gives them their own `bounds` — readings a cold agent opening the list meets, carried unread to this chunk's run.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — `graph` the ninth instance verb after `lint`, thirteen verbs and MCP tools, answering `{"nodes","states"}` with ids, names and closed words only, never a value — origin of contract/escher-session-graph/answer; v030-05 the prompt asks for; its answers crossing to the model provider is the scope's F3 crossing question (ratified at 2026-11-15T05-21-42-wrap).

@history_item n=30 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — `actions` after `graph`, fourteen verbs and MCP tools, one unpaged `{"nodes"}` line of id, owner, keyed and verbs with verb and cause, never a value, typed text or `NodeId` — origin of contract/escher-session-actions/answer; v030-06 the prompt asks for, its answers part of the F3 crossing.

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, its consumer of the TaskShell mount) — `escher-session coverage <app> <check>...`, no `--session` or `--record`, each check bounded at 1,048,576 + 1 bytes and refused `malformed` otherwise, replayed on a fresh boot answering `{"unexercised","checks","held"}` — origin of the contract/escher-session-coverage rows; a role, no MCP tool, so the plan names handler coverage out of the cold agent's reach while the pipe's kept checks could feed it.

@history_item n=32 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (121; decision 21 · contract 99 · absence 1), each verified against the code; the handles this chunk's picked rows carry and any amendment it writes must name.

@history_item n=33 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph verb; the coverage role and its mount consumer) — the founder's own word replaces the overseer's delegate confirms of the `graph` verb and the `coverage` role; the rows contract/escher-session-graph/answer and the contract/escher-session-coverage rows stand ratified, not PROVISIONAL.

@history_item n=34 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ contract/escher-session-command/form` and `~ contract/escher-session-mcp/tools` to fifteen verbs, `layout` after `actions`; `~ decision/no-caller-input-echoed` gains the exception that the layout answer's id names the call's id; `+ contract/escher-session-layout/answer` — the fifteen-tool listing and the layout answer v030-11 the prompt asks for; its id echo and placing words crossing to the model provider fall under F3.

@history_item n=35 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirms behind 2026-11-17-layout-on-the-carriers's four cli rows, `layout` among the pipe's fifteen `VERBS` on his own answer; the rows contract/escher-session-mcp/tools, contract/escher-session-layout/answer and decision/no-caller-input-echoed stand ratified.
