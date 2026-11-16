@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — the stand log line and escher-telemetry's scrub tests, with `telemetry_panic_hook` among the one-process-per-behaviour files: the chaining panic hook and the scrub this chunk must hold unchanged while catching a handler panic on the run path already have checks of their own.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, with no skip path; TimerTicks as the stand's time stand-in. The fixture app and the new target must follow the no-skip, bundled-font rule.

@history_item n=3 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — the agent-run five-command contract (selections `stand` · `all` · `{name}`, events carrying no captured test output). A new non-`stand_` target is reached by `run all` / `run {name}` alone, and no panic text may reach an event.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** (trap) — a deleting key press is not the same on every platform (the Backspace arm is compiled out on macOS). Any key the fixture's panicking or logging handler is driven by must read the same on every CI OS.

@history_item n=5 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the bridge tests sit in a `cfg(test)` child module with no constructor or `pub` widened; `tests/blitz-tests` keeps one target per behaviour beside shared modules that are no target. These are the origins of the `bridge-tests-without-widening` and `one-test-target-per-behaviour` rows. A test of handler attribution in the bridge must keep both.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — opens the Session lifecycle key and the process-check rules. Traps: an `#[ignore]` host child blocks a bare `--ignored` run; a child's inherited libtest lines are counted as tests; a sink-installing host at `trace` overruns an undrained pipe. A panicking handler on a host process meets all three.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` covers a sink-installing host at `trace` with needles read from the stand, and `telemetry_drop` proves third-party targets dropped. Traps: a check that must fail red on a leaking sink cannot use `start` on a piped host; a needle list read from the stand can outgrow a hand-kept one. This is the template for proving no log line carries the new answer content.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `Session::act` returns settled or `NotSettled(Busy)` with the instance still readable, not rolled back; settle neither advances time nor drains the changed set. A caught handler panic must fit this outcome model (the step not rolled back). The changed set is non-empty on a fresh harness, so drain it first.

@history_item n=9 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the schema's pins: every table stated in its test with its row count asserted, an assertion over a call carrying a row index only, and each pin under a mutation control. A new result field enters only as the schema tables admit, and its pins follow the row-index-only rule.

@history_item n=10 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the `stand_act_*` files drive `Session::run` in both layout modes. Rule set: a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. The new behaviour's checks inherit it.

@history_item n=11 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — refusal order `disabled` · `off-screen` · `covered`, `stale` / `not-found` lookup, and refused calls leaving the instance unchanged. A refused acting call runs no handler, so it must carry no caused error.

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — `stand_act_spans`: one `command` span line per call, fixed words and counts only, with typed text measured at 0 in process. Traps: a check reading a sink's capture counts lines by target, never in total; a green per-package run is not the workspace leg's green, because engine `tracing` is on in the workspace build only. Under the scrub-held boundary, the span gains no field.

@history_item n=13 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed-span second record class in the stand log was ratified by the founder (the origin of the `closed-spans-logged-as-lines` row). This is one of the log rulings that bound what of a log may enter an answer.

@history_item n=14 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — `BlitzTestsTargetsTest` pins one auto-discovered target per file, with upstream's `all` held at `test = false`. The new target needs no manifest line, and none may be added.

@history_item n=15 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (coverage scope) — introduced `cli_commands`, `cli_flow`, `host_timer` and the seven_guis shared module (stream drain, client runner, `Started` guard). `host_log` reads a typed sentinel 0 times over hosted calls. These are the command-line carrier checks the new field must reach.

@history_item n=16 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (test strategy and harness contract) — added a command-line and shell-flow test level and three host children. Session-wire `v2` states what crosses, with `call` and its bounds, ratified by the founder. Any new answer content on the socket is measured against this crossing.

@history_item n=17 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (unit, integration, e2e and CI) — the schema is driven over the socket, through the binary and in process, and typed text is measured on the session host's stderr under both builds. The new field needs the same carrier coverage.

@history_item n=18 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the MCP checks (`mcp_server`, `mcp_log`, `stand_mcp_tools`) and the ratified `tool_call` line (`tool`, `ending` only). MCP stdio-child checks are not unix-gated. Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This applies directly to any per-step capture layer on the subscriber.

