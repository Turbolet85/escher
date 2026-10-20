# Counts — 2026-10-19-visual-lint

Measured by `/andromeda-implement`: one firing of the block, `.andromeda/runs/2026-10-20T00-14-31-implement/`
(trail `gate-2026-10-19-visual-lint.json`), summary `entries 22 · green 15 · red 0 · recorded 0 · timeout 0 · not-run 7`.
The 7 not run are the four `operator` entries (16–19) and the three `gated:` defers (20–22).

| reading | forecast | measured | entry |
|---|---|---|---|
| `text_extent` tests | 3 passed · 0 failed | 3 passed · 0 failed | 1 |
| `lint.rs` unit tests | the two test lines | 2 passed (92 filtered out) | 2 |
| `stand_lint` tests | 5 test lines | 5 passed · 0 failed | 3 |
| escher-driver `info_span!` sites | 2 | 2 | 5 |
| silence census over the four new files | 0 | 0 (exit 1) | 6 |
| tracked files changed outside the six | 0 | 0 (exit 1) | 7 |
| untracked code files beyond the four | 0 | 0 (exit 1) | 8 |
| `agent-run.sh run stand` | 183 · 0 · 7 over 45 files | 183 · 0 · 7 (`run.end`, `outcome: passed`) | 10 |
| `agent-run.sh run text_extent` | 3 · 0 · 0 | 3 · 0 · 0 | 11 |
| a11y leg | 43 tests over 6 files | 43 passed over 6 targets (6 · 6 · 4 · 8 · 5 · 14) | 12 |
| workspace tally (`target/ci-logs/test.log`, fresh from entry 13) | 201 lines · 1055 · 0 · 12 | 201 lines · 1055 passed · 0 failed · 12 ignored | 14 |
| `ci-leg.sh fast` · `ci-leg.sh doc` | green | green | 13 · 15 |
| blitz-tests `tests/*.rs` files | 138 → 140 | 140 | — |
| blitz-tests `stand_*.rs` files | 44 → 45 | 45 | — |

Every forecast read as predicted on the first firing.

## The gated pass (plan step 12, the founder's F1 word option 1, inputs#I7)

Measured by `/andromeda-implement` in gated mode: one firing of the block, `.andromeda/runs/2026-10-20T07-11-30-implement/`
(trail `gate-2026-10-19-visual-lint.json`), entries 20–22 voided `block cleared`, summary `entries 22 · green 15 · red 3
(7,10,14) · recorded 0 · timeout 0 · not-run 4 · voided 3 (20,21,22)`. The base is HEAD 537a7ced8, two chunks past the
in-process take-up (2026-10-20-load-wait, 2026-10-20-select-by-id), whose own counts are the control below.

| reading | control (HEAD, select-by-id's evidence/counts.md) | measured | moved by | entry |
|---|---|---|---|---|
| founder-word F1 row | — | 1 | evidence/founder-word.md | 20 |
| `stand_lint` with the verb test | 5 tests | 6 passed · 0 failed | `the_lint_verb_answers_what_the_reader_reads` | 21 |
| seven_guis `cli_commands` | 12 passed | 13 passed | `lint_answers_one_line_of_findings` | 22 |
| `agent-run.sh run stand` | 190 · 0 · 7 over 47 files | 192 · 0 · 7 over 47 files | stand_lint +1, stand_mcp_tools +1 (`a_lint_tool_call_answers_the_readers_findings_and_parents_its_command`) | 10 |
| workspace tally | lines 204 · 1083 · 0 · 12 | lines 204 · 1087 · 0 · 12 | the two above, `cli_commands` +1, `mcp_server` +1 (`a_lint_tool_call_answers_its_line_of_findings`) | 14 |
| tracked files changed outside the six since e97a7f41b | — | 40 | 23 committed by the two later chunks (HEAD side) + 20 this tree's step 12 (19 listed gated + `stand_act_scroll.rs`, the scope record's companion), 3 on both sides | 7 |
| escher-driver unit tests | 99 | 99 passed (rows added inside existing tests: command 31 · 40 rows, json 8 rows, cli 96 rows, recording 10 calls) | — | 13 |
| `info_span!` sites · silence census · untracked code files | 2 · 0 · 0 | 2 · 0 · 0 | — | 5 · 6 · 8 |
| a11y leg · `ci-leg.sh fast` · `ci-leg.sh doc` | green | green | — | 12 · 13 · 15 |

Entries 10 and 14 read red on their `expect` atoms (183 · 1055), which the in-process take-up wrote before the two later
chunks moved the counts; each measured count is the control plus this chunk's tests, by name. Entry 7 is the guard that
nothing crossed a carrier before the founder's word; his word, option 1, is that crossing, so the guard's premise has
fallen and step 12's files are its hits.
