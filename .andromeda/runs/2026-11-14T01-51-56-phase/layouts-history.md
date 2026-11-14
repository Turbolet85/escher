@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand's TaskShell mount: skips Home, mounts one task through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. Origin of decision/headless-stand-pinned-viewport; a seeded-randomness fixture or stand surface for the proof must keep this mount and viewport, and must not take bundled fonts (v030-14) as its own work.

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became a further consumer of the TaskShell mount, booting through `seven_guis::stand` only, adding no markup, id, class, style, wrapper or order; a held instance reads as a fresh boot. Origin of decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot: a seed given to `Session::start` must leave a seeded held instance reading as a fresh boot of that seed, through the same stand boot.

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — two `session_host.rs` citations re-pointed after the host gained the lines handing the Timer's session its time step. The direct precedent for a per-run input (`with_time`) threaded through the host's boot with no claim change; a seed builder beside `with_time` will shift the same cited ranges and must re-point them.

@history_item n=4 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — the test-harness defaults citations re-pointed to the `Default` impl holding 800x600, scale 1, light. The chunk touches `HarnessOptions`; a seed field added there moves those cited lines, a trap to re-point by measurement, not mechanically.

@history_item n=5 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session <verb> [arguments] --session <dir>` with four endings (accepted 0, refused 1, usage 2, session error 3), host role `serve <task>` spawned by `start`. Origin of contract/escher-session-command/form and the exit-code rows; a seed argument on `start` (and passed to `serve`) enters this grammar and its usage line. Same chunk's period carries the founder's 2026-10-10 field rule the scope's P4 fork cites.

@history_item n=6 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role: local stdio MCP server, no listener or auth surface, booting one task in process on a `start` tool call through the same boot; "no ending writes back anything the caller typed" scoped to the commands. Origin of decision/mcp-server-local-stdio and decision/no-caller-input-echoed: a seed on the `start` tool adds no listener and must not be echoed back by a command's ending.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — `escher-session help` answers `describe()`'s seven keys of fixed table text, reaching no session; MCP tool descriptions are each verb's help plus result fields. Origin of contract/escher-session-help/answer and contract/escher-session-mcp/tools; a `start` seed argument changes `help`'s self-description and the `start` tool's schema, both named in the scope.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — precedent for growing a verb's arguments and the verb table across the command line and the `mcp` role together (eleven verbs, the new optional `--id`), ratified as a founder fork. Pattern a seed argument on `start` would follow on both carriers.

@history_item n=9 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` on `start`, host and `mcp` roles; owner-only `check-{n}.json` 0600 in a 0700 dir, `record-dir-not-private` and `recording-failed` session errors; a boundary widening ratified by the founder. Origin of decision/session-recording-replayable-check, contract/escher-session-record/check-file and record-dir-not-private: the closest precedent for a new optional argument on `start` passed to the host, and the kept check a recorded seed would extend.

@history_item n=10 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by `Check::replay`; a widening ratified by founder F3 B. The scope's "a kept check replays the run it recorded" rests on this: recording the seed in the check is a change to this ratified format, a fork to settle.

@history_item n=11 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — tool descriptions' help text changed as "the program's own fixed text inside a ratified carrier", no widening. Precedent for how a seed's help and description text would be classed.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, `escher-session lint`) — the `lint` verb on every carrier, answering `{"findings"}`, kept under `--record`. Origin of contract/escher-session-lint/answer; a seeded run's lint answer must read the same across runs of one seed.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answer `{"text"}` / `{"text","part","parts"}`. Origin of contract/escher-session-snapshot/answer; the proof compares snapshot text and diffs at every step, read through this answer.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`) booted by the host and `mcp` roles, the `start` tool's enum and usage line `apps:` naming them; the load page's in-process `LoadNet`. Origin of the contract/escher-session-served-tasks rows: a fixture reading randomness for the proof, if served, extends this set, the usage line and the enum; none of the six reads randomness today.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list entry) — reads a closed field set inside a ratified crossing as outside a widening, "by the field rule of 2026-10-10". Precedent for the scope's P4 fork whether a seed argument on the ratified carriers is a widening (closed value) or not (open value set).

@history_item n=16 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the screens' citations re-pointed after `use_graph_signal` lines and `dioxus_document.rs` / `schema.rs` additions; and the `graph` verb (thirteen verbs), ratified PROVISIONAL. Origin of contract/escher-session-graph/answer; trap: a root context or hook added in `dioxus-native-dom` and a `schema.rs` change shift these cited lines; graph answers must read the same across one seed.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — fourteen verbs, `actions` after `graph`, `tools/list` fourteen in table order with `help` last. Origin of contract/escher-session-actions/answer and the current verb count in contract/escher-session-command/form and contract/escher-session-mcp/tools; `actions` builds on a `HashMap` the scope lists as an unmeasured iteration-order source.

@history_item n=18 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a consumer of the TaskShell mount; escher-session's coverage role; the bridge's skeleton notes re-cited) — the `coverage` role replays each kept check on a fresh boot through the same boot `main` hands the command line, answering `{"unexercised","checks","held"}`; checks read bounded at 1,048,576 + 1 bytes; the skeleton notes' `dioxus_document.rs` citations moved one line. Origin of the coverage rows and the latest touch of decision/fixed-document-skeleton-main-mount: a recorded seed must reach the coverage replay's fresh boot, and a `dioxus_document.rs` edit re-points the skeleton citations again.

@history_item n=19 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (121, decision 21 · contract 99 · absence 1), each verified against the code. Every picked row's handle and wording dates from here; an amendment by this chunk names them by handle (`+` · `~` · `−`).