@history_item n=19 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` / `describe()` and its count row, `help` being the same bytes on CLI and MCP, and the remedies pinned. The scope requires `help` and the MCP listing to describe the new field, which moves these pins.

@history_item n=20 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (harness and session contracts) — a shot's PNG crosses in band in its answer line, a widening ratified by the founder's fork. A shot's log line holds no image byte. This is the precedent for new answer content on every carrier with nothing of it in the log.

@history_item n=21 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — coverage floors (escher-driver 97 %) and a zero-failure flakiness budget over the stand-repeat leg; `stand_act_spans` reads a not-settled call. A caught-panic path added to escher-driver must hold the floor and repeat green.

@history_item n=22 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the harness re-exports `NamedKey`) — the harness re-exports keyboard-types' `Key`, `Modifiers` and `NamedKey` (the `harness-reexports-key-types` row). A fixture driven by key presses names keys through these re-exports.

@history_item n=23 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (`advance` on every session, eight causes) — `advance` is accepted on every session and moves the clock by exactly `ms`. The causes are eight in order, `fonts-unavailable` last (`the_eight_causes_are_named_in_order`). `advance` is an acting verb whose answer gains the field, and no ninth cause may enter unless the schema admits it.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (clock checks) — `virtual_clock_engine` and `stand_virtual_clock` (the `one-virtual-clock` row's guards); `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep in a session check. The new checks move time by `advance` alone.

@history_item n=25 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the one-virtual-clock marks: `advanced_ms` exactly `ms`, the eight causes in order, and the clock moved on every session. The `advance-*` and `eight-refusal-causes-in-order` rows rest on this.

@history_item n=26 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (lifecycle contract) — `run` records each answered instance call as `tool` · `arguments` · `masked` · `accepted` · `digest`; wire `v3`; fifteen session errors. A check that records is `cfg(unix)`-gated item by item beside an off-unix `Unsupported` check. Whether the new field enters a record or its digest rests on this.

@history_item n=27 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (tests and counts) — `stand_session_recording` and seven_guis `recording`; the recorded `host_log` / `mcp_log` runs read 0 recording needles. The kept-check carrier's existing checks are the ones the new field must pass.

@history_item n=28 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the kept answer and replay) — each record keeps `answer` (the kept line as an object), and `Check::replay` stops at the first differing acceptance or kept answer. The committed checks are data under `tests/blitz-tests/checks/`, replayed by `stand_replay`. This is the scope's P4 point: a new answer field joins the kept answer and changes replay equality.

