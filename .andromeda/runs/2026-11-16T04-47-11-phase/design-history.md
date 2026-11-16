@history_item n=1 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — the settle rule's one answer for the keeps-animating set: settle returns `Settled` with `animating` read off the document's flag, waiting on and advancing nothing, and reading animation time only through the harness clock. This is the origin of `decision/settle-does-not-wait-on-animation` and `decision/animation-driven-by-virtual-clock`. Capturing a step's errors must not change when a step settles, and it must not hold settle open or move the clock.

@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves the app's own time, and `click`, `type` and `press` leave the clock where it was; a click beside a running animation returns settled. These are the acting verbs whose answers this chunk extends. The clock contracts (`contract/driver-verbs-and-virtual-clock/*`) must hold unchanged when the answer gains an error field.

@history_item n=3 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `scroll` is instant in every box and in the viewport, inside one settled step (`decision/driver-scroll-instant`), on the engine widening the founder ratified (2026-10-07). `scroll` is one of the acting verbs this chunk's capture covers, so a handler error caused by a scroll is gathered inside that one settled step.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the escher-session output form: one JSON line per command on stdout (the answer, the refusal or a session error's object, keyed by the schema's words), fixed lines on stderr, no colour, no ESC byte, no terminal test, and exits 0, 1, 2 and 3. This is the source of `decision/escher-session-agent-read-surface`, `decision/escher-session-no-colour`, `decision/driver-output-for-agents` and the exit-code rows, all on the founder's answers of 2026-10-10. A captured panic payload or log message on the answer line must stay inside that one uncoloured line; authored text could carry an ESC byte, which the shared Speaker asserts against.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: newline-delimited JSON-RPC 2.0 lines only, each reply one uncoloured line with no ESC byte, and an `Err` ending gives status 3. This is the source of `contract/escher-session-mcp/stdout` and `contract/escher-session-stdout/mcp-line`. The new answer field reaches MCP tool results, which must keep this form.

@history_item n=6 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` serves the self-description, and the exit statuses are read from the schema's `EXIT_STATUSES` table. A session error's object carries its fixed `remedy`, and over mcp `help`'s tool result is the self-description line. This is the source of `contract/escher-session-commands/help`, `decision/exit-statuses-from-schema-table`, `contract/escher-session-session-error/remedy` and `contract/escher-session-mcp/help-result`. The scope says `help` and the MCP tool listing describe the new field, and that no exit status or session error is added, so the table stays as it is.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — precedent: a new answer content (a PNG in band, two MCP content blocks) crossed onto the cli and mcp carriers only on the founder's fork 1 and his own word. This is the pattern for this chunk's boundary fork on what of a log may enter an answer. Every reply still stays one line with no ESC byte.

@history_item n=8 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the default user-agent sheet now reaches every document once, through `DocumentConfig::default()` (`decision/default-ua-sheet-via-default-config`). A fixture app beside the stand, for the panic and log proof, takes the sheet this way and needs no call of its own.

@history_item n=9 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock` its documents read. The driver's `advance` moves that clock by exactly its `ms` and moves the app's own time with it, while `click`, `type` and `press` leave it where it was. A scroll's `animating` reads true until 700 ms of that clock pass. This is the source of `decision/driver-advance-moves-one-clock` and `contract/driver-verbs-and-virtual-clock/advance`. A handler that panics or logs during `advance` runs inside this one clock step, and catching it must not change how far the clock moves.

@history_item n=10 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — the one-virtual-clock Motion clause was ratified by the founder (2026-10-18). The Motion rows this chunk rests on are therefore the founder's own and not delegate rulings, and changing them would need his word.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `record-dir-not-private` and `recording-failed` joined the exit-3 class with no new exit, as fixed text inside a ratified carrier. This is the source of `contract/escher-session-exit-status/3`. It is the precedent for keeping a caught handler panic out of the exit and session-error set (the scope allows no new exit status or session error). It also bears on how the new field enters a recording and its replay.

