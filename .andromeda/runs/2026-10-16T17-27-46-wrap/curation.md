CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A wrap holds its chunk's tree from P1 to its commit — an edit to a file the scope read counts made after P1 moves the new-text listing and halts the wrap, so it lands with a later chunk, before that wrap's P1" (confidence 0.8)
    Proof: at this wrap's resume the operator-directed `.gitignore` line made `cites.py added` exit 3 «the listing differs … First differing line 6, now: ### .gitignore — added 1 line(s) in 1 range(s)» (trail `cites-2026-10-16-file-splits.json`, ts 2026-10-16T17:35:40Z); the operator's ruling at the halt: revert it, «the line will come with a later chunk wrap, before its P1». Signals: explicit user correction +0.4 · verified by measurement +0.4.
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "Moving Rust items between modules strands two kinds of import"
    Proof: report Deviations 1 (rustc «unused import: `VERBS`», «unused imports: `BUSY_CLASSES`, … `NODE_FIELDS`» after the split; seven doc links re-pointed by path) and Deviations 2 (`mcp_tests.rs` use lines for non-moved names the parent no longer imported). Signals: verified by measurement +0.4 · specific technical detail +0.2 · no other durable home +0.2 (the next entry, «Fast-failing mutants», does not need it; no master, route annotation or contract carries it). Tier 3: no rule file's `paths:` covers `src/` (testing.md scopes `tests/**`).
  Filters: 2 dup · 0 task-specific · 0 conflict · 0 deferred (cap) · 1 recurrence-despite-learning (→ handoff)
  CLAUDE.md size: 148/200 · T1 2.7 KB, 0 over 600 B

Dispositions of the report's Decisions & corrections, and of this window:
- The operator's operator-pass directive (inputs#I1) → not a learning: a one-chunk directive, carried out.
- The operator's «run P1 only and stop» (inputs#I2) → not a learning: the wrap's two-window form, already the skill's.
- rustc reads a doc-link-only import as unused → applied (Tier 3, merged with the next).
- Plan step 4's allowed edit narrower than the move needed → applied (Tier 3, merged: the `use super::*` half).
- The guard refusal of a session-moving `cd` (re-issued once) → rejected — duplicate (host-linux «Paths & the working
  directory»: `cd` only inside a subshell); it recurred in this window once, re-issued as a subshell — the guard
  working (`guard-refused`), not a recurrence.
- The per-package blitz-paint `Intrinsic` warning → rejected — duplicate (CLAUDE.md Tier 1, 2026-10-10, extended
  2026-10-15).
- The ps census counting its own probing shell → recurrence-despite-learning (host-linux «Processes»: never match a
  pattern the invoking shell's own command line contains; select by `comm`) → handoff, Deferred learnings.
- The pre-CI commit staging the whole tree (precedent) → not a learning: the precedent `c71f7ed14` set, kept.
- escher-driver's 72 vs the measured 73 → not a learning: amended at P2 on the operator's directive (inputs#I3).
- This window: the `.gitignore` line and the listing halt → applied (Tier 1, above).
- This window: a tool's stdout sent to /dev/null against the wrap's rule, the counts recovered from its trail → not a
  learning: the skill's own rule, which already states it (pipeline telemetry, logged as friction).
- This window: the arch-only leaf group could not read the key file's 73 → not a learning: pipeline telemetry (friction).
