# Curation — 2026-10-13-range-input-interaction

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "escher-session refuses a state directory whose socket path passes 94 bytes … give a manual driver run a relative target/tmp/{name} directory" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "An added-line census reads inline test modules too" (0.8) · + "obs-plan words its silent-check census 'installs no subscriber'" (0.8)
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 1 deferred (→ handoff)
  No-other-home: the three applied entries (each 0.6 + 0.2)
  CLAUDE.md size: see the P7 health row
```

## Applied
- **T2 host-linux.md**: the state-directory socket-path bound. Score: verified by measurement 0.4 (a real refusal, `state-dir-too-long` exit 3, at implement's evidence drive) + specific detail 0.2 + no-other-home 0.2 = 0.8. Routed to host-linux.md as host/shell mechanics (tiebreaker 6). That file has no `paths:`, so the entry is judged at Tier 1's bar: one sentence, under 600 B.
  Proof: report.md *Decisions & corrections* "Host" — the implement evidence drive's one re-run in a relative `target/tmp/range-sess` dir (report Process hygiene row).
- **T3 session-learnings.md**: the added-line census reads inline test modules. Score: a real gate failure 0.4 + specific detail 0.2 + no-other-home 0.2 = 0.8. The inline modules sit in `packages/*/src`, which no path-scoped rule file covers, so tiebreaker 2 sends it to Tier 3.
  Proof: implement gate entry 9, red on its first firing (count 1, a test-module `.expect`), green after the `let … else` fix (report.md Outcome, Gates).
- **T3 session-learnings.md**: the silent census's master wording. Score: verified by measurement 0.4 (the P1 site sweep's exit 3, unfired control) + specific detail 0.2 + no-other-home 0.2 = 0.8. A sweep hazard (Filter 2 Keep). No rule file's `paths:` cover `.andromeda/`, so it goes to Tier 3.
  Proof: report.md *Decisions & corrections* "Sweep hazard" (silent census), this wrap's first P1 sweep.

## Report *Decisions & corrections*, entry by entry
- The operator-pass directive (inputs#I5) → not a project learning. Its cause, `gate.py run --operator` dropping hygiene's head and control lines, is a pipeline tool's defect, already in the overseer's record (F279(b)). The directive governed one pass.
- The operator at the P5 review (inputs#I4): the three leans → not a learning. Its home is the masters, amended at P2 (architecture [Scrolling and input], security-plan range row).
- The founder's three fork answers (inputs#I2, inputs#I3) → not a learning; the masters and sidecars hold them.
- The standing directive, no mutation runs (inputs#I1) → not a learning. It is the founder's standing word, relayed at every invocation; the pipeline-tool half is already host-linux.md's 2026-10-13 entry.
- Sweep hazard: the silent census wording → applied (T3).
- Sweep hazard: `.expect` in an inline test module → applied (T3).
- Lint: clippy `manual_clamp` → deferred by the Filter 5 cap (0.8, the lowest value of the four at 0.8: generic clippy knowledge) → handoff.
- Host: the socket-path bound → applied (T2).
- Measurement method: a throwaway probe test → not a learning. It was a one-off workaround, recorded in implement's evolve record.
- Guard refusals at implement → not a learning. The guard refused the act at the cost of one re-issue, and a hook that blocks the act is a home (Filter 4).
- Standing-learning recurrence check → none.
- This wrap's invocation, the two-window form → not a learning; it is the run's shape.

## This window
- A stray `.andromeda/runs/x` written by a mis-composed relative redirect, removed at once → not a learning. It was a one-off authoring error, recorded as friction.
- The citation sweep caught a disproved claim the report missed (test-plan.md:26) → telemetry (evolve), not curated.
- Curation-home citation row `.claude/docs/session-learnings.md` `host.rs:350` (out-of-range) → read: the entry quotes `host.rs:35` / `host.rs:350` as an example of a prefix collision, not a citation of the tree. No change.
