@history_item n=1 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (amd-mqyv3bpo, a driver call validated before anything runs) — set up the closed verb table with bounded argument kinds (`key` one of twelve names, `flag`), one answer per call in a fixed order, and a refusal holding nothing a call supplied, its `Fault` naming at most the schema's own argument name. It also ruled that the first surface reaching `validate` from outside is its own crossing question. A Ctrl/Meta/Alt modifier on `press` and a `part` on `graph` are new argument shapes in this table, so they inherit its order, bounds and no-panic rule (decision/closed-driver-verb-schema, decision/no-panic-on-call-input, decision/refusal-holds-no-call-input).

@history_item n=2 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (amd-ewhmiczw, one span with fixed-word or count fields) — the `command` span holds `verb` (the table's word), `cause` and counts, and never fields any argument. A new modifier flag or a graph part number must not become a span field (decision/one-command-span-per-call, decision/command-span-content-free, contract/driver-command-span/verb).

@history_item n=3 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (amd-y4xurbfq and amd-l3qkvs2z, the command line and the socket as external input surfaces) — the closed argv reader with fixed rules checks a call before the session address is asked for, no ending echoes anything typed, and named flags (`--shift` bare, `--part`/`--key` with one value) form the CLI's grammar. The socket's `call` grammar carries kinds `t`/`n`/`f`, decoded strictly. A `--ctrl` flag and `graph --part` land on both readers (decision/cli-closed-argv-reader, decision/no-echo-of-caller-input, contract/cli-flags/shift, contract/cli-flags/part, contract/session-socket-request/call).

@history_item n=4 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (amd-trieeiqe, the closed-field rule) — the founder's rule: a new field of a closed value set inside an already ratified carrier is no widening and does not come to him, while authored text, an open set, a new carrier or a new exit still does. The scope names this rule as the test for whether `press`'s modifier needs ratification (decision/boundary-widening-rule, decision/closed-value-field-no-widening).

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (amd-qj7unb2e, the MCP server's input and its crossing) — MCP tool arguments are typed before validation (string to text, i64 to number, bool to flag, anything else refused `malformed`), with bounded requests and results and protocol errors that carry no data. A new `press` modifier and a graph `part` reach MCP through this typing and through `validate` (contract/mcp-tool-arguments/value-kinds, decision/mcp-bounded-lines, contract/mcp-tool-result/size-bound).

@history_item n=6 marker=2026-10-12-self-description
**2026-10-12-self-description** (amd-t3m3x2fx, `help` and the program's fixed text) — `tools/list` descriptions and `help` are the program's own fixed table text inside the ratified carriers. Restating `press`'s help (its "no Ctrl" limit) or `graph`'s help for a `part` stays fixed text under this rule (decision/mcp-listing-fixed-text).

@history_item n=7 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (amd-i4czx32m and amd-dk7ys7xw, the kept check, its reader and wire v3) — a kept check's records use closed keys and schema-named arguments, read strictly by `Check::from_bytes`, and the socket wire is `v3`. A new `press` modifier argument or a `graph` part must be spellable by a record and read back by the strict reader (decision/recorded-check-strict-reader, contract/recorded-check/arguments, contract/session-socket-reply/ok-accepted).

@history_item n=8 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (amd-s7z7sy3e, each kept answer whole, the replay) — each kept answer is kept whole and masked, and `Check::replay` runs every step through `Session::run`, validated again. A paged graph step kept with its answer and replayed must hold to this (decision/kept-check-answer-whole, decision/replay-through-session-run, decision/recorded-check-untrusted).

@history_item n=9 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (amd-ox7pqekt, `snapshot`'s `part` on every carrier) — the precedent `graph --part N` is ruled to copy. It added the `part` kind (1 to 1000, `MAX_PART`, through `u32::try_from`), part 1 when over budget, the empty text past the last part, `Outcome::Part` written by `to_json` alone, part number and count never fielded, `--part` as a one-value CLI flag, and `host_log`/`mcp_log` part reads with 0 needles. All of it was ratified as a widening of the validated surface onto every carrier (contract/driver-argument-kinds/part, contract/cli-flags/part).

@history_item n=10 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (amd-ee5l6rjl and amd-qr32fehm, the paged decode and the host-sink part read) — the cold-agent pipe decodes a paged answer only as an object with exactly `text`, `part` and `parts`. A paged graph answer with other fields (`nodes`/`states` plus part counts) would not match that decode. The host sink's paged-part read was first measured here (`host_log` `snapshot --part 2`, 0 needles) (decision/paged-answer-closed-decoding).

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-gs5hhw4q and amd-qhtu6mrd, a `multiple` select's keys and clicks, `type` on a `multiple` select) — the engine's closed key match: Ctrl or Meta moves the current row by key and flips a row by click, Shift spans, and Ctrl+A is not modelled. The founder kept `press` without Ctrl as a limit there (residual `res-hfvjj77l`), so the driver cannot build a non-adjacent set. This is the engine rule a `press` Ctrl modifier would reach (decision/multiple-select-keys-and-clicks, decision/multiple-select-set-exits, decision/select-keys-closed-match, decision/select-choice-one-write-path).

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-rmpct6h3 and amd-kfo2mltt, the select verbs' help, the driver's `type` on any select) — `press`'s help states the keys on a closed select, and `type` on any select focuses with no click. Any restated `press` help for a modifier stays a fixed literal holding nothing a call supplies (decision/driver-type-on-select, decision/select-list-keys-and-painted-labels).

@history_item n=13 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-ykavrele, amd-w373to52, amd-grjmdj7g, amd-qctr2j3x, amd-kte5nwml and amd-7qx3pzfx, the graph verb and its crossing) — `graph` takes no argument, and any argument is refused `malformed` with nothing run on every carrier. Its answer is unpaged "a part argument would widen the crossing", and Cells reads 293,869 bytes, owned by `res-7cc5bi4k`. The answer holds names, flag words and booleans only, is kept whole in a kept check, and its span records `verb` alone with 0 needles in `host_log`/`mcp_log`. `graph --part N` reverses the no-argument rule and the unpaged control, and the founder has already ruled it a widening (decision/graph-answer-crossing, decision/graph-answer-unpaged, contract/driver-verbs/graph, contract/graph-answer/nodes-states, decision/kept-check-graph-answer).

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-ufb4xpzp, the Dioxus bridge's dispatch and the signal registry) — `DioxusEventHandler::handle_event` sets the registry's dispatch around `runtime.handle_event`, inert with no registry. The registry holds no state value and has no log site. Reading a stopped bubbling dispatch in the reached record touches this dispatch path (decision/dispatch-recorded-only-with-signal-registry, decision/signal-registry-holds-no-state-value).

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-pidyv3mh and amd-otb64thn, the unpaged actions answer and the actions verb's validation) — `actions` with an argument is refused `malformed`, and its unpaged size (106,088 bytes on Cells) was added to `res-7cc5bi4k` beside "An unpaged graph". Paging `graph` alone leaves the actions half of that residual standing (decision/kept-check-actions-answer, contract/refusal-causes/malformed).

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-p32xt5yc, the reached record and `handler_coverage`) — the bridge writes each chain node whose listeners hold the event into the root-context `ReachedRecord`, deduplicated by (id, event), counting a handler whose run removes its element, with no log site and no `NodeId`. The stated limit is `res-4dr3wt62`: `stopPropagation` is not read. This is the record a stop-aware fix changes (decision/reached-handler-record-without-log, decision/reached-record-deduplicated-survives-removal).

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-p4h6jy5b and amd-3fkrlw5u, the coverage role and its line) — `escher-session coverage <app> <check>...` reads caller-named checks as untrusted input under the byte bound. An unusable file is refused `malformed` holding nothing of the path, and no cause is added. The coverage line holds ids, owner ids, event names and two counts, adds no span, and has 0 needles in `host_log`. A stop-aware reached record must leave the line's content class unchanged (decision/coverage-check-files-untrusted, decision/coverage-answer-crossing, decision/coverage-line-no-user-data, contract/cli-roles/coverage, contract/coverage-answer/line).

