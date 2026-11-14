@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand registered the bundled DejaVu Sans for every generic, system fonts off, through `HarnessOptions.font_ctx`; a fixture-state boot must reuse that one boot path so a state's first screen measures text as the hand-reached screen does.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every keeps-animating member the same way (none holds it open, none waited on or advanced) and reads animation time only through the harness's clock; behind `decision/settle-does-not-wait-on-animation` and `decision/animation-driven-by-virtual-clock`: a session started in a state must settle its first screen by the same rule, never advancing the clock.

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `advance` moves the app's own time through the step the session's caller hands it, `click`/`type`/`press` leave the clock alone; a state reached by hand through `advance` (Timer) versus an app-supplied initial state must read the same clock, the precedent for the driver-verbs contract rows.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — fixed the cli output form: one JSON line per command keyed by the schema's words, usage error a fixed stderr line with stdout empty, no colour/ESC/terminal test, exit classes 0·1·2·3 (founder, 2026-10-10); a `start --state` argument and its refusals must keep that form (an unknown state a refusal or usage ending within the four statuses).

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role's stdout rules (JSON-RPC lines only, no ESC, Err ending to stderr status 3); the MCP `start` tool gaining a state argument stays under them.

@history_item n=6 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from schema `EXIT_STATUSES`, `help` serves the self-description, a session error carries its `remedy`; a new `start` argument must appear in `describe()`/`help` and any new refusal reuse the table, adding no status.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a shot of an app resolving no `sans-serif` family is refused `fonts-unavailable`; the shot's cli line and MCP blocks; a state-started session's shot must reach the same font context as a boot.

@history_item n=8 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the default user-agent sheet arrives once through `DocumentConfig::default()` (`decision/default-ua-sheet-via-default-config`); a fixture-state document must take its sheet the same way, not a second config, or its first screen diverges from boot.

@history_item n=9 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the one `VirtualClock` its documents read; `advance` moves it by exactly its `ms` and app time together (`decision/driver-advance-moves-one-clock`, `contract/driver-verbs-and-virtual-clock/*`); a session starting in a state must start that clock where a boot does, or a state reached by `advance` reads a different clock than one started in it.

@history_item n=10 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — ratified one-virtual-clock's `advance` clause by the founder (2026-10-18); the clock rows this chunk rests on are settled, not provisional.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording's `record-dir-not-private` and `recording-failed` sit in exit class 3 with no new exit, fixed text inside a ratified carrier; precedent for a kept check (`--record`) of a state-started session: new fixed refusals fit the existing classes.

@history_item n=12 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait moves no clock (`decision/load-wait-moves-no-clock`, `LOAD_WAIT`); the `load` task's done state reached by hand waits on its stylesheet, so a start-in-state for it must land the same settled screen without moving the clock.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the paged `snapshot` line (`{"text","part","parts"}`) and its MCP result; the proof compares a state's first screen to the hand-reached one by snapshot text, whole or in parts (Cells).

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — `escher-session` serves six tasks (counter · flight-booker · timer · crud · cells · load), listed in the usage line's `apps:` and the MCP `start` tool's enum, an unknown task a usage ending (`contract/escher-session-tasks/*`); also the scrollbar-band trap (a Cells click after a scroll needs an `advance` past the fade) and the load page's `LOAD_ANSWER_DELAY`; a per-task state set extends this served-task register and its by-hand paths meet these traps.

@history_item n=15 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's boot font registration (`boot_cells`, `boot_load` among every boot) is held in architecture §Standard Contracts → Headless stand, not here; a new state boot's font contract is stated there.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flight-type select overrides and the load page's group leave design-system) — stand screen values moved to layout-templates; Kept, on the founder's word: surface's cli item naming the six served tasks and the load-wait items stay here; stand-only state facts belong in layout-templates, the served-task set here.

@history_item n=17 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — render-time `MountedData` calls replay against the document as the previous render left it, so `set_focus_to` returns `false` for a node created in that same render; a trap if a fixture state is app-supplied at first render (e.g. focus or scroll in the initial state).

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — `graph` takes no argument, refused `malformed` with nothing run, same answer on socket, cli, MCP and a kept check; the precedent for a validated argument on a ratified carrier, a boundary widening ratified by the founder's fork, and for kept-check parity of a state-started session.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (citation ranges re-read) — the `EXIT_STATUSES` table and written JSON citations (`schema.rs`, `json.rs`) moved with added lines; this chunk edits `schema.rs` (`START`, the verb table, `describe()`), so the surface citations into it move again.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` line (`contract/actions-line/*`, `decision/actions-share-the-verbs-checks`), a new verb onto every ratified carrier as a boundary widening on `rul-izni57qv`; the nearest precedent for widening `start`'s carriers, and a state's first screen must answer `actions` as the hand-reached one does.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — `coverage <app> <check>...` replays each kept check on a fresh session (`contract/escher-session-commands/coverage-answer`, `contract/coverage-check-files/*`), strict bounded check reading, refusals holding nothing of the path; a kept check started in a state must replay from that state in coverage's boot, or be refused, and its four endings stay unchanged.

@history_item n=22 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows verified against the code; the picked rows (surface, surface-cli, motion areas) are that form, and any change this chunk makes lands as `+`/`~` row handles.
