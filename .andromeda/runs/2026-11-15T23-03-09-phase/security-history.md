@history_item n=1 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added the `CLI arguments (agent-run.sh)` row: a verb outside `boot run status cleanup logs`, a wrong argument count or a selection other than `stand`, `all` or an existing `^[a-z0-9_]+$` blitz-tests file stem prints usage and exits 2 before any cargo call. It also made the agent-run events hold harness metadata only and kept `target/agent-run/run.log` raw, never printed and gitignored. These are the source of decision/agent-run-selection-allowlist, contract/agent-run-cli/2, decision/agent-run-events-metadata-only and decision/agent-run-raw-log-unprinted. A smoke verb or selection added to agent-run.sh widens this closed argv and must keep the exit-2-before-cargo gate and the metadata-only events. Its "Kept" note says no port, socket or env var.

@history_item n=2 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — precedent for a script that drives the session binary. It added a `CLI arguments (cold-agent.sh)` row (verb allowlist, task allowlist, usage exit 2 before any precondition) and a log of counts and identities only. A new script argument boundary was recorded as an input row first. The cold-agent pipe is out of this chunk's scope, but its row is the template for any new argv the smoke takes.

@history_item n=3 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — its precondition is an executable `target/debug/escher-session` (exit 3), and the pipe builds nothing. Its fixed prompt names no element id or app slug. This is the closest precedent for a harness entry point that boots `escher-session`. The smoke differs because it builds the binary itself, so how it builds it (locked, see `2026-10-05-fork-ci-reached`) is new.

@history_item n=4 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — added the `CLI arguments (apt-install.sh)` row for a CI script's arguments, with usage exit 2 before anything runs and a pinning unittest over bad calls. It sets the rule that §Input Validation lists every script argument boundary row by row. A new smoke verb's argv needs the same row and a pin in the CI scripts' unittest leg, the shape the scope names for `test_agent_run.py`.

@history_item n=5 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — `--locked` covers every ci.yml cargo leg beside the flake and `dx bundle`, with `examples/wasm_hello` excepted. This is the origin of decision/locked-builds. A smoke that builds `escher-session` runs a cargo build, and that build must pass `--locked`.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — two relevant changes:
- The `embolden` entry added a locked cargo leg to `ci-leg.sh` in `LEGS`/`FAST_LEGS`, its place pinned by a check. This is the precedent if the plan finds an existing leg must run the smoke.
- The no-served-port entry registered two test-only loopback TCP binds and kept the driver session's Unix socket as the one listener (decision/session-socket-sole-listener, absence/served-network-api). The smoke must add no listener or port.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — registered the driver session socket as a ratified boundary widening:
- The socket is local IPC with no handshake, token or peer check (decision/driver-session-no-peer-authentication).
- The state directory is created `0700`, a group- or other-open one is refused (`StateDirNotPrivate`), and the socket is `0600` (decision/state-dir-owner-only, decision/session-socket-owner-only).
- The label is 1–32 bytes of `a-z0-9-`, checked before boot.
- `stop` removes the socket file and the directory and nothing else (decision/state-dir-deletes-only-own).
- The library and binary read no env var beyond `RUST_LOG`.

The same marker's sink entry carries a trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. A smoke asserting log silence must read a sink-installing host.

@history_item n=8 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — a sink-installing host drops outside-target records whole at every level, so no id or name prints at any `RUST_LOG`. This is the basis of decision/accessible-names-not-logged and decision/stable-id-not-logged. Its trap repeats that the check must run against a host that installs the sink (`host_log` is that check). Any smoke log reading inherits it.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the session socket and the command line became external input surfaces (ratified):
- The host role is `serve <task> --session <dir>`. `start` re-runs the binary through `current_exe()` with its streams closed and neither kills nor reaps the host.
- The idle expiry is 1,800 s. Trap: a watcher polling `attach` or `status` keeps a host alive.
- Requests take a closed grammar, a 16,384-byte request, a 1,048,576-byte answer and a 30 s answer read.
- The argv reader is closed, with `--session` exactly once and four endings (status 0/1/2/3). No ending echoes what was typed.
- Stdout is one JSON line per command. Usage and session-error messages go to stderr.
- Every surface passes `validate` first.

