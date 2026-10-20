# Gate entries 7, 10 and 14 — an operator edit of plan.md (the gated pass)

**Authority:** the operator's word, given in this session on 2026-10-20: «On my word, an operator edit of plan.md, noted
in evidence/gate-amend-gated.md: entries 10 and 14 re-expect HEAD 537a7ced8's measured counts plus this chunk's own
(192 · 0 · 7; 1087 passed), each basis named; entry 7 takes defer = "voided — the founder's F1 word (inputs#I7) rules
the crossing it forbade; its base also reads the two later chunks".»

**What was red.** In /andromeda-implement's gated firing of the block (`.andromeda/runs/2026-10-20T07-11-30-implement/`,
firing 1), three entries read red. All three are atoms written at the first take-up, before the founder's word and
before two later chunks were committed:
- **Entry 10** read `"passed": 192, "failed": 0, "ignored": 7` against its atom 183.
- **Entry 14** read `lines 204 passed 1087 failed 0 ignored 12` against its atom `lines 201 passed 1055`.
- **Entry 7** read 40 files against its atom 0.

**The edit.** These three changes, and nothing else in the plan:
- **Entry 10**'s atom becomes `contains "selection": "stand", "passed": 192, "failed": 0, "ignored": 7`.
  - Basis: HEAD 537a7ced8 measured `run stand` at 190 · 0 · 7 over 47 files
    (`escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/counts.md`).
  - This chunk adds 2: `stand_lint::the_lint_verb_answers_what_the_reader_reads` and
    `stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command`.
  - The stand file count stays 47, since the gated pass adds no `stand_*` file.
- **Entry 14**'s atom becomes `contains lines 204 passed 1087 failed 0 ignored 12`.
  - Basis: HEAD 537a7ced8 measured the workspace tally at `lines 204 passed 1083 failed 0 ignored 12` (the same record).
  - This chunk adds 4 passed: the two above, plus `cli_commands::lint_answers_one_line_of_findings` and
    `mcp_server::a_lint_tool_call_answers_its_line_of_findings`.
  - Lines are unmoved, since no new test target was added.
- **Entry 7** takes `defer = "voided — the founder's F1 word (inputs#I7) rules the crossing it forbade; its base also reads
  the two later chunks"`.
  - Its guard, that nothing crosses a carrier before the founder's word, is superseded: his F1 option 1 is that
    crossing, and plan step 12 lists its files.
  - Its base e97a7f41b also counts 23 files the two later chunks committed (2026-10-20-load-wait and
    2026-10-20-select-by-id).
  - It now reads `not run — defer (key)`.

**Re-fire.** `gate.py run --plan … --entry {n}`, in implement's run dir:

```
 10 integration green · exit 0 · 121.16s · 25931 B → 10.2.log · bash scripts/agent-run.sh run stand
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
 14 probe       green · exit 0 · 0.0s · 42 B → 14.2.log · grep -E '^test result:' target/ci-logs/test.log | awk '{p+… (146 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

Entry 14 read `target/ci-logs/test.log` as written at 2026-10-20T07:24:48Z by this run's entry 13 (`ci-leg.sh fast`).
