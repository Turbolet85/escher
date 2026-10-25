proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 27
    section: ## Standard Contracts
    change: in the adapter-crossing clause, replace «and that witness owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)» with «and that witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0` — an active assistive technology, which this version does not reach — on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)», the rest of the line unchanged
    sidecar: standard-contracts.md:27 — the windowed AT witness's residuals owner reads `re-carried:0.2.0`, not «(target: next)» (atowner, a site outside the pattern's reach)
    rationale: the report's Spec claims disproved (atowner) names `.andromeda/architecture-detail/standard-contracts.md:27` @1413 changed — the residuals line reads `re-carried:0.2.0` (`.andromeda/residuals.md:14`) and `escher-0.2.0/requirements.md:42-43` owes the witness at an active assistive technology this version does not reach; `:28` is the report's no-change (a true «still owed», no owner disposition)
    basis: .andromeda/residuals.md:14
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 149
    section: ## Standard Contracts
    change: in `covered`'s meaning sentence, replace «— the remedy, the nine causes and their names unchanged —» with «— the remedy, the eight causes and their names unchanged —», matching the same sentence's «the cause set — eight, in its order»
    sidecar: standard-contracts.md:149 — «the nine causes» corrected to eight (`CAUSES: [Cause; 8]`)
    rationale: the report's Spec claims disproved (causes9) names `.andromeda/architecture-detail/standard-contracts.md:149` changed — `CAUSES: [Cause; 8]` (`packages/escher-driver/src/refusal.rs:35`), the pin's `cause_names()` reading eight; the file's own line 147 states `Cause` closed at eight
    basis: packages/escher-driver/src/refusal.rs:35
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 261
    section: ## Standard Contracts
    change: in the blitz-tests target contract clause, replace «run by the same leg, which discovers seven test files since 2026-10-14-quality-gates, six until then» with «run by the same leg, which discovers eight test files, `test_capability_statement.py` the eighth — seven from 2026-10-14-quality-gates until it joined, six before»
    sidecar: standard-contracts.md:261 — the `ci-scripts` leg discovers eight test files (`test_capability_statement.py` joined; 104 tests)
    rationale: the report's Counts / qualifiers moved (ciscripts) names `.andromeda/architecture-detail/standard-contracts.md:261` («which discovers seven test files since 2026-10-14-quality-gates») changed — seven → eight test files, 100 → 104 tests, the fast leg's `ci-scripts` part reading `Ran 104 tests` · `OK`; Harness / gate surface: `test_capability_statement.py` joins `python3 -m unittest discover -s .github/scripts`
    basis: .github/scripts/ci-leg.sh:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 261
    section: ## Standard Contracts
    change: insert a continuation line after 261, before 262 — « the capability-statement contract — `docs/capabilities.md`, «What Escher can do», holds five sections in order (`## What it can do` · `## Stated limits` · `## Accessibility reach` · `## What an MCP client carries onward` · `## Where each claim is proven`), names every verb and every refusal cause as a code span and states its counts in words read from the schema's tables, none hand-copied — twelve verbs, eight refusal causes, fifteen session errors, thirteen node keys, four exit statuses — and README.md reads «the twelve driver commands as tools» and links it; pinned by `test_capability_statement.py` (`CapabilityStatementTest`, four cases), which reads `VERBS`, `NODE_FIELDS` and `EXIT_STATUSES` in `packages/escher-driver/src/schema.rs`, `Cause::name`'s match arms in `refusal.rs` and `SESSION_ERRORS` in `error.rs` by path from the repository root, folds the markdown's whitespace so a phrase may wrap a line, writes nothing and reads no env var, run by the same leg (.github/scripts/test_capability_statement.py:52-72; .github/scripts/test_capability_statement.py:75-104);»
    sidecar: standard-contracts.md — CI contracts gain the capability-statement contract, `docs/capabilities.md`'s counts pinned to the schema's tables by `test_capability_statement.py` in the `ci-scripts` leg
    rationale: the report's Changes — Files (new `docs/capabilities.md`, five sections; new `.github/scripts/test_capability_statement.py`), Symbols / APIs (the pin's helpers and `CapabilityStatementTest`'s four cases, read by path, writing nothing, reading no env var) and Harness / gate surface (joins the `ci-scripts` leg); the CI contracts group records each `ci-scripts` pin's contract beside the blitz-tests target contract at line 261; New text, by line lists test_capability_statement.py 52-72 and 75-104
    basis: .github/scripts/test_capability_statement.py:75-104
note: the atowner, causes9 and ciscripts corrections carry D-arch-resources as the scoped detector nearest a registry-text correction (the report files them under Spec claims disproved and Counts moved, which no scoped detector names); D-arch-decisions holds — the chunk adds no dependency, crate, runtime or library (Dependencies, Crates / modules: none), and no new socket, port, env var or crate needs registering (guard: clean)
