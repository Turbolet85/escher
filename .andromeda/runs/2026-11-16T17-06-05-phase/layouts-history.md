@history_item n=1 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the `escher-session` host became a third consumer of the `task_in_shell` mount; it adds no markup, id, class, style, wrapper or order, and a held instance reads as a fresh boot with the mount and pinned viewport held after commands. This is where decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot come from. The chunk's fixtures with a real failure (a panic, an `Err` chain, an engine record) cannot add stand markup or break the fresh-boot reading in either layout mode.

@history_item n=2 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the command form `escher-session <verb> [arguments] --session <dir>` and its four endings: accepted, one uncoloured JSON line on stdout, exit 0; refused 1; usage 2; session error 3. This is where contract/escher-session-command/form and contract/escher-session-exit-codes/0 come from. `caused` text has to ride inside that one uncoloured line with no new ending (scope: no new verb, exit, refusal cause or session error).

@history_item n=3 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: local over stdio, protocol lines only on stdout, echoing a request's admitted `id`. It scoped «no ending writes back anything the caller typed» to the commands. This is where decision/mcp-server-local-stdio, contract/escher-session-mcp/stdio and decision/no-caller-input-echoed come from. Error text is app-authored content, not caller input, but a record's message could quote typed text. The scrub and the password/file-input mask must keep this rule true on the command line, and the MCP stdout must stay uncoloured with no ESC byte even when a message holds control bytes.

@history_item n=4 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` answering `describe()`'s line: seven keys, every text fixed table text, no session reached. It also set the MCP tool descriptions as the verb's help plus its result fields. This is where contract/escher-session-help/answer and contract/escher-session-mcp/tools come from. The chunk extends `describe()` to name `caused`'s new fields, the cut and the count. The text must stay fixed table text, the key count stays seven unless amended, and the MCP acting tools' descriptions change with it.

@history_item n=5 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set the MCP result bound: a result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text. This is where contract/escher-session-mcp/result-bound comes from. The scope requires the bound to hold over `caused` text, so the per-message cut and record count have to keep an acting answer under it.

@history_item n=6 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>`, which keeps a session as one owner-only replayable check (`check-{n}.json` `0600` in a `0700` dir). `help` and `tools/list` read the same with and without it. It was a boundary widening ratified by the founder. This is where decision/session-recording-replayable-check comes from. A kept check is one of the carriers the chunk's text must ride, so this is a precedent for a widening that needs the founder's ratification.

@history_item n=7 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the `--record` check keeps per call its verb, arguments, acceptance, the answer's digest and the answer itself, masked as on every carrier. `Check::replay` reads it back and replays it. This is the row the chunk extends: the `caused` text lands in the kept answer and its digest. Replay has to compare it deterministically, and the mask has to apply to it as to every carrier.

@history_item n=8 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting tools' `settled` help text changed as «the program's own fixed text inside a ratified carrier», which also moved `help`'s self-description. This is the precedent for the chunk's `describe()` and tool-description change to `caused`'s help.

@history_item n=9 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` bounds a large answer by paging it, and a part past the last answers the empty text with its `parts` count. This is a nearby precedent for bounding an answer's text so that no cut is silent. The chunk instead takes a fixed cut plus a counted overflow and adds no paging argument (no new verb or flag).

@history_item n=10 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the host and `mcp` roles boot one of six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), listed in the usage line's `apps:` and the `start` tool's `enum`. This is where the contract/escher-session-served-tasks/* rows come from. The chunk's failing-handler fixture must not appear as a seventh served task in either list unless that row is amended.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — a verb answering «ids, component names, handler event names, registered state names, closed flag words and booleans only, never a value». It is the closed-words-only crossing that this chunk widens for `caused`. Moving app-authored text onto the carriers is a boundary widening owed founder ratification and security and architecture amendments at the wrap (scope sec-exun4ujh). Its source-citation entry («the screens' source citations after the graph registrations») re-pointed the `escher-session` entry's citation into the driver's `schema.rs`. The chunk will likely edit that file, so the citation needs re-pointing again.

@history_item n=12 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — the verb table reached fourteen verbs and `tools/list` lists them in order with `help` last. Its answer carries closed words, «never a value, typed text or `NodeId`». This is where contract/escher-session-command/form and contract/escher-session-mcp/tools stand now. The chunk keeps the count at fourteen, and it is the second closed-words precedent that this chunk's text crossing departs from.

@history_item n=13 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added the `coverage` role, a further consumer of the TaskShell mount that boots the kept check's app once per check through the same boot. Each check is read bounded at 1,048,576 + 1 bytes, and a check that cannot be read or is malformed is refused, holding nothing of the path. The role answers `{"unexercised","checks","held"}`. Its re-cite entry moved the `dioxus_document.rs` skeleton and head citations. This is where contract/escher-session-coverage/check-bound, contract/escher-session-coverage/answer and decision/session-roles-share-stand-boot come from. Kept checks with `caused` text grow and must still fit the check bound and replay through this role. Edits in dioxus-native-dom (the root boundary) may shift those citations again.

@history_item n=14 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified: the graph verb, the coverage role as a mount consumer, escher-session's coverage role) — the founder's own word replaced the PROVISIONAL delegate marks on the graph and coverage entries. The rows this chunk rests on (coverage, session roles, verb table) now stand ratified. Any change to them needs a fresh amendment, not a relabel.

@history_item n=15 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows (121 rows, each verified against the code), which produced every picked row's handle. From here on, amendments to this plan name rows by handle (`+`/`~`/`−`). Any change this chunk makes to the help, result-bound or recording rows is recorded against those handles.
