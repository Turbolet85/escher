@amendment marker=2026-11-08-handler-coverage
title: the reached record's unit tests
section: §1 Test Scope Summary (body: the dioxus-native-dom line; test-scope-summary.md: the dioxus-native-dom entry, the dioxus-native and stylo_taffy entry); §4 Unit Test Strategy (unit-test-strategy.md: a new dioxus-native-dom (the reached record) entry)
change:
  - The §1 dioxus-native-dom line names the bridge, its dispatch record and the reached record.
  - dioxus-native-dom reads 109 unit tests in twelve files (was 104 in eleven): four in `reached::tests` on the reached record, under no feature gate — a click recorded with the record on and nothing with it off, a bubbling dispatch recording its ancestor's listener of the same event, a handler whose run removes its element recorded by its id, the record's `Debug` printing its count alone — and nine on the bridge (was eight), the ninth `the_reached_record_holds_on_a_page_with_no_registered_state`.
  - reached.rs joins the dioxus-native-dom files the test search matches.
  - A new unit-test entry states the four `reached::tests` and the bridge test on a page with no registered state.
why: the report's Counts (`bridge_tests` 8 → 9, `reached::tests` 4 new) and its Coverage of new surfaces (the reached record, unit tests).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the stand reading with stand_handler_coverage
section: §3 Test Harness Contract (body: Proof, the newest reading; Test-data bootstrap of the 5-command contract); §9 CI Integration (ci-integration.md: Legs); §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: Flakiness budget)
change:
  - `run stand` reads passed 262 · failed 0 · ignored 7 over 54 files, `stand_handler_coverage` the file its `stand_` prefix picks up with no script change (was 244 · 0 · 7 over 52 files, `stand_graph` the newest), in the newest reading and in the bootstrap line.
  - The stand-repeat leg runs one `--test` per `stand_*.rs` file over 54 files (was 53), `stand_handler_coverage` joining by its prefix.
  - The flakiness budget's stand-repeat runs the 54 `stand_*.rs` files (was 53); with `stand_handler_coverage`, entering through the existing glob, CI#42530780903 green 17/17.
why: the report's Counts (`stand_*.rs` files 53 → 54, `run stand` 262 · 0 · 7 over 54 files), its Harness surface (a new `stand_*` target picked up by its prefix, no script, workflow or leg-list change) and its CI reading at the pre-CI commit, green 17/17.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: host_log's coverage run
section: §1 Test Scope Summary (test-scope-summary.md: the apps entry, the flight booker's select and the host's part read); §5 Integration Test Strategy (integration-test-strategy.md: the sink-host entry, a new Session host ↔ what it writes, a coverage run entry); §9 CI Integration (ci-integration.md: the by-level measurements)
change:
  - seven_guis' `host_log` holds 6 tests (was 5); its fourth, `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace`, runs `coverage crud` over a CRUD session kept through the binary with a typed sentinel and a row click and reads 0 id, name, coverage, sentinel, path and recorded needles on stderr at `trace`, under the `-p seven_guis` and the workspace build.
  - The paged Cells part read through the host's own sink is `host_log`'s fifth test (was its fourth), in §1, §5 and §9.
  - seven_guis' shared test module gains `coverage_needles()`, the CRUD screen's needles read from its UI graph, called by `host_log` alone.
  - §9's by-level record adds that `host_log`'s coverage run reads 0 needles under both builds.
why: the report's Counts (`host_log` 5 → 6, `coverage_needles()` called by `host_log` alone), its Outcome (obs: the coverage run green under the package and the workspace build) and its deviation 3 (the CRUD screen, not the Counter); the per-test ordinals the report held unchanged were disproved by this wrap's citation check, the new test standing fourth in the file.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: cli_commands and the coverage role
section: §1 Test Scope Summary (test-scope-summary.md: the apps entry); §4 Unit Test Strategy (unit-test-strategy.md: the schema-driven entry); §6 E2E Test Strategy (e2e-test-strategy.md: the Command line entry, a new Command line, handler coverage entry)
change:
  - seven_guis' `cli_commands` reads 18 tests (was 17), in §1, §4 and §6; the new `coverage_replays_a_kept_check_and_lists_the_handlers_it_left` runs the binary's `coverage` role over a Counter check the binary kept and reads the handlers it left in the one coverage line on stdout, status `0`.
  - `cli_commands` reads the usage line naming `--record` on `start`, the host role, the `mcp` role and the `coverage` role (`coverage <app> <check>...`), a misplaced `--record` and `coverage nowhere <dir>` each a usage error, status `2` (was the host and `mcp` roles, a misplaced `--record` alone the usage error).
