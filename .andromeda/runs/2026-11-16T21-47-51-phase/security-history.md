@history_item n=1 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Threat Model Summary: 4 rows) — retired `decision/caused-answer-crossing` and `decision/root-boundary-err-as-screen-content` and added `decision/caused-text-crossing` and `decision/root-boundary-err-crossing`. Caused now carries each record's message, location, cause chain and fields, and a content-named field reads as redacted. The founder ratified it as a boundary widening («Применить все 14 (Recommended)»). This chunk widens that same row: one more field, `error`, crosses as written. The row's text "a content-named field read as redacted" has to be restated, and by the scope's Touches that is a wrap card for a boundary widening.

@history_item n=2 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Logging & Monitoring: 3 rows) — `~ decision/escher-target-content-scrub` and `~ decision/escher-target-content-fields-withheld` grew the content-named set from eleven to twelve by adding `password`. `~ decision/escher-telemetry-allowlist-sink` says the scoped capture scrubs "by the same content-named set". This chunk splits the answer's set from the sink's, so the except clause ("the same set") becomes false for `error`. The two sink rows must stay unchanged, because `CONTENT_FIELDS` and its readers at `format.rs:107` and `:236` do not move.

@history_item n=3 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Input Validation: 2 rows) — `~ decision/mask-holds-on-every-carrier` gained the except "only a content-named field in it reads as redacted". `~ decision/socket-carries-screen-reading` gained "caused text scrubbed by field name". Both were ratified on the same Q1. Each now names the scrub set the answer used, so each needs a reword once `error` leaves the answer's set. The mask itself is untouched: `MASKED_VALUE` and `password` still read as redacted.

@history_item n=4 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Logging & Monitoring: no-screen-content-in-logs) — added "a step's caused text" to what reaches no log at any level. The guard is the founder check that reads the sink's lines at trace. This chunk's acceptance control rests on that check. The app record's error text must still occur 0 times on the sink's lines, and the engine record's `error=[redacted]` must still appear.

@history_item n=5 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Threat Model Summary: 2 rows) — added the first caused crossing (closed words and counts) and the root-boundary Err as screen content only. The text was left to a follow-up. This is the lineage that `caused-text-crossing` replaced, and the precedent that each step in widening caused's content is its own ratified crossing.

@history_item n=6 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Input Validation: 2 rows; API Security: 1 row) — `~ decision/driver-installs-no-subscriber` and `~ decision/driver-no-subscriber-no-env` gained the except "an acting step runs under escher-telemetry's scoped log capture, which prints nothing". `~ contract/driver-answer/settled` scoped the class-only promise to the busy reading. The capture this chunk edits (`capture.rs`, `Values::keep`) must keep printing nothing. Only what it keeps for the answer changes.

@history_item n=7 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set the original content-named redaction set at any target, with `error` among `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request` and `panic.payload`, and the seven engine-safe fields. This is where the `error` field was first classed as content to withhold. The sink keeps that classification and only the answer drops it.

@history_item n=8 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — set the sink's three outcomes: engine targets print only safe fields, `escher_` targets print with the content-named fields redacted, and any other target is dropped whole at every level. This explains the scope's premise correction: an app record's line does not exist on the sink (`format.rs:99`), so the control can only read an engine record's `error=[redacted]`. Trap carried from it: a "nothing in the logs" check must run against a host that installs the sink.

@history_item n=9 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (what a sink-installing host logs) — trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. The chunk's sink-unchanged control (`stand_act_error_text.rs`, the founder check over an in-memory writer) has to install escher's sink for its 0-occurrence and `error=[redacted]` readings to mean anything.

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (where a failed load's error text stops) — recorded that a failed load's text stops in `ResourceLoadResponse` and in the `Resource load failed` line, which no carrier takes. On that line the sink prints only `waiting_nodes`, with `url` and `error` in the scrub set. This chunk's acceptance control is that same engine warn (`document.rs:1844-1849`). The sink side of the row must still hold, and the answer side now carries the error text.

@history_item n=11 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches the handler) — `NetHandler::error` is a default that does nothing, and `blitz-net`'s error arm calls it. Trap: a do-nothing default can hide a failure behind a wrapper. This bears on which real failures can reach a record with `error = %e` that a step's capture sees (the picked row `decision/net-handler-error-default-noop`).

