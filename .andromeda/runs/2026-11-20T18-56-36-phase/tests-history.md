@history_item n=1 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — recorded the trap that a headless key press is not the same on every platform (the editor's Backspace arm compiled out on macOS, green on Linux, red on the macOS leg); a `press` with Ctrl (and Meta/Alt by kin) is exactly that kind of platform-dependent modifier, so its stand checks must be read on the macOS CI leg, not only locally.

@history_item n=2 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — put the event-bridge tests in the `cfg(test)` child module `dioxus_document::bridge_tests` because `DioxusEventHandler`'s fields are private, and recorded that they keep the `EventState` they pass as "the one way to read `propagation_is_stopped()`"; this is the precedent and constraint (decision/bridge-tests-without-widening) for reading a `stopPropagation` in the reached record without widening visibility.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — created the Session lifecycle key (session-start · host · wire · binary · checks · proof) that every driver argument change amends, with traps: an `#[ignore]` host child blocks a bare `--ignored` run, and a sink-installing host at `trace` overruns an undrained pipe.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — introduced `host_log` (a sink-installing host at `trace`, needles read from the stand, both streams drained by threads) with the trap that a must-fail-red check cannot `start` on a piped host; any new `press` modifier or `graph` part read driven through the host must keep this drain rule (decision/drain-host-pipes-while-running).

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — escher-driver's schema pins: the verb table, kinds, bounds, key names; `validate` with one malformed witness per rule per kind, the first-failure order, every table stated with its row count asserted; a new argument (`press` modifier, `graph` `part`) must extend these tables and their counts.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the harness first re-exported `Key` and `Modifiers` (what `press_with` takes) and the driver's `stand_act_keys` covered `press`; set the rule that a driver check's failure message names a layout mode, a task and an index, never an id or screen text, and that no deleting key appears outside `stand_act_keys`.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the closed `command` span line and `stand_act_spans`; the trap that the same check read green per package and red in the workspace leg, so a check reading a sink's capture counts lines by target, never in total (decision/sink-capture-counted-by-target); a `graph --part` or Ctrl press call must add a span row there counted by target.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (session-lifecycle keys) — made the command line a driven surface: flags spelt as the schema spells them (`--shift` bare), `cli_commands`/`cli_flow`/`host_timer`, the `v2` wire with `call` and its bounds, the session socket's crossing ratified; a Ctrl modifier flag or `graph --part` lands in exactly these contracts (command-form, session-wire `call`).

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (unit/integration/e2e sections) — the schema driven over the socket and through the binary by four seven_guis targets with eight mutation controls; noted the macOS CI leg is the only witness of the select-all modifier, the precedent for a Ctrl/Meta modifier being read only on macOS.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role writes one `tool_call` line per call, parent of `command` (contract/stand-log/tool-call-span), MCP checks not unix-gated but `mcp_server`'s comparison; trap that `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help`/`describe()` pinned with the remedies and describe's count row; `cli_commands` checks `help` and each cause's `remedy`; a `press` help that drops its no-Ctrl limit, or a graph `part` argument, moves describe's row and the help pins.

@history_item n=12 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `stand_keyboard` and `stand_act_keys` 3 (`press enter`/`space` as a click), `apple_keybinding` exercised on every OS; the keyboard path a Ctrl `press` extends, and the cross-OS modifier precedent.

@history_item n=13 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the per-crate coverage floors escher-driver 97 % · dioxus-native-dom 83 % · blitz-test-harness 90 % (decision/per-crate-coverage-floors) and the zero-failure flakiness budget; this chunk's driver, reached-record and harness edits must hold those floors.

@history_item n=14 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the harness re-exports keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` (decision/harness-reexports-key-types), so escher-driver names a key and a modifier with no dependency of its own — the type a Ctrl/Meta/Alt `press` modifier rests on.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — fixed the refusal causes at eight in order, `fonts-unavailable` last, seven served over MCP (decision/eight-refusal-causes-in-order, ratified 2026-10-18); a new argument's refusal must reuse `malformed`, not add a cause.

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — a recording keeps each answered instance call with `arguments` in schema order and a digest (contract/session-recording/record-fields), wire bumped to `v3`; a new `press` modifier or `graph` `part` argument enters every kept check's shape and the recording pins.

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — `Check::replay` through `Session::run`, committed checks under `tests/blitz-tests/checks/` replayed by `stand_replay` and kept byte-equal by seven_guis `recording`; a schema change to `press`/`graph` arguments must keep those committed checks fresh.

@history_item n=18 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — moving escher-driver tests into `*_tests.rs` `#[path]` files moved its coverage reading (98.33 → 98.07 %); a later split or new test file should forecast a coverage change against the 97 % floor.

@history_item n=19 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the precedent `graph --part N` follows: `ArgKind::Part` 1 to `MAX_PART` 1000, `Outcome::Part`, `snapshot --part` with 0/1001/`x` refused exit 1, the paged answer line `{"text":…,"part":n,"parts":m}`, `stand_snapshot_parts` through `Session::run` and `serve_mcp` in both modes, `stand_act_spans` and `host_log`/`mcp_log` each driving one part read; the founder's «Аргумент part, части по умолчанию».

@history_item n=20 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand_act_spans` drives a part 2 of the shared `many_items` page, and a paged `part` read through the host's own sink was left unmeasured as a residual; the same gap a `graph --part` read through the host opens.

@history_item n=21 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `test_capability_statement.py` (CapabilityStatementTest) pins `docs/capabilities.md`'s verb and cause names and schema counts in words and the README's MCP tool count; editing the stated limits (press's Ctrl, the unpaged graph, the bubbling limit) means editing a pinned page by hand.

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served pages) — `host_log` gained the paged Cells part read through the host's own sink at `trace` with `common::cells_needles(part)`, discharging that residual; the model for a `graph --part` sink check on Cells.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (select model) — the `multiple` select modelled, `stand_act_select` 10 with a Shift span through the driver, `select_interaction` 18, schema_tests pinning `type`'s help; the select's keys and rows that a Ctrl `press` would extend to a non-adjacent set.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (what no driver check reaches) — recorded that a non-adjacent `multiple` set, `press` carrying no Ctrl, and Ctrl+A are not reached, owned by the `press` residual (now `res-hfvjj77l`) on the founder's P5 word; this is the residual this chunk takes up.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — `schema_tests`, `refusal` and `json` pin the help of `click`, `type` and `press` as fixed `'static` text with `VERBS`, causes and `describe`'s count row; `press`'s help is the pinned text that states the Ctrl limit.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (standing select checks) — `select_interaction`'s 14-row `KEYS` table and `stand_keyboard`'s list walks re-pinned around the open list; a Ctrl key path adds rows to these tables.

@history_item n=27 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — retired the key-press panic past a dropped anonymous block, `key_press_after_container_lost_its_box` in both modes (decision/key-after-dropped-anonymous-block); a new modifier press path must not reopen it.

@history_item n=28 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — measured that the bridge's listener attributes over-report and the vdom walk's dynamic attributes read current listeners exactly (decision/listeners-read-from-vdom-walk); the reached record and any stop-aware reading must use the walk.

@history_item n=29 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — re-pointed the owner sentences to residual rows, `res-hfvjj77l` (press with no Ctrl) among them; the sites to re-say if this chunk fixes or keeps that residual.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (Cells graph answer) — measured `graph` on Cells at 293,869 bytes in one answer, `graph` taking no argument, the stated limit «An unpaged graph» owned by `res-7cc5bi4k`, held within the answer bound by `stand_graph` in both modes; the measurement and residual `graph --part` discharges.

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph verb in session and over MCP) — `stand_graph` 6 (every graph node a snapshot node, a read changing nothing and holding no typed text, the whole Cells answer within the bound) and "an argument to `graph` is refused `malformed` with nothing run" (contract/graph-command/argument); a `part` argument overturns that refusal pin on every carrier.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (session contract) — session-binary: `graph` the ninth instance verb, no argument (refused `malformed`), its answer one line `{"nodes":[…],"states":[…]}`; contract/escher-session-command-line/graph that a paged graph rewrites.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph across the socket) — the graph answer's content crossing the socket is a ratified boundary widening; scope's ruling makes `graph --part` a new argument inside that carrier.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (verb-table pins) — schema_tests: `graph` no argument, fields `nodes`/`states`, `GraphNodes`/`GraphStates` field kinds; describe's count row; cli_tests' line-reading rows; all move with a `part` argument (and paged fields) on `graph`.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (logs) — `graph` a word of the closed `command.verb`/`tool_call.tool` sets, `host_log`/`mcp_log` each drive one `graph` with `graph_needles()`, `stand_act_spans` reads it; a paged graph read joins these counts.

