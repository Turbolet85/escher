@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** (amd-7v2ajbq2, the allowlist scrub) — set the sink's scrub sets: `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload` redacted everywhere. This chunk's graph content (component, handler and state names, node ids) has no name in that set, so nothing may rest on the scrub to hide it. It must never become a field at all.

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (amd-vm35icer, a second sink installer) — made `escher-session` a sink-installing binary and recorded the trap that a zero read from a host that installs no sink says nothing about one that does. This is why the graph read's needle proof has to run through `host_log` / `mcp_log` (sink-installing). In-process checks with no subscriber cannot give it.

@history_item n=3 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (amd-yjzlnn3o) — added the two prefix sets: `escher_` targets print every field except the content-named ones, and every other target is dropped whole at every level. This is the rule the cascade's binding leans on: dioxus_core / dioxus_signals records are dropped. Also set the standing rule that a target is admitted only together with a scrub rule for it, so the graph reader must add no new target.

@history_item n=4 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (amd-pbgglsyj) — brought the stand checks' shared module `tests/blitz-tests/tests/common/mod.rs` into the no-subscriber census. Precedent for the extract's constraint that new graph checks and any shared helper they read join the census by name.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (amd-e5lmdcaj) — the schema (`validate`, `VERBS`) is silent. Rule: never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. A new graph verb in `VERBS` and any argument it takes come under this rule.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (amd-3iczuidb) — `Outcome` joined the `Debug`-deriving types nothing prints, logs or fields. The rule is kept for the span's owner: never field an `Outcome` with `Debug`. The new graph `Outcome` variant inherits this rule.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (amd-cschmkve, the command span as built) — built the `command` span: target `escher_driver`, INFO, `info_span!`, eight fields recorded only where they apply, plus a never-fielded list (any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal`, `Fault`). `tracing` became ungated in escher-driver, and the workspace build turns `blitz-dom/tracing` on. This is the source of the "eight fields" and "both builds" constraints.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (amd-6zxxoq74, the closed span's line) — added the closed-span line as a sink record class. Every pair of a closed span is judged by the scrub rule, and non-content-named values are held in memory and printed. So any graph field on `command` / `tool_call` would print under the `escher_` rule.

@history_item n=9 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** (amd-u2nw2mep) — the founder ratified the closed span's line. He also ruled that a diff's three sizes may reach a log while its content never does. This is the precedent the graph follows: counts at most, never content, and here not even counts, since a read verb records `verb` alone.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (amd-dnjkg5io) — the command line writes each answer as one line of JSON on stdout, an answer and never a log line, and `Outcome` is written in one place by `to_json`. Standing rule: the next surface adds no span or log field that holds what a call supplied or a screen reads. This binds the graph verb's carriers directly.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (amd-gftbr5vi, typed text measured in a host's log) — first measured `host_log`: the host at `RUST_LOG=trace`, under both builds, reads 0 typed-sentinel, id and name needles. This is the harness and the reading that the graph read's needle acceptance extends.

@history_item n=12 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (amd-2fbb2riy) — the snapshot gained three state readings as closed words and booleans. Neither the answer nor `Outcome`'s `Debug` is a log line, and no span field moved. Precedent for the «reacts to a click» closed boolean snapshot field.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (amd-3jlpr7el) — the founder ratified the `tool_call` span: `tool` (the table's word) and `ending` only, with the `command` child by nesting. `stand_mcp_tools`' in-process recorder and `mcp_log` at `trace` were added. A new graph MCP tool adds one `tool` word and no field, and its call parents one `command`.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** (amd-hk2vxiq4) — `help` joined both spans' closed sets with no field added. `describe()` writes no record, and the self-description is an answer, never a log line. The snapshot field moves `describe()` and the help texts, which must stay fixed text reaching no field.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (amd-wivka2yb) — precedent for a new verb: `screenshot` joined `verb` / `tool` with no field. An accepted shot records `verb` alone, the never-fielded list gained the image, and `host_log` / `mcp_log` read 0 content bytes on stderr under both builds.

@history_item n=16 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (amd-yvceabad) — the cold-agent pipe reads escher's `server.log` and counts ` span="tool_call" ` lines (`span_counts`). It is a reader of the ratified line and not an emitter. A new `tool` word grows what the pipe counts.

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (amd-wggssyt7) — an `advance` tool call ends `accepted` and parents one `command` child, read by `stand_mcp_tools`, and the span keeps its two fields. This is the pattern for pinning the new graph tool's `tool_call` → `command` parenting.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (amd-rbvxics4) — the recording opens no span and logs nothing. The kept check is a file at rest, never a log, in a `0700` directory, local only. A recording that ended ends a `tool_call` `error` with two fields. The new verb's recorded step must follow this.

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (amd-6x37n7n2) — the recorder keeps the answer whole as `answer`, masked as on every carrier. The two committed Counter checks are written by `record_checks.sh` and read by `stand_replay` and the freshness check. If the reacts-to-a-click field moves the Counter's snapshot text or diffs, those checks must be re-recorded under the same rules.

@history_item n=20 marker=2026-10-19-test-splits
**2026-10-19-test-splits** (amd-y2gzdcnt) — seven_guis' shared `tests/common/mod.rs` states `Needles`, `stand_needles`, `found` and `occurrences` once for `host_log` / `mcp_log`, and its assertion messages are a kind and a count, never a needle. This is where new graph needles (component, handler and state names) go, under the same message discipline.

