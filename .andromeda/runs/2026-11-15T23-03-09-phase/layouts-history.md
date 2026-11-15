@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: it skips Home and mounts one lean task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. This is the origin of decision/headless-stand-pinned-viewport, the boot the smoke's session goes through. The smoke may add no second way in.

@history_item n=2 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added `scripts/agent-run.sh` to the cli surface: usage `agent-run.sh <verb> [selection]`, verbs `boot · run · status · cleanup · logs`, selections `stand · all · <blitz-tests file name>`, usage on stderr with exit 2, stdout JSON lines only, raw cargo output kept in `target/agent-run/run.log`, `agent-run.ps1` its Windows pass-through. The exit grammar and event schema stay in test-plan §3. This is the source of every contract/agent-run-cli/* row. A new smoke verb or selection extends this usage row and must keep its stdout, run.log, exit-2 and pass-through clauses.

@history_item n=3 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added `scripts/cold-agent.sh` beside agent-run.sh with the same shape: usage error on stderr with empty stdout, exit 2 before any precondition, JSON lines only on stdout, raw transcript kept under `target/cold-agent/`, a `.ps1` pass-through. This is the precedent for a harness script that drives `escher-session`. The pipe itself is out of this chunk's scope.

@history_item n=4 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author ids, among them `counter-value` and `counter-increment`, adding no class, style, wrapper or order. These are the contract/stand-element-ids/counter-* rows a smoke call on the counter task would name, for example a click on `counter-increment`.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — added the `escher-session` host binary as a third consumer of the TaskShell mount. It boots through `seven_guis::stand` only and adds no markup, id, class, style, wrapper or order. A held instance reads as a fresh boot, and the mount and pinned viewport hold after commands. This is the origin of decision/held-instance-reads-as-fresh-boot (guard `stand_session_fresh`) and decision/session-roles-share-stand-boot, which the smoke rests on rather than re-proves. The binary is unix only.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — re-pointed two citations into `examples/seven_guis/src/session_host.rs` (the lean-task boot and the exit-code match). This is a precedent: when the host file moves, the cited exit-code and boot lines in the cli and desktop-native entries move with it.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session` the driver's command line: `escher-session <verb> [arguments] --session <dir>`, `start <task>` spawning the `serve` host role, and four endings: 0 accepted (one uncoloured JSON line on stdout), 1 refused, 2 usage (one fixed stderr line), 3 session error. The old two-argument form now reads refused `unknown-verb`. This is the source of contract/escher-session-command/form and contract/escher-session-exit-codes/0–3, the grammar the smoke's verdict must map onto. Trap: the operator ruled that `flow-names-no-element`, the id the counter flow's refusal step names, is the flow's own step and not a surface. If the smoke reuses `counter.sh`, this plan lists no such id.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role as a further consumer of the TaskShell mount, booting through the same boot. It scoped «no ending writes back anything the caller typed» to the commands, which is the origin of decision/no-caller-input-echoed. A smoke verdict must not echo the caller's state-dir or arguments. The role carries no listener or auth surface, matching the scope's no-new-listener boundary.

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `help` and its rule: `--session` exactly once on every command but `help`, and `--session` missing a directory or given twice is usage (2). The session error's object carries a fixed `remedy`, and statuses are read from `EXIT_STATUSES`. This shapes contract/escher-session-command/form and exit-codes/3, which the smoke's command lines and error mapping must respect.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot`, eleven verbs. An app with no font is refused `fonts-unavailable` (exit 1). The verb-table count behind the command-form row grew here. If a smoke call paints, it relies on the bundled DejaVu Sans of the pinned viewport.

@history_item n=11 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe's `run` needs a built `target/debug/escher-session` (exit 3, the pipe builds nothing) and uses the driver's own MCP server. This is the nearest precedent for a harness entry over the session binary, with the build-or-require choice made the opposite way from this chunk's «builds the binary» smoke.

@history_item n=12 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `start <task> [--record <dir>]`, `serve … [--record <dir>]` and `mcp [--record <dir>]`. Recordings are owner-only (`0700` dir, `0600` checks) with session errors `record-dir-not-private` and `recording-failed` (exit 3). This is the origin of decision/session-recording-replayable-check and the founder-ratified boundary widening. A smoke that records must use an owner-only directory.

@history_item n=13 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the `--record` check keeps, per call, its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by `Check::replay`. This completes decision/session-recording-replayable-check as the picked row states it, and is a candidate replayable form for the smoke's call.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, `escher-session lint`) — added `lint` as the eighth instance verb, twelve verbs; an app with no font is refused `fonts-unavailable` (exit 1), and a `lint` step is kept under `--record`. This is part of the verb-table count behind contract/escher-session-command/form.

@history_item n=15 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `snapshot [--part <n>]`, answering `{"text"}` or `{"text","part","parts"}`, with an out-of-bound part refused `malformed` (exit 1). This is the source of contract/escher-session-snapshot/answer, which the smoke reads if its call is a snapshot (as `counter.sh` does).

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — widened the served tasks to six (`counter · flight-booker · timer · crud · cells · load`): the host role boots Cells by `boot_cells` and the load page by `boot_load`. `start <task>` takes one plain word of the six, listed in the usage line's `apps:` and the MCP `start` tool's enum. The headless stand mounts one lean task or Cells, and the load page in its own TaskShell. This is the source of every contract/escher-session-served-tasks/* row, the set a smoke's named stand task is chosen from. The cold-agent pipe's `mcp --record` into `target/cold-agent/record` is a precedent for an owner-only, cleanup-removed state directory.

@history_item n=17 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — added `graph` as the ninth instance verb, thirteen verbs, listed after `lint` in the usage line. This is part of the verb-table count in contract/escher-session-command/form. The same marker's citation entry re-pointed the `escher-session` entry's schema citation and the document-skeleton notes behind decision/fixed-document-skeleton-main-mount.

@history_item n=18 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — added `actions`, making the command form the fourteen verbs of the verb table. It answers one unpaged uncoloured `{"nodes"}` line of ids, owners, keyed flags and verbs with causes, never a value, typed text or NodeId. This is the source of contract/escher-session-actions/answer and of «fourteen verbs» in contract/escher-session-command/form, the verb set the smoke drives as it stands and adds none to.

@history_item n=19 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount; escher-session's coverage role) — added the fourth role, `coverage <app> <check>...`, which boots through the same boot `main` hands the command line and adds no markup. A held instance still reads as a fresh boot in both layout modes. This completes decision/session-roles-share-stand-boot (host, mcp, coverage) and decision/held-instance-reads-as-fresh-boot as the picked rows state them. The same marker re-cited the bridge's skeleton notes behind decision/fixed-document-skeleton-main-mount.

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — converted the master's text into rows (121 rows: decision 21, contract 99, absence 1), each verified against the code. This is where every picked row's handle and wording, including the agent-run-cli, escher-session and stand rows, comes from. An amendment from this chunk names rows by handle (`+`/`~`).

@history_item n=21 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph verb; the coverage role as a mount consumer; escher-session's coverage role) — moved the graph verb and both coverage-role entries from PROVISIONAL to ratified on the founder's word, wording unchanged. The command-form, served-tasks and session-roles-share-stand-boot rows this chunk rests on now carry no PROVISIONAL mark.
