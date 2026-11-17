@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's TaskShell mount at the pinned 800 × 600, scale 1.0, Light viewport with DejaVu Sans (the row decision/headless-stand-pinned-viewport). The committed Counter checks this chunk replays as its byte-for-byte witnesses boot on this mount, so a split of the writer must leave the stand's readings, and so its answers, unchanged.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author ids, among them counter-value and counter-increment (the picked rows contract/stand-element-ids/counter-*). Those ids appear inside the answers kept in `tests/blitz-tests/checks/counter-{cli,mcp}.json`, which this chunk's replay witnesses compare whole and by digest.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — introduced the `escher-session` host (stderr lines carry `service.name=seven_guis`, nothing on stdout). This is the origin of the picked row contract/escher-session-host/log-service-name, and of the rule that the session lines `started` / `status` / `stopped` (forms the split moves) write only through the driver's answer writer.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session` the driver's command line: `<verb> [arguments] --session <dir>`, four endings (accepted one uncoloured JSON line on stdout exit 0, refused exit 1, usage exit 2, session error exit 3), the old form refused `unknown-verb`. These are the exit-code, command-form and unknown-verb rows the split must leave byte-identical. The writer's `impl Outcome` / `impl Refusal` / `impl SessionError` forms are among the unnamed families P4 must place.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: local stdio, no listener or auth, protocol lines only on stdout, the request's admitted `id` echoed, the end writing nothing on stdout. It also scoped "no ending writes back anything the caller typed" to the commands. These are the rows decision/mcp-server-local-stdio, contract/escher-session-mcp/{stdio,end} and decision/no-caller-input-echoed. `mcp.rs` with its `mcp/{framing,listing,result}.rs` children is the module precedent the scope follows, and `mcp/framing.rs` calls `json::` items that must keep resolving.

@history_item n=6 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` (seven keys, fixed table text, no session reached), `remedy` on the session error object, statuses read from `EXIT_STATUSES`, and MCP tool descriptions written as the verb's help plus its result fields. These are the help-answer, exit-code-3 and mcp/tools rows. The chunk's boundary keeps `VERBS` and the `help` self-description unchanged, and `describe.rs` calls the writer.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot` with its `{"width","height","png"}` line, refusals `fonts-unavailable` and `off-screen`, the MCP image block then text block, and the 1,048,576-byte result bound on the whole written form. These are the screenshot-answer, refusal and mcp result-bound/screenshot-result rows. The writer's `shot` form belongs to an unnamed family, and the bound reads the written bytes, so any changed byte would move it.

@history_item n=8 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>`: an owner-only replayable `check-{n}.json` (0600, 0700 dir) and session errors `record-dir-not-private` and `recording-failed`, exit 3. These are the decision/session-recording-replayable-check and contract/escher-session-record/* rows. `recording.rs` calls the writer, and its paths must keep resolving after the split.

@history_item n=9 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check now holds, for each call, its verb, arguments, acceptance, answer digest and the masked answer, with a shot kept as `{"width","height"}` without its `png`. This is the writer's `shot_without_image`, called from outside `json.rs`, which must keep resolving after the split. The kept answers and digests are exactly what `stand_replay` compares byte for byte, so this entry is the mechanism behind the chunk's "answers unchanged" witness.

@history_item n=10 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, 2026-10-20T07-57-08-wrap) — added the `lint` verb, answering `{"findings":[…]}` as one uncoloured line, refused `fonts-unavailable` with no font, and listed as an MCP tool after `screenshot`. This is the contract/escher-session-lint/answer row. The writer's lint forms (`finding`, `bounds`, `ratio`) are an unnamed family the split must place without changing its bytes.

@history_item n=11 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — each acting tool's `settled` field help now states the 5 s load wait, as the program's fixed table text inside `help`'s self-description and the MCP tool descriptions. This is a precedent: fixed help text counts as part of the carrier's written bytes, and the chunk must not move it.

@history_item n=12 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `snapshot --part` (1 to 1000, otherwise refused `malformed`) and the answers `{"text"}` or `{"text","part","parts"}`, with a part past the last returning empty text. These are the snapshot answer and part rows. The snapshot family is one the entry names, and its node and paged forms must move with no body edit.

@history_item n=13 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — set the six served tasks (counter, flight-booker, timer, crud, cells, load) in the usage line's `apps:` and the MCP `start` tool's `enum`. These are the contract/escher-session-served-tasks/* rows and part of the usage line (exit-code 2 row), which the split must leave byte-identical.

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the citations into the files the take moved) — a precedent for a module split: when code moved between files (blitz-vibey-script's `runtime/` and `dom/element/` children), the citations' paths were rewritten and not only their line numbers. The `json.rs` split moves cited writer code the same way, so the masters' pointer rows naming `json.rs` follow the files at the wrap.

@history_item n=15 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — two sub-entries. (a) The `escher-session` entry's schema citations were re-pointed after `schema.rs` grew; `schema.rs` with `schema/spec.rs` and `schema_tests.rs` is the scope's second module precedent. (b) Added the `graph` verb, answering `{"nodes","states"}` with ids, names, closed words and booleans only. That is the contract/escher-session-graph/answer row, and its writer forms (`graph_node`, `graph_state`, `write_readings`) are an unnamed family the split must place.

@history_item n=16 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb, actions) — added `actions`, answering an unpaged `{"nodes":[{id,owner,keyed,verbs:[{verb,cause}]}]}` with `owner` and `cause` left out when absent. That is the contract/escher-session-actions/answer row. Its `action_node` form and the `caused` forms it shares belong to the `caused` family the entry names, and the omit-when-absent bytes must survive the move.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — two sub-entries. (a) Added the `coverage` role: `coverage <app> <check>...`, each check read bounded at 1,048,576 + 1 bytes and refused `malformed` holding nothing of the path, and the answer `{"unexercised":[{id,owner,event}],"checks","held"}` as a written form of the command line alone. These are the coverage answer, check-bound and usage rows; the writer's `impl Coverage` is an unnamed family to place. (b) The bridge's skeleton notes were re-cited after `dioxus_document.rs` moved by one line (the row decision/fixed-document-skeleton-main-mount). The citation re-pointing it shows recurs for this chunk's `json.rs` pointers.

@history_item n=18 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into the 121 rows this chunk's picked rows come from, each verified against the code. It also set the rule that a master holds what a planner must respect while the code holds the rest. A pure refactor therefore adds or changes no row, which matches the scope's "no new requirement line".

@history_item n=19 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph verb; ratified — escher-session's coverage role) — the founder ratified the graph and coverage answer forms with no re-wording. These ratified written forms are among the carriers whose bytes the split must not move.

@history_item n=20 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — moved the verb table from fourteen to fifteen with `layout` (~ contract/escher-session-command/form, ~ contract/escher-session-mcp/tools), added + contract/escher-session-layout/answer, and gave decision/no-caller-input-echoed its exception (the layout answer's `id` echoes the call's id). This is the most recent growth of `json.rs`: the `layout` family (`placer_word` … `layout_answer`) that the audit's M1 measured, and that the split moves into its own module with the exception's bytes intact.

@history_item n=21 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirms behind the layout rows by his own word, with the rows unchanged. The `layout` answer form this chunk moves is therefore ratified, and the chunk must change none of its bytes.
