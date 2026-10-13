# Curation — 2026-10-13-upstream-sync-polish-and-ship

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "Run every pipeline tool bare or redirected to a file read whole — never piped into head, tail, grep, cut, sed or awk" (confidence 1.0)
    Proof: the founder's standing direction (inputs#I2), broken at implement P2 (`gate.py show … | head -40`), at the operator pass (`gate.py hygiene 2>&1 | tail -n 2`) and at this wrap (`splice.py summary …` over the seven fan-out files and every `sidecar.py check …` read through `| tail -n 1`). The verdict survived each time, and the direction was broken each time. Signals: explicit direction +0.4, repeated across three steps +0.3, "never" language +0.3. Home: host/shell mechanics → `host-linux.md` (no `paths:`, so judged by the Tier-1 bar: one sentence, 272 B). Filter 1: the generated body's "Read `$?` from the BARE command" covers exit reading only, not a verdict read through a pipe — an additive facet against a generated body, so it lands as a new `## Session Additions` entry.
  Tier 3 (.claude/docs/session-learnings.md): + "A phase control that reads a scratchpad file lands a host temp path in the run dir" (confidence 0.8)
    Proof: report *Decisions & corrections*, the hygiene sweep hazard; Deviations, operator pass entry 47 — `hygiene: refused 1 files — P1 1` on `.andromeda/runs/2026-10-13T14-26-44-phase/controls.txt:25 ×2 · tmp`, masked by hand. Signals: measured +0.4, specific technical detail +0.2, no-other-home +0.2 (not on the route, in no master or playbook, not refused by a hook; overseer1's ES44 is outside this repository). Home: a sweep hazard whose artifact is a run-dir file no rule file's `paths:` covers → Tier 3.
  Filters: 1 dup (→ handoff as recurrence-despite-learning) · 0 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: see the P7 health row

Decisions & corrections, each weighed:
- A standing directive broken twice (pipe into head/tail) → applied, Tier 2 host-linux.md (above). This window broke it too.
- Sweep hazard: `gh run view --log` colour codes as literal `^[[` text, Windows `tests\` → rejected, Filter 1 duplicate of `.claude/docs/session-learnings.md` "A job log fetched with `gh run view --log` carries its colour codes as literal text …" (2026-10-10, extended the same day). It is a DEFECT record that did not prevent the plan's gate entry 50, so it goes to the handoff as `recurrence-despite-learning`.
- Sweep hazard (hygiene): a phase control's scratchpad path → applied, Tier 3 (above).
- Guard refusal of a `cd` at implement Setup → not a learning: the guard already refuses the act (Filter 4's hook exclusion), re-issued once with absolute paths.
- Decision, the conflict resolution (upstream's `is_bool_attr`) → not a learning: its home is the masters, amended this wrap (architecture, security-plan, a11y-plan).
- Decision, the operator pass run on the operator's word → not a learning: chunk-specific record (Filter 2).
- Measured, the cold CI 1023 s → rejected, Filter 2 (a reading, task-specific).
- Measured, every predicted count held → not a learning: a positive measurement of the plan, recorded in the report.
- This window: the founder-word respelling class → not curated; its home is the playbook rule A appended at P2 on the founder's word.
- This window: the cascade sweep refusing an unfired-control pattern → not a learning: the tool working as designed.
