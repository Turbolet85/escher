proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 21
    section: "## Project Intent"
    change: "In the Front page entity, replace «ten since 2026-10-12-self-description and eleven since 2026-10-12-headless-screenshot, the page saying eleven since 2026-10-13-upstream-sync-polish-and-ship)» with «ten since 2026-10-12-self-description, eleven since 2026-10-12-headless-screenshot and twelve since 2026-10-19-visual-lint, the page saying eleven since 2026-10-13-upstream-sync-polish-and-ship and twelve since 2026-10-25-capability-statement)», and after «how to ask for `help` on both surfaces,» insert «since 2026-10-25-capability-statement a link to `docs/capabilities.md` as the statement of what Escher can do and its limits (README.md:48),»; the rest of the entity unchanged."
    sidecar: "project-intent Front page — README's MCP sentence counts twelve driver commands as tools (was eleven) and links docs/capabilities.md."
    rationale: "The report's Counts / qualifiers moved (pattern readme12, 1 standing at project-intent.md:21, changed): README.md:48 now reads «the twelve driver commands as tools» (VERBS holds twelve since 2026-10-19-visual-lint) and links docs/capabilities.md; Outcome (arch) met, entries 2, 3, 5."
    basis: .andromeda/architecture-detail/project-intent.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 21
    section: "## Project Intent"
    change: "Add after the Front page entity: «- **Capability statement:** `docs/capabilities.md` («What Escher can do», 304 lines) is escher's own statement of what 0.2.0 can do, in five sections in order — `## What it can do` · `## Stated limits` · `## Accessibility reach` · `## What an MCP client carries onward` · `## Where each claim is proven`; it states the driver's counts (twelve verbs in three levels, eight refusal causes, fifteen session errors, thirteen node keys, four exit statuses), each pinned to the schema's tables by `.github/scripts/test_capability_statement.py`; README.md:48 links it, and Viola's planning reads it through a pointer naming the repository, the path and a commit (`../viola/viola-0.2.0-incubator/research/escher-capabilities.md`, untracked and ignored in Viola). No build or doc step reads it.»"
    sidecar: "project-intent — new Capability statement entity: docs/capabilities.md, its five sections, its pin, README's link and Viola's pointer."
    rationale: "Task F1; the report's Changes → Files (docs/capabilities.md new, 304 lines, five sections), Cross-project / external claims (the Viola pointer, ignored by Viola's .gitignore:6) and Outcome (arch, v020-10) met."
    basis: escher-0.2.0/chunks/2026-10-25-capability-statement/report.md:8
    fact: F1
note: the body's broad Front page line (.andromeda/architecture.md:292) holds no count and stays true; F2 (docs/ in Directory structure) is no project-intent fact and is left to the new-detail pass.
