@history_item n=1 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the driver's verb-table pins read thirteen verbs): `VERBS` went from 12 to 13, with `graph` the ninth instance verb after `lint`. `FieldKind` went from 10 to 12, `NODE_FIELDS` from 13 to 14, describe's row became `[13, 14, 12, 3, 8, 4, 15]`, and cli_tests.rs reads 101 rows. This is the closest precedent for a new read verb: if this chunk adds a verb or a field kind, it moves every one of these pins together, and if it adds none, none of them may move.

@history_item n=2 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the session contract takes graph as its ninth instance verb): `graph` runs no step, waits on nothing and returns `Outcome::Graphed`. The recording routes every verb through `VERBS`, an argument is refused `malformed`, and the usage line lists `graph` after `lint`. The new read should follow this same no-step, change-nothing contract in session-start, session-binary and session-proof.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a graph answer crosses the session socket): carrying ids, names and closed flag words out on the socket was recorded as a boundary widening. The confirm is provisional, delegated while the founder is away, until his batch at the epoch boundary. A new read that answers on the ratified carriers is the same kind of founder's fork, as the scope says.

@history_item n=4 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb held in session and over MCP in process): `stand_graph` 6 runs in both layout modes. It checks that every graph node is a snapshot node, that a graph read changes nothing and holds no typed text (a `crud-name` sentinel occurs 0 times), and that an argument is refused `malformed`. `stand_mcp_tools` went to 7 with a `graph` tool call parenting one `command` span. These are the check shapes the new read's stand and MCP checks extend.

@history_item n=5 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the command line and MCP answer the graph): `cli_commands` went to 16 with `graph_answers_one_line_of_the_ui_graph`. `mcp_server` went to 7, with `tools/list` offering thirteen verbs in `VERBS` order. A new verb moves both counts and the ordered listing.

@history_item n=6 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a graph call logs its verb alone and no graph needle): `graph` joined the closed `tool`/`verb` sets with no span field added. `host_log` went to eight calls and 8 command-span lines, and `mcp_log` to eleven tool calls and 9 `command` lines. `graph_needles()` was added, and `stand_act_spans` reads `graph` once per mode. This is the precedent for the new read's log, needle and count moves.

@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (stand_graph joins the stand runs and the target counts): the newest readings are `run stand` 244 · 0 · 7 over 52 files, stand-repeat over 52 files, 159 `.rs` files and 158 targets, `common` read by 22 `stand_*` checks and `session_common` by 33. `run all` was not re-counted and stays at 743. These are the baselines a new stand check moves from.

@history_item n=8 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the stand checks' shared modules gain the graph's drives and readers): `GRAPH_DRIVES` was added to `common`, and the builder `graph()` and reader `graphed(session, call)` to `session_common`. This is the pattern for the new read's builder and reader. `common` tables are stated expectations, never a second mapping.

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cold-agent pipe speaks thirteen verbs): `valid_step`, `isolated` and test_cold_agent.py's `FAKE` take thirteen tools, `graph` answered empty and not required. A new verb on MCP moves the pipe and its fake together.

@history_item n=10 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading and the CLICKABLE table): adds `NodeState.clickable` and the measured `CLICKABLE` table (Circle Drawer 1, Cells 676, `cells-grid` not among them). The new read's actionable set draws on this reading and must agree with it.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit-test groups for the graph; its unit scope): `graph::tests` pins current listeners, a removed listener gone, and an option owned by its select. The `clickable` pin and the reserved `{owner}///{key}` form, which the walk never calls, are pinned too. The crate holds 100 unit tests in eleven files, and its one `cfg(test)` shim is `dispatch_to_vdom`. These are the node model and owner rules the read rests on.

@history_item n=12 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Cells screen and its graph answer, measured): Cells reads 53,859 B in 6 parts. The unpaged graph reads 293,869 B, a stated limit owned by `res-7cc5bi4k`, and `stand_graph` holds it within the answer bound. A whole-screen read built on the graph needs a check of its own that it stays within the answer bound. The plan's forecast of under 200 KB was disproved.

@history_item n=13 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the committed checks' byte sizes dropped; the unconfirmed flag's stated limit): `counter-cli.json` and `counter-mcp.json` were re-recorded through `record_checks.sh` when the answer format moved, so a recording or replay change re-records them in the same chunk. Separately, a conditional write that a run skips reads `unconfirmed` as a stated limit.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb's tests): the earlier precedent for a read-only verb on every carrier. It moved the verb table to twelve, the MCP listing and describe's row, `cli_commands`, `mcp_server`, `stand_mcp_tools`, `stand_act_spans`, `host_log`/`mcp_log` and the cold-agent pipe. `session_common` gained `lint()`/`linted()`, and the in-process scope came first, PROVISIONAL, until the founder's word.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness and session contracts gain the shot): `screenshot` joined `command.verb` and `tool_call.tool` with no field added. A shot changes nothing, and its id is refused the same way `click` refuses it. This precedent sets the rule that a new verb's id is resolved and refused through the same path as `click`.

