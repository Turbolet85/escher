CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): extended «2026-10-15 — A saved `cargo build` capture in a run dir holds the project root»
  Filters: 1 dup · 2 task-specific · 0 conflict · 0 deferred · 1 below threshold
  Extended: T3/session-learnings.md: "A saved `cargo build` capture in a run dir holds the project root" + "a jscpd report written with `--absolute` holds the same in-root paths; the same respell fixes it"
    Proof: implement's own hygiene read refused the three jscpd reports (`jscpd-base`, `jscpd-20261120T023348Z`, `jscpd-20261120T033744Z`) as absolute in-root paths and each was respelled by `gate.py respell` (`evidence/operator-pass.md` item 1); the respelled report re-read `stand reading: 42 pairs / 358 lines · … PASS`; the operator's wrap directive names the hazard (inputs#I4). Score 0.9: measured +0.4 · three separate runs +0.3 · technical detail +0.2.
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

Dispositions, Decisions & corrections (implement's, then After implement's):
- implement · the operator's directive inputs#I3 (F1 unanswered, pass by hand in plan order) — not a learning: this chunk's pass order, task-specific (Filter 2).
- implement · guard refusals (two `cd` outside the root, one `cat` heredoc with a file target), each re-issued once — rejected, duplicate (Filter 1): host-linux.md §Paths & the working directory and §Transports already state both; a refusal that cost its one re-issue is no recurrence.
- implement · entry 6's jscpd run timed 0.31 s while its report was freshly written — rejected below threshold (Filter 4): a one-off reading, already settled by the scratch re-run (score 0.2 − 0.3).
- implement · no one-shot control fired — not a learning: a fact of the run.
- after implement · the wrap directive inputs#I4 — its jscpd respell half → extended (above); its other halves (F1 carried, guard re-issue, no hand edit of a leaf, founder forks to founder-forks.md) — task-specific to this wrap (Filter 2).
- after implement · the operator pass recorded no correction — nothing to weigh.
- after implement · three moved facts stand in no master — not a learning: figures the code and the evidence hold (curation-guide §Analysis scope).
- citation row (citation-dispositions.md): `.claude/docs/session-learnings.md:368` `host.rs:350` out-of-range — stands: an illustrative example of a prefix match inside the entry's prose, no claim about the file.
