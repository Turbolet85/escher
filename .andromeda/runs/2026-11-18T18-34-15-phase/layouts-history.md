@history_item n=1 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added `scripts/cold-agent.sh` (`<verb> [task]`, verbs run · status · cleanup · logs; usage error to stderr, empty stdout, exit 2 before any precondition; stdout JSON lines only; transcript kept in `target/cold-agent/transcript.jsonl`). The origin of contract/cold-agent-cli/usage, exit-2 and stdout, which bind the one live run this chunk fires and the evidence it must never print.

@history_item n=2 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the pipe's one task became `stand` under `STAND_PROMPT`; `run` needs `claude`, `python3` and a built `target/debug/escher-session` (exit 3, the pipe builds nothing) and gives the client the driver's own MCP server (was a stub); the agent's check stays in `target/cold-agent/check.json`, never printed. Grounds decision/cold-agent-pipe-builds-nothing, decision/cold-agent-uses-driver-mcp and contract/cold-agent-cli/exit-3: the readiness checks must build the binary before the firing, and check.json stays uncommitted.

@history_item n=3 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the stand's six served tasks (counter · flight-booker · timer · crud · cells · load) on the host and `mcp` roles and the `start` enum; the flight booker's mode became a select (`flight-type`, `flight-type-one-way`, `flight-type-return`); the pipe's `stand` named six apps and its live server runs `mcp --record` into the owner-only `target/cold-agent/record`, removed by cleanup, checks never printed, replays without recording. Precedent for the last cold-agent run (0.2.0's baseline: six apps, 66 calls, 0 wrong) and the origin of contract/cold-agent-cli/record-dir and the served-tasks rows this run now reads at seven.

@history_item n=4 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: cli, 2 rows) — ~ contract/cold-agent-cli/usage six apps → seven; + contract/escher-session-served-tasks/order, listed after load in the usage line and the MCP `start` enum, on the founder's «В 0.3.0, до прогона» (F5). The immediate precursor: the live run must meet the seventh app, and v030-15's acceptance is to drop `caused` from the out-of-reach list.

@history_item n=5 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native, 8 rows) — ~ decision/stand-lints-clean-at-boot adds the order page; + decision/order-page-built-to-cause (Submit errors outside 10–20, a good Submit logs «order placed», Export panics) and + the six `order-*` element ids. The controls that make `caused_by_kind` read panic, error and log at least once each, which the verdict requires and the run must reach.

@history_item n=6 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — `escher-session mcp` added as an MCP server over stdio, local to the invoking user, no listener or auth surface, protocol lines only on stdout, echoing an admitted id; the `mcp` role a further consumer of the TaskShell mount; "no ending writes back caller input" scoped to the commands. Grounds decision/mcp-server-local-stdio, contract/escher-session-mcp/stdio and decision/session-roles-share-stand-boot: the carrier the cold agent drives, a crossing the founder ratified (2026-10-10).

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` became the driver's command line, `<verb> [arguments] --session <dir>` with four endings (0 accepted JSON line, 1 refused, 2 usage, 3 session error), the host role `serve`. Origin of contract/escher-session-command/form and the exit-code rows that the verdict's wrong-call and refusal counts read.

@history_item n=8 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` and the `mcp` role's tool listing, each tool description the verb's help plus its result fields. Origin of the tools/list shape (contract/escher-session-mcp/tools) a cold agent learns the fifteen tools from.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot [--id]` answering `{"width","height","png"}`, refused `fonts-unavailable` / `off-screen` (exit 1); over MCP an image block then a text block, a result with an image bounded at 1,048,576 bytes. Grounds contract/escher-session-screenshot/answer, the refusal rows and contract/escher-session-mcp/result-bound and screenshot-result; screenshot is a capability the verdict requires used, and its in-band image is the founder's ratified crossing (2026-10-12).

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` on `start`, the host and the `mcp` role: one owner-only replayable check (`check-{n}.json` 0600 in a 0700 dir), session errors `record-dir-not-private` and `recording-failed` (exit 3; over MCP isError). Grounds decision/session-recording-replayable-check and the escher-session-record rows; the verdict fails `recording-not-kept` / `no-check` / `check-failed` on them, and the founder ratified the widening (2026-10-19).

