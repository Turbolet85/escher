# Session Handoff

**Last Updated:** 2026-10-07T21:08:56Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-07-refusal-detection — the driver names why it refuses: stale, disabled, covered and off-screen are detected before anything runs, and a sixth verb, `scroll`, brings a target into view

## Position
- Done: 2026-10-07-refusal-detection (26 master records, all complete). Verified with it: v010-11.
- Next: "Driver command spans" (working-route.md:66) — promote and plan it with /andromeda-phase. It is the last markerless entry of Epoch 4: its wrap closes the epoch (the sidecar consolidation, and the Tier 3 review the founder set for that boundary).

## Work done
- `escher-driver`'s executor detects the five screen-level refusals in front of a call: an id the screen does not read is `stale` when the session's record of ids read holds it and `not-found` when not; a `click` or a `type` on a target that cannot take it is refused `disabled`, then `off-screen`, then `covered`, with nothing run. A sixth verb, `scroll`, brings its target into view and says in its result (`in_view`) whether it then is. The click point is now the centre of the snapshot's bounds. In the engine, `BaseDocument::scroll_into_view` scrolls every nested scrolling box and then the viewport, for every document, and `visible_region` is the new read-only reader. Source landed in the operator pre-CI commit `983d8973`.
- Proof: nine `stand_act_*` files (27 tests) and `scroll_into_view_nested` (7), nine mutation controls. Workspace 647 passed · 0 failed · 8 ignored over 153 result lines; fork CI green 16/16 on `983d8973` (CI#37673662374, at its third attempt — see Notes). Record: `escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md`, readings in its `evidence/`.

## Drift resolved
- 7 detectors, 59 proposals over six masters (architecture 27 · security-plan 8 · test-plan 14 · obs-plan 8 · layout-templates 1 · a11y-plan 1); 58 applied, 1 rejected as proposed and re-raised from the wrap's own measurement, 5 more raised (the `Refusal` bullet, design-system §Motion, the reader counts, `lib.rs` and three bare citations). 63 amendments over all seven masters and the keyed contract `session-lifecycle`; seven sidecar entries. Record: `.andromeda/runs/2026-10-07T20-27-47-wrap/fanout-results.md`, `cascade-dispositions.md`.
- 2 escalations, both resolved at this wrap (`operator-rulings.md` in the run dir): the sixth verb and the engine scroll for every document are each recorded as ratified by the founder (2026-10-07), his own choice relayed verbatim by the overseer, the option wording the overseer's.
- 15 leaf files re-derived; the second sweep left no stale row.

## Route owners written at this wrap (the operator's direction; the wrap's placement — move any that reads wrong)
- On "Driver CLI" (working-route.md:71): a scrolled box's own `bounds` read shifted by its scroll offset · the hit walk reaches a row scrolled out of its box · the session's record has no bound on an id's length.
- On "Quality gates" (:92): the CI package-install step has no timeout and no retry. It can recur at any chunk's CI run before that entry — say so if it should move earlier.
- Two more the direction did not name: on "Upstream sync ahead of agent surfaces" (:69), our merge surface in the inherited `scrolling.rs`; on "Cold-agent test" (:94), whether `covered` is the right word for a box with no height (`task-header-spacer`).

## Upstreamable (flags kept, as directed)
- **The nested `scroll_into_view`** — the widened method and the `visible_region` reader (the founder, 2026-10-07, relayed verbatim by the overseer). No issue or PR is open; nothing was read from upstream.
- **The bounds misreading** — `get_client_bounding_rect` subtracts a node's own scroll offset, so a scrolled box's own bounds read shifted (the operator, 2026-10-07, at this wrap). Predates this chunk; unfixed; in upstream-owned code.

## Notes
- **A rule to rule on.** One amendment had no playbook rule and was applied on the operator's direction: a body clause stating a measured limit of code the chunk did not edit (the bounds misreading), with its owner pinned on the route. Proposed rule, not appended: "A spec claim this chunk's measurement disproves, about code the chunk did not edit, is amended to state the measured limit — `as measured at` its evidence — and its fix is pinned as a CARRY on a route entry; routine." It needs the founder's word.
- This wrap ran in two windows: the first wrote the report and stopped on the operator's word; the second resumed from it. Curation read the report's Decisions & corrections and the second window only.
- The CI run needed two cancels and two re-runs after the install step hung twice; the method is in `.claude/docs/session-learnings.md` (top entry).
- Still not measured: typed text in a sink-installing host's log; the ios and android CI job logs (their jobs read `success`); the WPT `scrollIntoView` tests under the widened engine scroll (the runner's workflow is upstream-only). Still owed: the windowed witness of the accessibility-tree refresh.
- `Harness::scroll_into_view` has no check of its own: the driver's `scroll` is its one caller.
- `session_common/mod.rs:3` is still an over-long doc line (rustfmt leaves comments alone); cosmetic.
- Epoch 4 reads 9 entries, 1 markerless. No gated record, no PREREQ, no WATCH on the tail. Last failed command: none.

## Deferred learnings
- None deferred this session. One applied (Tier 3); the filtered ones are in `curation.md` in the run dir.
- recurrence-despite-learning: `.claude/rules/host-linux.md`, "The transport collapses a BACKSLASH PAIR" — a regex with a backslash pair sent in an inline heredoc was collapsed again at this chunk's operator pass.
- Carried, for review at the Epoch 4 boundary (the founder, 2026-10-07): "The project's Bash guards refuse a heredoc written to a file and a leading cd" (Tier 3, 2026-10-06) · "A chunk that moves cited source lines stales the masters' file:line citations" (Tier 3, 2026-10-05) · "A grep hit seen through a clipped view is not read" (Tier 3, 2026-10-06) · "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05) · and the recurrence record of "A per-crate `cargo clippy` is not the CI lint leg" (Tier 3, 2026-10-07; three recurrences, none this session).

## Session End Status
Completed normally at 2026-10-07 23:47:01
