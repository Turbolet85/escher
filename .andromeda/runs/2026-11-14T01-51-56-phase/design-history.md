@history_item n=1 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — §Motion: the harness drives animation from the `VirtualClock` its documents read; the driver's `advance` moves it by exactly its `ms` and the app's own time together, `click`/`type`/`press` leave it (rows `decision/animation-driven-by-virtual-clock`, `decision/driver-advance-moves-one-clock`, `contract/driver-verbs-and-virtual-clock/advance`). The precedent for one run-wide determinism source held by the harness and threaded through the session's step — a seed beside `with_time` follows this shape and must not perturb the one clock.

@history_item n=2 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — ratified the one-virtual-clock clause (advance moves the clock by exactly its `ms` and app time together). The clock rows this chunk rests on are founder-ratified; a seed must leave them as they read.

@history_item n=3 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every member of the animating set the same way, none waited on or advanced, and reads animation time only through the harness clock (row `decision/settle-does-not-wait-on-animation`). Why: the scrollbar fade read the wall clock — a host-dependent state source; two seeded runs reading the same state at every step depend on settle reading no host time.

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `advance` moves app time through the step a session's caller hands it (superseded on the animation clock by one-virtual-clock). Precedent for how a run's controlled input reaches the app through the session's caller — the app-facing seed source sits beside it.

@history_item n=5 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait moves no clock: `VirtualClock` stands still for at most `LOAD_WAIT` (5 s wall time) (row `decision/load-wait-moves-no-clock`). A wall-time bound inside a step is a non-determinism trap the seeded-run proof must not read across.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` output form: one uncoloured JSON line on stdout per command, keyed by the schema's words, usage/session errors on stderr, no terminal test, exit 0/1/2/3 (rows `decision/escher-session-agent-read-surface`, `decision/escher-session-no-colour`, `decision/driver-output-for-agents`, the exit-status/exit-code and stdout/stderr contracts). A seed argument on `start` keeps this form; a bad seed is a usage ending or a refusal within the four statuses, none added.

@history_item n=7 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: newline-delimited JSON-RPC 2.0 only, no ESC byte, `Err` ending on stderr with status 3 (rows `contract/escher-session-mcp/*`). If `start` takes a seed, the MCP `start` tool's input schema carries it on this ratified carrier.

@history_item n=8 marker=2026-10-12-self-description
**2026-10-12-self-description** — `escher-session help` self-description line and exit statuses read from the schema's `EXIT_STATUSES` table; a session error's object carries its fixed `remedy` (rows `contract/escher-session-commands/help`, `decision/exit-statuses-from-schema-table`, `contract/escher-session-session-error/remedy`, `contract/escher-session-mcp/help-result`). The scope names `help`'s self-description among the surfaces a seed argument touches.

@history_item n=9 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording's `record-dir-not-private` and `recording-failed` are session errors in the exit-3 class, no new exit, as fixed text inside a ratified carrier (row `contract/escher-session-exit-status/3`). Precedent for a kept-check (`recording`) change — if the seed is recorded, any new failure rides the existing classes.

@history_item n=10 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — cli: `escher-session` serves six tasks (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`), listed in the usage line's `apps:` and the MCP `start` tool's `enum` (rows `contract/escher-session-tasks/*`); also the overlay scrollbar band and the load page's `LOAD_ANSWER_DELAY` thread. A seed on `start` extends the same `start` tool; a randomness-reading fixture or stand surface for the proof must decide whether it joins the six (each served task is a row).

@history_item n=11 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flight-type select overrides and the load page's group leave design-system) — Kept: surface's cli item naming the six served tasks and the driver's load-wait items stay, settled on the founder's word «Оставить как есть». Adding a seventh served task (a randomness fixture) amends a founder-settled row; stand-only values belong in layout-templates, not design-system.

@history_item n=12 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand registers the bundled DejaVu Sans with system fonts off through `HarnessOptions.font_ctx`, so layout is the same on dev host and CI. The determinism precedent through `HarnessOptions` — the seed's natural home beside it; bundled fonts themselves are v030-14, out of this chunk.

@history_item n=13 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the `HarnessOptions.font_ctx` registration is held in architecture §Standard Contracts → Headless stand, not design-system. A harness seed option is likewise an architecture contract, not a design-system value.

@history_item n=14 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once, through `DocumentConfig::default()` (row `decision/default-ua-sheet-via-default-config`). A seed plumbed through document/harness config must keep the default config path the test harness and Dioxus documents take.

@history_item n=15 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — on every Dioxus document a render is recorded into a mutation buffer unborrowed, then replayed under one borrow (row `decision/buffered-render-replay`). An app-facing randomness source in `dioxus-native-dom` (root context or hook) read during render must not reintroduce a `RefCell already borrowed` borrow.

@history_item n=16 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part]` answers `{"text":…}` or `{"text":…,"part":n,"parts":m}` (rows `contract/escher-session-commands/snapshot`, `contract/escher-session-mcp/snapshot-result`). The proof compares snapshot text and diffs across two seeded runs; this is the answer form it reads.

@history_item n=17 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — the graph line carries ids, owner ids, names, closed flag words and booleans only, never a value (row `contract/ui-graph-line/content`). Content rule on answer lines: a seed's value stays out of every log/answer line unless ratified; graph/state order is a candidate for `HashMap` iteration-order nondeterminism (scope premise).

@history_item n=18 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actions` answer line, carrying ids, verb words, cause words and booleans only, never a value (row `contract/actions-line/content`); a boundary widening on a ratified carrier ratified by the founder. Precedent for the P4 fork whether a seed argument on `start` is a widening; `actions.rs` uses `HashMap` (scope premise on order).

@history_item n=19 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — `escher-session coverage <app> <check>...` replays each kept check on a fresh session; check files read bounded and parsed strictly, refused `malformed` with nothing of the file; line carries ids and counts only (rows `contract/escher-session-commands/coverage-answer`, `contract/coverage-check-files/*`, `contract/coverage-line/content`). If the seed is recorded in a kept check, replay and coverage must read it under the strict parse, and a check without one must still replay.

@history_item n=20 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a `screenshot` with no `sans-serif` resolved is refused `fonts-unavailable`; the cli/MCP shot answer form. A host-dependent state (fonts) gated by refusal — the sibling trust-layer concern (v030-14) this chunk leaves alone.

@history_item n=21 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (decision · contract · absence · overview), each verified against the code; every picked row this chunk binds by comes from this conversion, and an amendment by this chunk lands as `+`/`~`/`−` on a row handle.