@history_item n=18 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (amd-wdech32q and amd-rbx77nmf, ratified) — the coverage role's carrier and its line, PROVISIONAL under the overseer as delegate, are ratified by the founder's own word. The handler-coverage rows this chunk extends now stand ratified.

@history_item n=19 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (amd-gmjqtq6k and amd-fd62n3fn, a new verb argument on every carrier) — the most recent precedent for an argument crossing the socket, the command line, MCP and a kept check. The founder ratified it, `contract/recorded-check/tool` was recounted, and the command span records the verb alone (plus `cause` when refused) for non-acting verbs. A `graph` part follows the same pattern (contract/command-span/verb, contract/recorded-check/tool).

@history_item n=20 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (amd-u7vd4egj, mask guard) — the value mask on the graph, actions and layout answers, kept checks and replay is held by `stand_masks_on_new_answers`. A paged graph answer must stay under that guard (decision/mask-holds-on-every-carrier).

@history_item n=21 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (amd-jafaneba, caused text crossing) — each acting verb's answer carries `caused` with authored message and error text, with every other content-named field redacted. `press` is an acting verb, so a Ctrl `press` answer carries `caused` under this rule (decision/caused-text-crossing, decision/socket-carries-screen-reading).

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (amd-gxlry3s2, eight causes) — closed the refusal cause set at eight. A new argument shape (a modifier, a part) must be refused within the set, never by adding a cause (decision/closed-refusal-cause-set, decision/refusal-closed-cause-set).

@history_item n=23 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (amd-3q26qbsw, keyboard activation) — a key reaches the click default actions of any document: Enter on keydown, Space on keyup, gated by a closed element-name match and the `disabled` presence check. A modifier-carrying `press` passes through these same engine arms (decision/enter-activates-focused-control, decision/space-keyup-activates-focused-control, decision/disabled-read-by-presence).

@history_item n=24 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (amd-7rqmidgz, dispatch model) — the merged event-queue model: `isTrusted` by dispatch source, programmatic focus events, and a cancelled pointerup still clicking. This is the dispatch a stop-aware reached record reads, under `packages/blitz-dom/src/events/` (decision/event-is-trusted-by-source, decision/cancelled-pointerup-still-clicks).

@history_item n=25 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (amd-cmzcnw4j, the highlighted reading) — the snapshot and diff gained a fourth closed state, `highlighted`, under the closed-field rule. This is the latest precedent for a closed value inside the ratified carriers, and the readings a paged screen and a Ctrl-built select set surface (decision/snapshot-state-readings, contract/snapshot-text/state-words).