why: the report's Counts (`cli_commands` on linux 17 → 18; the usage-error rows 9 → 10 with `coverage nowhere <dir>`), its Symbols (the usage line gains its coverage role clause) and its Outcome (matrix#v030-07, met through the binary).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: stand_handler_coverage and its tables
section: §1 Test Scope Summary (test-scope-summary.md: the escher-driver entry); §2 Test Strategy (test-strategy.md: Recorded-check replay); §5 Integration Test Strategy (integration-test-strategy.md: a new Session ↔ held instance, handler coverage entry); §7 Test Data & Fixtures (test-data-fixtures.md: the Graph drives row, a new Handlers at boot and unexercised after drives row)
change:
  - `stand_handler_coverage` (8) holds `escher_driver::handler_coverage` over the six served tasks in both layout modes: a snapshot check listing exactly `HANDLERS_AT_BOOT`, equal to the graph's reading at boot; each `GRAPH_DRIVES` task's drives leaving its `UNEXERCISED_AFTER_DRIVES` row; two checks listing only what neither exercised; the load page reading its click exercised; a refused step exercising nothing and a keyboard activation its click; a handler whose step removes its element reading exercised; a typed sentinel never reaching the coverage line; a check of another app contributing nothing and not holding.
  - `handler_coverage` is covered by `stand_handler_coverage`, not by a unit test (§1).
  - `Check::replay` runs through the crate-private `Check::replay_each` with a no-op hook, `handler_coverage` its second caller: it replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step and lists the handlers no check exercised, so the recorded checks also measure what they leave unexercised.
  - `GRAPH_DRIVES` is read by `stand_graph` and by `stand_handler_coverage`, whose drives check covers all 7 tasks (was `stand_graph` alone).
  - A new fixtures row states `HANDLERS_AT_BOOT` (six tasks) and `UNEXERCISED_AFTER_DRIVES` (seven), the named handlers stated by hand, Cells' 676 grid-cell `click`s of its 678 handlers at boot generated by `grid_cells()`, `CELLS_CLICKED_BY_DRIVES` naming the two the drives click, held equal to the graph's own reading on a fresh boot.
