proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 261
    section: "## Standard Contracts"
    change: "In the blitz-tests target contract's closing clause, «run by the same leg, which discovers seven test files since 2026-10-14-quality-gates, six until then» → «run by the same leg, which discovers eight test files (seven before `test_capability_statement.py` joined, six before 2026-10-14-quality-gates)»."
    sidecar: "standard-contracts CI contracts: the `ci-scripts` leg's discovery seven → eight test files, `test_capability_statement.py` the eighth."
    rationale: "The report's Counts / qualifiers moved (pattern `ciscripts`) measures the `ci-scripts` leg at 104 tests over eight test files (`Ran 104 tests` · `OK`, evidence/counts.md) and names `.andromeda/architecture-detail/standard-contracts.md:261` («which discovers seven test files since 2026-10-14-quality-gates») changed; Harness / gate surface: the pin joins the leg through `python3 -m unittest discover -s .github/scripts` (.github/scripts/ci-leg.sh:27)."
    basis: .andromeda/architecture-detail/standard-contracts.md:261
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 261
    section: "## Standard Contracts"
    change: "After the blitz-tests target contract's clause, add: «; the capability-statement pin `.github/scripts/test_capability_statement.py` (`CapabilityStatementTest`, four cases, stdlib `unittest`) pins `docs/capabilities.md` and README.md to the driver schema's tables — it reads the verb names from `VERBS` in `packages/escher-driver/src/schema.rs`, the cause names from `Cause::name`'s match arms in `refusal.rs`, and the lengths of `SESSION_ERRORS` (`error.rs`), `NODE_FIELDS` and `EXIT_STATUSES` (`schema.rs`), and asserts every verb and every cause a code span of the statement, each count stated in words (`{word} verbs` · `{word} refusal causes` · `{word} session errors` · `{word} node keys` · `{word} exit statuses`, the markdown's whitespace folded so a phrase may wrap a line) and README.md holding «the {word} driver commands as tools»; it reads by path from the repository root, writes nothing and reads no env var, run by the `ci-scripts` leg through its discovery (.github/scripts/test_capability_statement.py:13-34; .github/scripts/test_capability_statement.py:52-57; .github/scripts/test_capability_statement.py:60-65; .github/scripts/test_capability_statement.py:68-72; .github/scripts/test_capability_statement.py:75-104; README.md:48)»."
    sidecar: "standard-contracts CI contracts: the capability-statement pin `test_capability_statement.py` added beside the other CI-script pin contracts."
    rationale: "F3: the report's Symbols / APIs names the pin module's helpers (`verb_names`, `cause_names`, `table_length`, `prose`) and `CapabilityStatementTest`'s four cases, reading by path from the repository root, writing nothing and reading no env var; Harness / gate surface: it joins the `ci-scripts` leg with no leg, job or workflow line changed; New text, by line lists those line ranges and README.md:48."
    basis: .github/scripts/test_capability_statement.py:75-104
    fact: F3
note: the body's broad CI contracts line (.andromeda/architecture.md:193) names no CI-script pin by file, so it needs no edit for F3; F1 (project-intent) and F2 (new, Directory structure) are outside this group.