@history_item n=29 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (counts and the shot's same-host limit) — `the_committed_checks_are_what_the_binary_records` reads both committed checks byte-equal to the binary's recording. A shot's PNG differs across OSes, so a byte-compared check holds no `screenshot` step. A new answer field forces both committed checks to be re-recorded.

@history_item n=30 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — added `tests/fixtures/mod.rs` (`boot`, `hold`, shared pages, `HeldNet`) as a third shared module. The test-data bootstrap was measured: fixtures are code with no seeding, and a page two checks share goes in `fixtures`. This places the scope's fixture app beside the stand.

@history_item n=31 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting steps wait on a load up to `LOAD_WAIT` with no time moved, `NotSettled(Loads)` at the bound (`load-wait-bound`, `driver-session/load-wait`). A wall-time read is allowed only in a check whose subject is wall time. A step's capture window spans this wait.

@history_item n=32 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` named targets) — `stand-repeat` runs the `stand_*` glob plus a fixed literal list of named targets, pinned by `test_stand_repeat_leg_runs_three_rounds`. A new target without the `stand_` prefix enters the leg only by being named there.

@history_item n=33 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (0.2.0 record and recording clients' stderr) — `host_log` reads 0 needles of each class over a recording session's clients at `trace`, and `stand_act_spans` drives a part read with its ids joining the needles. This is the needle-extension pattern for new answer content.

@history_item n=34 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — added `CapabilityStatementTest`: verbs and causes as code spans, counts in words read from the schema's tables, and claims mapped to their checks. A renamed or moved check changes the claim section by hand. A new answer field or claim moves `docs/capabilities.md` and possibly a count.

@history_item n=35 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served pages' boots and checks) — the stand boots Cells and the load page through dedicated boots, and `LoadNet` is the one network stand-in an offline stand allows (the `stand-boots-offline` row's exception). This is the precedent for a fixture or served page beside the lean tasks.

@history_item n=36 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' keyboard walk) — a `MountedData` call in a task spawned from a key handler panicked with `RefCell already borrowed`. This was the first measured handler-originated panic under a session.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list through the process checks) — `host_log` / `mcp_log` each gained a test with `option_needles()` read from a snapshot, reading 0 at `trace`. This is the precedent for adding a needle family when new answer content appears.

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — an engine panic at `position_in_containing_block` on any focused control was reachable by a remote page, and is now retired with its check. It precedes the scope's distinction: a caught panic here is the app handler's, not the engine's or the driver's.

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — `CountedHandler` forwards `NetHandler::error`, so a failed load ends the wait. Trap: a trait's do-nothing default compiles silently through a wrapper, so sweep a trait's implementers when extending it. This applies to any capture wrapper around handler or log paths.

@history_item n=40 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (target, legs and pages) — `dioxus_runtime_probe` reads Dioxus 0.7.10 public API in both layout modes, its hazard pinned by `should_panic` tests. It runs in the workspace `test` leg and `run all`, not `stand-repeat`, `run stand` or a11y. It measures the handler-identity links (component, handlers, effect) the answer's handler tie rests on.

@history_item n=41 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured) — the handler-spawned task's panic mechanism: `poll` holds `borrow_mut` across `render_immediate`. A precondition is asserted first by `hazard_ready`. This is the precedent for pinning a handler panic in a test.

@history_item n=42 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (listeners and a held signal's readers) — the bridge's listener attributes over-report, while the vdom walk's dynamic attributes read current listeners exactly (the `listeners-read-from-vdom-walk` row). Attributing a caused error to a handler must read that walk.

@history_item n=43 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the probe's two pins) — the `should_panic` pins were inverted to `…_spawned_acts_*`, a mutation control showing them as the discriminating witness. A handler-spawned task's timing on Dioxus 0.7 (polled inside `render_immediate`) decides which step a panic or log lands in.

@history_item n=44 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (hazard closed) — the buffered render retires the `RefCell` panic on every Dioxus document. The fixture's panicking handler must not lean on that retired path.

@history_item n=45 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's limit) — a task polled during render acts on the previous render's document, its limit owned by a residual. This bounds which records a step can attribute to nodes created in the same render.

@history_item n=46 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom unit scope and test-only shim) — the bridge's one `cfg(test)` shim, `DioxusDocument::dispatch_to_vdom`, and the dispatch record, with no function gaining a constructor or `pub` (the `bridge-tests-without-widening` row's exception). Any new bridge test reuses this shim rather than widening.

@history_item n=47 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a graph call logs its verb alone) — `graph` was added as a closed word with no span field. `host_log` / `mcp_log` read 0 graph needles, using `graph_needles()`. This is the direct precedent for the `command-span/graph` and `command-span/verb` rows, and the model for a no-new-span-field change.

@history_item n=48 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph held in session and over MCP in process) — `stand_graph`: every graph node a snapshot node, no `{owner}///{key}` id, no value or typed text in the answer, an argument refused. The handler identity (owner id, event name) the scope ties a caused error to is this graph's.

@history_item n=49 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the command line and MCP answer the graph) — `cli_commands` and `mcp_server` each gained a graph test, with the listing order `graph` after `lint`. This shows the per-carrier checks a new answer shape needs.

@history_item n=50 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (stand checks' shared modules) — `GRAPH_DRIVES` in `tests/common/mod.rs`, and `graph()` / `graphed()` in `session_common`. Handler identity read in the new checks comes through these.

@history_item n=51 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit-test groups) — `graph::tests` and `signal_registry::tests`: a handler's write is recorded against its dispatch, an entry's `Debug` prints counts alone, and a write by an element since gone is not handed to its ancestor. This is the dispatch-to-handler attribution precedent.

@history_item n=52 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the driver's verb-table pins) — the field-kind table moved, along with describe's count row and `NODE_FIELDS`. A new result field moves `FieldKind`, describe's row and the schema pins.

@history_item n=53 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the session contract and the socket) — `graph` is a reading verb that waits on nothing. A graph answer crosses the socket carrying names only, never a value or typed text, as a boundary widening ratified by the founder (`session-wire/graph-answer`). Authored text in an answer is exactly what that rule guards.

@history_item n=54 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (one actionable rule) — both actionable readers read one rule from the current listeners, never from the `data-dioxus-id` mark, which outlives a removed listener (the `one-actionable-rule` row). A handler lookup must not read the mark.

@history_item n=55 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (stand_actions) — the actions answer holds no value: a typed sentinel and `MASKED_VALUE` occur 0 times, an argument is refused `malformed` on every carrier, and a read changes nothing (the `actions-answer-holds-no-value` and `actions-*` rows). This is the security pattern for proving a new answer field leaks no input.

