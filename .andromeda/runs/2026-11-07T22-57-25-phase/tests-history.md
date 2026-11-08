@history_item n=1 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** (stand element-id checks counted; element_id unit tests) — set the `element_id` unit-test construction: a `DioxusDocument` from a plain app, re-rendered by a `GlobalSignal` flipped in `vdom.in_runtime`; a new `stand_` file picked up by prefix in `run stand`. The graph reader and registry unit tests must follow this construction, and link 1 reads through this id walk.

@history_item n=2 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** (snapshot checks counted; dioxus-native-dom unit tests re-counted) — added the rule that `snapshot` unit tests call `resolve(0.0)` through `inner_mut()` after `initial_build`. Standing practice: each count is appended as a dated link and earlier links are kept as history. The new snapshot field's unit tests and the crate's count move follow this.

@history_item n=3 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** (state checks counted; unit-level route for a `disabled` pin) — the snapshot `build` helper carries a no-op HTML parser, so unit tests write attributes through `DocumentMutator` and parsed markup is pinned in blitz-tests. Trap: a headless deleting key is compiled out on macOS, so prove a state change with clicks and typed characters. This binds the graph's link-3 witnesses (a handler's write).

@history_item n=4 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** (text form tests) — introduced `Snapshot::to_text`'s unit tests (state tokens and their fixed order), `stand_snapshot_text` under `SNAPSHOT_TEXT_BUDGET` (10,000 bytes) and the in-file ceilings. Trap: a search for `snapshot.rs:` also matches `stand_snapshot.rs:`, so anchor citation searches on the path. The new field's text token and its re-measured screen sizes land here.

@history_item n=5 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** (changed-set and diff coverage) — added `Snapshot::diff` unit tests over hand-built snapshots and `stand_diff`, where every step is held against the file's own reading of the two snapshots in both layout modes. The «reacts to a click» field becomes a diff reading that these checks must hold.

@history_item n=6 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (bridge tests, the shared stand module) — the bridge tests are a `cfg(test)` child module of `dioxus_document` because the handler's fields are private and it has no constructor. This is the precedent for reaching private state with no `pub` or constructor added for tests (dioxus-core hooks are `pub(crate)`). It also created `tests/common/mod.rs` with the `controls` / `rendered` tables as stated expectation. Trap: a sed or grep address that ends at a function's name also selects every item whose name starts with it.

@history_item n=7 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (driver-action census; counts re-read) — the precedent for adding snapshot node readings (selected / pressed / invalid). The dioxus-native-dom unit count and the builder, text and diff splits moved together, the Crud boot text moved 2034 → 2049 bytes, and the hosted snapshot line sizes were re-measured. The «reacts to a click» field moves the same set of figures.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (what the new and extended checks assert) — `stand_snapshot_state` asserts that every node's state readings equal its accessibility node's, and that no other node reads them, in both modes. This is a precedent for a stand check asserting a new field per node against the accesskit tree.

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** (the self-description's checks and counts) — added thirteen node keys with help, `differs` in `Reading::ALL` order, describe.rs and its count row, `help` answering equal to `describe()`, and `stand_mcp_masked`. The new node field moves `NODE_FIELDS` past thirteen, the node-key help, the describe count row and `differs`.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (harness and session contracts gain the shot) — the precedent for a new verb. `screenshot` joined `command.verb` and `tool_call.tool` with no span field added (in step with obs-plan §4), the verb table, MCP listing, `cli_commands`, `mcp_server`, `stand_mcp_tools` and the session-lifecycle key all moved, and the carriage was the founder's fork. A graph verb, if the fork adds one, moves the same surfaces.

@history_item n=11 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb's tests) — moved the verb table to twelve (eight instance-level) and the MCP listing to twelve, made describe's row `[12, 13, 12, 3, 8, 4, 15]`, and moved the cold-agent server, `valid_step`, `isolated` and `FAKE` to twelve tools. It also moved `stand_act_spans` calls per mode and `host_log` / `mcp_log` (one lint each, 0 needles). These are exactly the twelve-verb pins a new graph verb would move.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause and the restated help) — `schema_tests`, `refusal` and `json` pin `VERBS` at 12, causes at 8 and describe's row `[12, 13, 12, 3, 8, 4, 15]`. This is the current reading that the new field (13 → 14 node keys) and any new verb move.

