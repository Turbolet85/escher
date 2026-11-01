proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 157
    section: "## 1. Test Scope Summary"
    change: In the `stand_settle` clause, after «a load in flight reads `NotSettled(Loads)` until the check answers it, its `@import` included» insert «(a failed load ends the wait, read by `settle.rs`'s own unit test)», the rest of the line as it stands.
    sidecar: test-scope-summary — the `stand_settle` clause now says a failed load ends the wait, `stand_settle`'s own readings unmoved.
    rationale: report.md:49 reads `test-scope-summary.md:157` changed, a failed load ends the wait; report.md:25 has `CountedHandler` forwarding `NetHandler::error` and `settle_waiting` unedited. The clause's in-flight reading stays true of a load still in flight, so the edit adds and removes nothing of it.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:157
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 180
    section: "## 1. Test Scope Summary"
    change: Add a row following the entity — - **tests/blitz-tests — a failed load and the wait:** a failed load now ends the harness and driver wait, `CountedHandler` forwarding `NetHandler::error` and `settle_waiting`, unedited, returning where it would otherwise have run to `LOAD_WAIT`, so a failed stylesheet no longer reads `NotSettled(Loads)` for good on a harness document or on the driver; red before and green after, through `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it` (packages/blitz-test-harness/src/settle.rs:121-125; packages/blitz-test-harness/src/settle.rs:456-473) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: test-scope-summary — new row, a failed load ends the harness and driver wait through the counter's forward of `NetHandler::error`.
    rationale: report.md:25 and report.md:85 state the forward, the unedited `settle_waiting` and the red-before reading; report.md:103 names the test. Line 157 stands at 1900 characters, so the mechanism takes a row of its own and the line takes only the short clause.
    basis: packages/blitz-test-harness/src/settle.rs:121-125
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 228
    section: "## 1. Test Scope Summary"
    change: Add a row following it — - **tests/blitz-tests — `key_press_after_container_lost_its_box`:** 3 tests, its own target with no `stand_` prefix, each in both layout modes — a key press with focus on a control whose anonymous block was dropped does not panic for `hidden`, `style=display:none` and CSS alone, a select's choice still moves past the dropped block, and a control in no anonymous block reads the same; run by the workspace `test` leg and `run all`, not by `run stand`, the `stand-repeat` leg or the `a11y` leg (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-134; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:162-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: test-scope-summary — new row for the `key_press_after_container_lost_its_box` target, 3 tests in both layout modes.
    rationale: report.md:27 names the check, its 3 tests and no `stand_` prefix; report.md:23 and report.md:139 the three ways and the moved choice; report.md:36 and report.md:60 place it in the workspace tally and in no leg, the a11y and stand-repeat tallies unmoved.
    basis: tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 240
    section: "## 1. Test Scope Summary"
    change: The run clause reads «run as `select_popup` is, a named `--test` of the `stand-repeat` leg, and, since 2026-11-01-upstream-sync-ui-graph, with `--features blitz-paint/font-embolden` as the `embolden` leg, the last step of `fast`, with no CI job (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29)», the rest of the entity as it stands.
    sidecar: test-scope-summary — `select_popup_painted` is now also the `embolden` leg, last in `fast`, on the `font-embolden` build.
    rationale: report.md:64 adds the leg `embolden`, `--test select_popup_painted` under `blitz-paint/font-embolden`, in `LEGS` and last in `FAST_LEGS`, no CI job; report.md:143 and report.md:147 read it met. «Run as `select_popup` is» no longer says all, since `select_popup` is in no such leg.
    basis: .github/scripts/ci-leg.sh:28-29
note: F3 to F7 stay with the new-detail pass, no entity of this listing can carry them; the report also moves `test-scope-summary.md:8`, `:55`, `:100` and `:270` and the task names `test-plan.md:30` and the §3 pump semantics at `test-plan.md:133`, none of them in this listing, so check another group holds them; `select_popup`'s retired row-3 skip is `integration-test-strategy.md:68`, not this category's.
