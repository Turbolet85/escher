proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 108
    section: "## 6. Log Coverage"
    change: "add after line 108: `- observed absent — a subscriber, log site, env read or print in the eight checks that joined the `a11y` leg at 2026-10-24-a11y-re-read (`accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`), so the widened leg's log `target/ci-logs/a11y.log` holds libtest output alone · searched: the silence census over those eight files and two modules, reading 0 (exit 1); `packages/` unchanged (as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md, its gate entry the silence census)`"
    sidecar: "log-coverage absence group: the eight checks joining the a11y leg recorded silent — census 0 over the eight files and two modules, the leg's log libtest output alone (2026-10-24-a11y-re-read)"
    rationale: "The report's Outcome (obs) criterion and its gate list record the silence census over the eight joining files and two modules reading 0 (exit 1) and packages/ unchanged; the task's F1 states the fact, and the picked absence entity at log-coverage.md:108 is the group's entry of the same shape for the previous chunk's test additions, whose own claim stays true."
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:84
    fact: F1
note: No detector fires — the chunk adds no Rust symbol, log site, span, field, telemetry dependency or user-data logging (report Symbols / APIs none, Dependencies none, Coverage line instrumentation n/a · PII n/a); log-coverage#7.3's own text is unmoved.
