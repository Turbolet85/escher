# Session Handoff

**Last Updated:** 2026-10-06T21:16:48Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-06-id-stability-across-code-edits — feat(2026-10-06-id-stability-across-code-edits): an element left in place keeps its id when the code around it changes

## Position
- Done: 2026-10-06-id-stability-across-code-edits — ids hold across code edits (v010-16, verified). An unkeyed element under a keyed element of its own component reads `{key}//{relative path}`; every element an agent can act on reads an author key, checked by `DioxusDocument::unkeyed_actionable()`. Workspace 490 · 0 · 5 (127 result lines), stand `ok` 41, `Ran 64 tests`, fork CI green 16/16 on `75f12a09`.
- Next: "Snapshot state fidelity — enabled, checked, value, focused per control; disabled reads disabled, typed value reads back, password values masked (v010-05)" (working-route.md:43) — promote and plan it with /andromeda-phase.

## Work done
- The id grammar gained an anchored tier in `element_id.rs`; a new accessibility-gated `actionable` module holds the check; CRUD's rows read `crud-person-{id}` and Home's seven cards `task-card-{slug}`. Two new stand checks (`stand_id_edits`, `stand_actionable_keys`), four re-pinned.
- The three tasks outside the lean four are measured, not keyed: Temperature Converter 2 · Circle Drawer 3 · Cells 676 unkeyed actionable elements, pinned by a test.

## Drift resolved
30 detector proposals (architecture 13 · security-plan 4 · layout-templates 3 · test-plan 10) and 4 raised from the plan's expected-amendments list (a11y-plan 3 · design-system 1), all applied; obs-plan returned none and needed none. Six sidecar entries. The cascade re-derived CLAUDE.md, the two crate notes, four summaries and docs, and the a11y rule. 0 escalations: the grammar change and the public check carry the founder's ratification from phase.

## Notes
- Two CARRYs pinned: "Stand requirement sweep" owns keying the three non-lean tasks (the wrap's placement; "Stand a11y assertions" was the other candidate — move it if that reads wrong), and "Driver CLI" carries that the check has no wire form and must leave its `NodeId` field behind when it gets one.
- v010-01 carries a premise-correction note: an unkeyed element under a keyed element of its own component now reads an anchored path, and CRUD's rows read author keys.
- No gate deferral, so no PREREQ is pinned. No new learnings curated.
- Last failed command: none.

## Deferred learnings
- recurrence-despite-learning: "The project's Bash guards refuse a heredoc written to a file and a leading cd, and read payload prose too" (Tier 3, 2026-10-06). Seventh recurrence, at this wrap.
- recurrence-despite-learning: "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05). Recurred at this wrap: a citation tally typed into the report draft without a count, corrected before the fan-out.
- carried from session 15, still unreviewed, no recurrence this session: "On this host `grep` is ugrep, and a long bounded repetition can print nothing" (Tier 3, 2026-10-06).
Review with `/andromeda-wrap-session --review` if any should be applied.
