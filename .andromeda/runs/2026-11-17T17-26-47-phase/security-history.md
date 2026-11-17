@history_item n=1 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — (the range and keyboard-activation rows cite the named functions) a code-audit split moved bodies into private functions with no behaviour change; the rows' claims stood and only their citations followed the code. The direct precedent for this corrective json.rs split: no gate, carrier or log site moves, the added lines carry no unwrap/panic/print/tracing/env read, and the rows citing the moved code are re-pointed.

@history_item n=2 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the script runtime's module split: citations follow the moved code) when `runtime.rs` was split into child modules, 17 citations were re-pointed by hand to the new homes with no claim text changed. Same shape as splitting `packages/escher-driver/src/json.rs` into family children: every security row citing a json.rs line must follow its item to the child module.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the cited lines the chunk's edits moved; the graph verb on the validated surface; the graph answer's size) citations into escher-driver's `json.rs` were re-read after edits; the graph answer (ids, owner ids, component/state/event names, flag words `undeclared`/`unconfirmed`, booleans, never a value or NodeId) is written by `to_json` alone, unpaged under the 1,048,576-byte bound (`stand_graph`, Cells 293,869 bytes). The `graph_node`/`graph_state`/`write_readings` family must keep these bytes and that bound exactly.

@history_item n=4 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions verb's validation and its answer lines; the line locators the chunk's edits moved; the unpaged actions answer's size) the actions answer `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}` is written by json.rs's `action_node` alone, owner and cause left out when absent, unpaged under the answer bound (Cells 106,088 bytes); json.rs locators re-pointed. The `action_node` form moves byte-for-byte and the security rows naming `json.rs`'s `action_node` follow it.

@history_item n=5 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/actions-keyed-field`, `~ contract/actions-answer/keyed`, `~ decision/kept-check-actions-answer` moved to standing (keyed ratified). Fixes the `keyed` boolean as part of the ratified actions answer the split must write unchanged.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session socket carries calls and answers; the command line is an external input surface; ids, names and values leave in an answer) origin of the written forms: the diff written as JSON, `Refusal::to_json` from fixed cause strings, a `SessionError` with a fixed `kind`, one JSON line per command on stdout; the "JSON writer" is among the modules that read no env var and no clock and print nothing. Root of `decision/session-socket-carries-calls`, `decision/no-echo-of-caller-input`, `decision/closed-driver-verb-schema`; the split's new modules must keep the writer env-free, clock-free and print-free.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — a refusal holds nothing of the call: a closed cause set with fixed non-empty texts holding no path, a `Fault` carrying at most the schema's argument name. Source of `decision/refusal-holds-no-call-input` and `decision/refusal-closed-cause-set`; `impl Refusal`'s writer lands in a family module or the parent without any text change.

@history_item n=8 marker=2026-10-12-self-description
**2026-10-12-self-description** — (`help` and the program's fixed text; a session error's remedy and its table) a session error is written `{"error":{"kind","message","remedy"}}` from the fixed `SESSION_ERRORS` table; `differs` joined the diff node keys inside the ratified carrier; the MCP listing is fixed table text. Source of `decision/session-errors-fixed-table` and `contract/diff-node/differs`; `impl SessionError` and the diff writer must keep these key orders and texts.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the MCP server's input and its crossing; `serde_json` reviewed) MCP replies are hand-written, `serde_json` reading only and writing nothing. Grounds the scope's "output only: no serde writer replaces the hand-written one" and `decision/serde-json-read-only` / `decision/serde-json-untrusted-input-reader`; `mcp.rs` and `mcp/framing.rs` are callers of `json::` whose paths must keep resolving.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the screen's pixels in band; the engine's password mask, nine causes, no image in a log, `png`) a shot's line `{"width","height","png"}` with base64 in band, MCP image block then a text block without png; `png` the encoder and no `base64` crate. Rows `decision/screen-pixels-in-band`, `decision/screenshot-in-band`, `decision/no-base64-crate`, `contract/mcp-tool-result/screenshot`; the `shot` / `shot_without_image` forms (the latter called from `describe.rs`) move with their hand-written base64 intact and no dependency added.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a shot of an open list holds labels as pixels; what else of the list crosses the carriers) a hosted shot is pinned equal to an in-process one by `cli_commands`; an open list row crosses only as its own `bounds` in the snapshot and diff, no `expanded` key, `NODE_FIELDS` unchanged. Rows `decision/open-list-shot-pixels`, `decision/select-list-crossing`; `cli_commands` is one of the byte witnesses the split relies on.

@history_item n=12 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (three state readings inside the ratified carriers; the closed-field rule) the diff node carries `selected`/`pressed` only where present and `invalid` always, the text bare words; a new closed-value field inside a ratified carrier is no widening. Rows `decision/snapshot-state-readings`, `contract/diff-node/{selected,pressed,invalid}`, `contract/driver-session-reply/state-readings`; the snapshot/diff family (`NodeParts`, `reading_word`, `node`, `nodes`, `acted`) must keep which keys are always vs conditionally written.

@history_item n=13 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the snapshot's clickable reading) `clickable` written after `focused`, always in a diff node, the bare word only when true in the text; `NODE_FIELDS` fourteen, `Reading::ALL` twelve. Rows `contract/diff-node/clickable`, `contract/snapshot-text/clickable`; key order inside the diff node writer is part of the bytes the split must not move.

@history_item n=14 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — the snapshot has one text form, ids and names written in `str`'s Debug form so no field holds a line break or bare `"`, no JSON form, Display or parser; a password's or file input's value is `MASKED_VALUE`. Rows `contract/snapshot-text/id-field`, `contract/snapshot-text/name-field`, `contract/snapshot-value/masked-value`, `decision/masked-input-values`; the snapshot family's string escaping and mask must stay as written.