why: the report's Outcome (matrix#v030-07 met by `stand_handler_coverage`'s checks), its Symbols (`handler_coverage`, `Check::replay_each`) and its deviation 2 (Cells' rows partly generated, measured once and reconciled by hand).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the screens handler coverage reads
section: §5 Integration Test Strategy (integration-test-strategy.md: a new Handler coverage's screens entry)
change:
  - Handler coverage reads every screen the replays reach, the UI graph read at boot and after each replayed step, stated with one gap marker: OPEN — the screens handler coverage reads, a delegate's call · owner: the epoch-boundary PROVISIONAL discharge.
why: the overseer's technical call as the founder's delegate at P4 Q2, relayed by the operator (inputs#I4), PROVISIONAL until the founder's own word discharges it at the epoch boundary (`rul-3ndrhilc`); the report's Decisions & corrections place it at test-plan §5.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: handler coverage's stated limits
section: §2 Test Strategy (test-strategy.md: Claims mapped to checks); §5 Integration Test Strategy (integration-test-strategy.md: a new Handler coverage, its stated limits entry)
change:
  - Three limits stated, each with its owner: a handler inside a canvas is not counted, deferred to 0.4.0 (the incubator's canvas plans); a handler on no screen a replay reached is not listed, by construction, with no owner; a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread, owned by a residual row, target next.
  - The capability statement's claim map gains handler coverage's three limits, the handler-coverage claim pinned by `stand_handler_coverage` and `cli_commands`.
why: the operator's own approval on P5 review card 2 (inputs#I5) of its three Beyond the plan rows — the canvas deferral to 0.4.0, a new residual row for the `stopPropagation` limit, the limit by construction with no owner — and the report's New text (the three limits added to docs/capabilities.md).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the kept actions step's replay, measured
section: §1 Test Scope Summary (test-scope-summary.md: the session entry's `stand_replay`); §5 Integration Test Strategy (integration-test-strategy.md: A kept actions step's replay); contracts/session-lifecycle (session-proof)
change:
  - The replay of a kept `actions` step is measured (was owed, held by construction and unmeasured, a CARRY on «Handler coverage»): `stand_replay`'s `a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes` replays a recorded Counter session holding two `actions` steps on a fresh Counter, all 4 steps running, and the copy with one `keyed` flipped stops at step 3, keys exactly `["nodes"]`, in both layout modes, discharging the CARRY (`cry-sxrivdk3`).
  - `stand_replay` reads 4 tests (was 3), in §1 and in session-proof.
why: the report's Outcome (matrix#v030-06 met, discharging CARRY `cry-sxrivdk3`) and its Counts (`stand_replay` 3 → 4).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: escher-driver's unit tests with the coverage line and role
section: §1 Test Scope Summary (test-scope-summary.md: the escher-driver entry); §4 Unit Test Strategy (unit-test-strategy.md: the schema-driven entry, escher-driver (the `actions` read), a new escher-driver (the coverage line and role) entry); contracts/session-lifecycle (session-proof)
change:
  - escher-driver reads 104 unit tests in seventeen files, 67 inline and 37 in four `#[path]` files (was 102, 66 and 36): json.rs 8 (was 7), a coverage line keyed per handler, an absent owner left out; cli_tests.rs 6 (was 5), the `coverage` role reading its checks — a missing file, a non-check, one past the bound and another app's check each refused `malformed`, beside a failing boot and an accepted two-check run.
  - The command line's rule table reads 109 rows, the `coverage` role's among them (was 102 in §1, 61 in §4).
  - session-proof reads escher-driver's 104 unit tests (was 100), its per-file list gaining actions.rs 2, cli_tests.rs 6 and json.rs 8.
  - A new unit-test entry states the coverage line's form, `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent, and the role's refusals, its scratch files under the workspace `target/tmp`.
why: the report's Counts (`cli_tests.rs` 5 → 6, `json.rs` 7 → 8, the line table 102 → 109 rows) and its deviation 5 (the two unit tests beyond the plan's list).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the blitz-tests shared modules and targets
section: §2 Test Strategy (test-strategy.md: Directory pattern)
change:
  - `tests/common/mod.rs` is read through `mod common;` by 24 `stand_*.rs` checks (was 23) and `tests/session_common/mod.rs` by 35 (was 34), `stand_handler_coverage.rs` the latest.
  - blitz-tests holds 161 `.rs` files, 160 test targets and the unbuilt `all.rs` (was 160 and 159), `stand_handler_coverage.rs` the latest auto-discovered target with no manifest line.
why: the report's Counts (`mod common;` declarers 23 → 24, `mod session_common;` declarers 34 → 35, one new `stand_*.rs` file).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the session binary's coverage role
section: contracts/session-lifecycle (session-binary)
change:
  - The usage line also reads `coverage role: coverage <app> <check>...`.
  - The `coverage` role — `coverage <app> <check>...`, no `--session` or `--record` word among its paths, a ratified boundary widening — reads each named kept check bounded and through `Check::from_bytes`: accepted, the coverage line on stdout, status `0`; a file it cannot read, one past the bound, one that is no check or a check of another app refused `malformed`, status `1`, holding nothing of the path or the file; any other shape usage, `2`; a failing boot its session error, `3`.
why: the founder's own answer at P4 Q1 choosing the command-line role as handler coverage's carrier, relayed verbatim by the overseer (inputs#I4), a boundary widening (`rul-izni57qv`); confirmed at this wrap's halt (answer card Q1) by the overseer as delegate, PROVISIONAL, superseded by the founder's own word in the batch at the epoch boundary; the report's Symbols state the role's shape, bounded read and endings.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