@history_item n=13 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (the capability-statement pin, the ci-scripts tally) — created `test_capability_statement.py`: verbs and causes as code spans, five counts in words (`{word} node keys` from `NODE_FIELDS`), README's «the {word} driver commands as tools», and `## Where each claim is proven`. A renamed check changes that section by hand. The new field and the v030-05 claim move it.

@history_item n=14 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the two table tests' private helpers; the complexity ceiling) — `the_listing_is_the_twelve_verbs_of_the_table` asserts each tool through private `#[track_caller]` helpers whose messages name only the row and index, and reads 3 against the dev-host cognitive-complexity ceiling of 15. A new MCP tool row must keep this form and stay under the ceiling.

@history_item n=15 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the cold-agent run pipe on the escher server) — introduced `valid_step` (the verbs, closed keys) and `isolated` on the `mcp__escher__{verb}` tools, judged by replaying the check. These are the cold-agent verb sets a new verb moves.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the cold-agent pipe over six apps; its contract tests) — `isolated` reads twelve `mcp__escher__*` tools, and `test_cold_agent.py` has 37 tests with a fake `escher-session` speaking the twelve tools. A new verb moves the fake and the twelve.

@history_item n=17 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (the driver sweep and `WITNESSES`; the non-lean pin) — created `stand_non_lean_tasks` 3 and pinned the non-lean reading at 0 · 0 · 0 (was 2 · 3 · 676). This is the file whose pinned starting reading the «reacts to a click» field re-pins.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' reacting elements read 677) — the count moved from elements reacting to a click (Cells 676) to elements reacting to input (Cells 677: the 676 data cells plus `cells-grid`, with `onkeydown` and no role). The plan's forecast of 676 unmoved was disproved. This must be reconciled with the scope's «Cells 676» when the field's starting reading is re-pinned.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' grid as one Tab stop in the checks) — `cells-grid` took `tabindex` 0, `aria_label` `Cells` and an `onkeydown` with no role. This is the element behind the 676/677 split, and its click-reaction reading must be stated.

@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand's tables re-pinned to the flight booker's select) — `controls(task)` has 17 rows naming 14 controls, `flight-type` a `ComboBox`, and the select's options sit at the select's client rect. These shared tables are the stated expectation the new field is asserted against through `tests/common/mod.rs`.

@history_item n=21 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** (stand log format measured) — set the stand's stderr line format and the allowlist scrub, bound to obs-plan §3. This is the base of the «never logged» readings the graph content must meet.

@history_item n=22 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (`host_log`, `telemetry_drop`) — `host_log` reads needles from the stand booted in-process: no id or name on stderr at `trace` under both builds, a failure printing counts only. Trap: a needle list read from the stand outgrows a hand-kept one. The graph's component, handler and state names join these needles.

@history_item n=23 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (`stand_act_spans`, engine features by runner) — created `stand_act_spans` (0 ids, names and typed text per mode, eight span fields) and the rule that a check reading a sink's capture counts lines by target, never in total. A green per-package run is not the workspace leg's green. Graph no-log checks must count by target.

@history_item n=24 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (MCP checks, the `tool_call` line) — the `mcp` role writes one `tool_call` line per call, `tool` and `ending` only, and `mcp_log` covers the role at `trace` under both builds. Standing trap: `tracing` callsite interest is process-global, so a `with_default` check needs its own recorder per test. A graph tool call must stay within the `tool_call` closed set.

@history_item n=25 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (the 0.2.0 record, the asserted limits) — `stand_act_spans` has 36 calls per mode with a part-2 page's ids and names joining the needles, and `host_log` 3 checks the recording clients' stderr. This is the precedent for widening needle sets when a new read is driven.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list through the process checks) — `host_log` 5 and `mcp_log` 3 each gained a trace check reading 0 needles of new content (option names), using a shared `option_needles()` in seven_guis' `tests/common/mod.rs`. This is the direct precedent for graph-content needles (component, handler and state names).

