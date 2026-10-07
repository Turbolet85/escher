# Implement measurements — 2026-10-07-audit-corrections

Read from the gate run of 2026-10-07T03:34Z-03:36Z (run dir `.andromeda/runs/2026-10-07T03-27-51-implement/`,
trail `gate-2026-10-07-audit-corrections.json`), each number from that entry's own log. Entries are numbered as the
gate tool numbers the plan's block (26 entries).

## Forecasts against what was measured

| Forecast (plan, Implementation notes) | Measured | Entry | Verdict |
|---|---|---|---|
| new bridge tests 0 -> 6 | `6 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out` | 1 | holds |
| crate unit tests 49 -> 55 | `55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` | 2 | holds |
| the filter selects seven mutants | last line `7` | 3 | holds |
| seven tested, seven caught | last line `7 mutants tested in 36s: 7 caught` | 4 | holds |
| functions over the ceiling 78 -> 74 | last line `74` | 6 | holds |
| detector 287 · 13 · 9 -> about 278 · 3 · 0 | `280 4 0` | 7 | disproved in two of three: 280 clones, 4 stand pairs; 0 across two files holds |
| shared definitions in stand files 19 -> 0 | last line `0` | 9 | holds |
| shared items in the shared module: 9 | last line `9` | 10 | holds |
| assertion-macro lines 385 -> 384 | last line `384` | 11 | holds |
| restructured test names and the child: 4 | last line `4` | 12 | holds |
| inherited file: 0 lines removed, 3 added | last lines `0` and `3` | 13, 14 | holds |
| stand `ok` count 63 -> 63 | `"passed": 63, "failed": 0, "ignored": 1, "cargo_exit": 0, "outcome": "passed"` | 19 | holds |
| workspace 542 · 0 · 5 over 131 -> 548 · 0 · 5 over 131 | 548 passed · 0 failed · 5 ignored over 131 result lines (`target/ci-logs/test.log`, written by entry 21) | 21 | holds |
| CI scripts `Ran 64 tests`, unchanged | `Ran 64 tests in 3.381s` · `OK` (`target/ci-logs/ci-scripts.log`) | 21 | holds |

## Red before green — the mutation entry

- Red half (the plan's `baseline`, read at P5 on the untouched tree, 2026-10-07T03:23Z): exit 2, last line
  `7 mutants tested in 40s: 7 missed`.
- Green half (entry 4, the finished tree): exit 0, the log whole:

```
Found 7 mutants to test
ok       Unmutated baseline in 25s build + 1s test
 INFO Auto-set test timeout to 20s
7 mutants tested in 36s: 7 caught
```

## The detector's four remaining stand pairs (entry 7, report-only)

None lies across two files. All four are repeats inside one file:

- `tests/blitz-tests/tests/stand_diff.rs` 289-294 with 414-419, 6 lines
- `tests/blitz-tests/tests/stand_element_ids.rs` 155-163 with 176-184, 9 lines — the shared opening of
  `author_keyed_elements_read_their_key` and the function split out of `unkeyed_elements_read_their_component_path`;
  the plan named it as going "only if step 5's split removes it", and the split moved it without removing it
- `tests/blitz-tests/tests/stand_id_edits.rs` 168-177 with 188-197, 10 lines
- `tests/blitz-tests/tests/stand_snapshot.rs` 55-61 with 91-96, 7 lines

## Entry 8 — red by its own pattern, not by the tables

Entry 8 (the table comparison) read `red`: exit 1, 1453 B of output, 33 unmatched lines, every one on the base
side (33 lines opening `<`, 0 opening `>`).

Cause, measured: the entry selects the base copy of `controls` with the sed address `/^fn controls/`, which has no
closing character after the name. On the base file it matches two lines:

```
17:fn controls(task: LeanTask) -> &'static [(&'static str, Role)] {
302:fn controls_lie_inside_the_viewport() {
```

Line 302 is a test, 33 lines long through its closing brace, that stays in `stand_snapshot.rs` and is not a table.
The 33 unmatched lines are that test's. The shared module holds no item named `controls_lie_inside_the_viewport`,
and step 3 fixes its content at nine items, so no correct tree can read this entry green as written.

The same comparison with each function name closed by its parenthesis — `/^fn controls(/`, `/^fn rendered(/` on the
base side, `/^pub fn controls(/`, `/^pub fn rendered(/` on the module side, nothing else changed — run by hand on the
finished tree:

- sorted, as the entry sorts: exit 0, no output;
- unsorted, in file order: exit 0, no output (76 lines on each side);
- control, the module side with the row `("crud-update", Role::Button)` dropped: exit 1, one unmatched line, that row.

So the property the entry is for holds — the three tables in `tests/blitz-tests/tests/common/mod.rs` are the base
copies line for line, `pub` aside — and the entry as authored cannot say so. The plan was not edited.

---

# The re-entry run — the block as revised

Read from the gate run of 2026-10-07T03:44Z-03:46Z (run dir `.andromeda/runs/2026-10-07T03-44-12-implement/`,
trail `gate-2026-10-07-audit-corrections.json`), one call over the whole block, each number from that entry's own
log. This run wrote no source: the tree is HEAD `48f8b5f2` plus the stopped run's uncommitted edits (2 new files,
10 modified). The readings above are the stopped run's and stand beside these; nothing here is copied from them.

