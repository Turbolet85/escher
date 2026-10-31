proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "In the line's tail, replace «escher-driver 4950/5049 = 98.04 %, dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at the `Coverage report` job of CI#41192433140, read in escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)» with «escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %, floors 3 of 3 (as measured at CI#41235503227, read in escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»; the earlier readings on the line stay as history."
    sidecar: "quality-gates §10 Coverage floors: the standing per-crate reading moved to CI#41235503227 (98.04 %, 86.14 %, 96.03 %), floors unmoved, 3 of 3."
    rationale: "The report's Counts / qualifiers moved, `cover-read` and `ci-tally`, name quality-gates:16 as changed, with these three readings on CI#41235503227 (`evidence/ci.md`)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "In the leg's standing statement, replace «run the 50 `stand_*.rs` files» with «run the 51 `stand_*.rs` files»; the dated counts in the parenthesis stay as history."
    sidecar: "quality-gates §10 Flakiness budget: `stand-repeat` now runs 51 `stand_*.rs` files, `stand_act_popup` joining by its prefix."
    rationale: "The report's `stand-files` names quality-gates:22 as changed, `stand_*.rs` 50 → 51; Harness / gate surface says `stand_act_popup` joins `run stand` and `stand-repeat` by its prefix."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "In the leg's standing statement, replace «the fifteen named blitz-tests targets of its `--test` list» with «the seventeen named blitz-tests targets of its `--test` list»; the parenthesis's «with the fifteen CI#41192433140 read …» stays as history."
    sidecar: "quality-gates §10 Flakiness budget: the `--test` list of `stand-repeat` now names seventeen targets, `select_popup` and `select_popup_painted` added."
    rationale: "The report's `repeat-names` names quality-gates:22 as changed, fifteen → seventeen (`select_popup`, `select_popup_painted`; `ci-leg.sh:59-60`)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Add directly after line 22 a continuation line, one leading space as lines 15-16 hold, so line 22 stays under 2 000 chars: « With the seventeen, the job `Stand checks repeated` on CI#41235503227 read 228 · 1155 · 0 · 21, +9 · +147 over CI#41192433140's 219 · 1008 · 0 · 21, green (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»."
    sidecar: "quality-gates §10 Flakiness budget: the repeat job's newest reading, 228 · 1155 · 0 · 21 on CI#41235503227, added as a continuation line."
    rationale: "The report's `ci-tally` names quality-gates:22 as changed, stand repeat 228 · 1155 · 0 · 21 on CI#41235503227; the deltas are that reading less the 219 · 1008 · 0 · 21 line 22 states, and line 22 would otherwise reach the 2 000-char limit of the entity shape."
note: "test-plan:413's «fifteen named blitz-tests targets» moves with quality-gates:22 under `repeat-names` and lies outside this file; line 34's `clear_interaction_state_for_removed_node` (17.0) is a dated reading that the chunk's edit at document.rs:1408-1410 may have moved, the report holds no re-read, so it is left standing with no proposal."
