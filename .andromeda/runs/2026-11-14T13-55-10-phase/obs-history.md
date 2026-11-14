@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — escher-telemetry: stderr subscriber, chaining panic hook, allowlist scrub. Set the chaining hook (logs its ERROR `panic` event at `escher_telemetry::panic`, then runs the previous hook, so std still prints the raw message) — the row contract/escher-telemetry-panic-hook/previous-hook. Trap for this chunk: exploration catches panics inside a step; under a sink-installing binary each caught panic still fires the hook (ERROR line, `panic.payload` redacted, raw message on stderr), so where the command runs decides what a caught panic writes.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — a headless boot made in process installs no subscriber (row decision/headless-stand-no-sink). An in-process exploration and its checks must stay sink-free.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the one `println!` allowed in the stand checks is a re-executed child whose stdout the parent reads into assertions only; precedent if a panic-fixture or replay proof needs a fresh-process child.

@history_item n=4 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings — OTel export out of escher 0.1.0** — no OTel crate, egress or credential path (absence/telemetry-backend); exploration's report adds no backend or exporter.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session — a second sink installer** — `escher-session` became a sink-installing binary; trap recorded: a zero read from a host that installs no sink says nothing about one that does — an in-process silence reading of exploration does not prove a binary role's log.

@history_item n=6 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records dropped whole at every level (row decision/sink-target-allowlist); rule for later chunks: a target is admitted only together with a scrub rule for it. Any exploration span must sit at the `escher_driver` target or bring a scrub rule.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the span's cause field takes a `Cause` name; never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name (decision/debug-types-never-logged). Exploration reports refusals by closed cause and must keep that rule in its report and any span.

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the executor `Session::run` silent; `Outcome` joins the `Debug`-deriving types nothing prints, logs or fields. Rule kept for the span's owner: field a cause by `Cause::name`, never a `Command`, `Call`, `ArgValue` or `Outcome` with `Debug` — exploration runs every step through `Session::run`.

