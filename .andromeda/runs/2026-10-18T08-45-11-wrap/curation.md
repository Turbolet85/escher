# Curation — 2026-10-18-one-virtual-clock

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   extended testing.md: "Before calling a standing check unaffected by a change to data it could pin …" + "search the owning crate's own `src/` too — an inline `#[cfg(test)]` module beside the table can pin its length"
  Proof: implement P1's first `cargo test -p escher-driver --locked` read 2 failed — `json.rs`'s inline tests pinned 14 refusals and `describe.rs`'s a causes row of 9 after `CAUSES` dropped to 8 (report Decisions item 7; implement evolve record 2026-10-18T08:04:09Z-b; the scope record's two companion lines). Filter 1: additive facet (`src/` inline modules, a path the entry lacks). Filter 4: measured +0.4 · technical detail +0.2 · no other home +0.2 (no master, route annotation or hook carries it) = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): + "An atomic read-modify-write that builds on the MSRV and on the host toolchain"
  Proof: rustc's `deprecated` warning on `AtomicU64::fetch_update` at implement P1 ("renamed to `try_update` for consistency"), `try_update` absent from MSRV 1.91; the hand CAS loop changed the chunk's design (report Deviations item 1, Decisions item 9). Filter 4: measured +0.4 · technical detail +0.2 · no other home +0.2 = 0.8. Not path-scoped to a rule file (no Rust-source rule file exists) → Tier 3.
  Filters: 3 dup (two of them recurrences → handoff) · 0 task-specific · 0 conflict · 0 deferred (cap) · 2 below threshold
  No-other-home: "search the owning crate's own src/ for an inline test pinning a table's length" · "a hand compare_exchange_weak loop for an atomic update across MSRV and host toolchain"
  CLAUDE.md size: see the P7 health row

## Decisions & corrections — dispositions
1. The founder forks answered by the delegate (F1 B · F2 A · F2b A · F3 A) → not a learning: a decision, recorded PROVISIONAL in the masters and sidecars for the founder's epoch batch.
2. The P5 card's row 1 accepted → not a learning: a route placement, landed at P5.
3. The operator's direction after implement → not a learning: a one-off direction, done (evidence/operator-pass.md).
4. "run P1 only and stop" → not a learning: an invocation word.
5. The `cd` guard refusal at implement P1 → rejected, duplicate (Filter 1) of host-linux «Paths & the working directory», a defect record → `recurrence-despite-learning` to the handoff. It recurred again at this wrap's resume (Setup's first read), refused by the guard and re-issued once.
6. A guard refusal blocks the whole compound call, its other edits included → rejected, below threshold (Filter 4: measured +0.4 · detail +0.2 = 0.6; no-other-home does not apply — the guard hook already refuses the very act, a `cat`/`tee` heredoc with a file target).
7. A crate's inline unit tests pin a table's count → extended (testing.md, above).
8. `advance` now moves CSS animations, so a step's diff can be non-empty → rejected, below threshold (0.6): amended this wrap into architecture §Standard Contracts → Driver session ("a step's diff can be non-empty on any animated page"), its home.
9. `AtomicU64::fetch_update` deprecated on the host, `try_update` past the MSRV → applied, Tier 3 (above).
10. blitz-paint's `variant Intrinsic is never constructed` under a per-package build → rejected, duplicate (Filter 1) of the 2026-10-10 Tier 1 entry, extended 2026-10-15; a fact record, not a defect.
11. The P1 site sweep's unfired title and new-check controls; the `cli-advance` case variant → not a learning: the tool behaving as its contract states (the title's unfired control is the report's stated reading).

## This window
- A `cascade.py window` call piped into `head` was refused by the Bash guard and re-issued once as a plain python read → rejected, duplicate (Filter 1) of host-linux's 2026-10-13 Session Addition (run every pipeline tool bare, never piped into a filter), a defect record → `recurrence-despite-learning` to the handoff.
- The citation sweep's `out-of-range .claude/docs/session-learnings.md:145 host.rs:350` row, a curation home → no correction: the entry's `host.rs:35` / `host.rs:350` pair is an illustrative prefix example of a re-point hazard, not a claim about `packages/escher-driver/src/host.rs` (225 lines); the citing line stands, and the handoff's carried "unmeasured" note on it is closed.
