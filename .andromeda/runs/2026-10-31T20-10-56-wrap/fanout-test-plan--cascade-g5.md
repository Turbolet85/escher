proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 10
    section: "## 8. Mocking & Stubbing Discipline"
    change: "Add a row after the Shell row: | Shell (list redraws) | `select_popup`'s own hand-written `CountingShell`, a `ShellProvider` counting `request_redraw`, reads that opening, moving, scrolling and closing a select's list each ask a redraw | (tests/blitz-tests/tests/select_popup.rs:1034-1043; tests/blitz-tests/tests/select_popup.rs:1045-1090) |"
    sidecar: "§8 stand-ins: `select_popup`'s hand-written `CountingShell` added beside the shell fakes (F13)."
    rationale: "F13; the report's New text lists `struct CountingShell`, its `impl ShellProvider` and `opening_moving_scrolling_and_closing_each_ask_a_redraw` in the new `select_popup.rs`."
    basis: "tests/blitz-tests/tests/select_popup.rs:1034-1043"
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the stand-repeat clause, `50 files` reads `51 files`, and the prefix joiners end `... `stand_cells_served` and `stand_load_page`, and `stand_act_popup` since 2026-10-31-select-popup`."
    sidecar: "§9 Legs: stand-repeat's `stand_*.rs` count 50 to 51, `stand_act_popup` joining by its prefix."
    rationale: "Report Counts `stand-files` (`stand_*.rs` 50 → 51, test-plan ci-integration:8 changed) and Harness / gate surface (`stand_act_popup` joins `run stand` and `stand-repeat` by its prefix)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the a11y leg's membership clause, `14 files and 112 tests, over the tier-0 witnesses alone, 9 files and 59 tests` reads `14 files and 114 tests, over the tier-0 witnesses alone, 9 files and 61 tests`."
    sidecar: "§9 Legs: the a11y leg's stated tests 112 to 114, the tier-0 witnesses' 59 to 61, both by `stand_keyboard`'s +2."
    rationale: "Report `a11y-tally` 14 · 112 · 0 → 14 · 114 · 0 and `kbd-count` `stand_keyboard` 21 → 23, the leg still fourteen files; this wording lies outside that pattern, which the report says reaches its own wording and no further. 61 is arithmetic: the nine witnesses' 59 includes `stand_keyboard`, the whole +2 of the leg."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "`fifteen named targets as literal `--test` words` reads `seventeen named targets`, the list ending `... `checkable_change_event`, `text_range_no_change_event`, `select_popup` and `select_popup_painted``, and `naming the glob and the fifteen words` reads `naming the glob and the seventeen words`."
    sidecar: "§9 Legs: stand-repeat's named `--test` words fifteen to seventeen, `select_popup` and `select_popup_painted`."
    rationale: "Report Counts `repeat-names` (fifteen → seventeen, test-plan ci-integration:9 changed) and Harness / gate surface (`ci-leg.sh` gains two words at :59-60, `test_ci_workflows.py:367` asserts them)."
    basis: ".github/scripts/ci-leg.sh:59-60"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "`pinning both words beside the fifteen` reads `pinning both words beside the seventeen`."
    sidecar: "§9 Legs: the repeat leg's pin reads beside seventeen named words."
    rationale: "Report Counts `repeat-names`, test-plan ci-integration:10 changed."
    basis: ".github/scripts/test_ci_workflows.py:367"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "`none of `stand-repeat`'s fifteen named `--test` words` reads `none of `stand-repeat`'s seventeen named `--test` words`; the rest stands."
    sidecar: "§9 `pseudo_element_flow_classification`: none of stand-repeat's seventeen named words."
    rationale: "Report Counts `repeat-names`, test-plan ci-integration:16 changed; the two added words are `select_popup` and `select_popup_painted`, so the target is still none of them."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 28
    section: "## 9. CI Integration"
    change: "Add after the line: - **Red on `Test (macos)`, and its local witness:** CI#41233234207 at `8cd9c1286` read `completed · failure`, `Test (macos)` alone and sixteen success: `apple-font-embolden` turned CI red on macOS alone, no local gate building `blitz-paint/font-embolden`, so a local gate block read green over the defect; a paint change under that feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md"
    sidecar: "§9: a macOS-only red, no local gate building `blitz-paint/font-embolden`, and that feature's local witness command (F8)."
    rationale: "F8; report Decisions & corrections, Cross-project claims (CI, the first read) and Spec claims disproved `os-sensitive`. It sits beside the standing red record this entity holds."
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 28
    section: "## 9. CI Integration"
    change: "Add after the line: - **A red job's later targets:** `cargo` stops at the first failing target, so a red job's later targets go unread on that run — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md"
    sidecar: "§9: a red job's targets after the first failing one go unread on that run (F9)."
    rationale: "F9; report Decisions & corrections. It bears on how a red record like this entity's is read."
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 75
    section: "## 9. CI Integration"
    change: "Add after the line: Re-read on CI#41235503227, the final HEAD `715798ce2` (green 17/17): linux 217 · 1236 · 0 · 12; macos and coverage 198 · 1233 · 0 · 12; windows 198 · 1185 · 0 · 9; the a11y job 14 · 114 · 0 · 0; the stand repeat 228 · 1155 · 0 · 21 — +3 result lines and +55 passed on linux, macos and coverage over CI#41192433140, +3 · +53 on windows, no new result line and +2 passed on the a11y job, +9 · +147 on the stand repeat — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md"
    sidecar: "§9 Per-OS tallies: re-read on CI#41235503227, 217 · 1236 on linux."
    rationale: "Report Counts `ci-tally` and `a11y-tally` (test-plan ci-integration:75 changed); the differences are the reported tallies less this line's CI#41192433140 row."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 97
    section: "## 9. CI Integration"
    change: "Add after this re-count, before the cold total: ; re-counted with the select list's three new targets: 217 result lines, 1236 passed · 0 failed · 12 ignored, from 214 · 1181 — +3 result lines (`select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12) and +55 passed (those 43, blitz-dom's lib `select_list` 6, `stand_keyboard` +2, `stand_lint` +1, `cli_commands`, `host_log` and `mcp_log` +1 each), timings not re-measured — as measured on CI#41235503227's linux job (escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md)"
    sidecar: "§9 Local baseline: the workspace leg's re-count 214 · 1181 to 217 · 1236."
    rationale: "This line carries CI's test leg forward and read 214 · 1181 equal to CI#41192433140's linux job. Report `ci-tally` gives linux 217 · 1236 · 0 · 12, and Coverage of new surfaces names the tests, which sum to +55. The report states no local tally probe, so the source is the CI job."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 99
    section: "## 9. CI Integration"
    change: "`ci-leg.sh a11y` fourteen files reads `14 result lines · 114 passed · 0 failed · 0 ignored`, its source `as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md`."
    sidecar: "§9 Local baseline: the a11y leg's reading 112 to 114 passed."
    rationale: "Report Counts `a11y-tally` (test-plan ci-integration:99 changed) and the gate `lines 14 passed 114 failed 0`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 99
    section: "## 9. CI Integration"
    change: "In the a11y leg's per-file list, `stand_keyboard 21` reads `stand_keyboard 23`; `select_interaction 18` and `stand_act_select 10` stand."
    sidecar: "§9 Local baseline: `stand_keyboard` 21 to 23 in the a11y leg's list."
    rationale: "Report Counts `kbd-count` (`stand_keyboard` 21 → 23, test-plan ci-integration:99 changed) and `select-model` (`select_interaction` 18, `stand_act_select` 10 unmoved)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "The clause `read now under the same floors` reads: escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %, `coverage floor: 3 of 3 crates at or above their floors`, as measured on CI#41235503227, read in escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md."
    sidecar: "§10 Coverage floors: the current reading moves to CI#41235503227, floors 3 of 3."
    rationale: "Report Counts `cover-read` and `ci-tally`, quality-gates:16 changed in each."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "`run the 50 `stand_*.rs` files` reads `run the 51 `stand_*.rs` files`."
    sidecar: "§10 Flakiness budget: `stand_*.rs` 50 to 51."
    rationale: "Report Counts `stand-files`, quality-gates:22 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "`the fifteen named blitz-tests targets of its `--test` list` reads `the seventeen named blitz-tests targets of its `--test` list`; the dated clause `with the fifteen CI#41192433140 read` stands as history."
    sidecar: "§10 Flakiness budget: the named targets fifteen to seventeen."
    rationale: "Report Counts `repeat-names`, quality-gates:22 changed."
    basis: ".github/scripts/ci-leg.sh:59-60"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Open the parenthesis's history, after `§9 Legs;`, with: with the seventeen and `stand_act_popup` CI#41235503227 read 228 · 1155 · 0 · 21, +9 · +147, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md;"
    sidecar: "§10 Flakiness budget: the repeat tally 228 · 1155 · 0 · 21 on CI#41235503227."
    rationale: "Report Counts `ci-tally` (stand repeat 228 · 1155 · 0 · 21, quality-gates:22 changed); +9 · +147 is that tally less this line's 219 · 1008."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 34
    section: "## 10. Quality Gates & Coverage Targets"
    change: "After `clear_interaction_state_for_removed_node` (17.0) add: that reading taken before 2026-10-31-select-popup added the clearing of the list's field to that function, and not re-read since; the `85 functions` count is likewise that earlier tree's."
    sidecar: "§10 Complexity ceiling: `clear_interaction_state_for_removed_node`'s 17.0 and the 85 stated as read before the select list, not re-read."
    rationale: "Report The resolve: `clear_interaction_state_for_removed_node` clears the field for its select (`document.rs:1408-1410`), and the report states no new complexity reading, so it does not show that 17.0 holds on the changed function."
    basis: "packages/blitz-dom/src/document.rs:1408-1410"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 377
    section: "## 9. CI Integration"
    change: "`fifteen named blitz-tests targets outside the `stand_` prefix` reads `seventeen named blitz-tests targets outside the `stand_` prefix`."
    sidecar: "§9 body Legs: stand-repeat's named targets fifteen to seventeen."
    rationale: "Report Counts `repeat-names`, test-plan:377 changed; this is the broad line ci-integration#1.1 details."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 413
    section: "## 10. Quality Gates & Coverage Targets"
    change: "`the fifteen named blitz-tests targets` reads `the seventeen named blitz-tests targets`."
    sidecar: "§10 body Flakiness budget: the named targets fifteen to seventeen."
    rationale: "Report Counts `repeat-names`, test-plan:413 changed; this is the broad line quality-gates-coverage-targets#3.1 details."
note: "F12 has no picked entity to carry it, so the new-detail pass places it; the report's `run-all` row names test-plan ci-integration:75, yet that line as picked states no `run all` count, so no such edit is proposed there."
