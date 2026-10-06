# Curation — 2026-10-06-snapshot-state-fidelity

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "Never drive a stand or harness check with a deleting or other platform-bound key — prove a state change with clicks and typed characters, and read the #[cfg] lines above a match arm before trusting a grep hit of it" (confidence 1.0)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred · 1 below threshold
```

## Applied

### Tier 2 → `.claude/rules/testing.md` `## Session Additions`
The platform-bound key rule (test authoring: what a check may press).
Proof: the stand check `a_disabled_control_reads_disabled_until_the_app_enables_it` pressed `Key::Backspace` to
re-enable the flight booker's Book button; it read green on the Linux host and RED on the fork's macOS leg — CI run
37539756517, job `Test (macos)`, at the pre-CI commit `70b9997d` — because the editor's Backspace arm carries
`#[cfg(not(target_os = "macos"))]` (`packages/blitz-dom/src/node/text.rs:330-331`). The check before the push was a
grep hit on the arm and a Linux run. The operator directed the replacement — clicks and typing only — and the fix
commit `0f944800` read green on all 16 checks (CI run 37540961596). Signals: verified by a real gate failure +0.4 ·
the operator's direction to use the other form +0.4 · a specific technical detail with its context +0.2.
Record: `escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/evidence/operator-pass.md`.

## Not applied
- **Below the threshold (exactly 0.6 → rejected):** "a dioxus-native-dom unit test built with `DocumentConfig::default()`
  has no HTML parser, so `dangerous_inner_html` renders nothing there — pin parsed markup in blitz-tests." Signals:
  falsified a plan claim by measurement +0.4 · a specific technical detail +0.2. Neither conditional signal applies:
  the fact has a durable home — this wrap amended it into test-plan §3 (Crate-local test helpers) and the crate note.
- **Task-specific (2):** the operator's direction that the agent run this chunk's operator pass (an authority given
  for one pass, never a standing convention); the plan's ban on assertions that format a node's value (a constraint
  of this chunk's plan, already in its text).

## Recurrence noted (handoff → Deferred learnings)
- `recurrence-despite-learning`: "A chunk that moves cited source lines stales the masters' file:line citations"
  (Tier 3, 2026-10-05). Partial recurrence at this wrap's P1: the report's site list named the masters citing
  `mutation_writer.rs` lines and not those citing `snapshot.rs` lines (ten citations in four masters); the detectors
  found them and the cascade sweep confirmed none stale remains.