@history_item n=27 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served pages, `LoadNet` and the host-sink part read) — `host_log` 4 serves `cells` at `trace` and reads `snapshot --part 2` with 0 id and 0 name needles under both builds, from `common::cells_needles(part)`. Also `stand_cells_served`. The new field moves Cells' paged text, and these part reads must hold.

@history_item n=28 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (Cells' bytes corrected) — the Cells screen reads 47,089 bytes in 740 lines and 5 parts through the binary (47,949 had been stale). Rule: a plan quoting a flagged-old reading re-reads it. The new field moves these figures, which must be re-measured.

@history_item n=29 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs, the snapshot line sizes) — the hosted snapshot line reads 832 · 1,403 · 1,368 · 2,208 bytes on the lean tasks, re-measured when the select moved them. The new node field moves them again.

@history_item n=30 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the lifecycle contract gains the recording) — `run` records each answered instance call, and a recording keeps the instance verbs' calls. `is_masked_input` gained seven rows, the mask the graph's values must use. A graph verb's recorded step must be a call a record can spell.

@history_item n=31 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the kept answer, the replay, the committed checks) — records keep each answer (snapshot text and diffs). `counter-cli.json` and `counter-mcp.json` are replayed by `stand_replay` and kept byte-equal to what the binary records by seven_guis `recording`. The new field changes the kept answers, so the checks must be re-recorded.

@history_item n=32 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the schema driven over the socket) — json.rs's written forms carry the mask, and the session host's stderr reads 0 typed sentinels beside 0 ids and names under both builds. A graph value written must read as the mask.

@history_item n=33 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (the driver's calls covered) — the rule it sets: a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. Graph checks must not print graph content in failures.

@history_item n=34 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (escher-driver's unit tests 13 → 25) — every schema table is stated in its test with its row count asserted. Trap: a crate's count is stated in two word orders («13 unit tests», «unit tests 13»), so a count-site sweep must search both. This binds moving the 83 / 100 unit counts.

@history_item n=35 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (the parts check, `--part`) — dioxus-native-dom 81 (six on `text_parts`), escher-driver 100 (36 in four `#[path]` files), and `stand_act_spans` / `host_log` / `mcp_log` each driving the new read with 0 needles. This is the precedent for a new read crossing every carrier, and the escher-driver unit count the chunk moves from.

@history_item n=36 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (dioxus-native-dom's unit tests and the mutation buffer) — dioxus-native-dom reads 83 unit tests in nine files, and `mutation_buffer.rs` holds none. This is the count the graph reader, registry and field tests move.

@history_item n=37 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's limit) — a task polled during render acts on the document as the previous render left it, so a node created in the same render is not yet present (owner an open residual). A graph read that resolves nodes inside a render must respect this.

@history_item n=38 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the fix's witness on Dioxus 0.7, the probe's two pins) — the probe's pins became `…_spawned_acts_*` through the helper `spawned_focus_acts`. The probe still has 10 tests and its legs are unchanged. It also records dioxus-core 0.7.10's task scheduling. The probe must stay green as the graph's ground.

@history_item n=39 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (dioxus-native-dom's coverage reading) — dioxus-native-dom reads 2937/3387 = 86.71 % (floor 83 %), with `mutation_buffer.rs` counted inside the floor with no exclusion. This is the reading against which the new reader and registry code is counted.

@history_item n=40 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the per-OS CI tallies re-read) — the tallies on CI#42371051720 (linux 220 · 1310 · 0 · 12 …), and the rule that a tally move is attributed by grepping the test's own name, never the preceding `Running` line. The chunk's new tests move these.

@history_item n=41 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the target, its legs and its pages) — `dioxus_runtime_probe`, 10 tests, reached by the workspace `test` leg, `run all` and `run dioxus_runtime_probe`, not `stand-repeat` or `run stand`. Its pages are `toggled_listener`, `shared_state` and `hazard`, and it measures links 1–4. The graph reader is built on these.

@history_item n=42 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the runtime's listeners and a held signal's readers) — listener attributes over-report, while the vdom walk's `VNode.dynamic_attrs` / `mounted_dynamic_attribute` read exactly (`link_2_a_removed_listeners_attribute_stays`). `subscribers()` → `origin_scope` give exactly the reader scopes. Hook-local `use_signal` exposes neither link because `Scope.hooks` is `pub(crate)`. This grounds fork 2 A and the registry.

