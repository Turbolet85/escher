proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 28
    section: "## 2. Test Strategy"
    change: "Replace «the `select` model's limits in its two bullets» with «the `select` model's limits in its four bullets — the model's limits, since 2026-10-31-select-popup the highlight unread and the list's thumb and one row height, and its other limits», and after «pinned by `select_interaction` and `stand_act_select`» add «, the list a click, Enter and Space open read by `select_popup`, `select_popup_painted` and `stand_act_popup`»; the rest of the entity stands."
    sidecar: "test-strategy: the capability statement's `select` limits stand in four bullets, the list read by the three new checks."
    rationale: "The report's new-text listing for `docs/capabilities.md` adds «The highlight is unread» and «The list's thumb and one row height» between the two bullets the entity counts, and its Outcome reads `v030-17` met by `select_popup`, `select_popup_painted` and `stand_act_popup`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 52
    section: "## 2. Test Strategy"
    change: "Replace «read through `mod session_common;` by 31 checks (`stand_cells_served.rs` and `stand_load_page.rs` the latest; 29 with» with «read through `mod session_common;` by 32 checks (`stand_act_popup.rs` the latest; 31 with `stand_cells_served.rs` and `stand_load_page.rs`; 29 with»; the rest of the trail stands."
    sidecar: "test-strategy: `mod session_common;` readers 31 to 32, `stand_act_popup` the latest."
    rationale: "Report `mod-decl`: checks declaring `mod session_common;` 31 → 32, test-strategy:52 changed; its new-text listing shows `use session_common::{` at `stand_act_popup.rs` 20-23."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 53
    section: "## 2. Test Strategy"
    change: "Replace «the fifteen `stand_act_*.rs`» with «the sixteen `stand_act_*.rs`»."
    sidecar: "test-strategy: `stand_act_*` files fifteen to sixteen."
    rationale: "Report `act-files`: `stand_act_*` files fifteen → sixteen (`git ls-tree`: 15 → 16), test-strategy:53 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 53
    section: "## 2. Test Strategy"
    change: "Replace «read through `mod fixtures;` by 28 checks, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md — 22 `stand_*.rs`» with «read through `mod fixtures;` by 31 checks, counted anchored at line start, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the three since the census unnamed there; 28 as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md — 22 `stand_*.rs`»; the census's list stands as that reading's."
    sidecar: "test-strategy: `mod fixtures;` readers 28 to 31, the named split kept as the census's reading."
    rationale: "Report `mod-decl`: `mod fixtures;` 28 → 31 by `git grep -l -E '^mod {name};'`, test-strategy:53 changed; the report gives the totals only, so the named 22 and six stay dated to the census rather than restated."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 54
    section: "## 2. Test Strategy"
    change: "Replace «153 `.rs` files, 152 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the latest» with «156 `.rs` files, 155 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the latest `select_popup.rs`, `select_popup_painted.rs` and `stand_act_popup.rs`, each its own auto-discovered target with no manifest line (153 and 152 before them, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the latest», and close that parenthesis's trail as it stands by joining «(145 and 144 before» as «; 145 and 144 before»."
    sidecar: "test-strategy: blitz-tests `.rs` files 153 to 156 and test targets 152 to 155, three auto-discovered with no manifest line."
    rationale: "Report `target-files`: blitz-tests `.rs` targets 153 → 156 (`git ls-tree`); its sweep hit only citation digits and found no site in its wording, yet this entity states the count as «153 `.rs` files»; every manifest is unmoved (report Dependencies), so `all.rs` stays the one unbuilt file."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 26
    section: "## 4. Unit Test Strategy"
    change: "Replace «seven_guis' `cli_commands` (14)» with «seven_guis' `cli_commands` (15)»."
    sidecar: "unit-test-strategy: `cli_commands` 14 to 15."
    rationale: "Report `cli-count`: `cli_commands` 14 → 15, unit-test-strategy:26 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 26
    section: "## 4. Unit Test Strategy"
    change: "Replace «while the fifteen `stand_act_*` files drive it» with «while the sixteen `stand_act_*` files drive it»."
    sidecar: "unit-test-strategy: `stand_act_*` files fifteen to sixteen."
    rationale: "Report `act-files`: `stand_act_*` files fifteen → sixteen, unit-test-strategy:26 changed."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 27
    section: "## 4. Unit Test Strategy"
    change: "- **escher-driver (the list's help and `covered`):** since 2026-10-31-select-popup `schema_tests`, `refusal` and `json` pin the restated help of `click`, `type` and `press` and `covered`'s added clause, an option of a closed select lying under it, each fixed `'static` text, `VERBS` 12, causes 8 and `describe`'s count row unmoved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "unit-test-strategy: new row, the driver's pins on the restated help and `covered`'s clause."
    rationale: "Report Coverage, `driver help and covered text`: tests `schema_tests`, `refusal`, `json` pins, fixed `'static` text; Symbols, Driver: the restatement, the counts unmoved."
    fact: F14
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 39
    section: "## 4. Unit Test Strategy"
    change: "Replace «schema_tests.rs pins `type`'s help on a `multiple` select inside a standing test» with «schema_tests.rs pins `type`'s help on a select, restated at 2026-10-31-select-popup since `type` on any select focuses with no click, inside a standing test»; the rest of the entity stands."
    sidecar: "unit-test-strategy: `type`'s pinned help reads of any select, no longer a `multiple` select alone."
    rationale: "Report Symbols, Driver: `type` on any select focuses with no click, and `schema.rs`: `click`'s, `type`'s and `press`'s fixed help restated; the new-text listing for `schema_tests.rs` names no new test function."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 46
    section: "## 4. Unit Test Strategy"
    change: "Replace «its case naming the leg's fifteen named targets» with «its case naming the leg's seventeen named targets»."
    sidecar: "unit-test-strategy: the `stand-repeat` case names seventeen targets."
    rationale: "Report `repeat-names`: `stand-repeat`'s named `--test` words fifteen → seventeen (`ci-leg.sh:59-60`), unit-test-strategy:46 changed; Harness / gate surface: `test_ci_workflows.py:367` asserts them."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 82
    section: "## 4. Unit Test Strategy"
    change: "- **blitz-dom (node, a select's list):** since 2026-10-31-select-popup `node/select_list.rs` holds six inline unit tests: where a list opens, its width bounds, every edge finite, `size` read as a capped whole number, scroll bounds with least movement, and a point reading its row under scroll (packages/blitz-dom/src/node/select_list.rs:386-539; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "unit-test-strategy: new row, `node/select_list.rs`'s six inline unit tests."
    rationale: "Report Coverage: unit (`select_list` 6); its new-text listing names the six tests inside `mod tests` at 386-539."
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 227
    section: "## 4. Unit Test Strategy"
    change: "Replace «and the select's rules in `node/select.rs`» with «and the select's rules in `node/select.rs`, and the list's placement, scroll and row hit in `node/select_list.rs`»."
    sidecar: "test-plan §4: the blitz-dom node broad line names `node/select_list.rs`'s tests."
    rationale: "Report Coverage: unit (`select_list` 6); the broad line unit-test-strategy's node rows detail names `node/select.rs` alone among a select's unit tests, the task's Touches naming test-plan:227."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 30
    section: "## 5. Integration Test Strategy"
    change: "After «two readings stay as measured, each stated in `covered`'s fixed meaning» add «, which since 2026-10-31-select-popup also states that an option of a closed select lies under it: click the select first, then the option»; the two readings stand."
    sidecar: "integration-test-strategy: `covered`'s fixed meaning gains the closed-option clause."
    rationale: "Report `covered-text`: `covered`'s meaning gains a clause, integration-test-strategy:30 changed; `refusal.rs`: «an option of a closed select lies under it: click the select first, then the option», name, remedy and order unchanged."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 31
    section: "## 5. Integration Test Strategy"
    change: "Replace «(the fifteen `stand_act_*` files, §1)» with «(the sixteen `stand_act_*` files, §1)»."
    sidecar: "integration-test-strategy: `stand_act_*` files fifteen to sixteen."
    rationale: "Report `act-files`: `stand_act_*` files fifteen → sixteen, integration-test-strategy:31 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 31
    section: "## 5. Integration Test Strategy"
    change: "- **Session ↔ held instance, a select's list (since 2026-10-31-select-popup):** `stand_act_popup` 12 drives the list through a held session, in both layout modes — a `click` opening it and a `click` on a row choosing, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, two clicks on one point opening then closing, a closed option and a button under the list refused `covered`, a row off screen until `scroll`, `type` leaving no list open, a row's shot leaving it open, and a read with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423)"
    sidecar: "integration-test-strategy: new row, `stand_act_popup` on the session boundary."
    rationale: "Report Coverage: integ (`stand_act_popup` 12); its new-text listing names the twelve tests, and Outcome reads them met; this boundary's entity reads a select only through `stand_act_select`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 35
    section: "## 5. Integration Test Strategy"
    change: "Replace «`host_log` holds 4 tests and `mcp_log` 2» with «`host_log` holds 5 tests and `mcp_log` 3»."
    sidecar: "integration-test-strategy: `host_log` 4 to 5 and `mcp_log` 2 to 3."
    rationale: "Report Coverage: process (`cli_commands`, `host_log`, `mcp_log` +1 each), the task's Touches moving any text that counts them."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 38
    section: "## 5. Integration Test Strategy"
    change: "- **Session host ↔ what it writes, a select's list (since 2026-10-31-select-popup):** seven_guis' `host_log` gains a `#[cfg(unix)]` test, `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, that opens and chooses in a list through the host at `trace` under the state directory `hl-popup` with no option logged, and `mcp_log`'s popup test, `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, records nothing, leaving no `ml-popup` directory, since recording would have needed a `cfg!(unix)` site (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "integration-test-strategy: new row, the popup tests of `host_log` and `mcp_log`."
    rationale: "Report Test-only state directories (`hl-popup`, no `ml-popup`), Reverted / negative facts on `mcp_log`, Spec claims disproved `os-sensitive` on `#[cfg(unix)]`, and Outcome: `host_log`, `mcp_log` 0 needles at `trace`."
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 8
    section: "## 6. E2E Test Strategy"
    change: "Replace «`cli_commands` (14)» with «`cli_commands` (15)»."
    sidecar: "e2e-test-strategy: `cli_commands` 14 to 15."
    rationale: "Report `cli-count`: `cli_commands` 14 → 15, e2e-test-strategy:8 changed."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 10
    section: "## 6. E2E Test Strategy"
    change: "- **Command line, a select's list (since 2026-10-31-select-popup):** `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, under the state directory `cc-popup`, reads a hosted shot of an open list equal to the in-process one (examples/seven_guis/tests/cli_commands.rs:419-464; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "e2e-test-strategy: new row, `cli_commands`' hosted shot of an open list."
    rationale: "Report Coverage, `the list's pixels on the screenshot carriers`: `cli_commands` hosted shot equal to in-process; Test-only state directories: `cc-popup`."
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 12
    section: "## 6. E2E Test Strategy"
    change: "After «since 2026-10-19-session-recording `mcp_log` (2) reads a recording role's stderr at `trace` too» add «, since 2026-10-31-select-popup `mcp_log` (3) a select list opened and chosen through the role at `trace`, recording nothing»; the rest of the entity stands."
    sidecar: "e2e-test-strategy: `mcp_log` 2 to 3, its popup test."
    rationale: "Report Coverage: process (`mcp_log` +1); Reverted / negative facts: its popup test records nothing; Outcome: `mcp_log` 0 needles at `trace`."
note: The report's `target-files` found no master site for 153, but test-strategy's Directory pattern states it as «153 `.rs` files», proposed here; the 31 `mod fixtures;` readers' split is not in the report, so the 22-and-six split in test-plan §3 and test-strategy stays the census's until a measured reading names the three; F15 has no picked entity here and is the new-detail pass's.