@history_item n=16 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection**: set the refusal order `disabled` · `off-screen` · `covered` and added `stand_act_disabled`, `stand_act_obstructed` and `stand_act_scroll`. Also added `session_common`'s `refused`/`refused_unchanged` and `stand_act_ids`' measured accept/refuse rows. The read's per-verb causes must agree with these rows and helpers, the earlier cause in the order named where two hold.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand's tables and checks re-pinned to the flight booker's select): `stand_act_ids` became 68 accepted and 9 refused. The options `flight-type-one-way` · `flight-type-return` are refused `covered`, each at its select's client rect. This is the stand's one owner-served case, and the read must list it with that cause.

@history_item n=18 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages' boots and checks): adds `stand::boot_cells`/`boot_load`, `stand_cells_served` and `stand_load_page`. A click right after a `scroll` lands on the overlay scrollbar, so the walk takes `advance 5000` first. `host_log` reads a paged part through the host's sink. The read must be proven on these served screens, and its click-after-scroll reasons must agree with where a click lands.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause and the restated help): `covered`'s fixed meaning now says a closed select's option lies under it. `schema_tests`, `refusal` and `json` pin the help of `click`/`type`/`press` with `VERBS` and describe's row. A widened `VerbSpec` must keep these help pins.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`stand_act_popup`, the list through a held session): in the open list, a closed option and a button under the list are refused `covered`, and a row past the scrollport is `off-screen` until `scroll`. Every actionable element is keyed. These are the owner-served list cases, sixteen `stand_act_*` files, that the read must agree with.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select limits the list retires; `type` focuses any select with no click): a closed single select's option stays refused `covered`, under a group too, which is the founder's F9. `type` on any select focuses with no click, and `schema_tests` pins that help. Any `VerbSpec` statement of which elements `type` applies to must agree.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the shared-module readers and the blitz-tests targets): `mod fixtures;` is read by 31 checks, counted anchored at line start. An unanchored count reads one more because of the doc comment, a trap when counting a new fixtures reader.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement and the broad line on the list): `docs/capabilities.md`'s select limits grew to four bullets. Each claim's check in `## Where each claim is proven` is named by hand, only names and counts pinned. The read's claim needs its check named there by hand.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint on an open list, `Entry.in_select_box`): the lint skips an entry in a select's box, closed through `closed_select_of`, read from the element and never the role. This is a precedent for telling owner-served options apart when the actionable set is drawn.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `select` model's limits, now asserted): a closed single select's option is refused `covered`, under a group too. Only the reading of an option with no author key in `unkeyed_actionable` stays asserted by no check. That gap is a row the read's key answer could add.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms): adds `stand_act_transformed` 3 with `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`. The measured `off-screen` limit under a translated box is gone. An edit to `action_point` must keep this accepted, and the read must answer it accepted.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' reacting elements read 677; fixture readers read 28): the role-less reacting count became Cells 677 with `cells-grid`, a forecast disproved and re-pinned with its reason. The read's actionable set must account for these counts and how the pin is read.

@history_item n=28 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock**: the causes became eight (`time-unavailable` retired, `advance` accepted on every session), with `fonts-unavailable` the eighth and seven served over MCP. It was ratified at **2026-10-18-engine-debts**. The read names only causes from this closed set.

@history_item n=29 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§2–§10 counts and the harness readings): `covered`'s two measured readings are a box with no area, hit at its centre, and a box clipped on one axis. The read's `covered` reason must match these readings, including where nothing visibly covers the target.

@history_item n=30 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep**: adds `WITNESSES`, the requirement-to-check record of 16 rows, and the non-lean `unkeyed_actionable` pin at 0 · 0 · 0. If the plan adds a witness for v030-06 it extends `WITNESSES`, and the read's key answer must agree with the zero pin.

@history_item n=31 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits**: introduced `unkeyed_actionable` in `actionable.rs` with its four unit tests, and `stand_actionable_keys`. This is the actionable-key check the scope folds in "if it fits".

@history_item n=32 marker=2026-10-12-self-description
**2026-10-12-self-description**: `help` equals `describe()`, every cause is served with its remedy, describe's count row was added, and `help` reads the same bytes on MCP and the command line. The read's entry in the self-description moves these pins.

