proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom (document, mutator, net, form)"
    change: "Replace 'a named `select`'s entries have no test of their own' with 'the unit tests of packages/blitz-dom/src/form.rs read a named `select`'s entries, a `multiple` select's set less its disabled rows and a single select's choice through a group'."
    sidecar: "§1 blitz-dom (document, mutator, net, form): a named select's entries now hold unit tests in packages/blitz-dom/src/form.rs (2026-10-31-engine-debts)."
    rationale: "Report Crates / modules: the test module of packages/blitz-dom/src/form.rs gained two tests; its New text rows name a_multiple_select_submits_its_set_less_its_disabled_rows and a_single_select_submits_its_choice_through_a_group, so the absence the body states no longer holds."
    basis: "packages/blitz-dom/src/form.rs:558-592"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom covered behaviours"
    change: "Replace 'the single-choice `select` model' with 'the `select` model, single-choice and `multiple`, with `optgroup`', the rest of the list unchanged."
    sidecar: "§1 blitz-dom covered behaviours: the select model read by the checks is now single-choice and multiple, with optgroup (2026-10-31-engine-debts)."
    rationale: "Report Symbols / APIs F6 and F7 build a multiple select and optgroup; Outcome (F6, a11y), (F6, the click, driver) and (F7) are met by select_interaction 18, stand_act_select 10 and two more form tests, so 'single-choice' under-reads the covered model."
    basis: "escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, Symbols / APIs F6, F7"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom covered behaviours"
    change: "Replace 'while the select's are asserted by no check, nor is the process-wide `Parallel` traversal lock' with 'and the select's by `select_interaction` and `stand_act_select`; the process-wide `Parallel` traversal lock is asserted by no check'."
    sidecar: "§1 blitz-dom covered behaviours: the select's stated limits are now asserted; the Parallel traversal lock stays unasserted (2026-10-31-engine-debts)."
    rationale: "Report Expected amendments: test-plan §1 carries 'the select's limits now asserted'; Coverage of new surfaces names select_interaction and stand_act_select as the multiple select's and optgroup's checks."
    basis: ".andromeda/test-plan.md:19"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom covered behaviours"
    change: "Replace '`:open` matching a `details` or `dialog` carrying `open` has no test of its own' with '`:open` matching a `details` or `dialog` carrying `open` is read by blitz-tests' `open_state`'."
    sidecar: "§1 blitz-dom covered behaviours: the :open gap closes with open_state, 4 tests (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: the `open-state` pattern lists .andromeda/test-plan.md:19 as changed; Outcome (CARRY 2) is met by open_state 4."
    basis: ".andromeda/test-plan.md:19"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "In session-proof, replace '`stand_act_transformed` 2 (50; 48 before `stand_act_transformed`,' with '`stand_act_transformed` 3 (51; 50 until 2026-10-31-engine-debts added its third test, 48 before `stand_act_transformed`,', the older figures standing."
    sidecar: "§3 → Session lifecycle: stand_act_transformed 2 to 3, the running file total 50 to 51 (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: stand_act_transformed 2 → 3, and `n-act-transf` lists session-lifecycle.md:11 as changed; the running total adds that one test."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "In session-proof, replace '`stand_act_select` 5, the fifteenth (55 in all)' with '`stand_act_select` 10, the fifteenth (61 in all; 55 until 2026-10-31-engine-debts)'."
    sidecar: "§3 → Session lifecycle: stand_act_select 5 to 10, the driver-action total 55 to 61 (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: stand_act_select 5 → 10, and `n-act-select` lists session-lifecycle.md:11 as changed; with stand_act_transformed's +1 the total moves 55 → 61."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (fixtures)"
    change: "Replace '24 checks declare `mod fixtures;`' with '28 checks declare `mod fixtures;` (24 until 2026-10-31-engine-debts)', the split into `stand_*` files and other targets and the named readers restated as escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md reads them."
    sidecar: "§3 blitz-tests (fixtures): fixture readers 24 to 28, the split as the chunk's census reads it (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: fixture readers 24 → 28 (evidence/census.md), and `fixture-readers` lists .andromeda/test-plan.md:151 as changed; 22 plus 2 can no longer sum to the total, and the report states no split."
    basis: ".andromeda/test-plan.md:151"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Test-data bootstrap of the 5-command contract"
    change: "'`run stand` runs the 22 `stand_*` readers of the shared module, `run {file}` the two non-stand readers' takes the same new split of the 28 readers as the fixtures bullet, per escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md."
    sidecar: "§3 Test-data bootstrap: the fixture readers' split per selection follows the 28 readers (2026-10-31-engine-debts)."
    rationale: "The same claim as the fixtures bullet's 24, restated as 22 plus two; report Counts / qualifiers moved takes the total to 28, so this split cannot stand as written."
    basis: ".andromeda/test-plan.md:190"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Test-data bootstrap of the 5-command contract"
    change: "Replace '`run stand` 205 · 0 · 7 over 50 files, as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md' with '`run stand` 222 · 0 · 7 over 50 files, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md'."
    sidecar: "§3 Test-data bootstrap: run stand 205 · 0 · 7 to 222 · 0 · 7 over 50 files (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: agent-run.sh run stand 205 · 0 · 7 → 222 · 0 · 7 over 50 files, and `agent-run` lists .andromeda/test-plan.md:190 as changed."
    basis: ".andromeda/test-plan.md:190"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Legs"
    change: "The `stand-repeat` leg runs the stand checks, fifteen named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds; the rest of the bullet unchanged."
    sidecar: "§9 Legs: stand-repeat's named blitz-tests targets ten to fifteen (2026-10-31-engine-debts)."
    rationale: "Report Harness / gate surface: stand-repeat in .github/scripts/ci-leg.sh names five more targets (.github/scripts/ci-leg.sh:58-59); `ten-named` lists .andromeda/test-plan.md:377 as changed."
    basis: ".andromeda/test-plan.md:377"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Flakiness budget"
    change: "The `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the fifteen named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)."
    sidecar: "§10 Flakiness budget: stand-repeat's named blitz-tests targets ten to fifteen (2026-10-31-engine-debts)."
    rationale: "The same claim as §9 Legs; the report's `ten-named` lists .andromeda/test-plan.md:413 as changed."
    basis: ".andromeda/test-plan.md:413"
    dependent-of: D-tests-coverage
  - detector: D-new-detail
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers"
    file: new
    near: "- **blitz-tests (stand checks):** `tests/common/mod.rs` is the one statement of the tables and helpers the stand checks share"
    change: "- **blitz-tests (stand checks), Tab sequences of the served pages:** tests/blitz-tests/tests/common/mod.rs also states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, the load page's and Cells' Tab sequences."
    sidecar: "§3 Crate-local test helpers: tests/blitz-tests/tests/common/mod.rs states LOAD_TAB_ORDER and CELLS_TAB_ORDER (2026-10-31-engine-debts)."
    rationale: "Report Symbols / APIs F13 states both tables at tests/blitz-tests/tests/common/mod.rs:93-98; §3 has no category file, so the new fact is this document's proposal."
    basis: "tests/blitz-tests/tests/common/mod.rs:93-98"
  - detector: D-new-detail
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    file: new
    near: "- **Proof:** the contract tests `test_agent_run.py` (§4) and a live run"
    change: "- **Proof, re-counted with the engine debts:** scripts/agent-run.sh unchanged, `run stand`'s `run.end` reads passed 222 · failed 0 · ignored 7 over 50 files, +17 passed in files its `stand_` prefix already picks up (`stand_keyboard` +6, `stand_act_select` +5, `stand_contrast` +2, `stand_act_transformed`, `stand_lint`, `stand_load_page` and `stand_cells_served` +1 each), and `run all` reads passed 681 · failed 0 · ignored 12, the five new targets carrying no `stand_` prefix and reached through `run all` and `run {name}` alone, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md."
    sidecar: "§3 Agent-run Proof: re-counted, run stand 222 · 0 · 7 over 50 files, run all 681 · 0 · 12 (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved gives agent-run's readings and each target's move, and holds .andromeda/test-plan.md:176's rows as history, so the re-count stands beside them and leaves them unedited; the last run all those rows state is 633."
    basis: ".andromeda/test-plan.md:176"
note: "This file was written twice, the second Write only making paths repo-relative and tidying quoted wording, against the task's one-call rule. D-tests-framework and D-tests-obs-harness read no drift, and no Retired item occurs in the body or the contracts. escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md lies outside this task's paths, so the 28 readers' split is left for the orchestrator to read. Outside every detector's invariant: session-start's `type` replacing after the focusing click no longer holds for a `multiple` select (report F6, focus_multiple_select focuses with no click), and the slice s03 absence in §2 and §4 was not checked against blitz-shell's first unit test."
