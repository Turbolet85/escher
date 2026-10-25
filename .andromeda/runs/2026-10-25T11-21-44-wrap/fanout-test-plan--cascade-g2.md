proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 29
    section: "## 4. Unit Test Strategy"
    change: "the CI-scripts leg runs 104 tests over eight test files — test_ci_workflows.py's 33, test_wpt_diff_to_pr.py's 7, test_agent_run.py's 14, test_cold_agent.py's 33, test_wpt_area_changes.py's 5, test_blitz_tests_targets.py's 1, test_coverage_floor.py's 7 and test_capability_statement.py's 4 — as measured at 2026-10-25-capability-statement (the `ci-leg.sh fast` leg's `ci-scripts` part: `Ran 104 tests`, `OK`) and green at CI#40184674449 on `e7e71221`; 100 over seven files, as measured at 2026-10-15-cold-agent-test (`ci-leg.sh ci-scripts`: `Ran 100 tests`, `OK`; test_cold_agent.py 27 → 33) and green at CI#38713461945 on `539defba`; … (the older dated readings kept as written)"
    sidecar: "unit-test-strategy: the CI-scripts leg's tally 100 tests over seven files → 104 over eight, test_capability_statement.py's 4 added to the per-file list; the 2026-10-15 dated reading kept"
    rationale: "Report Counts / qualifiers moved, pattern `ciscripts`: `.andromeda/test-plan-detail/unit-test-strategy.md:29` @1002/@1017 («the CI-scripts leg runs 100 tests over seven test files», the per-file list) changed, @1303 (the dated 2026-10-15 measurement) no-change; the fast leg's `ci-scripts` part read `Ran 104 tests` · `OK` (evidence/counts.md), CI#40184674449 green 17/17"
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 30
    section: "## 4. Unit Test Strategy"
    change: "- **Capability statement pin:** test_capability_statement.py — `CapabilityStatementTest`, 4 tests: every `VERBS` verb (`packages/escher-driver/src/schema.rs`, lower-cased) and every `Cause::name` match arm (`packages/escher-driver/src/refusal.rs`) appears in `docs/capabilities.md` as a code span; the statement gives five counts as number words read from the schema's tables — `{word} verbs`, `{word} refusal causes`, `{word} session errors` (`SESSION_ERRORS` in `error.rs`), `{word} node keys` (`NODE_FIELDS`) and `{word} exit statuses` (`EXIT_STATUSES`), none hand-copied; and README.md holds «the {word} driver commands as tools» (.github/scripts/test_capability_statement.py:75-104; .github/scripts/test_capability_statement.py:76-81; .github/scripts/test_capability_statement.py:83-88; .github/scripts/test_capability_statement.py:90-100; .github/scripts/test_capability_statement.py:102-104)"
    sidecar: "unit-test-strategy: new entity — the capability-statement pin test_capability_statement.py, 4 tests tying docs/capabilities.md's names and five counts and README's MCP tool count to the schema's tables"
    rationale: "Report Symbols / APIs and Harness / gate surface: `.github/scripts/test_capability_statement.py` (108 lines) holds `CapabilityStatementTest`'s four cases and joins the `ci-scripts` leg; New text, by line lists the class at 75-104 and its four cases at 76-81, 83-88, 90-100, 102-104; Deviations: the count case reads five tables, not three"
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 30
    section: "## 4. Unit Test Strategy"
    change: "- **Capability statement pin — its reading:** it reads by path from the repository root, writes nothing and reads no env var, and no helper carries a `test_` prefix; `prose` folds markdown whitespace so a phrase that wraps a source line still matches, and each case asserts the phrase is in the text with the phrase as its failure message (.github/scripts/test_capability_statement.py:41-43); its red-before-green controls are recorded in escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/pin-controls.md"
    sidecar: "unit-test-strategy: new entity — the capability-statement pin's reading discipline (by path, no write, no env read, whitespace folded, the phrase as the message) and its recorded controls"
    rationale: "Report Symbols / APIs (the pin reads by path from the repository root, writes nothing, reads no env var; no helper carries a `test_` prefix; `prose` folds whitespace); Deviations and Decisions: its first run read «thirteen node keys» red on a line wrap, the pin folds whitespace since and asserts `phrase in text` with the phrase as its message; Coverage of new surfaces: controlled red-before-green (evidence/pin-controls.md); New text, by line: `prose` at 41-43"
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 217
    section: "## 4. Unit Test Strategy"
    change: "- **Capability statement:** test_capability_statement.py pins `docs/capabilities.md` to the Rust schema's tables — every verb and refusal cause named as a code span, the verb, cause, session-error, node-key and exit-status counts stated in words — and README.md's count of the driver commands an MCP client gets as tools"
    sidecar: "test-plan §4: a CI Python script kind added — the capability-statement pin over docs/capabilities.md and README.md, beside the workflow invariants, the blitz-tests target pin and the contract tests"
    rationale: "Report Harness / gate surface: `.github/scripts/test_capability_statement.py` joins the `ci-scripts` leg (`python3 -m unittest discover -s .github/scripts`); Expected amendments: test-plan §4 — `test_capability_statement.py` joins the CI scripts; the body's §4 lists each escher-written CI-script test kind and names none for it"
note: mocking-stubbing-discipline#3.1 left unproposed — the new .github/scripts/*.py file sits in its searched scope, but the report (Symbols / APIs; Untouched in the task) shows no stand-in or mocking import in the pin.