@history_item n=33 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement**: adds `CapabilityStatementTest`, four tests that pin each `VERBS` verb and cause as a code span, five counts in words and README's tool count, read by path with no write. A new verb or cause moves `docs/capabilities.md` and README in the same change.

@history_item n=34 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans**: added the span line and `stand_act_spans`. Trap: a check green per package was red in the workspace leg, so a check that reads a sink's capture counts lines by target, never in total.

@history_item n=35 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface**: added the `tool_call` line as the parent of `command`. Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test.

@history_item n=36 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli**: added the command-line test level, `cfg(unix)`, which runs the binary once per command and reads streams and status through the shared runner. A check that waits for an expiry watches the state directory. This is the frame for the read's command-line checks.

@history_item n=37 marker=2026-10-19-session-recording
**2026-10-19-session-recording**: a recording keeps each answered instance call, unix only. A check that records is `cfg(unix)`-gated item by item beside an off-unix `Unsupported` check. If the read becomes a recorded step, these pins move.

@history_item n=38 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay**: adds the kept answer, `Check::replay`, `stand_replay` and seven_guis `recording`, which reads the committed checks byte-equal. A shot's PNG differs by OS, so a committed check holds no `screenshot` step. Any recording or replay edit for the read must keep these green.

@history_item n=39 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures**: adds `tests/fixtures/mod.rs` with `boot`, `hold` and `covered`. A page two checks share goes in `fixtures`, and a page with one reader stays in its file. This applies to the read's covered, off-screen and owner-served minimal fixtures.

@history_item n=40 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep**: stand-repeat runs named non-`stand_` targets by a fixed list pinned by `test_stand_repeat_leg_runs_three_rounds`, now seventeen after **2026-10-31-select-popup**. `stand_act_spans` reads the shared `many_items` page. A new non-prefixed target must be added to that list and its pin.

@history_item n=41 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates**: set the coverage floors at escher-driver 97 %, dioxus-native-dom 83 % and blitz-test-harness 90 %, and a zero-failure stand-repeat budget with no retry. These are the founder's words, and this chunk's acceptance rests on them.

@history_item n=42 marker=2026-10-19-test-splits
**2026-10-19-test-splits**: a `*_tests.rs` `#[path]` file is not counted in the per-crate coverage reading, so moving a test module out of line moves the reading with no code change. A later split should forecast a moved reading, not an unchanged one.

@history_item n=43 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the two table tests' private helpers; the dev-host complexity ceiling): mcp_tests.rs's listing test asserts rows through private `#[track_caller]` helpers whose messages name the row and index only. The complexity ceiling is 15 on the dev host, and a closure keeps its branches counted. Extending the listing test should keep this form.

@history_item n=44 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id**: a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. A pinned measured reading goes red by design when the model changes. The read's checks follow this rule.

@history_item n=45 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema**: every table is stated in its test with its row count asserted, and an assertion over a call carries a row index only. Trap: a count is written in two word orders ("13 unit tests", "unit tests 13"), so a sweep for a count site must search both.

@history_item n=46 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract**: the agent-run contract. An empty `run stand` exits 1, and selections are `stand`, `all` and `{name}`. A new check without the `stand_` prefix is reached only by `run all` or `run {name}`.

@history_item n=47 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand**: stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, with no font skip path. This is the anti-pattern the extract restates.

@history_item n=48 marker=2026-10-20-load-wait
**2026-10-20-load-wait**: the "no sleep and no wall-clock read" rule for session checks gained a second exception only where wall time is the subject. A read that changes nothing has no such exception.

@history_item n=49 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the Dioxus runtime's listeners): the bridge's listener attributes over-report, because a listener removed on a re-render keeps its attribute while a click no longer acts. The vdom walk's dynamic attributes read current listeners exactly. The actionable set must take its handlers from the current-listener reading.

@history_item n=50 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the per-OS CI tallies re-read; dioxus-native-dom's coverage reading): the tallies re-read at `e26f1a207` are the base this chunk's tallies move from. A tally move is attributed by grepping the test's own name, never by the preceding `Running` line. dioxus-native-dom reads 86.71 %.

@history_item n=51 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit**: a scrolled box's `bounds` and its hit walk were fixed in the engine. Trap: a check written with a raw harness click outside its clipping box is found only by running the stand gates. This matters for off-screen and covered fixtures.

@history_item n=52 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the coverage leg excludes `keyboard_compat.rs` by filename): dioxus-native-dom's 83 % floor is read with `keyboard_compat.rs` excluded by filename. A reader that P3 adds to that crate is measured under this exclusion.
