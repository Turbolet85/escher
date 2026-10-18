CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A gated wrap with later chunks committed on top of its flip"
    Proof: three of the pipeline's base readings went wrong at this wrap, each measured. `gate.py scope` read the later chunks' 21 files as unrecorded (P1 and its re-fire, trail `gate-2026-10-18-engine-debts.json`). `cites.py added --base fb842671` listed 28 files of the later chunks' text (P1, trail `cites-2026-10-18-engine-debts.json`). `cascade.py sweep`'s default baseline 34bcf71c left every control unfired (exit 3), and a re-fire with `--baseline HEAD` swept clean (P2, trail `cascade-2026-10-18-engine-debts.json`). Plan entry 21 (the source-list probe) read 13 on the work tree against 0 at fb842671, `--skip`ped on the operator's word (inputs#I11). Score: verified by measurement +0.4 · repeated pattern, three tools at three steps +0.3 · specific technical detail +0.2 · could be task-specific −0.2 = 0.7.
  Filters: 0 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  CLAUDE.md size: see the P7 health read

Decisions & corrections (report-gated.md, with its addendum), one disposition each:
- The founder's word on the PROVISIONAL batch (inputs#I9): not a learning. It is a ruling, applied at P2 (71 marks ratified, six sidecar entries).
- The operator's invocation directive (inputs#I10): not a learning. It is this chunk's directive (P1 only, the residual's word, the scope ruling).
- For P5 (the BLOCKED-ON reads CLEARED): not a learning. P5 acts on it.
- For P7 (a)–(c) (the void, the forecast reds, the coverage flip and the consolidation): not a learning. P7 acts on it; the source-list probe's `--skip` is the operator's (inputs#I11 (c)).
- Pipeline gap (a gated take-up with later chunks on top): applied as Tier 3 above, carrying this window's third tool (the cascade baseline). The pipeline's own fix is the friction stream's (`contract.structural-blind-spot`, reconcile checkpoint).
- Sweep hazards — `Epoch 2` matching escher-0.1.0's epoch, the chunk title firing on no master, `PROVISIONAL` spanning three batches: rejected — Filter 2 (task-specific: one epoch label, one title, one mark's batches). The general hazard, epoch labels repeating across versions, is already the pipeline's own rule.
- Standing learning recurred (host-linux «Paths & the working directory»): → handoff as `recurrence-despite-learning`. This window, a `cd` into the skill's references dir was refused by the guard and re-issued once with absolute paths.
- The gated implement's outcome (entry 26 alone; an evolve record citing a non-existent trail, retracted; the code checkpoint appended after P2's): not a learning — telemetry of the tool (the friction stream is not curated).
- The resume directive (inputs#I11): not a learning. It is this wrap's directive.
- This window, not in the report: a pipeline tool's output piped into `head` and one redirected to a file were each refused by the guard and re-issued bare once. That is host-linux's 2026-10-13 rule (run a pipeline tool bare) → handoff as `recurrence-despite-learning`.
