proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 34
    section: "## Threat Model Summary"
    change: "In the trust-boundary sentence replace «A result carries ids, accessible names, control values (a password's or a file input's as the mask) and the snapshot text to the client — a boundary widening ratified by the founder (2026-10-10, …)» with «A result carries ids, accessible names, control values (a password's or a file input's as the mask), the snapshot text and an acting call's diff to the client — a boundary widening ratified by the founder (2026-10-10, …) — and, since 2026-10-19-visual-lint, a `lint`'s findings (stable ids, closed defect words and numbers, written `{\"findings\":[…]}`), a ratified boundary widening», the ratification's parenthetical and the rest of the line (the live trial, the cold-agent carriage) unchanged"
    sidecar: "threat-model-summary boundary-mcp-stdio: the MCP result's carried list names an acting call's diff and, since 2026-10-19-visual-lint, a lint's findings (mcpcarry, 2026-10-25-capability-statement)"
    rationale: "The report's Spec claims disproved (`mcpcarry`) measures the row naming ids, names, control values and the snapshot text, the diffs only inside its cold-agent sentence and no lint finding, while an MCP result carries snapshot text and diffs and, since 2026-10-19-visual-lint, a lint's findings (the escher-driver row; docs/capabilities.md «What an MCP client carries onward»); the diff on an accepted MCP tool result is already inside the 2026-10-10 crossing per the Input Validation `id` row, so it rides that ratification, and the findings ride the visual-lint widening"
    basis: .andromeda/security-plan-detail/threat-model-summary.md:34
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 27
    section: "## Threat Model Summary"
    change: "In the local-stdio vector replace «a result carries ids, names, control values (a password's or file input's masked), snapshot text and screenshots to the client» with «a result carries ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings to the client», the rest of the line unchanged"
    sidecar: "security-plan body §Threat Model Summary local-stdio vector: the carried set names diffs and a lint's findings, following the boundary-mcp-stdio detail (mcpcarry)"
    rationale: "The broad line the boundary-mcp-stdio group details lists the same carried set without diffs or lint findings; the report's `mcpcarry` correction to the detail site (threat-model-summary.md:34) moves it with it"
    basis: .andromeda/security-plan-detail/threat-model-summary.md:34
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 35
    section: "## Threat Model Summary"
    change: "Add after the row's last line, in its continuation shape: « Since 2026-10-25-capability-statement a committed public document, `docs/capabilities.md` (its section «What an MCP client carries onward»), states this crossing: the boundary is the spawning client holding the child's stdio, with no authentication; the carried set is ids, names, masked values, the snapshot text and diffs, a shot as an `image` block painted masked and a lint's findings, and no description is carried; what the client does onward is the client's — a statement of the ratified crossing, adding no carrier (as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md)»"
    sidecar: "threat-model-summary boundary-mcp-stdio: docs/capabilities.md now states the MCP crossing, its carried set and the client's onward use, adding no carrier (F1, 2026-10-25-capability-statement)"
    rationale: "The task's F1; the report's Outcome (security) marks the MCP section met — boundary, carried set, no description, onward use the client's — and its guard entry reads no carrier, verb, field, exit, log field or span added"
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 217
    section: "## Input Validation"
    change: "Replace «the witness owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)» with «the witness owned by its `.andromeda/residuals.md` line, `re-carried:0.2.0`, which this version does not reach (escher-0.2.0/requirements.md:42-43), on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)», the rest of the line unchanged"
    sidecar: "input-validation aria-states row: the AT witness's owner reads its residuals line re-carried:0.2.0, not «(target: next)» (atowner, 2026-10-25-capability-statement)"
    rationale: "The report's Spec claims disproved (`atowner`) names this site among the three outside the pattern's reach, each changed: the residuals line reads `re-carried:0.2.0` and escher-0.2.0/requirements.md:42-43 owes the witness at an active assistive technology, which this version does not reach"
    basis: .andromeda/residuals.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 194
    section: "## Input Validation"
    change: "Replace «that witness owned by a `.andromeda/residuals.md` line (target: next)» with «that witness owned by its `.andromeda/residuals.md` line, `re-carried:0.2.0`, which this version does not reach (escher-0.2.0/requirements.md:42-43)», the rest of the line unchanged"
    sidecar: "input-validation id row: the adapter witness's owner reads its residuals line re-carried:0.2.0, not «(target: next)» (atowner, 2026-10-25-capability-statement)"
    rationale: "The same owner fact as the report's `atowner` sites: this clause names what the platform adapter hands an assistive technology and owns that witness «(target: next)», the residuals line now reading `re-carried:0.2.0`; it is neither among the report's no-change rows (the still-owed clauses, the actionable-key owner phrase) nor its listed changed rows, a site its `(target: next)` filter by adapter or assistive technology also fits"
    basis: .andromeda/residuals.md:14
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/secret-management.md
    near: 15
    section: "## Secret Management"
    change: "Add after the row's last line, in its continuation shape: « Since 2026-10-25-capability-statement the committed capability statement `docs/capabilities.md` and its pin `.github/scripts/test_capability_statement.py` hold no credential, token, auth header or host home path (the chunk's secret search over both: 0 hits), and the pin reads by path from the repository root, writes nothing and reads no environment variable — no new secret source (as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md)»"
    sidecar: "secret-management: the committed capability statement and its pin hold no credential and the pin reads no env var (F2, 2026-10-25-capability-statement)"
    rationale: "The task's F2; the report's Coverage of new surfaces (secret search entry: 0 hits over the statement and the pin), its Outcome (security, entry 4: 0) and its Symbols line (the pin reads no env var) — beside this row's clause that committed live evidence holds no key, auth header or token"
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "Replace «its remedy, the cause set (nine) and the cause names unchanged» with «its remedy, the cause set (eight) and the cause names unchanged», the rest of the line unchanged"
    sidecar: "error-handling Refusal row: the cause set restated as eight, not nine (causes9, 2026-10-25-capability-statement)"
    rationale: "The report's Spec claims disproved (`causes9`): `CAUSES: [Cause; 8]` at packages/escher-driver/src/refusal.rs:35 and the pin's `cause_names()` read eight; the same row's opening already says the set is closed at eight"
    basis: packages/escher-driver/src/refusal.rs:35
note: input-validation.md:194's owner clause is a site the report's atowner rows do not list (proposed as a dependent); error-handling.md:8's «the rest, seven … by the executor» beside two by validate sums past eight — not measured by the report, left for Validate.
