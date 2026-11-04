@carried n=1 label=test-plan/test-scope-summary fact=fct-tz35iqmq
why: the probe's served task beside the lean one must boot through `stand::boot_cells` or `stand::boot_load`, run in both layout modes, and on the load page wait out `LoadNet`'s 200 ms answer before reading what changed.
quote: each test in both layout modes, picked up by `run stand` and the stand-repeat leg by their prefix — `stand_cells_served` 5,
  on the stand's own Cells (`stand::boot_cells`, a screen of 47,089 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`,
  as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md) through `Session::run`: Cells walked in
  parts through the driver (`cells_is_walked_in_parts_through_the_driver`, v020-09's witness); committed cells reading their
  values and a cell past the viewport read in the last part; a click right after a `scroll` landing on the shown overlay
  scrollbar, so the walk takes `advance 5000` before its click; the lint at boot reading its measured findings — and
  `stand_load_page` 3, on the load page (`stand::boot_load`): a click on `load-fetch` waiting out the load `LoadNet` answers
  after 200 ms and `load-done` showing "Loaded"
@carried n=2 label=test-plan/test-scope-summary fact=fct-cyo4xvcu
why: the probe's lean boot must stay offline with `boot_load` alone carrying a net provider, and any probe code kept in the stand module must re-count the `stand::` unit reading.
quote: the stand module holds three more unit tests — `the_stand_options_stay_offline` (every boot but `boot_load` keeps
  `net_provider: None`), `cells_boots_in_the_shell` and `the_load_page_click_leaves_a_load_in_flight` — `cargo test -p
  seven_guis --locked --lib stand::` reading 5 passed (examples/seven_guis/src/stand.rs:283-329)
@carried n=2 run=2 label=test-plan fact=fct-4kvmkl3r
quote: library unit tests of the stand's timer step and the Timer's ticks
@binding n=1
**test-scope-summary (the served boots) ↔ test-data-fixtures and mocking-stubbing-discipline**: the probe's served task boots
  through `stand::boot_cells` or `stand::boot_load` over `stand::options(incremental)`, `boot_load` alone replacing
  `net_provider` with `LoadNet` and every other field kept, every lean boot offline (test-plan/test-scope-summary fct-cyo4xvcu)
  (test-plan fct-552qmxvp) (test-plan fct-hsnu3a5x)
@binding n=2
**test-scope-summary (the load page's 200 ms answer) ↔ integration-test-strategy (wall time)**: a probe reading on the load
  page waits out `LoadNet`'s answer through the harness's or the driver's wait, with no sleep or wall-clock read of its own,
  the 200 ms sleep being the stand's (test-plan/test-scope-summary fct-tz35iqmq) (test-plan fct-ubyqdssx)
@binding n=3
**test-scope-summary (the stand's counts) ↔ quality-gates-coverage-targets and the agent-run proof**: a probe check kept as a
  `stand_` file in both layout modes joins `run stand` and the stand-repeat leg's zero-failure budget and moves their newest
  reading, and a probe unit test kept in the stand module moves the `stand::` reading of 5 passed
  (test-plan/test-scope-summary fct-cyo4xvcu) (test-plan fct-glytb3w3) (test-plan fct-temy7g7i)
