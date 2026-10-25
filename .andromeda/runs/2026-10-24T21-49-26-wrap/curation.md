# Curation — 2026-10-24-a11y-re-read wrap, P3

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "The session hooks write the handoff in the middle of a wrap" (confidence 0.8) · + "An agent's hand-back is its return" (confidence 0.7)
  Filters: 1 dup (a recurrence → handoff) · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold
  No-other-home: "a count alternative with no leading word boundary also matches inside a longer number or a slice name"
  Extended: T3/session-learnings.md: "A sweep pattern for a bare count also matches a citation's digits" + "a count alternative with no leading `\b` matches inside a longer number or a slice name"

## Applied

- T3 new — "The session hooks write the handoff in the middle of a wrap" (measured +0.4 · specific detail +0.2 · no other home +0.2 = 0.8)
  Proof: the classifier pair's diff-tree (before `f0873aef…`, after `6c427c69…`) carried `M .claude/session-handoff.md`, its one hunk the `## AUTO-COMPACT 2026-10-25 01:26` block (mtime 01:26:20 local); the Stop hook's `23:08:52 → 23:55:04` hunk stood in every before tree of the run; the path check stopped and the operator ruled — `batch-notes.md`, pair 3.
- T3 new — "An agent's hand-back is its return" (operator's ruling +0.4 · «from here on» +0.3 = 0.7)
  Proof: the operator's relay of overseer1's ruling F342 at the leaf pair («under Agent spawns the hand-back is both the return and the wake … Don't wait for notifications from here on»); applied at pair 5 — `batch-notes.md`.
- T3 extended — "A sweep pattern for a bare count also matches a citation's digits" + the leading-`\b` facet (measured +0.4 · specific +0.2 · no other home +0.2 = 0.8)
  Proof: the report's `a11y-tally` pattern `six (test )?files|6 (test )?files|…` listed 41 standing rows, 25 of them other counts sharing the token («the 17 listed s06 files», «86 files», «16 files», «26/36/46 files») — report.md, Decisions & corrections, and the pre-pass trail `cascade-2026-10-24-a11y-re-read-pre-pass.json`.

## Decisions & corrections — each entry's disposition

1. The operator's word for P2 («forced sample», two-window form) — not a learning: this wrap's direction, carried out (`cascade-sample.txt`, FORCED).
2. The operator pass run in this session by hand, entry by entry — not a learning: this chunk's direction, recorded in evidence/operator-pass.md.
3. Sweep hazard — `gh run view --log` spells ESC as the two literal characters `^[`; strip `(\x1b|\^\[)\[[0-9;]*m` — rejected, Filter 4 at 0.6 (measured +0.4 · specific +0.2): it reached a master home this wrap (test-plan-detail/ci-integration.md, the Per-OS test tallies group, its fact F3), so no conditional signal; the 2026-10-11 `gh api` entry is its neighbour, another facet.
4. Sweep hazard — `6 (test )?files` with no word boundary — extended (above).
5. A witness claim corrected before it was recorded (`aria-checked` asserted by `accessibility_states`, not `accessibility_merged_states`) — corrected at its source: P2 wrote it into a11y-plan-detail/ci-integration.md; no curation entry.
6. Guard refusal — the Bash guard refused a `cd` into the skill's references and the call was re-issued once with absolute paths — not a learning: a hook refuses the very act (Filter 4's exclusion), `guard-refused`.
7. The P4 fork's answer (inputs#I1) — not a learning: the founder's ruling, recorded in the masters and their sidecar entries.

## This window's candidates beside the report

- The provenance tokens the first P2 apply wrote (23 sites) — Filter 1 duplicate of «Provenance is counted by class and span, not by the words beside it» (2026-10-19), a defect record → `recurrence-despite-learning`, logged in the handoff's Deferred learnings.
- The out-of-range row of the P2 citation sweep, `.claude/docs/session-learnings.md:223 host.rs:350` (the file 231 lines at the base) — read: the entry's own illustration of a prefix collision («`host.rs:35` heads `host.rs:350`»), not a citation of a line; a preserve-verbatim curation home, left as it stands.
