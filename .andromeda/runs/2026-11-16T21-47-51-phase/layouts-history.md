@history_item n=1 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting tools state the load wait: the `settled` result-field help in the driver's schema (and with it `help`'s self-description and the MCP tool descriptions) changed as "the program's fixed text inside a ratified carrier", recorded as an amendment to the cli `escher-session` entry. Direct precedent for this chunk's rewording of `caused`'s result-field help in `packages/escher-driver/src/schema.rs` (served by `help` and as tool descriptions, rows contract/escher-session-mcp/tools, contract/escher-session-help/answer): fixed text, no new carrier, but a sidecar entry is expected.

@history_item n=2 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` (seven keys, fixed table text, no session reached) and made each MCP tool's description the verb's help plus its result fields. Origin of rows contract/escher-session-help/answer and contract/escher-session-mcp/tools; the `caused` help this chunk edits flows into both, so the seven-key shape, fixed-text rule and table order must hold.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' source citations after the graph registrations) — re-pointed the `escher-session` entry's citation into `packages/escher-driver/src/schema.rs` after lines were added there. Trap: this chunk edits `schema.rs` (`caused`'s help, ~:193-195), so any layout citation into that file may shift and need re-pointing at wrap; the same entry's second item (graph verb, 13 verbs) shows the verb-count rows this chunk must not move.

@history_item n=4 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — brought the command line and MCP to the fourteen verbs, `actions` answering ids/verbs/causes and never a value or typed text. Origin of rows contract/escher-session-command/form, contract/escher-session-mcp/tools and contract/escher-session-actions/answer; this chunk adds no verb, cause or tool, so these counts and the never-a-value boundary of `actions` stay as written.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role over stdio: JSON-RPC 2.0, protocol lines only on stdout, echoing the admitted id, a ratified crossing (founder, 2026-10-10) recorded in the security plan. Origin of row contract/escher-session-mcp/stdio; MCP is one of the carriers on which `caused`'s `error` field now crosses unscrubbed, so the widening rests on this ratified carrier and changes no protocol line.

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set the MCP result bound: a result holding an image bounded at 1,048,576 bytes by its whole written form, a text-only one by its text. Origin of row contract/escher-session-mcp/result-bound; an unredacted `error` text makes a `caused` answer longer, but `TEXT_CUT` (1024 B) and per-record/per-capture limits stay, so the text-only bound must still hold.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session <verb> [arguments] --session <dir>` the driver's command line with its four endings (accepted one uncoloured JSON line on stdout exit 0; refused 1; usage 2; session error 3). Origin of rows contract/escher-session-command/form and contract/escher-session-exit-codes/0; the `caused` item rides the exit-0 answer line, whose shape and endings this chunk leaves unchanged.

@history_item n=8 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` keeps a session as one owner-only replayable check, a boundary widening ratified by the founder (F1 A, F2 B). Origin of row decision/session-recording-replayable-check; a kept check is one of the carriers of `caused` named in this chunk's boundaries.

@history_item n=9 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the `--record` check keeps each call's answer itself, "masked as on every carrier", read back and replayed by `Check::replay` (founder's F3 B). Directly bears: the kept answer of an acting verb now holds `caused`'s `error` text unredacted, so the kept-answer masking and replay digest must still agree with the live answer; precedent that widening what an answer carries is a founder-ratified boundary change recorded on the cli entry.

@history_item n=10 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount; escher-session's coverage role) — added the `coverage` role, booting each kept check's app through the same stand boot and replaying it, answering ids, owner ids, event names and counts only. Grounds row decision/session-roles-share-stand-boot; replay of kept checks that now carry `error` text must still pass under this role.

@history_item n=11 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — graph verb; coverage role as mount consumer; coverage role) — the founder's own word replaced the delegate PROVISIONAL marks on the graph and coverage entries. Shows the PROVISIONAL-then-ratified pattern this chunk's own widening card (founder's «Открыть error в caused») follows at its wrap; the rows on the cli and desktop-native surfaces it rests on now stand ratified.

@history_item n=12 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the six served tasks including `load` (booted by `boot_load`, listed in the usage line's apps and the MCP start tool's enum), and the load page with `load-fetch`, `load-done` and the in-process `LoadNet` answering after 200 ms. Origin of rows contract/escher-session-served-tasks/load, decision/load-page-only-network and contract/stand-element-ids/load-fetch; a real fetch failure on the load page is the natural source of an engine `Resource load failed` record carrying `url` and `error`, so the stand's network provider stays the load page's alone and adds no screen.

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines) — stated that `LoadNet` answers from a thread of its own, so a driver `click` on `load-fetch` waits it out and returns with the message shown. Grounds row decision/load-page-only-network's timing; an acceptance check driving a failing load through `load-fetch` relies on that wait for the record to be in the step's capture.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence and its button's keys) — `load-fetch` reads as taking Enter and Space, the load page's Tab sequence named by `LOAD_TAB_ORDER`. Source of row contract/stand-element-ids/load-fetch's wording; no stand markup may move under this chunk.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed the non-lean tasks' controls so `unkeyed_actionable()` reads 0 on all seven tasks (operator's "Key all three"). Origin of row decision/every-stand-control-keyed; any stand fixture this chunk's check adds must keep every actionable element keyed.

@history_item n=16 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand skips Home and mounts one task in TaskShell at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. Origin of row decision/headless-stand-pinned-viewport; the error-field check (`tests/blitz-tests/tests/stand_act_error_text.rs`) boots through this stand and must not change it.

@history_item n=17 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host as a further consumer of the `task_in_shell` mount, booting through the stand's boot and adding no markup, id, class, style, wrapper or order. Origin of row decision/session-roles-share-stand-boot (later extended by the mcp and coverage roles); the carriers this chunk widens all boot through this shared stand boot.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (121), each verified against the code, the fates by the founder's approval. Every picked row's handle and wording comes from this conversion; an amendment this chunk makes names the row it changes by handle (`~ {handle}`), as 2026-11-15-session-binary-smoke did.

@history_item n=19 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage`: the row form of a cli-surface amendment, a promise was/now by handle. Format precedent should this chunk's wrap amend a cli row (e.g. contract/escher-session-help/answer or contract/escher-session-mcp/tools for the reworded `caused` help).
