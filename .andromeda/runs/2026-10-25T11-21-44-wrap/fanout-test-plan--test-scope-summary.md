proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: ## 1. Test Scope Summary
    change: the `ci-scripts` count reads «104 tests in eight test files (§4; 100 in seven until test_capability_statement.py's 4 joined; 94 until 2026-10-15-cold-agent-test, …» in place of «100 tests in seven test files (§4; 94 until 2026-10-15-cold-agent-test, …», the rest of the line's history unchanged
    sidecar: test-scope-summary — the CI Python scripts' tally moved 100 → 104 tests, seven → eight test files, with test_capability_statement.py's four
    rationale: the report's Counts / qualifiers moved (pattern `ciscripts`) names this row changed — the fast leg's `ci-scripts` part read `Ran 104 tests` · `OK` with `.github/scripts/test_capability_statement.py` joining `python3 -m unittest discover -s .github/scripts` (Harness / gate surface; evidence/counts.md)
    basis: .andromeda/test-plan-detail/test-scope-summary.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: ## 1. Test Scope Summary
    change: after the clause of `BlitzTestsTargetsTest` (…`.github/scripts/test_blitz_tests_targets.py:17-26)`) insert «, the four of `CapabilityStatementTest`, which pin `docs/capabilities.md` to the schema's tables — every verb of `VERBS` and every cause of `Cause::name` a code span in the statement, the verb, refusal-cause, session-error, node-key and exit-status counts stated in words as `schema.rs`, `refusal.rs` and `error.rs` hold them, and README.md holding «the {word} driver commands as tools» — reading by path, writing nothing, reading no env var (.github/scripts/test_capability_statement.py:75-104)»
    sidecar: test-scope-summary — the CI Python scripts row names CapabilityStatementTest's four pins of docs/capabilities.md and README.md against the driver schema's tables
    rationale: the report's Symbols / APIs and Coverage of new surfaces name the pin module's `CapabilityStatementTest` and its four cases as the unit tests of `docs/capabilities.md` (controlled red-before-green, evidence/pin-controls.md); its class spans 75-104 in New text, by line; the row lists the ci-scripts leg's named test classes, so the new class is new detail in its record
    basis: .github/scripts/test_capability_statement.py:75-104
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 98
    section: ## 1. Test Scope Summary
    change: the adapter-crossing gap's owner reads «— a gap owned since 2026-10-14-stand-a11y-assertions by a `.andromeda/residuals.md` line, re-carried to 0.2.0 (`re-carried:0.2.0`) and not reached in this version, on the founder's F1 …» in place of «… by a `.andromeda/residuals.md` line (target: next) on the founder's F1 …», the rest of the line unchanged
    sidecar: test-scope-summary — the AT-adapter coverage gap's residuals owner restated (target: next) → re-carried:0.2.0, which this version does not reach
    rationale: the report's Spec claims disproved by measurement (`atowner`) measures the windowed assistive-technology witness's residuals line as `re-carried:0.2.0` (`.andromeda/residuals.md:14`), owed by `escher-0.2.0/requirements.md:42-43` «at an active assistive technology, which this version does not reach», and changes every site naming that owner «(target: next)», the body's `.andromeda/test-plan.md:48` among them; this detail row names the same owner of the same gap (no check reaching the adapter crossing) in the wording family the pattern did not reach
    basis: .andromeda/test-plan.md:48
note: proposal 3 is an `atowner` site the report's site list omits (found by the same `(target: next)` + adapter filter over test-plan's detail file) — the orchestrator may re-home its detector; D-tests-framework and D-tests-obs-harness hold (unittest discover unchanged, no harness, status or log-format change).