@history_item n=9 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the session's record of ids is bounded, no reader outside the crate, no `Debug`, never printed/logged/fielded (decision/session-id-record-unprinted); the cause domain fully returned by code. Exploration re-reaching paths from fresh sessions must not expose that record.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans — the driver's command span as built** — one INFO `command` span at `escher_driver` per `Session::run` call, eight fields recorded only where they apply, never-fielded list (contract/escher-driver-command-span/*, contract/command-span/*, decision/driver-spans-carry-no-content); `tracing` ungated in escher-driver (decision/escher-crates-always-trace). Each explored step writes its own `command` span; an exploration span, if any, holds fixed words and counts only.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the closed span's line, PROVISIONAL) — closed-span line format and scrub of span fields (contract/escher-log-line/span-on-close-only); later ratified.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings — the closed span's line ratified** — decision/closed-span-lines ratified as a boundary widening; diff sizes (`added`/`removed`/`changed`) may reach a log, content never. Any count exploration fields stands on this ruling's limits.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli — the session host writes the command span; the command line writes answers, not logs** — command line answers are one JSON line on stdout, never logs (contract/escher-session-stdout/command-answer, exit codes 0-3); standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads. Binds exploration's report on any carrier.

@history_item n=14 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log) — `host_log` proved a typed sentinel absent from a sink-installing host at `trace` under both builds; if exploration drives `type` through a binary, the same needle reading is the precedent.

@history_item n=15 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface — the `tool_call` span, ratified** — a new log field is a boundary widening needing the founder's ratification; `tool_call` fields `tool`/`ending` closed words (decision/driver-span-fields-print). Bears on the carrier fork: an MCP exploration tool joins `tool`'s closed set.

@history_item n=16 marker=2026-10-12-self-description
**2026-10-12-self-description** — a session-level verb records its table word beside `unknown-verb` in `command`; `help` added to closed sets with no field added; endings and the error object's fixed `remedy` (contract/escher-session-error-object/remedy). Precedent for adding a word to the verb/tool sets without a field.

@history_item n=17 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a shot's image leaves only in its answer, never fielded or logged (decision/shot-image-leaves-only-in-answer, contract/command-answer/shot); `fonts-unavailable` cause. Exploration holds no `screenshot` step and no pixel at rest.

@history_item n=18 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `busy` driven on a real line by `stand_act_spans` (contract/command-span/busy); exploration's re-keying fixtures would record it the same way.

@history_item n=19 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe's replay children write stderr to `/dev/null` (decision/replay-children-uninstrumented); `check.json` held stand ids and texts, never printed.

@history_item n=20 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — cause set eight (`time-unavailable` retired); an accepted `advance` records `verb` and the acting fields; no time fielded. Defines the refusal-cause rows exploration reports by.

@history_item n=21 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts — the PROVISIONAL batch ratified** — the eight cause names and the `advance` readings ratified by the founder (contract/refusal-cause/*).

@history_item n=22 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span and logs nothing; the Recording directory row (`0700`, one `0600` check per session, call content at rest, local only); `mcp --record` refused open dir status 3; wire `v3` `recording-failed` (decision/recording-dir-private, decision/recording-bounds, contract/record-refusals/record-dir-not-private, contract/recording-file/*, contract/mcp-role-exit-status/3, contract/host-wire/*). Exploration's owner-only kept checks extend this register.

@history_item n=23 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — `Check::replay` silent, each replayed step writes its own `command` span; the recorder keeps the answer whole (a shot's without `png`) (contract/kept-check/*, contract/recording-file/answer); Committed checks row. Exploration's paths must replay green through `Check::replay`; committing any would be a new widening.

@history_item n=24 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — `recording_tests.rs` / `recording_off_unix_tests.rs` silent; recording unix only; host_log/mcp_log needle helpers. Exploration's recording inherits the unix-only split.

@history_item n=25 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the stand checks' `fixtures` module silent; the panicking and refusing-control fixtures this chunk needs land in or beside it under the same census.

@history_item n=26 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint — the visual lint recorded silent and spanless** — an in-process `Session::lint` with no verb, no new field (PROVISIONAL then); precedent for an in-process-only capability on the carrier fork.

@history_item n=27 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — a new verb added to the table rides the `command` span with no field, the founder's «Без поля»; the template if exploration becomes a verb.

@history_item n=28 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the founder's word dropped the lint's PROVISIONAL «no new field»; confirms no-new-field as standing for new driver surfaces.

@history_item n=29 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — `passes` on a step that waited on a load is the settle that went quiet (contract/command-span/passes); the load wait opens no span. Bears on navigation steps exploration declines.

@history_item n=30 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read records `verb` alone; a part's answer never logged (decision/part-answer-never-logged, decision/hosted-call-through-session-run).

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and scrub limits, its pin reading none, so a change is made there by hand (decision/capability-statement-updated-by-hand); a kept check holds a lint's findings at rest. Any new span/verb from exploration must be hand-edited into the statement.

@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar — the cold-agent pipe's verdict fields, its live child's recording** — `record/` owner-only via `mkdir -m 700`, replay children without `--record` (decision/cold-agent-record-owner-only, decision/replay-children-uninstrumented); no live kept check committed.

@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence) — the stand's six served tasks and their boots (`boot_cells`, `boot_load`, `LoadNet`) read silent; exploration is proven on these served tasks.

@history_item n=34 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `file:` refusal in the script error surface) — origin of decision/no-file-loads-from-non-file-documents; bears on the navigation boundary (exploration loads nothing new).

@history_item n=35 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the stand_actions check is silent) — `session_common` readers 34, `actions` module and `actionable_nodes` silent; exploration's check will move the reader count again.

@history_item n=36 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set) — the actions answer carries ids, owner ids, verb words, cause words, booleans only; at rest in the recording directory; ratified widening (contract/actions-answer/*). Exploration reads this answer to enumerate actions.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the cold-agent pipe's VERBS fourteen) — the pipe's `VERBS` copies must follow any new verb; uncounted verbs add no verdict field.

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actions joins the command span's verbs and the tool_call span's tools) — verb table fourteen words (contract/command-span/verb, contract/command-span/actions); template for counts that move if exploration becomes a verb or tool.

@history_item n=39 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read opens no span and adds no site) — `Session::actions` changes nothing in the document (decision/actions-read-changes-nothing), reached inside the call's `command` span.

@history_item n=40 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb read at 0 needles) — `stand_act_spans` 38 calls per mode; `host_log`/`mcp_log` needle readings; the measures an exploration surface would extend.

@history_item n=41 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role, a further reach of the same sink install) — the nearest precedent: a binary role replaying kept checks, adding no span, its line an answer; ratified widening, the edits PROVISIONAL (delegate overseer). Directly bears on the carrier fork.

@history_item n=42 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the session_common module's readers gain stand_handler_coverage) — readers 35; exploration's check moves the count.

@history_item n=43 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (host_log's paged-part test is its fifth) — trap: adding a `host_log` test ahead of others shifts per-test ordinals stated in four sections.

@history_item n=44 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's endings gain the coverage role) — role shape, bounded read, refusals `malformed` holding nothing of the path, usage clause; template for an exploration role's endings and its untrusted budget/directory refusals (decision/escher-session-fixed-diagnostics).

@history_item n=45 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the command span's reach gains the coverage role) — each replayed step writes its own `command` line, the role adding no span (decision/one-span-per-command); PROVISIONAL edits.

@history_item n=46 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage module, the reached record and their check log nothing) — the silence census pattern over a new module and check; the coverage line never logged (decision/coverage-line-never-logged).

@history_item n=47 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (host_log's coverage run reads 0 needles at trace) — a binary role measured at `trace` under both builds with path and kept-check needles; precedent if exploration lands on a binary carrier.

@history_item n=48 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage line's carried fields) — a new answer's carried set is a boundary widening needing ratification (PROVISIONAL here); exploration's closed report needs the same.

@history_item n=49 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows; every picked row's current form dates from this conversion.

@history_item n=50 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/no-file-loads-from-non-file-documents`: engine sub-resource refusal with fixed text, no log site; the current row this chunk's navigation boundary rests beside.

@history_item n=51 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` except: a re-run child prints ids or digests its parent reads; exploration reads `DEFAULT_SEED` alone and its checks stay within this exception.