@history_item n=12 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Error Handling) — `+ contract/blitz-net-provider-error/file-forbidden`: a fixed display text holding no URL or path, with no log site of its own. When a fetch error's Display reaches the newly opened `error` field, this variant stays path-free. Other error texts may hold a URL, which is the founder's named price («a failed fetch's URL»).

@history_item n=13 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — set the closed-field rule (rows `decision/boundary-widening-rule` and `decision/closed-value-field-no-widening`). A new field of a closed value set inside a ratified carrier is no widening, but a field holding authored text still is. The `error` field's text is authored, open text, so opening it in caused is a widening that needs the founder's word. The scope carries that word («Открыть error в caused») with its wrap card.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — set the rule that the program's own fixed `'static` table text crosses on ratified carriers (row `decision/mcp-listing-fixed-text`) and that `help` and the MCP listing serve each result field's help. This is the precedent for rewording `caused`'s help at `schema.rs:193-195` as fixed program text inside ratified carriers, not a new crossing.

@history_item n=15 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the MCP crossing with its bounds (`decision/mcp-result-crossing`, `decision/mcp-bounded-lines`, the 1,048,576-byte result bound) and the `tool_call` span, whose fields lie outside `CONTENT_FIELDS`. Standing: the next carrier is a new question. This chunk adds no carrier, so the error text rides the existing MCP result under the same bound.

@history_item n=16 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the session socket carries calls and answers (`decision/session-socket-carries-calls`) under the 16,384-byte request and 1,048,576-byte answer bounds, and no ending echoes caller input (`decision/no-echo-of-caller-input`). The opened `error` text crosses in answers on that carrier, inside the unchanged bounds. `TEXT_CUT` (1024 bytes) and the per-record and per-capture caps keep it bounded.

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check keeps each answer whole at rest (`decision/kept-check-answer-whole`), masked as on every carrier, with the committed `counter-cli.json` and `counter-mcp.json` byte-compared with a fresh recording. An acting step's caused `error` text now lands at rest in a kept check's answer. A committed check that holds a caused item with an `error` field would change bytes.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — a kept check is owner-only (`0700`/`0600`), and a recorded masked `type` is kept as `MASKED_STAND_IN` (rows `decision/recorded-type-masked`, `contract/recorded-check/masked-stand-in`). This is the at-rest medium the unscrubbed `error` text now reaches. The masked stand-in is untouched.

@history_item n=19 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — set the committed statement `docs/capabilities.md` of the MCP crossing's carried set (`decision/capability-statement-mcp-crossing`) and the secret search over it. If the statement names caused's scrub, it has to follow when `error` crosses as written.

@history_item n=20 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (and its ratification **2026-10-09-founder-rulings**) — set one command span of fixed words and counts (`decision/command-span-content-free`), and the sink's closed-span lines, where a content-named span field's value is never stored (`decision/closed-span-lines`, the `format.rs:236` span-field store). That reader stays on `CONTENT_FIELDS`, so the scope's "sink unchanged" includes spans: no `error` span field value may start being stored.

@history_item n=21 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (Logging & Monitoring) — the previous rewording of `decision/no-screen-content-in-logs`, extending it to in-process reads. It shows that the row is amended by appending to its list. This chunk changes no log line, so that row should need no change.

@history_item n=22 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — a password's or file input's value reads as `MASKED_VALUE` and never as the text or the host path (`decision/masked-input-values`, `decision/password-file-value-masked`, `contract/snapshot-value/masked-value`), while the DOM keeps the path. Only an author's own `error` text can carry a value in caused. `password` and `path` stay redacted in the answer's set.
