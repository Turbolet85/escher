# Session Handoff

**Last Updated:** 2026-10-19T20:19:12Z
**Branch:** build/escher-0.2.0 · 0 ahead of origin/build/escher-0.2.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — upstream's `:focus-within` and WPT pin cherry-picked, Taffy stays the registry 0.15

## Position
- Done: 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness. Upstream `38ca4e777` (#1226, `:focus-within`)
  and `17de049b1` (#1228, the WPT pin) were taken by no-commit cherry-picks; the merge base `23b9b26a9` is unmoved and
  the Taffy-main commits are still out. CI#39312031677 green 17/17.
- Next: «Test splits» (Epoch 4), by `/andromeda-phase`.

## Work done
- Wrapped in two windows: P1 in the first, P2–P7 resumed from `.andromeda/runs/2026-10-19T19-54-24-wrap/`.
- The four CARRYs (Taffy-main, `keyboard_compat.rs`, the engine-debts merge surface, the clock's merge surface) now
  sit on «Upstream sync ahead of polish and ship», each citing this chunk's `evidence/readings.md` (P5 card rows 1–4).

## Drift resolved
- 14 amendments across architecture, security-plan, test-plan, obs-plan and a11y-plan, with 0 escalations:
  - `:focus-within` now matches;
  - focus outside the document is refused, and a focus set or cleared requests a redraw ([DOM semantics]);
  - the sync line reads `17de049b1`;
  - the per-sync lock record;
  - upstream's two focus targets;
  - 138 files / 137 targets;
  - per-OS tallies, the local baseline and the run-all series, with the two earlier 2026-10-19 chunks' unwritten moves attributed;
  - the silence census.
- One proposal was rejected as registry over-reach. 9 leaf findings were applied. Citations: 233 re-pointed, 0 changed, 6 stretched.

## Notes
- PROVISIONAL: the take (upstream's every-document `:focus-within`, the not-in-document focus refusal and the focus
  redraw) rests on F1 answered by the operator as delegate. It is marked PROVISIONAL in architecture, a11y-plan,
  security-plan, test-plan and obs-plan bodies and in this chunk's sidecar entries. The wrap that closes Epoch 4 lists
  it for the founder's boundary batch (P5 card row 5).
- Last failed command: none outstanding.

## Deferred learnings
- recurrence-despite-learning: host-linux §Paths («`cd` only inside a subshell»). The Bash guard refused a `cd` at
  implement Setup and again at wrap P1; it held at one re-issue each.
- recurrence-despite-learning: T3 «2026-10-12 — A sweep pattern for a bare count also matches a citation's digits».
  At P1, `\b575\b` matched four line-range citations and none of the figure's sites.
- recurrence-despite-learning: T3 «2026-10-18 — A count the plan names may stand in no master». Two 2026-10-19 wraps
  left test-plan's tallies and the run-all series unwritten. This report's run-all sweep then missed the series'
  last figure, 546 at test-plan.md:118.
