# Curation — 2026-10-20-select-by-id (wrap 2026-10-20T05-49-12)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A write-set probe must allow the files a new enum variant's exhaustive matches live in" (confidence 0.8)
    Proof: plan gate entry 12 read red in implement firings 1 and 2 on `packages/blitz-dom/src/events/mod.rs` alone
    (count 1 — two exhaustive `DomEventData` matches gaining a `Change` arm), its P5 must-fail control having planted
    `lib.rs`; amended on the operator's word (`chunks/2026-10-20-select-by-id/evidence/gate-12-amend.md`), firing 3
    green. Signals: verified by a real gate failure +0.4, the operator's correction +0.4.
  Filters: 3 dup (2 of them recurrences → handoff) · 1 task-specific · 0 conflict · 4 below threshold · 0 deferred (cap)

Dispositions — the report's *Decisions & corrections*:
- Dioxus re-writes a dynamic `selected` on every re-render → rejected, Filter 4 (0.6: measured +0.4, detail +0.2;
  no conditional signal — this wrap amended it into architecture [DOM semantics])
- Dioxus deep-clones a static template, so a per-element append hook misses its children → rejected, Filter 4 (0.6;
  amended into architecture [DOM semantics])
- A write-set probe missed the companion its own step required → applied, Tier 3 (above)
- The operator directed the operator pass in plan order → not a learning (a decision of that pass)
- The Bash guard refused a `cat` heredoc → not a learning (the guard already refuses the act)
- `ElementState` is not re-exported, a probe swapped copies through `snapshot_node_and` → rejected, Filter 2
  (task-specific)
- Sweep hazard: a bare count matches citation line numbers → rejected, Filter 1 (dup: the 2026-10-12 entry «A sweep
  pattern for a bare count also matches a citation's digits», docs/session-learnings.md; `cascade.py sweep` also warns on a
  bare-number pattern)
- Implement's outcome basis → not a learning

Dispositions — this session:
- An in-line count edit just before a history parenthesis makes its chunk markers read as new provenance
  (test-plan.md:22, :68, :104; session-lifecycle.md:11) → Filter 1 duplicate of the 2026-10-19 entry «An in-line
  edit beside a provenance parenthesis reads as new provenance», a DEFECT record → recurrence-despite-learning, to
  the handoff
- The report's P1 sweep missed «twenty-seven checks» (a word form, obs-plan.md:70) → Filter 1 duplicate of the
  2026-10-20 entry «A count stated in more than one form needs every form in its sweep pattern», a DEFECT record →
  recurrence-despite-learning, to the handoff
- The temp-dir disk quota stopped every shell command (EDQUOT), a retry ran → rejected, Filter 4 (one-off, cause not
  identified)
- A `cascade.py replace` block keyed on a line an earlier Edit's inserted line moved reads `anchor moved` → rejected,
  Filter 4 (one-off; the tool refuses and names the new line)
