# Curation — 2026-10-06-cold-agent-run-pipe

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A grep hit seen through a clipped view is not read" (confidence 0.8)
    Proof: the report's "no master states the 37" was false — the authoring grep returned test-plan.md:127 viewed through `cut -c1-220`; the count sat at offset 1554 of 1773; the test-plan drift detector named it; re-run with offsets: 18 rows (fanout-results.md; report.md Counts bullet).
                                              + "Grepping `agent-run` also finds the cold-agent marker and agent-runnable" (confidence 0.8)
    Proof: the report counted a11y-plan.md:57 ("agent-runnable invariants") as an agent-run site (the a11y detector flagged it); the cascade sweep's `agent-run` pattern returned 12 new-text rows that were the marker `cold-agent-run-pipe` (sweep-listing.txt).
  Filters: 0 dup · 1 task-specific (the census gate's word ban in the three scripts — this chunk's gate) · 0 conflict · 0 deferred · 2 below threshold (hygiene P1 reads placeholder-less temp-dir prose as a host path — 0.2, and the gate enforces it; the claude client's `--tools ""` / `--allowedTools mcp__stub` behaviour — amended into the masters this wrap, its home)
  Recurrence: "The project's Bash guards refuse a heredoc written to a file and a leading cd" (session-learnings.md, 2026-10-06) — a cat heredoc to a run-dir file was blocked again → handoff Deferred learnings as recurrence-despite-learning.
  No-other-home: "A grep hit seen through a clipped view is not read" · "Grepping `agent-run` also finds the cold-agent marker and agent-runnable"
  Signals: each = verified by measurement +0.4 · specific technical detail +0.2 · no-other-home +0.2 (other signals exactly 0.6; next-entry signal not applicable — "Stable element ids" does not need either fact).
