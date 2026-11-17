@history_item n=1 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — OTel export out of escher: no OTel crate, egress or credential path (the source of `decision/no-otel-export`). Why it matters: the split adds no crate, dependency or export path; `Cargo.lock` stays unchanged, so the row holds.

@history_item n=2 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — recorded that `Command`, `Call` and `ArgValue` derive `Debug` but are never printed, logged or fielded, and that a cause is fielded by `Cause::name` alone. This is the root of `decision/debug-types-never-logged` and the refusal-cause rows. Why it matters: the family modules that write refusals and causes must keep writing closed words and must add no `Debug` print or field.

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Outcome` joined the `Debug`-deriving types that nothing prints, logs or fields. Standing rule: never field a `Command`, `Call`, `ArgValue` or `Outcome` with `Debug`. Why it matters: every `impl Outcome`, `impl Refusal` and `impl SessionError` writer that moves into a family module carries this constraint.

@history_item n=4 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — built the `command` span with its eight fields and its never-fielded list (the screen text, any argument, `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal`, `Fault`). Also marked the closed span's line PROVISIONAL. Why it matters: this is the source of `contract/command-span/field-count` and `decision/driver-spans-carry-no-content`. The writer split must leave both spans and their fields untouched; the scope's no-behaviour-change boundary rests on this.

@history_item n=5 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line was ratified. The founder ruled that a diff's three sizes may reach a log while its content never does. Why it matters: the diff family's writer feeds only the answer. The `added`/`removed`/`changed` counts stay span fields, set by the executor and not by the writer.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the command line writes each answer, refusal or error object as one line of JSON on stdout, an answer and never a log line, and "an `Outcome` is written in one place, by `Outcome::to_json`". A sub-entry measured typed text absent from a host's log at `trace` under both builds. This is the origin of `contract/escher-session-stdout/command-answer`, the exit-code rows, `decision/debug-types-never-logged` and `decision/sink-host-read-at-trace`. Why it matters: splitting `json.rs` must keep one writer per outcome, callable from one place, not duplicated across families. `cli.rs`'s callers must keep resolving.

@history_item n=7 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `tool_call` span with two closed fields, `tool` and `ending`, ratified. `serve_mcp`'s replies are a further stdout write, never a log line. Why it matters: this is the source of `decision/tool-call-span-two-closed-fields` and `contract/escher-session-mcp/stdout`. `mcp.rs` and `mcp/framing.rs` call into `json` (`Object`, `list`, `string`, `shot_without_image`), and those paths must still resolve after the split, with the MCP JSON line byte-identical.

@history_item n=8 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined the closed sets. The session-error object gained a fixed `remedy`. The module `describe` installs nothing and prints nothing. Why it matters: this is the source of `contract/escher-session-error-object/remedy` and `contract/escher-session-commands/help`. The `SessionError` writer and `describe.rs`'s use of `json` move with the split while the `help` self-description stays byte-identical, as the scope requires.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `Outcome::Shot` is written by `to_json` and the MCP image block only. A shot's image leaves only in its answer: 0 image bytes on stderr under both builds. Why it matters: this is the source of `contract/command-answer/shot` and `decision/shot-image-leaves-only-in-answer`. `shot` and `shot_without_image` are unnamed forms whose landing P4 settles, and they must keep the image-then-JSON-line order over MCP.

@history_item n=10 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe's two replay children run `escher-session mcp` without `--record`, with stderr to `/dev/null`. Why it matters: this is the source of `decision/replay-children-uninstrumented`. No writer change may alter what those uninstrumented children answer.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording opens no span and logs nothing. A kept check is a file at rest, never a log line, and the wire gained the fixed `recording-failed` reply. Why it matters: this is the source of `contract/kept-check/arguments`, `contract/kept-check/accepted` and `contract/host-wire/recording-failed`. `recording.rs` calls the writer, and its kept forms must not move a byte.

@history_item n=12 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorder writes the answer itself, a shot's without `png`, with its digest. The committed checks `tests/blitz-tests/checks/counter-{cli,mcp}.json` are read by `stand_replay`. Why it matters: this is the source of `contract/kept-check/answer`, `contract/kept-check/answer-digest` and `contract/recording-file/answer`. The committed checks' kept answers and FNV-1a digests are the byte-for-byte witnesses the scope names for an unchanged writer.

@history_item n=13 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — a test-only move of escher-driver's `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs` was recorded silent: the census reads 0 print, tracing and subscriber sites, and escher-driver still holds 2 `info_span!` sites. Why it matters: this is the precedent for moving `json.rs`'s inline `mod tests` into a `json_tests.rs` or per-family files, and for the obs amendment such a move draws: a silence census over the new files and an unchanged `info_span!` count.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — `Outcome::Linted` is written by `to_json` alone as `{"findings":[…]}`, never logged or fielded. A lint's line holds no finding. Why it matters: this is the source of `contract/command-answer/linted` and `contract/command-span/lint`. The lint forms (`finding`, `bounds`, `ratio`) are unnamed families whose landing P4 settles.

@history_item n=15 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read records `verb` alone and never fields the part number or the count. `Outcome::Part` is never logged. Why it matters: this is the source of `decision/part-answer-never-logged` and `contract/command-span/snapshot-part`. The part answer `{"text":…,"part":n,"parts":m}` belongs to the snapshot family and must keep its bytes.

@history_item n=16 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — a complexity factoring done with private items only (no public symbol, verb, field, carrier, env var or log site) was recorded as one log-coverage `observed absent` row backed by a silence census over the changed files. Why it matters: this is the closest precedent for this corrective, no-behaviour-change split and for the shape of its expected obs amendment.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — a `multiple` select's chosen set leaves only as form entries, as the snapshot's `value` (the first chosen option) and as tree nodes, never as a log line. Why it matters: this is the source of `contract/snapshot/multiple-select-value` and `decision/select-chosen-set-never-logged`. These are forms of the snapshot node writer that the split moves.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — an open list's row crosses the snapshot and the diff by its own `bounds`, with `differs` naming `bounds` within its closed set. A shot of an open list carries labels as pixels, ratified. Why it matters: this is the source of `contract/snapshot-diff/open-list-row-bounds` and `decision/open-list-shot-carries-labels`. Both are written by the snapshot and diff family the split creates.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — `Outcome::Graphed` is written by `to_json` alone as `{"nodes":[…],"states":[…]}`, never logged. The graph answer's carried set is ratified, and the snapshot node gained the `clickable` key after `focused`. Why it matters: this is the source of `contract/command-answer/graphed`, `contract/graph-answer/*`, `decision/graph-answer-never-logged`, `contract/snapshot-node/clickable` and `contract/snapshot-text/clickable`. The graph forms (`graph_node`, `graph_state`, `write_readings`) and the node writer's key order must stay as they are.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the actions answer is written by `to_json` alone as `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, with `owner` and `cause` left out when absent, ratified. A sub-entry re-pointed the cited line ranges in `json.rs` after added lines. Why it matters: this is the source of `contract/actions-answer/*` and `contract/command-span/actions`. `action_node` must keep its omission rules, and the json.rs line citations are the kind the split's moved files will re-point.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — `Coverage::to_json` writes the coverage line by the crate's hand writer with `serde_json` writing nothing, as a command-line answer never logged. A sub-entry re-pointed `json.rs` and `cli.rs` citations. Why it matters: this is the source of `contract/coverage-line/*`, `contract/coverage-role-stdout/coverage-line`, `contract/escher-session-coverage/*` and `decision/coverage-line-never-logged`. `impl Coverage` is an unnamed form whose landing P4 settles. The hand writer stays output-only, with no serde writer.

@history_item n=22 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows: a master holds what a planner must respect, and the code holds the rest. Why it matters: obs rows now name contracts by handle and not by `json.rs` line. A pure move should change no row, and any pointer to `json.rs` that follows the files is leaf work, not a row amendment.

@history_item n=23 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added `decision/exploration-adds-no-span`. Also extended `contract/escher-session-coverage/malformed` to refuse a check format other than 1. Why it matters: both are picked rows. The split must leave the explored calls' answers and the coverage role's refusals as they are.

@history_item n=24 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the graph answer's carried set and the coverage role, line and command-span reach, which had been PROVISIONAL. Why it matters: the graph and coverage forms the split moves now rest on ratified rows, so any byte change to them would reopen a founder ruling.

@history_item n=25 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added `decision/layout-read-opens-no-span`. `stand_layout` installs its own recorder, an exception to `decision/headless-stand-no-sink`. Why it matters: the `layout` answer family the split creates writes an answer that the read produces with no span, and the split must add none.

@history_item n=26 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — the driver sets escher-telemetry's scoped capture around an acting step. `decision/driver-library-installs-no-subscriber`, `decision/headless-stand-no-sink` and `decision/driver-emits-no-event` were reworded around it. Why it matters: the `caused` answer form grew out of this capture, and the writer split must keep the driver library sink-free and event-free.

@history_item n=27 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — the caused answer carries each record's text. `decision/answers-echo-nothing-typed` gained the caused-item exception, `decision/scrub-by-field-name` replaced the sink-only scrub, and `contract/telemetry-scrub-set/url` and `error` gained the capture clause. Why it matters: this epoch's `caused`, `cut_text` and error-text forms in `json.rs` came from this chunk, and they are the code audit's M1 growth the split targets. The `caused` family must keep its text bound and its `[redacted]` writing.

@history_item n=28 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — a caused item's `error` field crosses as written, cut at the text bound. The capture judges by the answer's own hidden set, and `decision/answers-echo-nothing-typed` and `decision/scrub-by-field-name` were reworded. Why it matters: these are the current rules the `caused` family module (`with_caused`, `cut_text`, `caused_item`, `caused_past_item`) implements, so the move must keep the hidden set and the cut identical.

@history_item n=29 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added `contract/layout-answer/content` and `logging`, and `contract/command-span/layout`. The verb table went to fifteen words, and the needle set of `decision/sink-host-read-at-trace` gained the layout answer's keys (`laid_out`, `placed`, `shifts`). Why it matters: the `layout` family (`placer_word` … `layout_answer`) writes these keys, and the host and mcp trace reads guard that none of them reaches a log.

@history_item n=30 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — `contract/layout-answer/logging` now names the cold-agent transcript as a carrier, and the two delegate-confirmed cards (the error field in caused, `layout --id`) were ratified by the founder. Why it matters: this is the last change to the layout and caused rows the split rests on. Both are now the founder's own word, so the split must change none of their bytes.
