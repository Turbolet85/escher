proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 38
    section: "## Design Philosophy"
    change: "Add after line 38, under Behave like a browser, one item: `  - The default sheet's chosen-row pair on a `multiple` select, `#c8c8c8` behind `#000000`, is Chromium's inactive selected pair, 12.56:1 as `select_interaction` measures it (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:691-712)`"
    sidecar: "Design Philosophy, Behave like a browser, gains the `multiple` select's chosen-row pair as a named engine's values (2026-10-31-engine-debts)."
    rationale: "The report's Changes, Schema / config, names this new engine-default pair as Chromium's inactive selected pair, measured by `select_interaction`; line 38 records Chromium's overlay scrollbar values the same way, and the group's broad line says web semantics are written against named browser engines."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:44
note: D-arch-resources and D-arch-decisions find no violated occurrence here, and no passage of this file describes as standing an item the report's Retired bullet names.
