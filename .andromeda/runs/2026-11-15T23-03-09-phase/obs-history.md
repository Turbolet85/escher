@history_item n=1 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** (the agent-run harness log) — created the agent-run harness log contract the smoke extends: `scripts/agent-run.sh` writes python3-`json` lines (`boot`, `run.start`, `test`, `run.end` printed and appended to `target/agent-run/events.jsonl`; `status`, `cleanup` printed only), harness metadata and not a telemetry sink, nothing read inside a `failures:` block, raw output to `run.log`, state local and uploaded by no CI leg. A smoke verb or verdict line must keep every one of these: python-json encoding, no content-named key, no captured output, `run.log` unprinted, cleanup scoped to its own state.

@history_item n=2 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** (the cold-agent pipe log, scrub position and artifact) — the precedent for a second harness beside agent-run: its own python3-`json` events and verdict, no `tracing`/OTel/env read, raw transcript and client stderr never printed, state local, gitignored, removed by its own `cleanup`. A smoke with its own JSON-line verdict follows the same shape and lands in §3/§6/§8/§9 the same way.

@history_item n=3 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the cold-agent pipe log on the escher server) — the one existing harness that spawns `escher-session` itself: it sets `RUST_LOG=info` on that child only, sends its span lines to a never-printed `server.log` it only counts, sends replay children's stderr to `/dev/null`, and the pipe itself installs no subscriber and reads no env var. Precedent and trap for the smoke: if it reads the binary's stderr it must do so as a count into a private file, and a `RUST_LOG` it sets is a child-only setting, not a harness env read.

@history_item n=4 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the cold-agent pipe's verdict fields, its live child's recording and the six apps' carriage) — fixes how a harness spawns the session binary (`exec env RUST_LOG=info <abs target/debug/escher-session> mcp --record …`), owner-only state (`mkdir -m 700`), verdict fields as counts, booleans and closed words only, read from span lines. The smoke's verdict line falls under the same rule: closed words and counts, no screen content.

