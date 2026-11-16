@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the driver command line's output form) — set §Surface: cli's `escher-session` form: one line of JSON on stdout per command (answer, refusal or session error's object keyed by the driver schema's own words), nothing else there; no colour, no ESC byte, no terminal test; exit status carries the class 0 · 1 · 2 · 3. The origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and contract/escher-session-stdout/command-line-line: the error text now riding `caused` must stay inside that one uncoloured line, so a panic payload, error message or log record carrying an ESC byte or a newline must be escaped by the JSON writer, never written raw.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the `mcp` role's stdout) — added the `mcp` role to the cli surface: newline-delimited JSON-RPC 2.0 lines only, a reply per request, a tool result's text the JSON line (screen text for `snapshot`), no ESC byte in any reply (asserted on every reply by the shared `Speaker`), an `Err` ending's fixed message on stderr with status 3. Origin of contract/escher-session-mcp/stdout, /other-results, /err-ending and contract/escher-session-stdout/mcp-line: the acting verbs' MCP tool results carry `caused` with its new text as that JSON line, and the `Speaker` guard will read the app- and engine-authored text for ESC bytes.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** (`escher-session help` and the exit-status table) — exit statuses read from the schema's `EXIT_STATUSES` table whose meanings `help` serves; `help` answers the self-description as one uncoloured JSON line, status 0, reaching no session; a session error's object carries its fixed `remedy`; over MCP a `help` result is that line. Origin of decision/exit-statuses-from-schema-table, contract/escher-session-commands/help, contract/escher-session-mcp/help-result and contract/escher-session-session-error/remedy: this chunk's `describe()` must name `caused`'s new fields and the cut's and count's meanings (and an unread boundary) through the same self-description, keeping it one line, and adds no exit or session error to the table.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the shot's answer line, the MCP image block) — precedent for a large payload crossing the cli answer line and the MCP role: base64 image text inside the one uncoloured line, an MCP shot answered as an image block then the text block, every reply still one line with no ESC byte. Shows how contract/escher-session-mcp/other-results was carved for a non-default result; the error text takes the default (the JSON line) and must hold the same one-line, no-ESC form under the MCP result bound.

@history_item n=5 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the two recording session errors in the exit-3 class) — the exit-3 class line names `record-dir-not-private` and `recording-failed` among session errors with no new exit; fixed text inside a ratified carrier. Origin of contract/escher-session-exit-status/3 as it reads; this chunk adds no exit, refusal cause or session error, so these lines must read unchanged after it. Also the recording carrier: a kept check's answer is kept whole, so `caused`'s text is recorded and replayed.

@history_item n=6 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (the paged `snapshot` answer line) — precedent for bounding a large answer on the cli surface: `{"text":…,"part":n,"parts":m}`, one uncoloured line, and over MCP a paged `snapshot` result became the JSON line. The precedent for a bounded answer saying it is cut and how much remains, which this chunk's message cut and record count follow in their own form.

@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line; the unpaged graph, a pending limit; citation ranges re-read) — `graph` answers one uncoloured line on stdout, the same answer on the socket's `call`, over MCP and in a kept check, carrying ids, names, closed flag words and booleans only, never a value or typed text; its crossing a founder-ratified boundary widening (PROVISIONAL, then ratified by 2026-11-15T05-21-42-wrap). The unpaged graph (293,869 bytes on Cells) was recorded as a stated limit with a residual owner; the `escher-session` item's citations (`schema.rs` `EXIT_STATUSES`, `json.rs` written JSON) re-pointed. Precedent for this chunk: text crossing every carrier is a boundary widening owing its amendment at the wrap, and a size gap is stated as a limit with an owner, never silent; the cited `schema.rs`/`json.rs` ranges will move again with this chunk's edits.

@history_item n=8 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line; the unpaged `actions` answer, a stated limit) — `actions` answers one uncoloured JSON line on the socket, command line, MCP and kept check, ids, verb words, closed cause words and booleans only, never a value or typed text; a boundary widening ratified by the founder; its unpaged size measured (106,088 bytes on Cells) within the 1,048,576-byte answer bound and stated as a limit. Precedent for this chunk's bounds: measure the answer against the MCP and socket answer bound and state the limit; and the contrast — earlier answers carried closed words only, this chunk is the first to carry app- and engine-authored text, a widening owed its own amendment.

@history_item n=9 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's answer line, endings and refusals) — `escher-session coverage <app> <check>...` replays kept checks; each check path read bounded at 1,048,576 + 1 bytes and parsed strictly; a file that cannot be read, past the bound, no check or another app's check is refused `malformed`, status 1, the refusal holding nothing of the path or the file; no exit added. Origin of contract/coverage-check-files/read-bound and /refusal: kept checks now hold `caused` with its text, so a recorded check grows and must still parse strictly under that bound, and a refusal still says nothing of the file's content.

@history_item n=10 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer line; ratified — the coverage role's answer line, endings and refusals) — both PROVISIONAL entries above now ratified by the founder's own word, text unchanged. The coverage-check-file rows and the graph crossing this chunk rests on are settled, not provisional.

@history_item n=11 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (`advance` moves app time and leaves the animation clock alone) — first wiring of the acting verbs: `advance` moves the app's own time through the step a session's caller hands it, `click`, `type` and `press` leave the harness clock. The acting verbs whose answers carry `caused`; this chunk adds text only and must not move any verb's clock behaviour (contract/driver-verbs-and-virtual-clock/*).

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the harness drives animation from the documents' clock, PROVISIONAL) — the harness drives animation from the `VirtualClock`; the driver's `advance` moves it by exactly its `ms` and the app's own time together; `click`, `type`, `press` leave it. Origin of decision/driver-advance-moves-one-clock and contract/driver-verbs-and-virtual-clock/advance·click·press·type; ratified by 2026-10-18-engine-debts below. Collecting a step's records into `caused` must not tick or advance that clock.

@history_item n=13 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — ratifies one-virtual-clock's `advance` moving the clock by exactly its `ms` and the app's own time together. The motion rows this chunk's acting answers sit under are ratified, not provisional.

@history_item n=14 marker=2026-10-20-load-wait
**2026-10-20-load-wait** (the load wait moves no clock) — while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s wall), the `VirtualClock` stands still. Origin of decision/load-wait-moves-no-clock: records an acting step causes during a load wait (the `load` task's stylesheet fetch) land in `caused` without any clock moving to collect them.

@history_item n=15 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (an into-view scroll never animates a nested box) — the driver's `scroll` is instant in every box and the viewport inside one settled step, founder-ratified. `scroll` is one of the acting verbs whose `caused` gains text; its step boundary (one settled step) is the window the step's records are collected in.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (Cli item) — `escher-session` serves six tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), listed in the usage line's `apps:` and the MCP `start` tool's enum. Origin of contract/escher-session-tasks/*: the stand's tasks are where research measures real message lengths; no task is added or dropped. Its Motion item (the load page's delayed stylesheet, a click waiting it out) is a source of engine records on the `load` task.

@history_item n=17 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flight-type select overrides and the load page's group leave design-system — its Kept clause) — kept, on the founder's own word, surface's cli item naming the six served tasks and the driver's load-wait items in motion and the body. Why those rows still stand in design-system while the stand's other facts moved to layout-templates; the six-task rows and the load-wait row this chunk is bound by are settled.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** (the masters as rows) — the master became rows (137), each verified against the code; a master holds what a planner must respect and the code holds the rest. Every picked row's handle (the motion, surface and surface-cli contracts and decisions) was minted here, and the cli surface's NOT YET MEASURED navigation and platform notes stay absences.
