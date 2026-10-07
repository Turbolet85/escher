# Session Handoff

**Last Updated:** 2026-10-07T15:02:28Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-07-act-by-id — a call runs on a held instance: `Session::run` executes the driver's verbs in process, by stable id, and returns after settle with the diff

## Position
- Done: 2026-10-07-act-by-id (25 master records, all complete). Verified with it: v010-03 · v010-06 · v010-09 · v010-10.
- Next: "Refusal detection" (working-route.md:64, v010-11) — promote and plan it with /andromeda-phase. It carries three CARRYs; the newest says what a click does today to a target that cannot take it.

## Work done
- `escher-driver` gains the executor (`execute.rs`): `Session::run(&Call)` validates, runs the verb, settles and returns an `Outcome` — the screen's text, or whether the instance went quiet and the diff. `click` and `type` take a stable id (`not-found` otherwise); `advance` moves time through `Session::with_time` (`time-unavailable` without). In process only: the socket is unchanged. The crate names `accessibility` on dioxus-native-dom. The harness re-exports `Key` · `Modifiers` and gains `apple_keybinding`; the stand gains `timer_step`; the Timer host carries it. Source landed in the operator pre-CI commit `f8eb42c8`.
- Proof: six `stand_act_*` files (17 tests), five mutation controls. Workspace 629 passed · 0 failed · 8 ignored over 149 result lines; fork CI green 16/16 on `f8eb42c8` (CI#37633611745) — the macOS and windows job logs were read: all six files ran, the deleting-key test `ok`. Record: `escher-0.1.0/chunks/2026-10-07-act-by-id/report.md`, readings in its `evidence/`.

## Drift resolved
- 7 detectors, 61 proposals over four masters (architecture 22 · security-plan 13 · test-plan 16 · obs-plan 10); 60 applied, 1 rejected as proposed and re-raised from the wrap's own measurement, 8 more raised (a11y-plan, design-system, layout-templates and five citation moves). 68 amendments over all seven masters and the keyed contract `session-lifecycle`; seven sidecar entries. No escalation. Record: `.andromeda/runs/2026-10-07T14-22-35-wrap/fanout-results.md`, `cascade-dispositions.md`.
- 14 leaf files re-derived; the second sweep left no stale row. One Session Addition in `testing.md` corrected in place (the harness now synthesizes the Apple delete binding).

## Rulings applied at this wrap (`operator-rulings.md` in the run dir)
- The founder (relayed verbatim): a new route entry, "Range input interaction" (working-route.md:84) — a boundary widening by his word; no placement was given, so it sits ahead of "Stand keyboard harness", its first dependent. Say so if it belongs earlier. No capability was added to the ledger with it.
- The founder: the playbook rule proposed at the 12-34 wrap is appended. The founder: test functions stay in the code audit's `over_ceiling` scalar — that question is closed. The founder: the Tier 3 learnings below are reviewed at the Epoch 4 boundary.
- The overseer's technical answer: `advanced_ms` is the time delivered to the app; 60000 past the Timer's cap is right as built — stated so in architecture and test-plan.

## Notes
- **One judgment to check.** Three clauses carrying "ratified by the founder" (a snapshot's text and a diff are returned to their caller only) had a status clause beside them — "no driver, CLI or MCP command exposes it yet" — which the executor made false. The ratified rule is kept word for word; the status clause now says a driver command returns it in process, under the operator's "in process" answer at the plan's forks. Applied without a halt as not a boundary widening (fanout-results.md, check 1).
- The `select` half of the old carry ("no `select` or range interaction model") is in no ruling and has no owner; no stand task holds a `select`. It is written into the new entry's carry.
- No standing check boots `escher-session timer`; booted by hand once. Owner: a CARRY on "Driver CLI", beside the typed-sentinel proof, which moved there.
- Still not measured: typed text in a sink-installing host's log; the ios and android CI job logs (their jobs read `success`); `log.file` occurrences. Still owed: the windowed witness of the accessibility-tree refresh.
- Not checked by any local gate: the MSRV build and the windows, macOS, iOS and android legs — CI#37633611745 on `f8eb42c8` is their witness; this wrap's commit adds records and spec text, no source.
- `session_common/mod.rs:3` is a 147-character doc line (rustfmt leaves comments alone); cosmetic, left for a source-touching chunk.
- Epoch 4 reads 9 entries, 2 markerless; Epoch 6 reads 8. No split was asked for.
- No gated record, no PREREQ, no WATCH on the tail. Last failed command: none.

## Deferred learnings
- None deferred this session; three candidates fell below the threshold (`curation.md` in the run dir).
- Carried, for review at the Epoch 4 boundary (the founder, 2026-10-07): "The project's Bash guards refuse a heredoc written to a file and a leading cd" (Tier 3, 2026-10-06) · "A chunk that moves cited source lines stales the masters' file:line citations" (Tier 3, 2026-10-05) · "A grep hit seen through a clipped view is not read" (Tier 3, 2026-10-06) · "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05) · and the recurrence record of "A per-crate `cargo clippy` is not the CI lint leg" (Tier 3, 2026-10-07; three recurrences, none this session).

## Session End Status
Completed normally at 2026-10-07 20:50:26
