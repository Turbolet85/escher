proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: "## 1. Test Scope Summary"
    change: "«100 tests in seven test files (§4; 94 until …» becomes «104 tests in eight test files (§4; 100 in seven until `test_capability_statement.py` added its 4; 94 until …» — the rest of the dated history unchanged"
    sidecar: "test-scope-summary: the ci-scripts leg's tally 100 → 104 tests, seven → eight test files (test_capability_statement.py's 4)"
    rationale: "report Changes › Counts / qualifiers moved: the `ci-scripts` leg 100 → 104 tests, seven → eight test files, pattern `ciscripts` naming `.andromeda/test-plan-detail/test-scope-summary.md:8` («100 tests in seven test files») changed; the fast leg's part read `Ran 104 tests` · `OK` (evidence/counts.md)"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: "## 1. Test Scope Summary"
    change: "after «…pins the blitz-tests manifest to one test target per file with upstream's `all` target unbuilt (.github/scripts/test_blitz_tests_targets.py:17-26),» add «the four of `CapabilityStatementTest`, which pin `docs/capabilities.md`'s verb and cause names and its five table counts to the schema's tables and README.md's MCP tool count — `docs/capabilities.md`, a document with no runtime surface, in test scope through this pin alone (.github/scripts/test_capability_statement.py:75-104),»"
    sidecar: "test-scope-summary: the CI Python scripts scope gains the capability-statement pin, docs/capabilities.md and README.md's tool count in scope through it alone"
    rationale: "task New facts F3; report Changes › Symbols / APIs (`CapabilityStatementTest`'s four cases, README.md «the {word} driver commands as tools») and Coverage of new surfaces (`docs/capabilities.md`, no runtime surface, tests unit `test_capability_statement.py`, 4); class block at New text, by line 75-104"
    basis: .github/scripts/test_capability_statement.py:75
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 11
    section: "## 1. Test Scope Summary"
    change: "«among them the contract tests of the CI install script `.github/scripts/apt-install.sh`, of the coverage floor script and of the blitz-tests manifest,» becomes «among them the contract tests of the CI install script `.github/scripts/apt-install.sh`, of the coverage floor script and of the blitz-tests manifest, the capability statement's pin over `docs/capabilities.md` and README.md,» — the rest unchanged"
    sidecar: "test-plan §1: the CI Python scripts scope names the capability statement's pin among what the leg tests"
    rationale: "report Changes › Harness / gate surface: `.github/scripts/test_capability_statement.py` joins the `ci-scripts` leg through `python3 -m unittest discover -s .github/scripts` (`.github/scripts/ci-leg.sh:27`), no leg, job or workflow line changed; CI#40184674449 green"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 48
    section: "## 1. Test Scope Summary"
    change: "«— owed, owner: its `.andromeda/residuals.md` line (target: next)» becomes «— owed, owner: its `.andromeda/residuals.md` line (`re-carried:0.2.0`); 0.2.0 does not reach an active assistive technology»"
    sidecar: "test-plan §1: the AT witness's residuals owner reads re-carried:0.2.0, not (target: next)"
    rationale: "report Changes › Spec claims disproved by measurement: the windowed AT witness's residuals line reads `re-carried:0.2.0` (`.andromeda/residuals.md:14`) and `escher-0.2.0/requirements.md:42-43` owes it at an active assistive technology this version does not reach; `.andromeda/test-plan.md:48` @458 named among the three sites outside the `atowner` pattern, changed"
    basis: .andromeda/test-plan.md:48
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 98
    section: "## 1. Test Scope Summary"
    change: "«— a gap owned since 2026-10-14-stand-a11y-assertions by a `.andromeda/residuals.md` line (target: next) on the founder's F1 …» becomes «— a gap owned since 2026-10-14-stand-a11y-assertions by a `.andromeda/residuals.md` line, which reads `re-carried:0.2.0` (0.2.0 not reaching an active assistive technology), on the founder's F1 …» — the rest unchanged"
    sidecar: "test-scope-summary: the adapter-crossing gap's residuals owner reads re-carried:0.2.0, not (target: next)"
    rationale: "the same disproved owner as test-plan.md:48 (report Changes › Spec claims disproved by measurement: the residuals line reads `re-carried:0.2.0`, `.andromeda/residuals.md:14`); the task's Touches: any category entity naming this witness's owner as `(target: next)` changes the same way — this entity's clause, outside the `atowner` pattern's reach, does"
    dependent-of: D-cascade
note: the two count-and-list proposals at test-scope-summary.md:8 edit the same line and can land as one edit; the new dated clause is worded without a chunk marker, the D-cascade at line 98 rides the body proposal at test-plan.md:48
