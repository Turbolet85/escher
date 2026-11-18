# Curation — 2026-11-18-cold-agent-bar (wrap 2026-11-18T09-02-58-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   extended testing.md: "In a `gate.py control` file a control's `case` names the test function alone …" + its case reader knows no unittest result line (confidence 0.7)
  Tier 3 (.claude/docs/session-learnings.md): extended "A phase P5 control that plants a home-dir path trips hygiene at the operator pass" + a met one is rewritten to the `<user>` form without asking (confidence 0.7)
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 3 below threshold
  Extended: T2/testing.md: "In a `gate.py control` file a control's `case` names the test function alone" + "the unittest result line"
  Extended: T3/session-learnings.md: "A phase P5 control that plants a home-dir path trips hygiene" + "rewrite a met one without asking"
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

## Applied
- T2 testing.md, extension of the 2026-11-14 `gate.py control` entry (0.7: verified by measurement +0.4, repeated 4 controls +0.3).
  Proof: every control on the `ci-scripts` entry read INDETERMINATE «no result line» under gate v1.37, each case read by hand (evidence/control-1.txt, control-2.txt; report Decisions & corrections, entry 2).
- T3 session-learnings.md, extension of the 2026-10-27 placeholder entry (0.7: the operator's explicit ruling +0.4, «without asking» standing language +0.3).
  Proof: inputs#I4 (implement's relay-1.md), the operator's word; met at implement P4 hygiene on the phase's p5-controls.md:10, rewritten, hygiene then clean.

## Each Decisions & corrections entry
Implement's:
1. A deletion control on a prompt goal is vacuous when the goal's words stand elsewhere — rejected, Filter 4 at 0.6 (measured +0.4, detail +0.2); its home is test-plan `decision/prompt-goal-control-phrase-once`, landed at P2 (tx 2109).
2. `gate.py control` reads no unittest case line — applied (extended, T2 testing.md).
3. Guard refusals (a `cd` moving the cwd) — not a learning: the guard enforces it.
4. The operator's `<user>` placeholder ruling — applied (extended, T3).
5. A unittest capture saved verbatim carries host paths in its tracebacks — rejected, Filter 4 at 0.6 (measured +0.4, detail +0.2); §Hygiene's capture rule already carries the remedy.
After implement (the close's):
6. The founder's five answers (F1–F5) — not a learning: rulings, landed in the masters (F4) and the route (F5, P5).
7. «`caused` out of reach» superseded — not a learning: the route's (P5).
8. The README interim heading on the founder's word — not a learning: a ruling.
9. The operator's wrap directive — not a learning: this wrap's direction.
Conversation:
10. Leaf comparers in the whole mode missed two lines the step-2 sweep listed stale — not curated: telemetry of the tool (the evolve record).
Citation row routed to P3 (`citation-dispositions.md`): `.claude/docs/session-learnings.md:354`'s `host.rs:350` is an illustrative example of a prefix collision, no claim about the file — kept as written.