@history_item n=5 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence and the host-sink part read measured) — set the six stand task slugs (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`) in the usage line and the `start` tool; `contract/escher-session-exit-codes/2` builds its usage line from them. The smoke's "named stand task" is one of these six.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (a second sink installer; the scrub's reach scoped to the measurement) — made `escher-session` the second sink installer (`decision/sink-installed-first-in-main`, `decision/scrub-sink-reach`), scoped "a headless boot has no sink" to in-process boots (`decision/headless-stand-no-sink`), and added the session state directory row (socket file only, local, no CI upload: `decision/session-state-dir-private`). Trap recorded: a zero read from a host that installs no sink says nothing about one that does — the smoke boots the real binary, under its sink.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (outside-target records dropped; both sink-installing binaries measured by level) — the `escher-session` by-level stderr readings (0 lines unset, 1 at `info` …), stdout 0 bytes from the sink, `RUST_LOG` filter default `warn` (`contract/telemetry-sink/rust-log`, `contract/escher-sink-env/rust-log`, `contract/telemetry-sink/stream`), and the `hl-trace` state dir. Basis for the smoke expecting empty stderr with `RUST_LOG` unset (`decision/empty-stderr-check-unsets-rust-log`); rule: a target is admitted only with a scrub rule.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — established `escher-session`'s command line: each answer, refusal or error object one JSON line on stdout, an answer never a log line (`contract/escher-session-stdout/command-answer`), endings and exit statuses 0 · 1 · 2 (`contract/escher-session-exit-codes/*`), the host's command span per hosted call, and the workspace-build engine line (`decision/engine-tracing-by-invocation`). Standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads — the smoke adds none.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log; the session host by level under both builds) — the host's stderr by level under the package-alone and the workspace build (0 lines unset), typed sentinel 0 at `trace`, and the enumerated check state directories (`ss-*`, `hb-*`, `hl-*`, `cc-*`, `cf-*`), each empty after `stop` or idle expiry. A smoke state dir under `target/tmp/…` joins this list (`decision/session-state-dir-private`, `decision/session-logs-carry-no-page-content`).

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built; `tracing` ungated in escher-driver) — escher-driver takes `tracing` ungated (`decision/escher-crates-always-trace`, `decision/escher-crates-tracing-ungated`), the `command` span's eight fields, and the by-invocation finding that a workspace build turns engine tracing on (`decision/engine-tracing-by-invocation`). The smoke's build mode (package-alone vs workspace) decides whether engine lines can appear on the binary's stderr at `info`.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the closed span's line, PROVISIONAL; a span's fields under the scrub; typed text read in process) — defined the closed-span line shape and that no line prints at the default `warn` (`contract/command-span/line`, `contract/escher-driver-span-lines/default-level`), plus the exception of re-run children installing the sink over an in-memory capture (`decision/headless-stand-no-sink` except). The smoke at default `RUST_LOG` sees no span line.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** (the closed span's line ratified) — ratified the closed-span line as a second record class; any smoke reading of span lines rests on this ratified shape, and adding a record class would be a new widening for the founder.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the `tool_call` span, ratified; the `mcp` role on the sink) — the `mcp` role: JSON-RPC lines only on stdout, the sink stderr-only, role endings and usage (`contract/mcp-role-stdout/json-rpc`, `contract/mcp-role-exit-status/*`), `mcp_log` reading the role at `trace` (`decision/sink-host-read-at-trace`). Binds the smoke if it drives the call through the `mcp` role rather than `start`/`call`.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` in the two spans' closed sets) — `escher-session`'s endings gained `help` (0 · 1 · 2), the session-error object a fixed `remedy`, the usage line `help`; the self-description is an answer on stdout, never a log line (`decision/escher-session-fixed-diagnostics`). Part of the exit grammar the smoke's verdict maps.

@history_item n=15 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (recording opens no span and logs nothing; the recording-directory row) — wire `v3` with the fixed `recording-failed` reply (`contract/host-wire/v3`, `contract/host-wire/recording-failed`), the refused-directory status-3 endings (`contract/escher-session-commands/open-recording-directory`, `contract/escher-session-exit-codes/3`) and the local-only Recording directory row (`decision/local-state-never-uploaded`). The smoke's session uses the v3 wire and, if it records, an owner-only directory.

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the committed checks) — the Committed checks row: the two Counter checks written by `record_checks.sh` beside the flows, public, no log line, no CI upload (`decision/committed-kept-checks-clean`, `decision/committed-checks-hold-no-private-data`). If the smoke reuses `counter.sh`/`record_checks.sh` or commits anything, it stays a scripted stand-Counter session with no typed text, host path or secret.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role, a further reach of the same sink install) — the `escher-session` install row: every role (host, mcp, coverage) is a further reach of the same sink install in the same `main`, answers on stdout never a log line. A smoke verb is a caller of the binary, not a new role, and must add no reach.

@history_item n=18 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's endings gain the coverage role) — the latest change to `escher-session`'s usage line and endings (status 0 · 1 · 2 · 3, `malformed` holding nothing of the path); the exit grammar the smoke reports under is this table (`decision/fixed-diagnostics`).

@history_item n=19 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — escher-session's coverage role, a further reach of the same sink install) — ratified the coverage-role install row (was PROVISIONAL); the row the smoke rests on is now the founder's word.

@history_item n=20 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — escher-session's endings gain the coverage role) — ratified the endings/usage item (was PROVISIONAL); the exit grammar the smoke maps is settled.

@history_item n=21 marker=2026-10-19-test-splits
**2026-10-19-test-splits** (the log checks' shared fragment and its silence census) — the log checks' assertion messages name a kind and a count, never a needle or a captured line (`decision/log-check-failures-name-kind-count`). A smoke failure reason follows the same rule: no captured output, no needle.

@history_item n=22 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** (the re-exec child's captured ids) — origin of the `decision/checks-silent` exception for re-run children printing what the parent reads; any smoke check that captures the binary's stdout reads it into assertions only, never a log.

@history_item n=23 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (3. Observability Harness Contract) — `~ decision/checks-silent` except widened to "ids or digests"; the latest edit of a picked row the smoke's contract test sits under (checks install no subscriber, read no env var, print nothing).

@history_item n=24 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (the headless-stand census covers the checks' shared module) — the silence census names shared modules too (`decision/checks-silent` "their shared modules"); a helper module the smoke's test adds must be read silent as well.

@history_item n=25 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (the headless stand installs none) — origin of `decision/headless-stand-no-sink`: in-process boots install no sink, read no env var. The smoke boots through the binary, so it runs under the sink, not this rule.

@history_item n=26 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (no image in a log) — a shot's image leaves only in its answer (`decision/shot-image-leaves-only-in-answer`); if the smoke's call is a `screenshot`, its verdict carries no image byte.

@history_item n=27 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (a part read's span and its silence) — a part read's answer and counts are never fielded or logged (`decision/part-answer-never-logged`); a smoke `snapshot` call keeps its answer out of the verdict and any log.

@history_item n=28 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (the record among the silent values) — the session's record of ids: bounded, no reader outside the crate, no `Debug`, never printed (`decision/session-id-record-unprinted`); the smoke reaches none of it.

@history_item n=29 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the stderr readings) — a client's own stderr and the host were read by build and level, 0 needles; the smoke's `call` clients are such clients (`decision/session-logs-carry-no-page-content`).

@history_item n=30 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (the workspace host's engine line printed twice) — trap: the workspace-built host prints an engine INFO line (twice) that the package-alone build does not; the smoke's stderr expectation must state its build (`decision/engine-tracing-by-invocation`).

@history_item n=31 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (the recording clients read) — `host_log`'s standing check of clients' stderr at `trace`, and the state directories gaining `hl-client`/`hl-client-rec`; precedent that a new state dir name is recorded in the §9 row.

@history_item n=32 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** (OTel export out of escher 0.1.0) — `decision/no-otel-export`: no OTel crate, egress or credential path; the smoke's "no new env read or credential path" boundary rests on it.

@history_item n=33 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (per-leg CI failure logs) — per-leg CI logs in `target/ci-logs/`, uploaded on failure only; if the smoke's contract test runs in the existing unittest leg, its output lands there and adds no artifact (a new upload is a founder widening).

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cold-agent pipe's verbs take graph) — precedent for a harness's contract test in `.github/scripts/test_cold_agent.py` with a fake server, recorded in §6; the smoke's unittest-leg contract test (a `cargo` shim, like `test_agent_run.py`) is recorded the same way.

@history_item n=35 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the cold-agent pipe's VERBS fourteen, actions uncounted) — the latest harness verb-table change and its §6 recording: a harness verb set grows with its contract test and adds no verdict field unless counted; the agent-run verb set (`boot run status cleanup logs`) gaining a smoke verb follows this pattern.

@history_item n=36 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** (the masters as rows) — turned every area into rows; all the picked rows (agent-run events, escher-session exit codes, sink contracts) took their present form here, so a smoke amendment at the wrap is a `+`/`~` row edit, not prose.
