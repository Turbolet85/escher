proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 11
    section: "## 8. Mocking & Stubbing Discipline"
    change: "Add after this line the row: | Shell | `CountingShell`, `select_popup`'s in-file `ShellProvider` fake since 2026-10-31-select-popup, counts `request_redraw` calls, read by `opening_moving_scrolling_and_closing_each_ask_a_redraw` | (tests/blitz-tests/tests/select_popup.rs:1034-1043; tests/blitz-tests/tests/select_popup.rs:1045-1090) |"
    sidecar: "§8 Mocking: new Shell row, `select_popup`'s `CountingShell`, a hand-written `ShellProvider` fake counting redraw requests (2026-10-31-select-popup)."
    rationale: "The report's New text, by line lists `struct CountingShell` at 1034-1037, `impl ShellProvider for CountingShell` with `request_redraw` at 1039-1043 and its reader at 1045-1090, in the new file `tests/blitz-tests/tests/select_popup.rs` that Changes names; the Shell rows at lines 10-11 name no such stand-in."
    basis: tests/blitz-tests/tests/select_popup.rs:1034-1043
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness find no drift in this file; no item the report's Retired bullet names stands here, and the new row needs no change to body line 351, whose Stand-ins list already names the shell."
