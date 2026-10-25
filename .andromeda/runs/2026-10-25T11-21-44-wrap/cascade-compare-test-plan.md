# Cascade compare — test-plan · wrap · 2026-10-25-capability-statement
E items 6 · held 3 · lost at classify 2 · lost at strong 0 · beside a placed fact 0 · cascade only 0 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### test-data-fixtures#3.7 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 28
    section: "## 7. Test Data & Fixtures"
    change: "add a data-seeds row after the Python script entries row — | Repository text (CI-script pin) | no fixture file: `test_capability_statement.py` reads the files it pins by path from the repository root — `docs/capabilities.md` (whitespace folded, so a phrase may wrap a markdown line) and `README.md`, and the schema's tables in `packages/escher-driver/src/{schema,refusal,error}.rs` — beside an inline `NUMBER_WORDS` table (one … twenty); it writes nothing and reads no env var | (.github/scripts/test_capability_statement.py:13-34; .github/scripts/test_capability_statement.py:37-72) |"
    sidecar: "test-data-fixtures: data-seeds row added for the capability-statement pin's data — repository text read by path and an inline NUMBER_WORDS table (2026-10-25-capability-statement)"
    rationale: "The report's Changes → Symbols / APIs names the new pin module's NUMBER_WORDS (one … twenty) and its helpers read, prose (whitespace folded), verb_names, cause_names and table_length, reading by path from the repository root, writing nothing, reading no env var; the seeds table states each test's data source and holds no row for a test whose data is the committed repository text itself."
    basis: .github/scripts/test_capability_statement.py:13-34

### unit-test-strategy#5.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 45
    section: "## 4. Unit Test Strategy"
    change: "After the Cold-agent pipe block, add a new `<!-- detail: procedure · unit -->` block holding one item: «- **Capability statement:** test_capability_statement.py — `CapabilityStatementTest`, four tests, reading by path from the repository root, writing nothing, reading no env var, no helper carrying a `test_` prefix: every verb of `VERBS` (packages/escher-driver/src/schema.rs, lower-cased) is a code span in `docs/capabilities.md`; every cause of `Cause::name`'s match arms (packages/escher-driver/src/refusal.rs) is a code span; the counts are stated in words — `{word} verbs` · `{word} refusal causes` · `{word} session errors` · `{word} node keys` · `{word} exit statuses`, each word from `NUMBER_WORDS` (one … twenty) over the verbs and causes read and the lengths of `SESSION_ERRORS` (error.rs), `NODE_FIELDS` and `EXIT_STATUSES` (schema.rs); and README.md holds «the {word} driver commands as tools»; the statement's whitespace is folded (`prose`) so a phrase may wrap a markdown line, each `assertIn` carrying its phrase as its message; controlled red-before-green (escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/pin-controls.md) (.github/scripts/test_capability_statement.py:75-104) — as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md», then `<!-- /detail -->`"
    sidecar: "test-plan §4 detail: new item for the CI-script pin test_capability_statement.py (CapabilityStatementTest, four cases pinning docs/capabilities.md's verbs, causes and table counts and README's MCP-tool count to the schema's tables)"
    rationale: "The report's Symbols / APIs and Harness / gate surface add `.github/scripts/test_capability_statement.py` (108 lines) to the `ci-scripts` leg with `CapabilityStatementTest`'s four cases and its helpers; Deviations record the five tables and the whitespace fold; Coverage of new surfaces records it controlled red-before-green (`evidence/pin-controls.md`); the category file holds one item per CI-script test file (blitz-tests targets at :36, cold-agent pipe at :42) and none for this one; New text, by line lists 75-104 for the class"
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:42

## Held

- test-scope-summary#1.1
- test-scope-summary#11.1
- unit-test-strategy#3.1

## Cascade only
