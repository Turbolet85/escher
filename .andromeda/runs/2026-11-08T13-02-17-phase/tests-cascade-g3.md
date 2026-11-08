@carried n=1 label=test-plan/test-scope-summary fact=fct-tz35iqmq
why: the read must be proven on the stand's served Cells and load page in both layout modes, its answer on Cells sized against a screen read in parts, and its per-verb reason for a click after a scroll agreeing with where that click lands.
quote: `stand_cells_served` 5, on the stand's own Cells (`stand::boot_cells`, a screen of 53,859 bytes in 740 lines, 6 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md) through `Session::run`: Cells walked in parts through the driver (`cells_is_walked_in_parts_through_the_driver`, v020-09's witness); committed cells reading their values and a cell past the viewport read in the last part; a click right after a `scroll` landing on the shown overlay scrollbar, so the walk takes `advance 5000` before its click
@carried n=1 run=2 label=test-plan fact=fct-optamucx
quote: the executor's verbs, its refusals and `Session::lint` are read by integration checks, not unit tests — the crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
@carried n=1 run=3 label=test-plan fact=fct-prxcxiqp
quote: (31 readers with `stand_cells_served`, which reads the module's call builders, `Acted` and `act`, and `stand_load_page`, which reads `act`, `click`, `lint`, `linted` and `snapshot`;
@carried n=1 run=4 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 244 · failed 0 · ignored 7 over 52 files, `stand_graph` a file its `stand_` prefix picks up with no script change
@carried n=2 label=test-plan/test-scope-summary fact=fct-poisqw4h
why: the flight booker's select and its two options at the select's client rect are the stand's owner-served node, which the read must list with the select as owner, and a read answered through the host must hold 0 id and 0 name needles at trace as the paged part read does.
quote: `stand_snapshot` reads the flight booker's `flight-type` as `ComboBox` "Flight type", value `one-way`, holding its two `ListBoxOption`s, one selected, each at the select's client rect (`the_flight_type_select_holds_its_two_options`, tests/blitz-tests/tests/stand_snapshot.rs:122-170); seven_guis' `host_log` reads a paged part of a served Cells screen through the `escher-session` host's own sink — `serve cells` at `RUST_LOG=trace` in `hl-cells`, a sentinel committed into a part-2 cell first, then `snapshot --part 2`, 0 id and 0 name needles under both builds
@carried n=2 run=2 label=test-plan fact=fct-jydhmppz
quote: unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id and its reserved virtual form, the actionable-key check, the bridge and its dispatch record, the UI graph reader
@binding n=1
**test-scope-summary ↔ integration-test-strategy**: the read is proven on the stand's served screens in both layout modes through a held session, Cells and the load page among them (test-plan/test-scope-summary fct-tz35iqmq), its refusals read by integration checks rather than unit tests (test-plan fct-optamucx), and a new stand check of it joins the session_common readers and the `run stand` count (test-plan fct-prxcxiqp) (test-plan fct-glytb3w3)
@binding n=2
**test-scope-summary ↔ test-data-fixtures**: the stand's one owner-served case is the flight booker's select with its options at the select's client rect (test-plan/test-scope-summary fct-poisqw4h); a covered, off-screen or further owner-served case the stand lacks comes from a minimal fixture beside it, the graph and actionable-key readers' unit tests the node model it draws on (test-plan fct-jydhmppz)
@binding n=3
**test-scope-summary ↔ ci-integration**: a read answered through the `escher-session` host keeps 0 id and 0 name needles at `RUST_LOG=trace` under both builds, as the host's paged part read does (test-plan/test-scope-summary fct-poisqw4h)
