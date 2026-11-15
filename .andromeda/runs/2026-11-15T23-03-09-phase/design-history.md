@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — added the cli surface's `escher-session` bullet: one JSON line on stdout per command (answer, refusal or session-error object in the schema's words), a usage error one fixed stderr line with stdout empty, a session error its fixed stderr message, no colour, no ESC byte, no terminal test, and the exit class 0 accepted · 1 refused · 2 usage · 3 session error (the founder's answers, 2026-10-10). This is the origin of decision/escher-session-agent-read-surface, decision/driver-output-for-agents, contract/escher-session-stdout/command-line-line, the stderr rows and both exit-status/exit-code tables. The smoke must report its verdict under exactly this grammar and must not add colour or a terminal test to anything on this surface.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — gave the `escher-session` bullet its `mcp` role: newline-delimited JSON-RPC lines only on stdout, uncoloured, no ESC byte, still no terminal test, and an `Err` ending writes its fixed message on stderr with status 3 (the source of contract/escher-session-exit-codes/3's "an mcp role ending in Err also exits 3"). Relevant if the smoke's one call goes through the mcp role rather than the command line.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — made the four exit statuses read from the schema's `EXIT_STATUSES` table, which `help` serves; `escher-session help` answers one uncoloured JSON line, status 0, reaching no session; a session error's object carries its fixed `remedy`. This is the source of decision/exit-statuses-from-schema-table, contract/escher-session-commands/help and contract/escher-session-session-error/remedy. A smoke that checks exit codes should take them from that table and not hardcode a second copy. `help` is a cheap call to run, but because it reaches no session it does not exercise the boot path.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — Typography Loading: a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted, and the stand's bundled font passes. cli: `escher-session screenshot [--id]` answers one uncoloured line `{"width","height","png"}` with base64 inside it, status 0, and the mcp role answers with an image block then a text block (the founder's fork 1). This is the source of decision/screenshot-never-skips-text and both fonts-unavailable contracts plus contract/escher-session-commands/screenshot. If the smoke calls `screenshot`, the line is large (base64 PNG), and it passes only because the stand registers its bundled font.

@history_item n=5 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the exit-status class line names the recording's `record-dir-not-private` and `recording-failed` among the session errors (status 3, no new exit), which is the source of contract/escher-session-exit-status/3. Relevant to the smoke's state directory: it is caller-named and relative under `target/tmp/…`, owner-only as every session's is, so a non-private directory is a status-3 session error, not a new ending.

@history_item n=6 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `escher-session snapshot [--part <part>]` answers one uncoloured line, `{"text":…}` for a whole screen or `{"text":…,"part":n,"parts":m}` for a screen read in parts, status 0; over mcp a paged snapshot result is that JSON line. This is the source of contract/escher-session-commands/snapshot. `snapshot` is the natural "at least one call" for the smoke (as `counter.sh` does it), and its verdict must parse the paged or unpaged shape.

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the cli and typography items) — `escher-session` serves six tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), listed in the usage line's `apps:` and the MCP `start` tool's `enum`, and an unknown task is a usage ending. The bundled DejaVu Sans applies to every boot, `boot_cells` and `boot_load` included. This is the source of every contract/escher-session-tasks row: the smoke's "named stand task" must be one of these six, and an unknown name exits 2. The load page waits `LOAD_ANSWER_DELAY` (200 ms) before answering its stylesheet, so a smoke on `load` takes that delay.

@history_item n=8 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the seven_guis headless stand registers the bundled DejaVu Sans for every generic with system fonts off on native (`build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`). That is why a stand session's screenshot passes the fonts-unavailable refusal on any host and its layout is the same on the dev host and the CI runner, which keeps a smoke's verdict deterministic.

@history_item n=9 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's font registration is no longer a design-system typography value; it is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`. It was PROVISIONAL, later ratified (2026-10-27-upstream-sync-engine-debts). The smoke's reliance on the bundled font is therefore stated in architecture, not here.