@history_item n=43 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (blitz-tests' counts with dioxus_runtime_probe) — `run all` 743 · 0 · 12, `run stand` 237 · 0 · 7 over 51 files, and blitz-tests 158 `.rs` / 157 targets. These are the proof counts and directory count the chunk moves.

@history_item n=44 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`run stand` 237 and `run all` 727) — set `run stand` 237 · 0 · 7 over 51 files, the stand count the chunk's new graph checks move above. `run stand` runs the `stand_*` fixtures readers and `run {file}` the non-stand ones.

@history_item n=45 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (a third shared module) — `tests/fixtures/mod.rs` with `boot` / `hold` and the rule that a page two checks share goes in `fixtures`, while a page with one reader, or whose ids differ, stays in its file. No `run fixtures` selection. This decides where a probe page reused by a graph check lives.

@history_item n=46 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the shared-module readers and the blitz-tests targets) — `mod session_common;` has 32 readers and `mod fixtures;` 31, counted anchored at line start (an unanchored count reads one more). blitz-tests has 156 → later 158 files. A new graph check moves these reader counts.

@history_item n=47 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`BlitzTestsTargetsTest`) — the fork keeps one target per file, with `tests/all.rs` declared and never built. A new graph check is its own auto-discovered target with no manifest line.

@history_item n=48 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the floors and the flakiness budget) — set the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure `stand-repeat` budget, on the founder's word. These bind the chunk's coverage and every new `stand_*` check.

@history_item n=49 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the coverage leg excludes `keyboard_compat.rs`) — dioxus-native-dom's reading excludes `keyboard_compat.rs` by filename regex. This is the condition under which the 83 % floor is read for the touched crate.

@history_item n=50 marker=2026-10-19-test-splits
**2026-10-19-test-splits** (a split moves the coverage reading) — a `*_tests.rs` `#[path]` file is no record of the per-crate reading, so moving tests out of line moves the crate's coverage. A later split should forecast a reading change, not none. This matters if graph tests go in a `#[path]` child file.

@history_item n=51 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` runs ten named targets) — non-`stand_` targets enter `stand-repeat` only as literal `--test` words in `ci-leg.sh`, pinned by `test_stand_repeat_leg_runs_three_rounds`. `run stand` keeps its prefix rule. A new non-stand graph target that must repeat joins this list.

@history_item n=52 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the stand-repeat leg, seventeen named words and 51 files) — `stand-repeat` names seventeen targets over 51 `stand_*.rs` files, and the sweep test's reach covers them. A new `stand_` file moves 51, and a new named target moves seventeen.

@history_item n=53 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the 0.2.0 sweep test's reach over the stand-repeat leg) — the sweep test reads `ci-leg.sh` as one text, so a target dropped from `stand-repeat` alone reads green while the `a11y` leg still names it. Membership is held jointly with `test_ci_workflows.py`. A trap for adding a named target.

@history_item n=54 marker=2026-10-20-load-wait
**2026-10-20-load-wait** (`stand_load_wait`) — «no sleep and no wall-clock read» in session checks gained its second exception (a check whose subject is wall time). Graph checks advance time only through the driver.

@history_item n=55 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (`advance` on every session) — `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep in a session check, and the harness's time is a `VirtualClock`. This is the other wall-time exception graph checks must not add to.

@history_item n=56 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** (a textarea's value read on the fixture) — a new snapshot reading's check was placed in the existing `stand_snapshot_state` because the audit runs the snapshot unit against a fixed test set, and a new stand file would sit outside it. This is a precedent for where a snapshot field's check lives.

@history_item n=57 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** (the mutation tier's recipe) — cargo-mutants runs under nextest's `mutants` profile with `--baseline=skip` on the dev host only. The next audit of the touched crates' new code (dioxus-native-dom included) must run under that profile.