Summary line: `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3` — the three not run are the
operator entries 24, 25 and 26.

## Forecasts against what this run measured

| Forecast (plan, Implementation notes) | Measured | Entry | Verdict |
|---|---|---|---|
| new bridge tests 0 -> 6 | `6 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out` | 1 | holds |
| crate unit tests 49 -> 55 | `55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` | 2 | holds |
| the filter selects seven mutants | last line `7` | 3 | holds |
| seven tested, seven caught | last line `7 mutants tested in 36s: 7 caught` | 4 | holds |
| the crate builds with default features off | exit 0 | 5 | holds |
| functions over the ceiling 78 -> 74 | last line `74` | 6 | holds |
| detector 287 · 13 · 9 -> about 278 · 3 · 0 | `280 4 0` | 7 | recorded, as the stopped run read it: 280 clones, 4 stand pairs, 0 across two files |
| the three tables are the base copies | exit 0, no output | 8 | holds — the entry the revision changed, green as revised |
| shared definitions in stand files 19 -> 0 | last line `0` | 9 | holds |
| shared items in the shared module: 9 | last line `9` | 10 | holds |
| assertion-macro lines 385 -> 384 | last line `384` | 11 | holds |
| restructured test names and the child: 4 | last line `4` | 12 | holds |
| inherited file: 0 lines removed, 3 added | last lines `0` and `3` | 13, 14 | holds |
| the 13 in-file id tests byte-identical to the base | exit 0, no output | 15 | holds |
| no manifest, lockfile, markup, script, workflow or engine change | exit 0 | 16 | holds |
| no print, log site, env read, file write or listener added | last line `0` | 17 | holds |
| stand `ok` count 63 -> 63 | `"passed": 63, "failed": 0, "ignored": 1, "cargo_exit": 0, "outcome": "passed"`; artifact `target/agent-run/events.jsonl` read `fresh` | 19 | holds |
| workspace 542 · 0 · 5 over 131 -> 548 · 0 · 5 over 131 | 548 passed · 0 failed · 5 ignored over 131 result lines, every one `ok` (`target/ci-logs/test.log`, written by entry 21 of this run) | 21 | holds |
| CI scripts `Ran 64 tests`, unchanged | `Ran 64 tests in 3.376s` · `OK` (`target/ci-logs/ci-scripts.log`, written by entry 21 of this run) | 21 | holds |
| rustdoc `-D warnings` over the workspace | exit 0 | 22 | holds |
| the a11y leg keeps its three files | exit 0; three result lines, 6 · 6 · 3 passed, 0 failed | 23 | holds |

## The mutation entry, green half, this run

Entry 4, exit 0, the log whole:

```
Found 7 mutants to test
ok       Unmutated baseline in 25s build + 1s test
 INFO Auto-set test timeout to 20s
7 mutants tested in 36s: 7 caught
```

The red half is the plan's `baseline` (the untouched tree, 2026-10-07T03:23Z): exit 2, last line
`7 mutants tested in 40s: 7 missed`.

## The detector's four remaining stand pairs, this run

Entry 7 prints three numbers only, so the pairs were listed by running the entry's own detector command once more
by hand on the same tree, with the report read pair by pair instead of counted (2026-10-07T03:46Z; it read `280 4 0`
again). None lies across two files:

- `tests/blitz-tests/tests/stand_diff.rs` 289-294 with 414-419, 6 lines
- `tests/blitz-tests/tests/stand_element_ids.rs` 155-163 with 176-184, 9 lines
- `tests/blitz-tests/tests/stand_id_edits.rs` 168-177 with 188-197, 10 lines
- `tests/blitz-tests/tests/stand_snapshot.rs` 55-61 with 91-96, 7 lines

## Not run by this skill

Entries 24 (the hygiene read before the pre-CI commit), 25 (the pre-push gate, the clean-tree guard and the push)
and 26 (the CI read on the pushed sha) are the operator pass's. The CI acceptance criterion is unproven until 26
reads `verdict: green`.
