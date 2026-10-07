# Curation — 2026-10-07-audit-corrections

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A sed or grep address that ends at an item's name also selects every item whose name opens with it"
  Filters: 1 dup · 2 task-specific · 0 conflict · 0 deferred
  No-other-home: "A sed or grep address that ends at an item's name also selects every item whose name opens with it"
```

## Applied
- Tier 3 — "A sed or grep address that ends at an item's name also selects every item whose name opens with it" (confidence 0.8: verified by measurement +0.4, specific technical detail with context +0.2, reached no other durable home +0.2).
  Proof: the chunk's table-comparison gate read red on a correct tree at the first implement run — exit 1, 33 unmatched lines, all on the base side, the 33 lines of a test whose name opens with the table's (`evidence/implement-measurements.md`, "Entry 8"); the plan's entry was revised to close each name and read green at the re-entry run (exit 0, no output). The plan's own control for the entry had been built with the same address and had compared equal (research.md, the revision finding, recorded there as an inference).
  No-other-home: the fact sits in no route annotation, no master body, no playbook or drift-base rule and no matrix note; the test-plan sidecar entry's Why names it as a trap, and a sidecar is history, not a loaded home.

## Rejected
- duplicate — "a module in a subdirectory of the integration-test directory is no test target, and is where the stand checks' shared tables live": this wrap's cascade wrote it into `.claude/rules/testing.md`'s generated body and the masters carry it.
- task-specific — "67 of 90 citations keep their numbers because most cite the head of a file": a reading of this chunk's line map.
- task-specific — the operator's direction to run the operator pass by hand: an authorisation given for this chunk's commit and push, not a standing convention.

## Not candidates
- The second implement run's evolve records carry `skill: "implement"` where the first run's carry `skill: "andromeda-implement"`: telemetry of the tool, recorded in the report's Decisions & corrections.
- The mutation tool's `--workspace` and `--output` behaviours: collector invocations, read at the chunk's plan, not in this session.

## Deferred learnings carried from earlier wraps
Neither of the two recurrences the last handoff carried recurred in this chunk: the scripted edits of the first implement run were followed by `cargo fmt --all` (its step record), and no Bash guard refusal occurred in this session.