@history_item n=10 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flight-type select overrides and the load page's group leave design-system; F2's move ratified) — removed the stand-only lines, and its Kept clause settled, on the founder's own word, that surface's cli item naming the six served tasks stays in design-system. The tasks rows the smoke names are a settled cli-surface contract, not stand values parked in layout-templates.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — `escher-session graph` takes no argument, and an argument is refused `malformed` with nothing run. It answers one uncoloured line `{"nodes":[…],"states":[…]}`, status 0, the same on the socket, in a kept check and over mcp. The line carries ids, names, closed flag words and booleans only. This is the source of contract/escher-session-commands/graph-answer, graph-arguments and contract/ui-graph-line/content. It was ratified by 2026-11-15T05-21-42-wrap.

@history_item n=12 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unpaged graph, a pending limit) — the `graph` answer is unpaged: Cells at boot answers 293,869 bytes in one line (the limit "An unpaged graph", residual `res-7cc5bi4k`). This is a trap for the smoke: if its call is `graph` on `cells`, the verdict reader must take a ~290 KB line, so a smaller task or call keeps the smoke cheap.

@history_item n=13 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (citation ranges re-read after the graph chunk's code) — re-pointed the `escher-session` item's citations: the `EXIT_STATUSES` table moved to `schema.rs:557-595` and the written JSON to `json.rs:400-462`. These are where the exit grammar and the JSON line the smoke reads are defined. The smoke edits neither file, so it should move none of these citations.

@history_item n=14 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line) — `escher-session actions` takes no argument (an argument is refused `malformed`). It answers one uncoloured line `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, status 0, listing actionable nodes in snapshot pre-order, each verb's cause decided by the same `validate` and `check_target` the call meets, and a node that resolves to no element reads `stale`. This is the source of decision/actions-share-the-verbs-checks and the actions-line, actions-answer and actions-arguments contracts. It is a possible read-only smoke call, and it adds no verb.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged `actions` answer, a stated limit) — the `actions` answer is unpaged: Cells at boot answers 106,088 bytes for 678 nodes, within the 1,048,576-byte answer bound (residual `res-7cc5bi4k`). The same size consideration as the graph applies if the smoke calls `actions` on `cells`.

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the escher-session coverage role's answer line, endings and refusals) — `escher-session coverage <app> <check>...` replays kept checks on fresh sessions and answers `{"unexercised":[…],"checks":n,"held":m}`, status 0. Its one shape has no `--session` and no `--record` (any other shape exits 2). Check files are read bounded at 1,048,576 + 1 bytes and parsed strictly, and refusals are `malformed`, status 1, holding nothing of the path. It adds no exit status. This is the source of the coverage contracts. Coverage is the one role that boots its own sessions from argv, so a smoke built around it would take that shape rather than start/stop. It was ratified by 2026-11-15T05-21-42-wrap.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage answer's three stated limits) — handlers in a canvas are not counted, a handler on no reached screen is not listed, and a bubbling dispatch counts every ancestor listener. These matter only if the smoke uses `coverage` and asserts on its counts.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows (137 rows), each verified against the code, with the founder's approval of their fates. Every picked row (decision/contract handles in typography, surface and surface-cli) dates from this conversion. The smoke must hold to these rows as they stand and add no carrier, verb, refusal cause or session error, which matches the scope's boundaries.

@history_item n=19 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — added `+ decision/unavailable-families-use-default-font` (text falls back to the default font when no listed family is available) beside the picked decision/no-generic-family-lays-out-sans. Together they explain why `can_paint_text`, behind the screenshot refusal, reads the sans-serif generic alone. That matters to a smoke that calls `screenshot`.

@history_item n=20 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified: the graph answer line; ratified: the coverage role's answer line, endings and refusals) — moved both the `graph` and the `coverage` records from PROVISIONAL (delegate the overseer) to ratified on the founder's own word, text unchanged. Every cli-surface command the smoke might drive now stands on ratified records, and the smoke may not re-open any of them.
