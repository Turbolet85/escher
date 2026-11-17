@history_item n=1 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — set the rule that a driver command waiting on a load with a clock is a new decision, and that a settle verb or busy reply on the wire is a new crossing question; `layout` must stay a no-wait read (rows decision/time-moves-only-on-advance, decision/no-wait-verb-fields).

@history_item n=2 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built `validate` as the closed verb table with bounded argument kinds (`id` 1 to 1024 bytes), one answer per call in a fixed order, no panic on call input, refusal texts fixed and free of call content; the fifteenth verb's schema row and `id` argument go through this same gate (decision/closed-driver-verb-schema, decision/no-panic-on-call-input, decision/refusal-holds-no-call-input).

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — a caller-supplied id is looked up anew on every call through `get_node`, never by index, and nothing public runs a `Command` that did not come from `validate`; `layout --id` takes an id the same way (decision/id-lookup-anew-each-call, decision/one-validation-path-for-every-carrier).

@history_item n=4 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — an unknown id is refused `stale` or `not-found` against the session's bounded id record (4096 ids, fed from its own snapshots); a sixth verb was ratified as a widening of the validated surface; `layout`'s refusals reuse exactly these causes, no new one (decision/unknown-id-refused-stale-or-not-found, decision/session-id-record-bounded, contract/driver-id-refusal/*).

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — fixed the one `command` span per call with fixed-word/count fields, `verb` the table's word, `cause` the only refusal content logged, no argument fielded; the scope's span for `layout` records `verb` and `cause` alone (decision/one-command-span-per-call, decision/refusal-log-cause-name-only).

@history_item n=6 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified that counts of a diff may reach a log while its content may not, a field holding a node, id, name or value being a new crossing question; the scope goes further for `layout` (no count of the answer on the span), consistent with this rule.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (socket, command line, answers) — the session socket ratified to carry calls and answers (ids, names, masked values, snapshot text, diff, no `NodeId`), the command line made an external input surface through a closed argv reader checked before the session address, no ending echoing caller input, refusals written from fixed strings; `layout --id {id} --session {dir}` rides this crossing (decision/session-socket-carries-calls, decision/cli-closed-argv-reader, decision/cli-echoes-nothing-typed).

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — set the standing rule that a new field of a closed value set inside a ratified carrier is no widening, while authored text, an open value set, a new carrier or a new exit still is; the layout answer (stable ids, closed words, numbers) and its unexplained element's new closed word are judged by this rule (decision/boundary-widening-rule, decision/closed-value-field-no-widening, decision/snapshot-state-readings).

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the MCP crossing: bounded request lines and results, typed tool arguments refused `malformed` before `validate`, fixed protocol errors with no data, the `tool_call` span (`tool`, `ending`) parenting `command`, `serde_json` read-only; the new `layout` tool sits inside this crossing (decision/mcp-result-crossing, decision/mcp-bounded-lines, contract/mcp-tool-call-span/*).

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` and the `tools/list` descriptions cross as the program's own fixed table text, ratified with the rule that page/call/screen text or a new carrier still widens; `layout`'s help and result fields must stay fixed `'static` table text (decision/mcp-listing-fixed-text, contract/mcp-server/help).

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — precedent for adding an instance verb that takes an `id` looked up as `click`'s and is refused with the existing causes, its content never fielded on the span; the verb-count and MCP-tool-count moves of a new verb were recorded here first.

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the refusal cause set closed at eight (`time-unavailable` retired); the scope's «no new cause» keeps `layout` inside this closed set (decision/closed-refusal-cause-set, decision/refusal-closed-cause-set).

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check at rest (0600 files in a 0700 directory, never named or logged), its strict bounded reader, `call_of` so a replayed step passes `validate` again, and a call outside the instance verbs not recorded; a recorded `layout` step falls under these rows (decision/recording-directory-owner-only, decision/recorded-check-strict-reader, contract/recorded-check/tool).

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer stored whole (masked as on every carrier), `Check::replay` through `Session::run` alone, nothing run on another app's session; the scope's «answer kept whole, `Check::replay` replaying it» extends this (decision/kept-check-answer-whole, decision/replay-through-session-run).

@history_item n=15 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (in-process lint, then the gated part) — the direct precedent: an in-process session read (`Session::lint`, PROVISIONAL «in process only, crosses nothing») later became a verb on every ratified carrier by the founder's ratification, its findings never fielded, the record of ids fed by it; also added the read-only text extent reader answering none for an unresolved id; `layout` retires `decision/layout-read-in-process-only` the same way (decision/lint-findings-crossing, decision/text-extent-read-only-none-when-unresolved).

@history_item n=16 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — only acting steps wait on a load (bounded by `LOAD_WAIT`), while read verbs wait on none and no verb has a wait argument or field; `layout`, a non-acting read, must join the no-load-wait list (decision/time-moves-only-on-advance).

@history_item n=17 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the committed `docs/capabilities.md` states the MCP crossing and its carried set, adding no carrier, with carried sets reconciled at every site; the chunk must add the layout answer to those carried sets (decision/capability-statement-mcp-crossing).

@history_item n=18 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the secret search (`/home/|api[_-]?key|authorization:|bearer `) covers `README.md`, `docs/capabilities.md` and the pin; the chunk edits both documents (tool count), so the absence must still hold on them.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms; public select readers) — layout-geometry readers on every document read through transforms, answer none on a non-finite edge, and resolve their node through `get_node` so a stale id reads none; a layout answer's geometry rests on the same readings (decision/visible-region-through-transforms, decision/stale-node-ids-never-panic).

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (select list crossing and readers) — the select list crosses only as a row's own bounds and pixels, with `closed_select_of` answering none for an open list; a layout answer for a select or its rows must not add a list field (decision/select-list-crossing, decision/select-readers-resolve-node-closed-select-none).

@history_item n=21 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (descriptions to the adapter) — descriptions exit only to the platform adapter, no driver or MCP verb, field, log field or span carrying one; the new verb's answer must carry no description (decision/descriptions-adapter-only).

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the thirteenth verb `graph`: no argument (any refused `malformed`), runs no step, feeds the id record, written by `to_json` alone, `verb` alone on its span, 0 needles in `host_log`/`mcp_log`, kept in checks, verb counts moved in schema, MCP, cold-agent `VERBS` and the recorded-check `tool` set, unpaged answer under the bound; the closest template for `layout`'s carrier rows and count moves.

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the fourteenth verb `actions` ratified on every carrier; the shared `check_target` so the actions read lists the cause a call would return; `actions` reads no load and changes nothing; the answer unpaged under the bound; verb table at fourteen, ten instance verbs; `verb` alone on the span. The chunk adds `layout` to every node's verb list and moves these counts to fifteen and eleven (decision/actions-answer-crossing, decision/actions-read-shares-target-check, decision/actions-read-changes-nothing, contract/driver-verbs/actions).

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the coverage role reads caller-named kept checks as untrusted through the bounded strict reader and replays them; a kept `actions` step's replay is pinned by `stand_replay`, whose digests move with a new verb word; a kept `layout` step reaches this reader too (decision/coverage-check-files-untrusted, decision/kept-check-actions-answer).

@history_item n=25 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role's carrier and its line, so the second production reader of kept checks stands ratified for the new verb's steps.

@history_item n=26 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — ratified the actions answer's `keyed` field (rows decision/actions-keyed-field, contract/actions-answer/keyed, decision/kept-check-actions-answer to standing); the actions answer the chunk extends with the `layout` verb word has no provisional field left.

@history_item n=27 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — the kept check's opening record gained a strictly read `format` and exploration became a third way into the owner-only medium; a recorded `layout` step lives inside this format (contract/kept-check-file/entry, contract/kept-check-file/medium).

@history_item n=28 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — the driver session socket exists on unix only while an in-process session runs on linux, windows and macOS; the cross-OS digest re-read the chunk owes runs on that in-process path (decision/driver-session-unix-socket-only).

@history_item n=29 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added `decision/session-layout-read-in-process` (id from Rust code alone, resolved through the run path's stale/not-found resolution, no step, answers stable ids, closed words and numbers) and named the layout answer in `decision/no-screen-content-in-logs`; this chunk moves the read onto every carrier, so the in-process row must be rewritten and the log row's coverage measured over the carriers.

@history_item n=30 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — gave decision/driver-installs-no-subscriber and decision/driver-no-subscriber-no-env the scoped-capture exception for acting steps only; `layout` is no acting step and must run outside that capture, installing nothing.

@history_item n=31 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — widened decision/socket-carries-screen-reading and decision/mask-holds-on-every-carrier with caused text, and added a step's caused text to decision/no-screen-content-in-logs; the chunk's answer rows must keep these exceptions bound to acting steps, the layout answer carrying no caused text.

@history_item n=32 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — last edit to decision/mask-holds-on-every-carrier and decision/socket-carries-screen-reading (a caused item's error field kept as written); the held CARRY on contract/driver-session-reply/no-attribute-text came from this exception and stays out of this chunk, the layout answer carrying no attribute text.