This defines the exit grammar the smoke reports under, and its lifecycle (start → call → stop).

@history_item n=10 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — every driver call is checked against a closed verb table with bounded argument kinds, one answer in a fixed order, and a refused call changes nothing. A refusal is a closed cause set with fixed texts holding no path. This is the origin of decision/closed-driver-verb-schema, decision/no-panic-on-call-input and decision/refusal-holds-no-call-input. Rule: keep refusal texts fixed and free of call content. The smoke drives existing verbs and must add none.

@history_item n=11 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the executor runs only commands that came from `validate` (decision/one-validation-path-for-every-carrier). An id naming no element is refused with nothing run. This is the basis of the unknown-target rows the smoke's refused-call step would exercise.

@history_item n=12 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — refusals are detected after `validate`: `stale`/`not-found` for an unknown id, then `disabled`, `off-screen`, `covered` in that order, with nothing run. This is the origin of decision/target-refusal-order, decision/unknown-id-refused-stale-or-not-found and contract/driver-id-refusal/*. It applies if the smoke runs a refused call, as `counter.sh` does.

@history_item n=13 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — one `command` span per call holds fixed words and counts only. Of a refusal, only its cause name is logged. This is the origin of decision/command-span-content-free, decision/refusal-log-cause-name-only and decision/driver-outcomes-never-logged. The smoke's events and verdict must not carry call content, answers or refusal texts.

@history_item n=14 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified that a diff's three sizes may reach a log and its content never may (decision/diff-counts-only-in-logs). Standing rule: a field holding a node, id, name or value is a new crossing question. This applies to anything the smoke's verdict line records.

@history_item n=15 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — stated the closed-field rule: a new closed-value field inside a ratified carrier is no widening, while authored text, an open value set, a new carrier or a new exit is (decision/boundary-widening-rule, decision/closed-value-field-no-widening). This is the yardstick for any field the smoke's JSON-line verdict adds.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP stdio server became the third validated carrier, binding nothing, with the `id` echo as its one caller echo. A second entry under this marker added the `tool_call` span and states that the role's stdout holds protocol lines only. This is the source of the MCP clauses in decision/one-validation-path-for-every-carrier, decision/accessible-name-exits and the exception in decision/session-output-holds-no-caller-input. It is relevant if the smoke drives the `mcp` role instead of the command line.

@history_item n=17 marker=2026-10-12-self-description
**2026-10-12-self-description** — two relevant changes:
- `StateDirTooLong` states its bound (94 bytes on Linux, 90 elsewhere) with no path and gives a remedy (contract/session-errors/state-dir-too-long, decision/state-directory-long-path-refused). The scope's relative `target/tmp/…` state directory follows from this bound.
- The session-error object gained `remedy`, all from one fixed table (decision/session-errors-fixed-table), and diff nodes gained `differs` (contract/diff-node/differs).

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — measured the off-Linux 90-byte state-dir bound on the macOS leg: 91 bytes ends `state-dir-too-long` with status 3 and no path. This confirms both bounds behind the scope's relative state-dir choice.

@history_item n=19 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the 30 s answer bound and the closed pipe are pinned by checks (contract/driver-command-line/answer-bounds). `TMPDIR` is read only by escher-driver's client unit test, and the library reads nothing beyond `RUST_LOG` (decision/driver-reads-only-rust-log). A smoke using `$TMPDIR` or any other env read would have to be registered.

@history_item n=20 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — `TMPDIR` is read at a second test-only site. The environment-values list names every read, test code included. A harness smoke that reads env must join that list.

@history_item n=21 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — `RUST_LOG` was registered as the one runtime env read (contract/environment-variables/rust-log). The smoke's session runs under it, and contract/escher-session-output/accepted-stderr expects empty stderr when it is unset.

@history_item n=22 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a settle reports a load in flight at once and never waits on one, bounded at 64 passes (decision/settle-never-waits-on-loads). Rule: a command that waits on a load with a clock is a new decision. This is relevant to what the smoke's call may wait on.

@history_item n=23 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — an acting command waits on a load for at most `LOAD_WAIT` (5 s) under the 30 s answer read, then answers `settled: false` (decision/bounded-load-wait, decision/load-wait-bound, contract/driver-answer/settled). The smoke's call can rely on that bound rather than a wait of its own.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the refusal cause set narrowed to eight (`time-unavailable` retired), and `advance` is accepted on every session (decision/closed-refusal-cause-set, decision/time-moves-only-on-advance, contract/driver-argument-kinds/milliseconds).

@history_item n=25 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` is allowed at most once on `start`, the host role and `mcp`, and is usage elsewhere. A refused record directory gives status 3 with nothing spawned (contract/cli-roles/serve). A second entry under this marker moved the wire to `v3` (`hello v3`, `stop v3`, `call v3 …`) and made session errors 15. It also records that `CARGO_MANIFEST_DIR` is read at build time by escher-driver tests for the workspace `target/tmp`. This is the grammar behind contract/session-socket-request/* and the precedent for a `target/tmp` state directory.

@history_item n=26 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the committed checks are written by `record_checks.sh` (with a `0700` `rec/`) and compared byte for byte. That script stands beside `counter.sh`/`flight.sh`, which the scope weighs reusing. Committed checks are the one exception to owner-only and must hold no typed text, host path or secret.

@history_item n=27 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — `main` hands `command_line` a closed six-slug `TASKS` table (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`). The stand boots offline except for the load page's in-process `LoadNet`. The smoke "boots a session on a named stand task" only from this closed table.

@history_item n=28 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — seven_guis `host_log` is the standing check that reads `start … --record`, snapshot, part, lint and stop clients at `RUST_LOG=trace` with 0 needles. It guards decision/no-screen-content-in-logs. A smoke over the same binary should not duplicate or weaken that guard.

@history_item n=29 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added the `part` argument kind (1–1000) and the `--part` CLI flag (contract/driver-argument-kinds/part, contract/cli-flags/part). A part's number, count and text reach no log.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — several relevant changes:
- `graph` became the thirteenth verb. It takes no argument, an argument is refused `malformed` with status 1, and it is answered as one JSON line on the CLI. It crosses as a ratified widening (contract/driver-verbs/graph, contract/graph-answer/nodes-states).
- The unpaged graph answer stays under the 1,048,576-byte bound (decision/graph-answer-unpaged).
- `clickable` joined the diff node (contract/diff-node/clickable) as a closed value inside the carrier.

@history_item n=31 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actions` became the fourteenth verb. It takes no argument, is refused `malformed` with status 1 on the CLI, and its answer is `{"nodes":[{"id","owner","keyed","verbs":[…]}]}` (contract/driver-verbs/actions, contract/actions-answer/*). The answer is unpaged under the bound (decision/actions-answer-unpaged). Its log silence is checked by `host_log` and `mcp_log`.

@history_item n=32 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — dropped the PROVISIONAL mark from the actions answer's `keyed` field (contract/actions-answer/keyed now reads only "A boolean on each listed node").

@history_item n=33 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added the `coverage <app> <check>...` role with no `--session` and no socket. An unreadable file or a non-check is refused `malformed` (status 1) holding no path, and a failed boot gives status 3. The coverage line is the stdout content for exit 0, and the role adds no span. This is why contract/escher-session-exit-codes/0 and /1 name the coverage role. It is another existing entry that drives the binary, and a candidate for what the smoke reuses or must not disturb.

@history_item n=34 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role's command-line carrier and its stdout line (previously PROVISIONAL, delegate overseer). The exit-code rows that cite the coverage role are now settled.

@history_item n=35 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `~ decision/driver-session-unix-socket-only`: the socket exists on unix only, while an in-process session runs on linux, windows and macOS. A smoke driving `escher-session` through its socket is unix-only (decision/session-socket-unix-only) and cannot run on the windows leg.

@history_item n=36 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — `+ decision/fixture-name-closed-table`: a fixture state name is checked against the app's closed table before any session starts. `+ decision/reach-error-closed-set`: reach errors have fixed texts naming no tried name. This applies if the smoke boots on a named fixture state.

@history_item n=37 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the security master became rows: each picked decision/contract handle comes from this conversion, verified against the code. Earlier prose entries above explain those rows. Amendments since then name rows by handle.
