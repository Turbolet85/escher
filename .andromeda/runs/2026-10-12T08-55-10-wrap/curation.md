CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "`cascade.py sweep --pattern` takes its ids after one flag"
    Proof: measured at this wrap's P1 — 18 `--pattern` flags ran 1 pattern (the report's Decisions & corrections, escher-0.1.0/chunks/2026-10-12-self-description/report.md). Score: measured 0.4 + technical detail 0.2 + no other home 0.2 = 0.8.
                                              + "A sweep pattern for a bare count also matches a citation's digits"
    Proof: P1's `ws805` pattern listed `design-system.md:76`, the citation `render.rs:805-812`, beside the count at `test-plan.md:330` (sweep-pre-pass.txt in this run dir; the report's Decisions & corrections). Score: measured 0.4 + technical detail 0.2 + no other home 0.2 = 0.8.
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
  Recurrences (→ handoff, Deferred learnings):
    - recurrence-despite-learning: "Committed run-dir and evidence text must not spell a host temp path, even as prose" (session-learnings.md, the hygiene entry) — a phase relay written verbatim carried a sibling directory's host path, refused by hygiene at the operator pass and masked there.
    - recurrence-despite-learning: "The rustfmt write hook leaves a crate root unformatted …", Extended 2026-10-06 ("an edit made through a Bash script never fires the hook") — implement's python string replacements left `cli.rs` and `cli_commands.rs` unformatted, and the fast leg read red.
  No-other-home: "`cascade.py sweep --pattern` takes its ids after one flag" · "A sweep pattern for a bare count also matches a citation's digits"
  CLAUDE.md size: 146/200 · T1 2.0 KB, 0 over 600 B
