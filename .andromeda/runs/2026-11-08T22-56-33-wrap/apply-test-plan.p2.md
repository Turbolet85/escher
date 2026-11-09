@apply_row n=1 ref_file=fanout-test-plan--cascade-g5.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g5.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g5.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g5.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g5.md ref_n=5 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=4
@apply_row n=6 ref_file=fanout-test-plan--cascade-g5.md ref_n=6 v=applied edit=E5
@apply_row n=7 ref_file=fanout-test-plan--cascade-g5.md ref_n=7 v=applied edit=E6
@apply_row n=8 ref_file=fanout-test-plan--cascade-g5.md ref_n=8 v=applied edit=E7
@apply_row n=9 ref_file=fanout-test-plan--cascade-g5.md ref_n=9 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=8
@apply_row n=10 ref_file=fanout-test-plan--cascade-g5.md ref_n=10 v=applied edit=E8
@apply_row n=11 ref_file=fanout-test-plan--cascade-g5.md ref_n=11 v=applied edit=E9
@apply_row n=12 ref_file=fanout-test-plan--cascade-g5.md ref_n=12 v=applied edit=E10
@apply_row n=13 ref_file=fanout-test-plan--cascade-g6.md ref_n=1 v=applied edit=E11
@apply_row n=14 ref_file=fanout-test-plan--cascade-g6.md ref_n=2 v=applied edit=E12
@apply_row n=15 ref_file=fanout-test-plan--cascade-g6.md ref_n=3 v=applied edit=E13
@fact after=fct-csattoyz as=n1
- **Session ↔ held instance, handler coverage:** `stand_handler_coverage` (8) holds `escher_driver::handler_coverage` over the six served tasks in both layout modes — a snapshot check listing exactly `HANDLERS_AT_BOOT`, equal to the graph's reading at boot; each `GRAPH_DRIVES` task's drives leaving its `UNEXERCISED_AFTER_DRIVES` row; two checks listing only what neither exercised; the load page reading its click exercised; a refused step exercising nothing and a keyboard activation its click; a handler whose step removes its element reading exercised; a typed sentinel never reaching the coverage line; and a check of another app contributing nothing and not holding (tests/blitz-tests/tests/stand_handler_coverage.rs:212-423) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=@n1 as=n2
- **Handler coverage's screens:** handler coverage reads every screen the replays reach, the UI graph read at boot and after each replayed step (`inputs#I4`, Q2); OPEN — the screens handler coverage reads, the overseer's delegate call · owner: the epoch-boundary PROVISIONAL discharge — as stated at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=@n2 as=n3
- **Handler coverage, its stated limits:** a handler inside a canvas is not counted, deferred to 0.4.0 (the incubator's canvas plans); a handler on no screen a replay reached is not listed, by construction, with no owner; a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread, owned by a residual row, target next (docs/capabilities.md:273-281) — as stated at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-kzdrls5d expect_rev=1
old: owed:** no test exercises the replay of a kept `actions` step — `Check::replay` runs every instance-verb step through `Session::run`, so it holds by construction, unmeasured; the replay test is owed, a CARRY on «Handler coverage» — as stated at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md, Addendum 1
new: measured:** `stand_replay`'s `a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes` replays a recorded Counter session holding two `actions` steps on a fresh Counter, all 4 steps running, and the copy with one `keyed` flipped stops at step 3, keys exactly `["nodes"]`, in both layout modes, discharging the CARRY on «Handler coverage» (`cry-sxrivdk3`) (tests/blitz-tests/tests/stand_replay.rs:263-329) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-adkwyvky expect_rev=3
old: `host_log` holds 5 tests and `mcp_log` 3
new: `host_log` holds 6 tests and `mcp_log` 3

@fact after=fct-adkwyvky as=n4
- **Session host ↔ what it writes, a coverage run:** seven_guis' `host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` runs `coverage crud` over a CRUD session kept through the binary with a typed sentinel and a row click, and reads 0 id, name, coverage, sentinel, path and recorded needles on stderr at `trace`, the coverage needles the CRUD screen's, from `common::coverage_needles`, under the `-p seven_guis` and the workspace build (examples/seven_guis/tests/host_log.rs:141-226; examples/seven_guis/tests/common/mod.rs:454-479) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-csryo2zo expect_rev=4
old: `cli_commands` (17)
new: `cli_commands` (18)
old: the host role and the `mcp` role, a misplaced `--record` a usage error, status `2`
new: the host role, the `mcp` role and the `coverage` role (`coverage <app> <check>...`), a misplaced `--record` and `coverage nowhere <dir>` each a usage error, status `2`

@fact after=fct-csryo2zo as=n5
- **Command line, handler coverage:** `cli_commands`' `coverage_replays_a_kept_check_and_lists_the_handlers_it_left` runs the binary's `coverage` role over a Counter check the binary kept and reads the handlers it left in the one coverage line on stdout, status `0` (examples/seven_guis/tests/cli_commands.rs:522-571) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-a24rqe7v expect_rev=1
old: read by `stand_graph`
new: read by `stand_graph` and by `stand_handler_coverage`, whose drives check covers all 7 tasks

@fact after=fct-5d6i6kkv as=n6
| Handlers at boot and unexercised after drives (stand tasks) | `HANDLERS_AT_BOOT: [(&str, &[(&str, &str)]); 6]` and `UNEXERCISED_AFTER_DRIVES: [(&str, &[(&str, &str)]); 7]` in the stand checks' shared module, the handlers at boot of the six served tasks and the handlers each `GRAPH_DRIVES` task's drives leave unexercised: the named handlers stated by hand, Cells' 676 grid-cell `click`s of its 678 handlers at boot generated row by row by `grid_cells()`, `CELLS_CLICKED_BY_DRIVES` naming the two the drives click; held equal to the graph's own reading on a fresh boot, as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md; read by `stand_handler_coverage` | (tests/blitz-tests/tests/common/mod.rs:336-380; tests/blitz-tests/tests/common/mod.rs:382-401; tests/blitz-tests/tests/common/mod.rs:406-412) |

@fact id=fct-mrjxzany expect_rev=3
old: — 53 files,
new: — 54 files,
old: `stand_graph` and `stand_actions`
new: `stand_graph`, `stand_actions` and `stand_handler_coverage`

@fact id=fct-prdavblg expect_rev=1
old: (examples/seven_guis/tests/host_log.rs:228-371); its earlier
new: (examples/seven_guis/tests/host_log.rs:228-371), and `host_log`'s coverage run reads 0 needles under both builds (§5 Session host ↔ what it writes, a coverage run); its earlier

@fact id=fct-3ctmcebn expect_rev=3
old: run the 53 `stand_*.rs` files
new: run the 54 `stand_*.rs` files
old: (§9 Legs; with `stand_actions`,
new: (§9 Legs; with `stand_handler_coverage`, entering through the existing `stand_*.rs` glob, CI#42530780903 green 17/17, as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md; with `stand_actions`,
@pattern id=tp2-01
fixed: the replay test is owed
@pattern id=tp2-02
fixed: `host_log` holds 5 tests
@pattern id=tp2-03
fixed: `cli_commands` (17)
@pattern id=tp2-04
fixed: the usage line naming `--record` on `start`, the host role and the `mcp` role
@pattern id=tp2-05
fixed: stand_*.rs` — 53 files
@pattern id=tp2-06
fixed: the 53 `stand_*.rs` files