@history_item n=36 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph drives and readers) — `GRAPH_DRIVES` drives each conditional write once and `session_common` gains `graph()`/`graphed()`; the unconfirmed-flag limit (decision/unrun-declared-write-reads-unconfirmed) and the bridge's one `cfg(test)` shim `dispatch_to_vdom` (decision/bridge-tests-without-widening's except), both of which a stop-aware reached record must respect.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — one actionable rule from current listeners, never the `data-dioxus-id` mark (decision/one-actionable-rule); `actions` added across command line, MCP, spans and logs with the shared `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone` helper and an argument refused on every carrier — the full carrier sweep a new argument repeats.

@history_item n=38 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (reached record) — `reached::tests` pin a bubbling dispatch recording its ancestor's listener of the same event; the unit test a stop-aware fix must invert or extend.

@history_item n=39 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage) — `handler_coverage` over `Check::replay_each`, `HANDLERS_AT_BOOT`/`UNEXERCISED_AFTER_DRIVES` tables, a refused step exercising nothing; the check and tables a stopped-dispatch fix moves.

@history_item n=40 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stated limits) — the three limits stated in `docs/capabilities.md`: canvas deferred to 0.4.0, unreached screens by construction, and a bubbling dispatch counting every ancestor with `stopPropagation` unread, owned by a residual (now `res-4dr3wt62`); this chunk's third debt (decision/handler-coverage-bubbling-counts-ancestors, contract/handler-coverage-limits/bubbling-dispatch).

