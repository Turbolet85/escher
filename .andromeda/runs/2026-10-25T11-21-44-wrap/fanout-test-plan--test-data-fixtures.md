proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 28
    section: "## 7. Test Data & Fixtures"
    change: "add a data-seeds row after the Python script entries row — | Repository text (CI-script pin) | no fixture file: `test_capability_statement.py` reads the files it pins by path from the repository root — `docs/capabilities.md` (whitespace folded, so a phrase may wrap a markdown line) and `README.md`, and the schema's tables in `packages/escher-driver/src/{schema,refusal,error}.rs` — beside an inline `NUMBER_WORDS` table (one … twenty); it writes nothing and reads no env var | (.github/scripts/test_capability_statement.py:13-34; .github/scripts/test_capability_statement.py:37-72) |"
    sidecar: "test-data-fixtures: data-seeds row added for the capability-statement pin's data — repository text read by path and an inline NUMBER_WORDS table (2026-10-25-capability-statement)"
    rationale: "The report's Changes → Symbols / APIs names the new pin module's NUMBER_WORDS (one … twenty) and its helpers read, prose (whitespace folded), verb_names, cause_names and table_length, reading by path from the repository root, writing nothing, reading no env var; the seeds table states each test's data source and holds no row for a test whose data is the committed repository text itself."
    basis: .github/scripts/test_capability_statement.py:13-34
note: D-tests-coverage, D-tests-framework and D-tests-obs-harness hold for this category — the pin runs under the existing `python3 -m unittest discover -s .github/scripts` ci-scripts leg with its own 4 tests, and no harness, status shape or log format changed; Retired is none.
