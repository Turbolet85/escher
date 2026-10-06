CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A case-insensitive grep for an upper-case acronym matches inside ordinary words" (confidence 0.8)
    Proof: this wrap's P1 expected-amendment sweep — `grep -c -i -E 'MPL|stylo_taffy'` read architecture.md at 81, `grep -c -E '\bMPL'` at 1 (the default.css header); the second pipeline (`grep 'stylo_taffy' … | grep -i -c -E 'licen|MPL'`) read 8, the case-sensitive form 2. Signals: verified by measurement +0.4 · specific technical detail +0.2 · no-other-home +0.2 (not on the route, not in a master, not a playbook/drift-base rule).
                                              + "Committed run-dir and evidence text must not spell a host temp path, even as prose" (confidence 0.8)
    Proof: operator-pass hygiene read, 2026-10-06 — `hygiene: refused 1 files — P1 1` on `.andromeda/runs/2026-10-06T11-11-14-phase/security.md:31 ×1 · tmp`; reworded, re-read `hygiene: clean` (chunk evidence/operator-25-hygiene.txt). Signals: verified by a real gate refusal +0.4 · specific technical detail +0.2 · no-other-home +0.2.
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
  Recurrence (Filter 1, defect entry matched — logged to the handoff, not re-added): "The project's Bash guards refuse … a leading cd" — a leading `cd` into the skill references directory was refused this session (third recurrence after two at 2026-10-06-id-persistence).
  No-other-home: "A case-insensitive grep for an upper-case acronym matches inside ordinary words" · "Committed run-dir and evidence text must not spell a host temp path, even as prose"
  Candidates considered and not curated: the stamp-ahead hook refusing an estimated report timestamp (one-off, already enforced by the hook: -0.3 one-off; 0.2 total); the operator directing the agent to run the operator pass (a per-chunk operator decision recorded in the report and handoff, not a rule).
