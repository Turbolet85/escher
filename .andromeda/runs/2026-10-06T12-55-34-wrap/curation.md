# Curation — 2026-10-06-accessibility-tree-identity

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + a11y.md: "On the dev host the AT-SPI bus reads `IsEnabled` false, so AccessKit's `update_if_active` never runs the tree build in a windowed boot — a windowed smoke proves the boot only; prove tree output headlessly …" (confidence 0.8)
    Proof: `busctl --user get-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled` → `b false`; the windowed `seven_guis_native` (accessibility forced) stayed up for 15 s with the closure never exercised — escher-0.1.0/chunks/2026-10-06-accessibility-tree-identity/evidence/smoke.md. Signals: verified by measurement +0.4 · specific technical detail +0.2 · no other durable home +0.2 (no master, route annotation, playbook rule or matrix note carries it).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 1 task-specific · 0 conflict · 0 deferred
    - dup (generated body): "a feature-gated item can pass under `cargo test --workspace` and be compiled out under `-p blitz-tests` when the workspace pins `default-features = false`" — this wrap's cascade wrote it into `.claude/rules/a11y.md`'s generated body, and P2 amended it into architecture (Dioxus DOM bridge) and a11y-plan §1; its score is 0.6 (measured +0.4, detail +0.2) and neither conditional signal applies (a master owns it) → reject.
    - task-specific: "Dioxus spells the attributes `r#for` and `aria_label`" — one-off (0.2 − 0.3).
    - rejected below threshold: "the stamp-ahead hook refuses an estimated time; read `date -u`" — one-off, enforced by the hook itself (0.4 − 0.3).
  Recurrence (→ handoff Deferred learnings): "The project's Bash guards refuse a heredoc written to a file and a leading cd" (Tier 3, 2026-10-06) — a `cat > scope-record.md <<EOF` call was refused whole at /implement P1 this session (the fourth recurrence).
  No-other-home: "On the dev host the AT-SPI bus reads IsEnabled false …"