@history_item n=21 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (amd-lqyenybp) — `tests/blitz-tests/tests/fixtures/mod.rs` joined the silence census (no print, `tracing`, subscriber, env read, socket or listener) and its readers were counted. New graph checks that read it extend that count.

@history_item n=22 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (amd-trvpoib3) — the lint's `Finding` / `Reason` derive `Debug`, and nothing logs, fields or prints one. The no-new-field rule was first set PROVISIONAL here and later ratified. Precedent for the graph's content types.

@history_item n=23 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (amd-vrhzfubf, the gated part: the `lint` verb) — the closest precedent for this chunk. A new instance verb went into `VERBS` (twelve words, eight instance-level), and its `Outcome::Linted` is written by `to_json` alone. An accepted `lint` records `verb` alone, a `lint` tool call parents one `command`, and `host_log` / `mcp_log` each drive one call with 0 needles. Also F2 «Без поля»: the founder's own word for no new span field.

@history_item n=24 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (amd-4q2wobhh) — a part read (`Outcome::Part`) records `verb` alone. dioxus-native-dom's `Snapshot::text_parts` opens no span and records no field. `stand_act_spans` counts its calls per mode. Precedent for the graph reader in dioxus-native-dom and for the read-verb `verb`-alone question the extract leaves to implementation.

@history_item n=25 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (amd-a6pxq634) — `stand_act_spans` went to 36 calls per mode, with part reads' ids and texts among the needles and none on a line. A recording session's clients are read at `trace`. If the graph verb joins `stand_act_spans`, its call count moves.

@history_item n=26 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (amd-bcuh7k4j) — `docs/capabilities.md` restates the escher-driver spans and their closed-word fields. Its pin reads none of those claims, so a change to the span set is made there by hand. A new verb word in `command` / `tool_call` has to be checked against it.

@history_item n=27 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (amd-6reehbne) — the pipe's `span_counts` counts accepted `tool_call` lines per counted verb (`lint` · `screenshot` · `advance`), and `capability-unused` fires on an unused counted verb. A new graph tool word bears on whether it becomes a counted verb (obs ↔ arch binding).

@history_item n=28 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (amd-zogtm42c, the served stand's silence) — the silence census reads 0 over `stand.rs`, `flight_booker.rs`, `stand_cells_served`, `stand_load_page` and the shared modules (fct-qu4fsnfx). `host_log`'s fourth test reads a hosted part read, 0 needles under both builds. Moving the stand's signals onto the registry must keep this census at 0.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-w5hr2k4x, the `fixtures` module's readers) — `fixtures/mod.rs` readers went to 28, and the §3 `session_common` count was left stale. These are counts the new graph checks extend, with a known gap in them.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-hv3i7j3n) — added `option_needles` to seven_guis' shared module, plus new `host_log` / `mcp_log` tests reading page text at `trace` with 0 needles. `host_log` tests are `#[cfg(unix)]`, and `mcp_log`'s records nothing, to avoid a `cfg!(unix)` site. This is the template for graph-content needle tests.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-2iwce5bg) — help texts and `covered`'s meaning are fixed `'static` text reaching no field, and `command` keeps eight fields (`stand_act_spans`). dioxus-native-dom readers were recorded as instrumentation n/a. The snapshot field's help-text move and the new dioxus-native-dom readers follow this.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-7g47dwps) — the select checks were recorded silent. `session_common` readers went to thirty-two and `stand_act_*` to sixteen. The census row form is the one the graph checks' row follows.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-tjocwgx3) — a row's `bounds` crossing the snapshot and diff inside a closed set adds no field or carrier, under the field rule of 2026-10-10. `SelectListRow.label` is page text in a read type, never logged. Precedent for the closed boolean field (not a widening) and for the graph's names held in read types.

@history_item n=34 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (amd-q5gqadnt) — the latest restatement of the standing counts this chunk re-asserts: escher-driver holds 2 `info_span!` sites, `stand_act_spans` its eight fields, and `host_log` / `mcp_log` read 0 needles at `trace` under both builds. The `flush_queued_mounted_events` guard was recorded as opening no span.

@history_item n=35 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (amd-qokfz7t5, the silence row) — `dioxus_runtime_probe` was recorded silent in log-coverage alone, with no §3 row. Its census pattern included `tracing_subscriber|bind\(`. This is the form a non-`stand_` graph target's silence row takes.

@history_item n=36 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (amd-f6nw3lvy, read values stay in the test) — the probe reads components, listeners and reader scopes, and no read value leaves it. Its failure messages name only mode, task/page and link/index. The entry names the UI graph (values masked, never logged) as building on this, which sets the graph checks' failure-message discipline.

@history_item n=37 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (amd-qrj3ym2p) — `MutationBuffer`: crate-private, holds ids, names, values and text in process only, `Debug` prints `len` alone, and nothing logs or fields it. The replay re-fires `MutationWriter`'s `trace!` calls. This is the holder pattern the signal registry matches, and it locates the existing content-carrying `trace!` sites the graph walk must not add to.

@history_item n=38 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (amd-kdvlpiyf) — the probe's renamed `…_spawned_acts_*` pins and `spawned_focus_acts` were recorded silent with the census over `dioxus_runtime_probe.rs`. The graph's checks may sit beside or build on that file.

@history_item n=39 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** (amd-jrf2exdt) — the dioxus-native-dom bridge's `trace!` count was fixed at 13 in `mutation_writer.rs`, and the snapshot module was recorded as holding no log site. The new graph and registry modules in the same crate must leave that count unmoved.
