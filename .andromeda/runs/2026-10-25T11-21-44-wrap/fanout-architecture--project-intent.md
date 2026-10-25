proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 21
    section: "## Project Intent"
    change: "In the Front page record, replace «ten since 2026-10-12-self-description and eleven since 2026-10-12-headless-screenshot, the page saying eleven since 2026-10-13-upstream-sync-polish-and-ship» with «ten since 2026-10-12-self-description, eleven since 2026-10-12-headless-screenshot and twelve since 2026-10-19-visual-lint, the page saying twelve», and add after the MCP clause «and the page links `docs/capabilities.md` as the statement of what Escher can do and its limits (README.md:48)»; the rest of the record unchanged"
    sidecar: "architecture §Project Intent → Front page: README counts the MCP client's tools as twelve and links docs/capabilities.md; the retired «the page saying eleven» clause corrected"
    rationale: "The report's Counts / qualifiers moved (pattern readme12, 1 standing row, project-intent.md:21, changed): README.md:48 now reads «the twelve driver commands as tools» (VERBS holds twelve since 2026-10-19-visual-lint) and links docs/capabilities.md; Expected amendments names architecture §Project Intent → Front page; Outcome (arch) README bullet met"
    basis: escher-0.2.0/chunks/2026-10-25-capability-statement/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 23
    section: "## Project Intent"
    change: "New record, in its own `<!-- detail: values · intent -->` block after the Front page block: «- **Capability statement:** `docs/capabilities.md` («What Escher can do») is escher's own statement of what it can do and its limits, in five sections in order — `## What it can do` · `## Stated limits` · `## Accessibility reach` · `## What an MCP client carries onward` · `## Where each claim is proven`; it names the twelve verbs in three levels, the eight refusal causes, fifteen session errors, thirteen node keys and four exit statuses, each count in words and pinned to the schema's tables (`VERBS`, `Cause::name`, `SESSION_ERRORS`, `NODE_FIELDS`, `EXIT_STATUSES`) by `.github/scripts/test_capability_statement.py` in the `ci-scripts` leg, which also pins README's «the {word} driver commands as tools»; the root `README.md` links it; Viola's planning reads it through an untracked, ignored pointer naming repository, path and commit (`../viola/viola-0.2.0-incubator/research/escher-capabilities.md`), so no Viola commit carries it (docs/capabilities.md; .github/scripts/test_capability_statement.py)»"
    sidecar: "architecture §Project Intent: new Capability statement record — docs/capabilities.md, its five sections, its counts pinned by test_capability_statement.py, README's link and Viola's pointer"
    rationale: "The report's Changes → Files adds docs/capabilities.md (five sections) and the pin; Symbols / APIs names the pin's four cases and the five tables it reads; Cross-project claims name Viola's ignored pointer; Outcome (arch) and (v020-10) met — the statement is the chunk's whole product-facing fact and this category holds escher's public pages (Front page)"
    basis: escher-0.2.0/chunks/2026-10-25-capability-statement/report.md:8
note: D-arch-resources and D-arch-decisions fire nothing here (report: no symbol, crate, dependency or resource added); the body's Front page broad line (.andromeda/architecture.md:292) may want «and links the capability statement» — a body edit, outside this file.
