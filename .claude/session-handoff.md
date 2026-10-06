# Session Handoff

**Last Updated:** 2026-10-06T11:07:22Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-06-id-persistence — feat(2026-10-06-id-persistence): stable ids persist — same id across re-render, remount and fresh process on the stand

## Position
- Done: 2026-10-06-id-persistence — every lean stand element keeps its stable id across a re-render, a remount and a fresh process, in both layout modes.
  - CRUD rows are now keyed by a model-assigned person id, so a row's id follows its person.
  - v010-02 is verified. Its remount clause was refined: the html/head/body/#main skeleton keeps its `NodeId`.
  - Local gates are green, and fork CI on the pre-CI commit `8c1dd035` is green (CI#37449052968, 16/16).
- Next: "Project README — the repository front page describes escher, not Blitz", the next entry of Epoch 2. The operator asked for it this wrap. Promote and plan it with /andromeda-phase. After it comes Accessibility-tree identity (v010-03).

## Work done
- `crud.rs` changes: `Person.id`, `key: "{person.id}"`, and the filter moved into the `for` iterator.
- New: `tests/blitz-tests/tests/stand_id_persistence.rs` (3 stand checks + 1 ignored re-exec child).
- Workspace tests went 441 · 0 · 4 → 444 · 0 · 5. `run stand` now reads 20 ok stand events.
- Details: `escher-0.1.0/chunks/2026-10-06-id-persistence/report.md`.

## Drift resolved
- architecture:
  - the Dioxus DOM bridge records persistence and the person-keyed CRUD row;
  - a `key:` inside an `if` within a `for` does not key a Dioxus list;
  - Process-wide state registers both test-binary re-exec spawns (the operator's P5 ruling: not a widening).
- security-plan: the `id` row now says a row key is a model-assigned `u64`.
- layout-templates: the row key changed from `{i}` to `{person.id}`.
- test-plan: §1 coverage and the §3 and §9 re-counts.
- obs-plan: "no `println!` in the stand checks" now carries the re-exec child as its one exception.
- 0 P2 escalations. One P7 escalation was resolved with the operator: v010-02's acceptance was refined with evidence (the skeleton exception).

## Notes
- FOR THE FOUNDER (carried, unchanged this wrap):
  - The cold-agent pipe's crossings (FOR DISCUSSION 7), the falsy-`disabled` engine fix and the `coverage-report` upload widening stand PROVISIONAL.
  - Opt-in OTel export is DEFERRED (CARRY on "Driver command spans").
- Decisions this chunk (operator):
  - keep the filtered-iterator row loop;
  - make plan entry 3 whitespace-insensitive;
  - the agent ran the operator pass;
  - insert the README entry ahead of Accessibility-tree identity;
  - refine v010-02 with evidence.
- Last failed command: none.

## Deferred learnings
- recurrence-despite-learning: "The project's Bash guards refuse a heredoc written to a file and a leading cd, and read payload prose too" (Tier 3, 2026-10-06). It recurred twice this chunk: a `cat >>` evidence heredoc and a `cd` into the scratchpad.
- recurrence-despite-learning: "On this host `grep` is ugrep, and a long bounded repetition can print nothing" (Tier 3, 2026-10-06). A `grep -oE '.{0,60}…'` site sweep failed with "exceeds complexity limits".
Review with `/andromeda-wrap-session --review` if any should be applied.

## Session End Status
Completed normally at 2026-10-06 13:27:27
