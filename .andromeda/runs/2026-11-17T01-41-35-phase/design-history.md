@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the driver command line's output form) set the cli surface's form every verb keeps: one uncoloured JSON line on stdout per command (answer, refusal or session error's object, keyed by the schema's words), no ESC byte, no terminal test, usage error one fixed stderr line, exit 0 · 1 · 2 · 3; the origin of contract/escher-session-stdout/command-line-line, escher-session-exit-codes/* and escher-session-exit-status/*. `layout --id {id} --session {dir}` must answer in exactly this form and add no exit status.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the `mcp` role's stdout) newline-delimited JSON-RPC 2.0 lines only, a tool result's text the JSON line, no ESC byte, an Err ending's fixed message on stderr with status 3; origin of contract/escher-session-mcp/stdout, err-ending and other-results. The new MCP tool `layout` falls under other-results: its text is the command's JSON line.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — (`help` and the exit-status table) exit classes read from the schema's `EXIT_STATUSES` table, `help` answers the self-description as one uncoloured line reaching no session, a session error carries its fixed `remedy`, the MCP `help` result is that line; origin of contract/escher-session-commands/help, escher-session-mcp/help-result and escher-session-session-error/remedy. The scope has `help` describe `layout` from the tables via `describe()`, with its result fields, and the MCP listing place it before `help`.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the shot's answer line, no font refused) a `screenshot` is refused `fonts-unavailable` when no `sans-serif` family resolves, and the cli/MCP shot answer form was set (MCP image block, then text block). This is the reasoning behind decision/no-generic-family-lays-out-sans. The layout answer's numbers depend on the face the text measures in, and `layout` is a read that adds no new cause, so it must not take on the screenshot's font refusal.

@history_item n=5 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the recording session errors in the exit-3 class) `record-dir-not-private` and `recording-failed` were named as session errors at status 3 with no new exit; source of contract/escher-session-exit-status/3. Precedent for how a carrier-level addition sits inside the four statuses. `layout`'s recorded step in a kept check rides the same recording and adds no session error.

@history_item n=6 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — (the paged `snapshot` answer line) `{"text":…}` / `{"text":…,"part":n,"parts":m}`, with the MCP paged result being the JSON line; source of contract/escher-session-commands/snapshot. The cross-OS snapshot digests the scope re-reads are this answer, and per the scope they should not move with the fifteenth verb.

@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the cli surface's graph answer line) `graph` crossed every ratified carrier (socket `call`, command line, MCP text block, kept check) as one uncoloured JSON line carrying ids, names, closed words and booleans only, and added `cli-graph` to the Self-Validation subjects list; source of contract/escher-session-commands/graph-answer. This is the founder-ratified form `layout` is ruled to follow, the closest precedent for a new answer row and subject (e.g. `cli-layout`).

@history_item n=8 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the unpaged graph, a pending limit) an unpaged answer was recorded as a stated limit in docs/capabilities.md, owned by residual `res-7cc5bi4k`, under a pending subject `cli-unpaged-graph`. The `layout` answer is one element's, bounded, so it should need no such limit. The chunk edits docs/capabilities.md, which these limits live in.

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (citation ranges re-read) the `escher-session` item's citations into the driver's `schema.rs` (`EXIT_STATUSES`) and `json.rs` (the written JSON) were re-pointed when the chunk's code moved them. This chunk adds a schema row and a JSON written form to those same files, so the same re-point is owed at wrap.

@history_item n=10 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the `actions` answer line) `actions` crossed every carrier as `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, listing each actionable node with the verb table's id-taking instance verbs and the cause each would be refused with, decided by the same `validate` + `check_target`. It also added `cli-actions` and recorded the boundary widening on pattern `rul-izni57qv`. Source of decision/actions-share-the-verbs-checks and contract/actions-line/*. The scope's price: `layout` joins every listed node's verbs with no cause, so its target check must match `Session::layout`'s own refusals (`stale` / `not-found`). Otherwise a listed cause would differ from what the call returns.

@history_item n=11 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the unpaged `actions` answer, a stated limit) Cells at boot answered 106,088 bytes for 678 nodes, within the 1,048,576-byte bound, a stated limit owned by `res-7cc5bi4k`. A fifteenth verb word on every node makes that answer larger, and that measurement and the stated limit's figures in docs/capabilities.md move with it. This is the same move that shifts the cross-OS actions digests.

@history_item n=12 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage role's answer line, endings and refusals) `coverage <app> <check>...` replays kept checks, each check path read bounded at 1,048,576 + 1 bytes and parsed strictly, adding no exit status; source of contract/coverage-check-files/read-bound. A kept check holding a `layout` step must parse under `Check::from_bytes` and replay under this reader. The step must not change the check-file bound or the role's endings.

@history_item n=13 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — (ratified — the graph answer line; ratified — the coverage role's answer line) the founder's own word turned the PROVISIONAL graph and coverage records into ratified ones. The carrier form `layout` copies (graph) and the kept-check reader it rides (coverage) now stand ratified, with no provisional caveat to carry.

@history_item n=14 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — (the driver's `advance` and the animation clock) first fixed that `click`, `type` and `press` leave the harness clock where it was. This is the lineage of contract/driver-verbs-and-virtual-clock/*. `layout` runs through `Session::run` with no step, pump, tick or settle, so it must leave the VirtualClock as those verbs do.

@history_item n=15 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — (settle's answer for the keeps-animating set) settle waits on and advances no animating member and reads animation time only through the controlled clock; origin of decision/settle-does-not-wait-on-animation. `layout` takes no settle at all, so the row must hold without any settle from it.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (the harness drives animation from the documents' clock) the harness drives animation from the `VirtualClock` the documents read. `advance` moves it by exactly its `ms` and the app's time together, while `click`, `type` and `press` leave it. Source of decision/animation-driven-by-virtual-clock, decision/driver-advance-moves-one-clock and contract/driver-verbs-and-virtual-clock/advance. The new read verb must not tick that clock.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder, Epoch 2 close) ratified the one-virtual-clock clause (`advance` moving the clock by exactly its `ms` and the app's time together). The Motion rows the chunk must respect are ratified, not provisional.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — (the load wait moves no clock) an acting step waits on a load in flight for at most `LOAD_WAIT` with the VirtualClock still; source of decision/load-wait-moves-no-clock. The scope has `layout` take no load wait, so `layout` is not an acting step.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (Cli: `escher-session` serves six tasks) the served tasks became `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, listed in the usage line and the MCP `start` enum. These six are the tasks `stand_cross_os_readings` digests at boot and after one step. They are the scope of the cross-OS re-read this chunk owes.

@history_item n=20 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — (the headless stand registers the bundled DejaVu Sans on native) pinned text measurement to one bundled face so layout matches on the dev host and the CI runner. The layout answer's numbers, and the cross-OS digests re-read on linux, windows and macOS, rest on this single face.

@history_item n=21 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's font registration leaves for architecture's Headless stand) the stand's bundled-font registration moved to architecture §Standard Contracts → Headless stand, beside `font_ctx()`, and is no longer a design-system value. That move was ratified by 2026-10-27-upstream-sync-engine-debts. Any font question behind the layout numbers is architecture's to answer, not a typography row's.

@history_item n=22 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Typography: 2 rows) added + decision/unavailable-families-use-default-font: text whose families are all unavailable lays out in the engine's default font, and font-relative units measure against it. The rule was ratified as upstream's, guard `font_family_fallback`. It governs which face the sizes in a layout answer measure in.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the tree, the snapshot and a shot read of an open list) a row of an open select list carries its own `bounds` in the snapshot, read from the list's bounds before its client rect. `Session::layout`'s unexplained mapping answers with the snapshot's bounds. The new closed word for an engine-unexplained element must keep those snapshot bounds and not invent a reading.

@history_item n=24 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — (the masters as rows) the master's text became the rows this chunk is bound by: 364 records became 137 rows, each verified against the code. Every picked handle (the surface/cli contracts, the motion decisions, the typography decisions) dates from this conversion. A change to a carrier contract (e.g. the actions line's verbs, the tool listing) is now a `~` row amendment by handle.