@history_item n=15 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: the `lint` verb's findings on every carrier) `Outcome::Linted { findings }` holds ids, closed defect words and numbers only, written by `to_json` alone as `{"findings":[…]}`, never fielded or logged. Rows `decision/lint-findings-crossing`, `decision/lint-findings-closed-content`; the `finding` / `bounds` / `ratio` forms land in a module without gaining a log site.

@history_item n=16 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Outcome::Part { text, part, parts }` written by `to_json` alone as `{"text":…,"part":n,"parts":m}`; a whole-screen MCP snapshot result is the screen text itself. The part answer is a snapshot-family form whose two shapes must survive the move byte for byte.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage role; the coverage line's content and its silence in the logs) the coverage line `{"unexercised":[{"id","owner","event"}],"checks":n,"held":m}` is hand-written by `Coverage::to_json`, owner left out when absent, crossing only as the role's stdout line, no new span. Rows `decision/coverage-answer-crossing`, `decision/coverage-line-no-user-data`, `contract/coverage-answer/line`, `contract/coverage-line/*`; `impl Coverage` moves without changing its keys or gaining a carrier.

@history_item n=18 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — (ratified — the coverage role; the coverage line's content) the coverage role's carrier and line were ratified by the founder's own word. The coverage line form the split relocates is a ratified carrier's answer, not provisional.

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check keeps each answer as the line the driver wrote (a shot as width and height only) with its FNV-1a 64 digest; the two committed checks `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` are read by `Check::from_bytes` and compared byte for byte with a fresh recording (`stand_replay`). Rows `decision/kept-check-answer-whole`, `decision/committed-counter-checks`, `contract/recorded-check/digest`; these are the strongest witnesses that the split moved no written byte.

@history_item n=20 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (wire v3, fifteen session errors) the socket's grammar is `v3` with replies `ok v3 accepted|refused <json>`, `ok v3 oversize`, `ok v3 recording-failed`; `SESSION_ERRORS` at fifteen. Rows `contract/session-socket-reply/*`, `contract/session-socket-request/*`; the session-line writers (`started`, `status`, `stopped`) and the refusal/error forms feed these replies and must keep them.

@history_item n=21 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `~ contract/recorded-check/start-record`: the kept check opens with `{"tool":"start","arguments":{"app":…},"format":1}`; `~ contract/kept-check-file/entry`. `recording.rs` is a caller of `json::`; the opening record's bytes are part of the committed checks the split must leave identical.

@history_item n=22 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `+ decision/caused-answer-crossing`, `+ decision/root-boundary-err-as-screen-content`, `~ contract/driver-answer/settled` (scoped to the busy reading), `~ decision/driver-installs-no-subscriber` / `~ decision/driver-no-subscriber-no-env` (scoped log capture). History of the `caused` family's origin; the split must not add a subscriber or env read to the writer.

@history_item n=23 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `− decision/caused-answer-crossing → decision/caused-text-crossing`, `− decision/root-boundary-err-as-screen-content → decision/root-boundary-err-crossing`, `~ decision/mask-holds-on-every-carrier`, `~ decision/socket-carries-screen-reading`, `~ decision/no-screen-content-in-logs` (a step's caused text reaches no log). The `caused` family (`with_caused`, `caused_item`, `caused_past_item`) writes this ratified widening; moving it must keep item fields, the redaction and no log site.

@history_item n=24 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/caused-text-crossing` (error kept as written within the text bound, every other content-named field redacted), `~ decision/mask-holds-on-every-carrier`, `~ decision/socket-carries-screen-reading` (scrubbed by the answer's own field names), `~ decision/escher-telemetry-allowlist-sink`. Fixes `cut_text`'s bound and the answer's own hidden field set as part of the caused bytes the split must carry unchanged.

@history_item n=25 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ decision/socket-carries-screen-reading` (layout readings), `~ decision/session-layout-read-in-process`, `+ contract/driver-verbs/layout`, `~ decision/select-list-crossing` (the layout answer's select-list placing word), `~ contract/actions-answer/verbs` (every node lists layout with no cause), `+ decision/kept-check-layout-answer`, `~ contract/command-span/verb`. The `layout` family (`placer_word` … `layout_answer`) took these forms in Epoch 5 — the growth the audit's M1 measured — and the actions form's layout entry ties two families together.

@history_item n=26 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — `~ contract/driver-session-reply/no-attribute-text` (a reply carries no attribute text except an author's record message or a caused item's `error`), `~ decision/cold-agent-screen-to-provider` (layout answers carried); and the delegate confirms of the error-field and layout-on-the-carriers cards ratified by the founder. Confirms the caused and layout forms the split moves are ratified, standing crossings.

@history_item n=27 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — `~ decision/upstream-sync-dependency-set`, `+ decision/svg-image-href-fetch`, `− decision/free-disk-only-on-stand-repeat`. The base this chunk builds on (HEAD `d875c3d22`); the scope's "no new crate or dependency, `Cargo.lock` unchanged" is measured against the lock this take left.