@history_item n=41 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (coverage role and line) — `escher-session coverage <app> <check>...` with its refusals and exit statuses, the coverage line `{"unexercised":[…],"checks":n,"held":m}`, `cli_commands`' coverage test and `host_log`'s coverage run at `trace`; contracts a stop-aware coverage reading must keep.

@history_item n=42 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — decision/handler-coverage-reads-every-replayed-screen moved from provisional to standing on the founder's ratification.

@history_item n=43 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — the engine's click handler is held below the complexity ceiling, upstream's arms included, an exception to decision/complexity-ceiling-host-only; an edit to blitz-dom's dispatch for a stop must keep that function under the ceiling.

@history_item n=44 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added contract/session-recording/exploration-checks (one kept check per explored path, replaying green); new arguments reach explored paths' checks too.

@history_item n=45 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the session binary's coverage role, previously provisional.

@history_item n=46 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — overview/session-lifecycle restated: the `run session` selection runs the binary's `cli_flow` flows; a new flag must keep those flows green.

@history_item n=47 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — decision/no-change-read-control-needs-pending-work: a control of a read that must change nothing needs pending work; a paged `graph` read's change-nothing check needs such a control.

@history_item n=48 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — contract/driver-session/acting-verbs gained `caused`; `press` with a modifier returns the same acting-verb result shape.

@history_item n=49 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — decision/socket-carries-commands extended to a step's caused, and decision/mcp-result-decoded-whole (one shared helper decodes the MCP result string, after a Windows red); any new MCP comparison uses that helper.

@history_item n=50 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (carrier rows) — the last carrier-wide verb change: tools/list, command-form, usage-line, verb-table, mcp-tools-list order and socket-carries-commands each amended by row; the row set a `press` flag or `graph [--part <part>]` usage change must amend the same way.

@history_item n=51 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (sweeps) — decision/count-and-rename-sweeps after a missed numeric pin and a missed renamed witness; a count or help change here needs both sweeps.

@history_item n=52 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — added decision/harness-graph-tests-enable-accessibility (a harness unit test of the UI graph turns the feature on through the dev-dependency) and decision/pinned-viewport-read-from-document; both bind harness-level graph tests.

@history_item n=53 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — decision/typed-password-never-returned widened to graph, actions and layout answers and kept/explored checks, guarded by `stand_masks_on_new_answers`; a paged graph answer must still hold the mask; also added decision/requirement-witness-inside-stand-repeat for the new capability's witness.

@history_item n=54 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — decision/capability-statement-pinned, capabilities-doc-pinned-to-schema, capability-check-names-held-to-schema and capability-statement-from-schema now also hold the a11y leg file count and stand-repeat named-target count to `ci-leg.sh`; a new check added to either leg reddens ci-scripts until `docs/capabilities.md` re-states it.

@history_item n=55 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — decision/caused-owner-pinned-as-graph-reads added (a check pins the graph's owner reading), and count-and-rename-sweeps gained the exception that a count-worded test name pinned as a witness is kept.

@history_item n=56 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — contract/session-run/changed-node-states now names `highlighted`, as a press in an open list does; the previous debts chunk, whose press-in-list reading a Ctrl press extends.