@history_item n=56 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actions through the command line and MCP) — `cli_commands`, `mcp_server` and `stand_mcp_tools` gained `actions` through a shared read-verb helper (the `cli-actions/*`, `mcp-actions/argument` and `command-span/actions` rows). These are the carrier checks the scope names.

@history_item n=57 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions call in the span and log checks) — `host_log` / `mcp_log` each drive one `actions` call, with `actions_needles()`, and `stand_act_spans` reads 38 calls per mode. Its counts move when acting calls answer differently.

@history_item n=58 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (verb table at fourteen) — `VERBS` 14 and describe's row `[14, 14, 12, 3, 8, 4, 15]`, with the field-kind table keyed per verb and the `amended` help rows. A new acting-verb result field lands in these tables.

@history_item n=59 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (a kept actions step's replay, owed a test) — "holds by construction" through `Check::replay` was ruled insufficient, so the replay test was owed. The new field's kept-check carrier needs a measured replay test, not a by-construction claim.

@history_item n=60 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record's unit tests) — `reached::tests`: a bubbling dispatch records its ancestor's listener, a handler whose run removes its element is recorded by its id, and `Debug` prints counts alone. This is the reached record the scope names as a candidate attribution mechanism.

@history_item n=61 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage) — a refused step exercises no handler, while a keyboard activation exercises its click (`handler-coverage/refused-step`); a handler removing its element still reads exercised. `Check::replay_each` is a crate-private hook beside `Check::replay`. This is the closest precedent for tying a step to the handler it ran.

@history_item n=62 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (host_log's coverage run, cli_commands) — `host_log`'s coverage run reads 0 id, name, sentinel and path needles at `trace` under both builds, using `coverage_needles()`. The coverage role's usage and refusals are pinned in `cli_commands`.

@history_item n=63 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stated limits) — a bubbling dispatch counts every ancestor listener it passed, while `stopPropagation` is unread (owned by a residual). Attributing a caused log to "the handler" inherits this limit.

@history_item n=64 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the kept actions step's replay, measured) — `stand_replay`'s actions test: a copy with one flipped field stops at its step, naming exactly the differing keys. This is the template for proving the new field's replay.

@history_item n=65 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-driver's unit tests) — `json.rs` pins the coverage line's written form, with `owner` left out when absent. The command line's rule table has 109 rows. A new field's written form joins `json.rs`'s pins.

@history_item n=66 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the session binary's coverage role) — a role refusing a bad file as `malformed`, holding nothing of the path, was recorded as a boundary widening (PROVISIONAL at the time). This is a precedent for carrier changes going through the founder.

@history_item n=67 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows, each verified against the code. Every picked row's handle and guard date from here.

@history_item n=68 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (`~ decision/layout-tests-in-both-layout-modes`) — the both-modes rule gained its named exception for upstream targets taken in a sync. A new fork target is no such exception and must run both modes.

@history_item n=69 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (`~ contract/harness-construction/from-vdom`) — `from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context. A fixture app booted through it gets the default seed.

@history_item n=70 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (`+ decision/cross-os-readings-by-answer-digest`, `~ decision/stand-checks-bundled-font`) — `stand_cross_os_readings` compares answer digests (snapshot, lint, actions, and one acting step) on the six served tasks against one committed table. A new field in an acting step's answer can move those digests and the committed table.

@history_item n=71 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (`+ contract/session-recording/exploration-checks`) — a panicked path's kept check holds the steps before the panic. `explore`'s per-path panic catch is the only one today, and the scope keeps its ending unwidened.

@history_item n=72 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** (`+ contract/agent-run-selection/session`, `~ contract/agent-run-verbs/run`, `~ decision/test-data-is-code`, `~ overview/session-lifecycle`) — `run session` runs seven_guis' `cli_flow` flows. Fixtures compile into their targets by `boot` or the run's own cargo call. A flow whose answers gain the field runs through this selection.

@history_item n=73 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (`+ decision/no-change-read-control-needs-pending-work`) — a control of a read that must change nothing needs pending work, since a pump on a settled screen is invisible. A capture that must not alter the step is controlled the same way.

@history_item n=74 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (`+ contract/session-layout/changes-nothing` and others) — the most recent in-process `Session` addition, asserted on the stand in both modes. This is the precedent for adding a session-level read without a new verb.
