# Session Handoff

**Last Updated:** 2026-10-07T12:55:00Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-07-command-and-refusal-schema — the driver's command and refusal schema: one verb table, a validation that refuses a malformed call before anything runs, eight causes each with a fixed remedy

## Position
- Done: 2026-10-07-command-and-refusal-schema (24 master records, all complete). It claimed no capability; coverage is unchanged, and v010-11 and v010-14 stay pooled.
- Next: "Act by id" (working-route.md:62, v010-09 · v010-06) — promote and plan it with /andromeda-phase. It carries seven CARRYs, the newest the wiring of `advance`.

## Work done
- `escher-driver` gains three private modules, held in process: `schema` (the verb table `VERBS` — `snapshot` · `click` · `type` · `press` · `advance` — with each verb's argument and result shapes, five argument kinds with bounds, twelve key names), `refusal` (eight `Cause`s each with a fixed name, meaning and remedy; `Fault`; `Refusal`, which holds nothing of a call) and `command` (`Call`, `Command`, `Key`, and `validate`, which receives no `Session`). Nothing runs a command, reaches the socket or serves the schema. Source landed in the operator pre-CI commit `772c770f`.
- Proof: 12 unit tests (crate 13 → 25), four mutation controls. Workspace 610 passed · 0 failed · 8 ignored over 143 result lines; fork CI green 16/16 on `772c770f` (CI#37620696026). Record: `escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md`, readings in its `evidence/`.
- The operator's three answers, as built: time on a held instance moves by `advance`, in milliseconds, through a step the session's caller supplies (stated, not wired); no driver command waits on a load; the table holds the core five verbs and later entries add theirs.

## Drift resolved
- 7 detectors, 13 proposals over four masters (architecture 4 · security-plan 3 · test-plan 4 · obs-plan 2); 10 applied as proposed, 3 rejected as proposed and re-raised from the orchestrator's own measurement, 1 raised for a11y-plan (the `disabled` cause). 14 edits over five masters and one keyed contract (tests `session-lifecycle`), five sidecar entries. No escalation. Record: `.andromeda/runs/2026-10-07T12-34-00-wrap/fanout-results.md`, `cascade-dispositions.md`.
- The sweep left no stale claim; 10 leaf files re-derived. The three "no driver, CLI or MCP command exposes … yet" clauses and every "no command can type yet" stand: nothing executes a verb.

## Notes
- **A rule is proposed for the playbook and waits on the operator's word:** a proposal graded `escalate` by a detector's own severity, where that detector reports its invariant holding and the plan's reviewed list names the change, is applied without a halt. Third wrap of the class (06-51 escalated it, 08-23 and this one applied it). Not appended.
- Two of the 13 proposals carried coordinates read from the tree — the report gave none for the new test modules, the class the previous handoff recorded. Handled the same way. Say so if that class should halt instead, or if the report should always carry a new test module's line range.
- The report's own count-site search missed one site (the keyed contract states "unit tests 13"); the test-plan detector found it, and the report carries a marked correction.
- Route: seven CARRYs pinned from this chunk — `advance`'s wiring on "Act by id"; detection and two open decisions (`stale` against `not-found`; `off-screen` with no verb that scrolls) on "Refusal detection"; the cause field's domain on "Driver command spans"; "one verb table" on "Driver CLI", "MCP surface", "Self-description" and "Headless screenshot". One residual: the cost of no wait on a load. Epoch 4 reads 9 entries, 3 markerless; no split was asked for.
- `Command`, `Call` and `ArgValue` print an id and typed text under `Debug`: no span or log fields them (obs-plan §4, carried on "Driver command spans").
- Not checked by any local gate: the MSRV build and the windows, macOS, iOS and android legs — the fork's CI run on `772c770f` is their one witness; this wrap's commit adds records and spec text, no source.
- Still not measured: typed text in a sink-installing host's log (the CARRY on "Act by id"); `log.file` occurrences. Still owed: the windowed witness of the accessibility-tree refresh (the CARRY on "Stand a11y assertions").
- Left open on purpose: whether test functions belong in the code audit's `over_ceiling` scalar (the Epoch 3 audit's question to the founder).
- No gated record, no PREREQ, no WATCH on the tail.
- Last failed command: none.

## Deferred learnings
- None deferred this session. One Tier 3 entry was corrected in place: the per-crate clippy learning stated the wrong cause (it is blitz-dom's `file-input` feature being off in a per-crate build, not `--all-targets` or test code).
- recurrence-despite-learning: "A per-crate `cargo clippy` is not the CI lint leg" (Tier 3, 2026-10-07) — run again at implement, in the session that had just read this line; third recurrence.
- carried, still unreviewed: "The project's Bash guards refuse a heredoc written to a file and a leading cd" (Tier 3, 2026-10-06) · "A chunk that moves cited source lines stales the masters' file:line citations" (Tier 3, 2026-10-05) · "A grep hit seen through a clipped view is not read" (Tier 3, 2026-10-06) · "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05).
Review with `/andromeda-wrap-session --review` if any should be applied.