@history_item n=12 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — while an acting step waits on a load in flight, for at most `LOAD_WAIT`, the `VirtualClock` stands still (`decision/load-wait-moves-no-clock`). Records captured during a step include any emitted during that wait. The capture must not tick the clock or extend the wait.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — precedent for changing an answer line's shape on every carrier: `snapshot` gained `part` and `parts`, and its MCP result became the JSON line, on the founder's ratified fork. Adding the error field to the acting verbs' answers changes the line in the same way and takes the same per-carrier care.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — escher-session serves six stand tasks (counter, flight-booker, timer, crud, cells and load), and an unknown task is a usage ending. After a scroll, a click in the shown scrollbar's band takes an `advance` past the fade first, and the load page's delayed answer makes a click wait. These shape what the stand can prove and why the panic and log proof needs a fixture beside it, since no stand task panics or logs from a handler.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid's into-view move carries no animation) — the stand's arrow-key handler calls `scroll_to(Instant)` synchronously, because the async form, a task spawned from a key handler, panicked with `RefCell already borrowed` and was not shipped. This is a recorded handler-panic trap on the stand, and the kind of panic this chunk now catches on the run path and returns in the answer.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — the answer card read a row's `bounds` and a select's `expanded` as outside the boundary widening, because each is a closed set inside the ratified crossing. This is the same line the scope draws: a closed value set is no widening, while authored text such as a panic payload or a log message is one.

@history_item n=17 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the buffered render removes the `RefCell already borrowed` panic of a `MountedData` call made during render, on every Dioxus document. A call in a task polled during render acts on the document as the previous render left it. This changes which handler panics can still happen during a step, and it is the run path that the capture and the catch wrap.

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — `graph` answers the whole UI graph as one uncoloured JSON line on the socket, the command line, MCP and in a kept check. The line carries ids, owner ids, component names, state names, handler event names, closed flag words and booleans only, never a value or typed text (`contract/escher-session-commands/graph-answer`, `contract/ui-graph-line/content`). It was a boundary widening the founder ratified. The new field ties each error to its handler through this graph's identity (owner id and event name) and must keep its no-value content rule for everything except what the founder's fork admits.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unpaged graph, a pending limit) — the graph answer is unpaged: Cells at boot answers 293,869 bytes, and residual `res-7cc5bi4k` owns it. Answer size is a standing concern, so the per-call bound on captured records has to keep the acting answer small as well.

@history_item n=20 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (citation ranges re-read after the graph chunk's code) — the `escher-session` item's citations were re-pointed to `EXIT_STATUSES` at `packages/escher-driver/src/schema.rs:557-595` and the written JSON at `packages/escher-driver/src/json.rs:400-462`. This chunk edits `schema.rs` and `json.rs`, so these citations will move again and must be re-pointed at wrap.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line) — `actions` lists actionable nodes, each with its verbs and refusal causes, on every carrier. The line carries ids, owner ids, verb words, closed cause words and booleans only, never a value, typed text or `NodeId` (`contract/actions-line/*`, `contract/escher-session-commands/actions-answer`). Its crossing was a boundary widening on pattern `rul-izni57qv`, ratified by the founder. This is the nearest precedent for a new driver answer field and its ruling path, and for the scope's rule that the answer never carries a `NodeId`.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged `actions` answer, a stated limit) — Cells answers 106,088 bytes within the 1,048,576-byte answer bound, and `res-7cc5bi4k` owns it. The captured errors and logs must fit inside that same answer bound, which is why the scope bounds the capture per call.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's answer line, endings and refusals) — `coverage` replays kept checks and names unexercised handlers by `id`, `owner` and `event`, with ids, event names and counts only and no exit status added. It shows that handler identity through the UI graph is already a carrier word set. It also bears on how a kept check holding the new answer field replays.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage answer's three stated limits) — one stated limit is that a bubbling dispatch counts every ancestor listener it passed, with `stopPropagation` unread. Tying an error to "the handler that ran" meets the same bubbling question: one step can run several ancestor handlers.

@history_item n=25 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (decision, contract, absence, overview). Every picked row this chunk is bound by, the Motion clock rows and the Surface cli rows, comes from this conversion and was verified against the code. An amendment from this chunk names its change by row handle (`+`, `~`, `−`).

@history_item n=26 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the cli surface's graph answer line) — the graph answer line, PROVISIONAL until then, is now the founder's own word. The UI graph identity that this chunk's errors are tied to is settled ground.

@history_item n=27 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the escher-session coverage role's answer line, endings and refusals) — the coverage role's handler identity (`id`, `owner`, `event`) and its kept-check replay are ratified. These are the handler words and replay path this chunk's answer field reuses.
