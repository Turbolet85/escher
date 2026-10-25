proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 34
    section: "## Threat Model Summary"
    change: "In the boundary-mcp-stdio Trust boundary item, the carried list «A result carries ids, accessible names, control values (a password's or a file input's as the mask) and the snapshot text to the client» reads «A result carries ids, accessible names, control values (a password's or a file input's as the mask), the snapshot text, its diffs and a lint's findings (stable ids, closed defect words and numbers) to the client»; the rest of the item unchanged."
    sidecar: "security-plan Threat Model Summary: the MCP stdio boundary's carried list names a step's diffs and a lint's findings beside ids, names, masked values and the snapshot text (mcpcarry)."
    rationale: "The report's Spec claims disproved by measurement (pattern mcpcarry) measures the site at threat-model-summary.md:34, under the boundary-mcp-stdio label at :30: its carried list names the diffs only inside the cold-agent sentence and no lint finding, while a result carries snapshot text and diffs and a lint's findings; its Expected amendments carry this as security-plan §Threat Model Summary, the site :34, the two label rows unchanged."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:34
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 26
    section: "## Threat Model Summary"
    change: "In the boundary-session-socket Trust boundary item, the answer's carried list «an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out» reads «an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text, the diff and a lint's findings (stable ids, closed defect words and numbers) out»; the rest of the item unchanged."
    sidecar: "security-plan Threat Model Summary: the session socket boundary's answer list names a lint's findings, as the MCP row's does (mcpcarry's sibling occurrence)."
    rationale: "The same carried-list claim stands on the socket row: it names ids, names, masked values, snapshot text, diff and a shot's pixels but no lint finding, while the report's mcpcarry fact states a lint's findings cross a result since the visual lint on every carrier the report cites (CLAUDE.md escher-driver row, docs/capabilities.md); outside the mcpcarry pattern's MCP-only reach — the orchestrator confirms it against the ratified socket carrier before applying."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:26
    dependent-of: D-new-detail
note: "security-plan.md body :27 (boundary-mcp-stdio broad line: «snapshot text and screenshots») and :25 (session socket: no lint finding) carry the same omission in the body, outside this category file — the body's owner should take them; D-security-input · D-security-auth · D-security-deps pass (no new runtime input surface, no auth or secret touched, no dependency added)."
