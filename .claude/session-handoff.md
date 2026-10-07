# Session Handoff

**Last Updated:** 2026-10-07T23:09:25Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-07-driver-command-spans — one span per driver command: verb, refusal cause, settle reading and diff size, printed by the sink as a closed-span line

## Position
- Done: 2026-10-07-driver-command-spans (27 master records, all complete). It claimed no capability. **Epoch 4 — Driver core is closed**: 9 entries, none markerless.
- **The project is paused at the Epoch 4 boundary** (the operator, 2026-10-07, at this wrap's invocation): no Epoch 5 entry is taken up until told. The first markerless entry is "Upstream sync ahead of agent surfaces" (working-route.md:69) — do not promote it and do not run /andromeda-phase on it without the operator's word.
- What stands at the boundary, none of it started: the founder's PROVISIONAL batch (below) · his review of the carried Tier 3 learnings (Deferred learnings) · the rule waiting for his word (Notes) · /andromeda-evolve-diagnose for Epoch 4 (no proposals file targets it).

## Work done
- `escher-driver`: every call handed to `Session::run` leaves one `tracing` span — target `escher_driver`, name `command`, INFO — with eight fields recorded only where they apply (`verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`): fixed schema words, bools and counts. `tracing` is the crate's third dependency, ungated. `escher-telemetry`'s sink prints one line per closed span beside one per event, each pair through the same target allowlist and scrub. Source landed in the operator pre-CI commit `44ad3887`.
- Proof: `stand_act_spans` (2 tests, 2 ignored children, both layout modes: one driver line per call, 0 of 36 ids, 0 of 8 names, 0 of the typed text in the capture), 2 driver and 5 sink unit tests, nine mutation controls. Workspace 656 passed · 0 failed · 10 ignored over 154 result lines; fork CI green 16/16 on `44ad3887` at its first attempt (CI#37694873705). Record: `escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md`, readings in its `evidence/`.

## Drift resolved
- 7 detectors, 56 proposals over five masters (architecture 16 · security-plan 7 · test-plan 16 · obs-plan 16 · a11y-plan 1; design-system and layout-templates none): 37 applied as proposed; 19 rejected as proposed — they cited source lines the report does not carry — and raised again from the wrap's own measurement, all applied. 5 more raised by the wrap, one of them found by the cascade sweep. Amendments over five masters and the keyed contract `session-lifecycle`; 8 sidecar entries; 14 leaf files re-derived. Record: `.andromeda/runs/2026-10-07T22-32-46-wrap/fanout-results.md`, `cascade-dispositions.md`.
- 2 escalations, both resolved with the operator and both left PROVISIONAL (next section).

## PROVISIONAL — the founder's batch at the Epoch 4 boundary (kept, not discharged; the batch waits)
1. **The sink prints a closed span — a second record class in escher's sink.** A boundary widening answered by the operator at the chunk's plan forks, not by the founder; kept marked on the operator's word at this wrap's invocation. Bodies: architecture `:132`, `:193` · security-plan `:379` · obs-plan `:147` (the owning statement), `:298` · test-plan `:102` and a11y-plan `:87` by pointer.
2. **The diff's three list lengths reach a log.** The founder ratified "a diff leaves the process through the returned value only — no log, event, socket or file carries it" (2026-10-07). The command span records the lengths of the returned diff — counts only, no node, id, name or value — and the sink prints them at `info`. The ratified sentence is not reworded; the fact is stated beside it, marked, on the operator's word at this wrap's escalation halt. Bodies: architecture `:136` · security-plan `:116`. Found by the architecture detector; the plan's list had not named it.
- Leaves carrying a mark, and the whole listing: the last section of `cascade-dispositions.md` in the run dir. No mark stood in any body before this wrap.

## Route owners written at this wrap (the wrap's placement — move any that reads wrong)
- On "Driver CLI" (working-route.md:71), as the operator directed: the workspace build compiles the engine's `tracing` call sites in. `cargo test --workspace` turns `blitz-dom/tracing` on (blitz's default `tracing`, through blitz-shell and blitz-html); a `-p blitz-tests` or `-p seven_guis` build does not. Placed on the entry that owes the typed-text proof in a host's log: that proof runs in the workspace leg.
- On "Quality gates" (:92), not named by the direction: the span's `busy` field has no reading on a real line — no stand step reads not settled.

## Notes
- **Not measured, stated so in the masters:** the two seven_guis binaries' stderr by level as a workspace-wide build makes them (the 0 · 1 · 1 · 1 readings are of a `-p seven_guis` build) · typed text in a sink-installing host's log (it reads 0 in an in-process capture) · the windowed stand by level at this chunk (its boot smoke at `info` only) · the MSRV, iOS and android CI job logs (their jobs read `success`).
- **A named limit, not exercised** (security-plan §Logging & Monitoring): if a field formatter errors at span creation, `tracing-subscriber 0.3.23` prints the span's attributes with `Debug` to stderr itself. The sink's formatter returns no error.
- **A rule to rule on, carried from the last wrap and still the founder's:** "A spec claim this chunk's measurement disproves, about code the chunk did not edit, is amended to state the measured limit — `as measured at` its evidence — and its fix is pinned as a CARRY on a route entry; routine." Not appended to the playbook.
- This wrap ran in two windows: the first wrote the report and stopped on the operator's word; the second resumed from it. Curation read the report's Decisions & corrections and the second window only.
- The epoch-close sidecar consolidation ran and had nothing to do: seven sidecars, consolidation set 0, prunable 0 (architecture's one unresolved `Supersedes` stands as it was).
- The cause of the workspace-build fact was read at this wrap from cargo's feature graph: `evidence/feature-unification.md` in the chunk folder.
- Still owed from earlier chunks: the windowed witness of the accessibility-tree refresh; `Harness::scroll_into_view` has no check of its own; `session_common/mod.rs:3` is an over-long doc line (cosmetic).
- No gated record, no PREREQ, no WATCH on the tail. Last failed command: none.

## Deferred learnings
- None deferred. One applied (Tier 3: `tracing-subscriber`'s fmt layer); the filtered ones are in `curation.md` in the run dir.
- recurrence-despite-learning: "A per-crate `cargo clippy` is not the CI lint leg" (Tier 3, 2026-10-07) — run again at this chunk's implement, red on blitz-dom, discarded. The fourth recurrence.
- Curation conflict, for the operator: `.claude/rules/host-linux.md` says "The transport collapses a BACKSLASH PAIR `\\` to `\` before bash sees it, inside a quoted heredoc too"; this chunk's report records two payloads holding backslash pairs, sent in quoted python heredocs, that "landed as written". One counter-reading; the rule stands unedited.
- Carried, for the founder's review at the Epoch 4 boundary (set by him, 2026-10-07): "The project's Bash guards refuse a heredoc written to a file and a leading cd" (Tier 3, 2026-10-06) · "A chunk that moves cited source lines stales the masters' file:line citations" (Tier 3, 2026-10-05) · "A grep hit seen through a clipped view is not read" (Tier 3, 2026-10-06) · "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05) · and the recurrence record above.