@history_item n=11 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call its verb, arguments, acceptance, answer digest and the masked answer (a shot without its png), read back and replayed by `Check::replay`. The kept checks under `target/cold-agent/record/` therefore carry screen content: never printed, copied or committed by this chunk.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint (each lean task 0 findings at boot, `crud-person-14` `clipped` after twelve Creates); its gated part added the `lint` verb (one line `{"findings"}`, refused `fonts-unavailable`) on every carrier, kept under `--record`. Grounds contract/escher-session-lint/answer and decision/stand-lints-clean-at-boot; lint is a capability the verdict requires used.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` (1–1000, out of bound refused `malformed`), `{"text","part","parts"}` for a screen read in parts, an empty text past the last part. Grounds contract/escher-session-snapshot/answer and part; a part read is a capability the verdict requires used, and a bad `--part` counts as a refusal.

@history_item n=14 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — each acting tool's `settled` field states that a load in flight is waited for up to 5 s. Bears on the load page's 200 ms `LoadNet` answer the agent meets, and on the run's wall time against the 600 s client timeout.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the engine paints an open select list (opened by click, Enter or Space; Escape, a second click or a click outside closes it); while open each row reads its own bounds in the snapshot and the diff, after a close the rows read back at the select's client rect; the exceptions to the into-view scroll and visible-region rules for an open list's rows; the lint skips `Entry.in_select_box`. Grounds decision/open-list-rows-own-bounds and the excepts of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes; a list opening is a capability the verdict requires, and `covered`'s added clause under an open list (CARRY cry-32fid2ry) can move `wrong_calls`.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — the flight booker's boot lint read 0 (a closed select's options skipped by `overlaps`), the boot `at_boot` `[0, 0, 0, 0]`; `load-fetch` takes Enter and Space; `cells-grid` one Tab stop and keyboard region named "Cells". Grounds decision/stand-lints-clean-at-boot, decision/flight-type-select-boot-reading and contract/stand-element-ids/load-fetch and cells-grid, the screens the agent drives.

@history_item n=17 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — an into-view scroll moves every holding scroller then the viewport; on the stand the CRUD list clips its rows, the last row after twelve Creates refused `off-screen` until a driver `scroll`. The founder-ratified rule under decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes, and a refusal cause the verdict's `refused_by_cause` reads.

@history_item n=18 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a scrolled `crud-list` reads its own four figures and a click naming it lands on the row at its centre; Home's scrolling box hides the Cells card until scrolled. Readings a cold agent's CRUD and scroll calls rest on.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the `graph` verb (ninth instance verb after `lint`), one line `{"nodes","states"}` of ids, component and handler event names, state names, closed flags and booleans, never a value; thirteen MCP tools. Origin of contract/escher-session-graph/answer; `graph` is a capability the verdict requires used, and its answer to the model provider is ratified by F4.

@history_item n=20 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph verb; the coverage role, two entries) — the PROVISIONAL delegate confirms on `graph` and on the `coverage` role replaced by the founder's own word, text unchanged. The graph row and the session-roles row this run rests on stand ratified.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` verb (tenth instance verb after `graph`), one unpaged line `{"nodes"}` of id, owner, keyed and verbs with cause, never a value, typed text or NodeId; fourteen MCP tools; the founder's «Новый глагол actions» ratifies its crossing on every carrier. Origin of contract/escher-session-actions/answer; `actions` is a capability the verdict requires used.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the `coverage` role (`coverage <app> <check>...`), replaying kept checks on a fresh boot through the same boot, answering `{"unexercised","checks","held"}`; a further consumer of the TaskShell mount. Grounds decision/session-roles-share-stand-boot and is where the reach statement places what the run did not reach (handler coverage).

@history_item n=23 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — ~ contract/escher-session-command/form and contract/escher-session-mcp/tools fourteen → fifteen verbs, `layout` after `actions`; ~ decision/no-caller-input-echoed gains the layout id except; + contract/escher-session-layout/answer. `layout` is a capability the verdict requires used, and the fifteen tools are what the README states on a pass.

@history_item n=24 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the delegate confirm of the layout card ratified by the founder's own word («Подтвердить обе»), rows unchanged. The layout rows this run exercises stand on his word.

@history_item n=25 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand skips Home and mounts one task in TaskShell at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. Origin of decision/headless-stand-pinned-viewport, the screen the agent reads; the bundled fonts are named in the reach statement.

@history_item n=26 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host as a consumer of the TaskShell mount through the stand boot only, adding no markup; a held instance reads as a fresh boot, mount and viewport holding after commands. Origin of decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot.

@history_item n=27 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author ids (counter, flight, timer, crud), no class, style, wrapper or order added. Origin of the stand-element-ids rows the agent aims at; the flight mode ids were later replaced by `flight-type*` (2026-10-25-cold-agent-bar).

@history_item n=28 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows read `crud-person-{person.id}` (after 2026-10-06-id-persistence keyed them by person id), following their person under filter, Create and Delete; Home's task-card ids. Grounds contract/stand-element-ids/crud-person-id.

@history_item n=29 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — author ids on the Temperature Converter, Circle Drawer (dialog included) and Cells controls; `unkeyed_actionable()` reads 0 on all seven tasks. Origin of decision/every-stand-control-keyed and the temp, circle and cell id rows.

@history_item n=30 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs named by attributes alone (`for`, `aria-label` "Departure date" / "Return date"); with 2026-10-10-snapshot-states (aria-invalid, listbox/option, aria-selected) and 2026-10-14-stand-a11y-assertions (`crud-list` named "People"), the ground of decision/stand-a11y-by-attributes-alone and the names the agent reads in snapshots.

@history_item n=31 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (66 records → 121 rows), each verified against the code, fates by the founder's approval. Explains why the picked rows exist as rows and that their later changes are recorded as `~` / `+` row amendments.
