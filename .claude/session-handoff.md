# Session Handoff

**Last Updated:** 2026-10-07T08:50:25Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-07-sink-target-allowlist — escher's log sink drops every record from a target outside its allowlist; neither sink-installing binary prints an id or a name at any level

## Position
- Done: 2026-10-07-sink-target-allowlist (22 master records, all complete) — the fix the founder ruled should follow the host-log finding. It claimed no capability; coverage is unchanged.
- Next: "Settle detection" (working-route.md:58, v010-10) — promote and plan it with /andromeda-phase.

## Work done
- `escher-telemetry`'s formatter admits two target families — the engine prefixes, scrubbed to safe fields, and the new public `ESCHER_TARGET_PREFIXES` (`escher_`), content-named fields redacted — and drops every other target's record whole: zero bytes, every level, whatever `RUST_LOG` names. Source and tests landed in the operator pre-CI commit `25d9b72d`.
- Proof: 5 new unit tests (10 in the crate), `telemetry_drop` (blitz-tests, one ignored child), `host_log` (seven_guis, the real `escher-session` binary at `trace`), each seen red on the unfixed sink first. Workspace 579 passed · 0 failed · 8 ignored over 142 result lines; `run stand` 72 ok; fork CI green 16/16 on `25d9b72d` (CI#37592418443). Record: `escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md`, readings in its `evidence/by-level.md`.
- Measured, both binaries, four `RUST_LOG` settings, before → after: `escher-session` 0 · 1 · 1165 · 1501 → 0 · 1 · 1 · 1 stderr lines; the windowed stand (its first reading by level) 1 · 3 · 12,413 · 47,482 → 0 · 1 · 1 · 1. No id and no name at any level.

## Drift resolved
- 7 detectors, 40 proposals over five masters (architecture 8 · security-plan 8 · test-plan 10 · obs-plan 13 · a11y-plan 1; design-system and layout-templates none), all applied, plus 3 same-claim sites the cascade sweep found; 5 sidecar entries; two keyed contracts edited (obs `pii-scrubbing-wire`, tests `session-proof`). No escalation. Record: `.andromeda/runs/2026-10-07T08-23-43-wrap/fanout-results.md`, `cascade-dispositions.md`.
- The two bootstrap marks are discharged for escher's own sink; they stay open for the upstream apps' `fmt::init()` and the WPT runner's `env_logger`.
- 31 citations into the three moved files re-pointed; all 56 citations into the chunk's files resolve.

## Notes
- **The cost of dropping every level** (the lean the operator approved at the plan review): a third-party WARN or ERROR no longer reaches a sink-installing binary's stderr. The one instance measured: the windowed stand's default-level line `WARN winit_wayland::window::state` is gone. The full loss listing, WARN and ERROR under their own headings, is in the report.
- **Still not measured:** typed text in a sink-installing host's log (the CARRY on "Act by id", now pointing at `host_log` as the check to extend); `log.file` occurrences (the bodies say "by construction, not measured"). **Still owed:** the windowed witness of the accessibility-tree refresh (the CARRY on "Stand a11y assertions").
- Route: one CARRY pinned on "Driver command spans" — a span reaches stderr only under an engine or `escher_*` target. Epoch 4 reads 9 entries, 5 of them markerless; no split was asked for.
- The operator pass (hygiene, pre-CI commit, push, CI read) was driven by the agent on the operator's direction; its record is `evidence/operator-pass.md`. CI jobs were read by conclusion only — no job log was opened.
- Six obs-plan proposals arrived graded `escalate` by their detector's own severity while the detector reported its invariant holding; they were applied as routine on the playbook and the plan's approved expected amendments. Say so if that class should halt.
- Not checked by any local gate: the non-unix arms, the MSRV build, `host_log` on macOS — the fork's CI run is their one witness.
- Left open on purpose: whether test functions belong in the code audit's `over_ceiling` scalar (the Epoch 3 audit's question to the founder).
- No gated record, no PREREQ, no WATCH on the tail.
- Last failed command: none.

## Deferred learnings
- None deferred this session (no candidate survived the filters; nothing written to any tier).
- carried, still unreviewed: "A chunk that moves cited source lines stales the masters' file:line citations" (Tier 3, 2026-10-05) · "A grep hit seen through a clipped view is not read" (Tier 3, 2026-10-06) · "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05).
Review with `/andromeda-wrap-session --review` if any should be applied.
