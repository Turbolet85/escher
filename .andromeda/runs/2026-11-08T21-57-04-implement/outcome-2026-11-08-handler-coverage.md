# Outcome — 2026-11-08-handler-coverage · green + smoke

**Outcome:** green + smoke. The gate block ran once and read green on every non-operator entry: `entries 19 · green 16 ·
red 0 · recorded 0 · timeout 0 · not-run 3`. The three not-run entries are the operator legs 17 · 18 · 19. Trail:
`.andromeda/runs/2026-11-08T21-57-04-implement/gate-2026-11-08-handler-coverage.json`.

- Entry 13 (`agent-run.sh run stand`) measured **262 passed · 0 failed · 7 ignored over 54 files**, 8 of the tests
  from `stand_handler_coverage`. The plan predicted 54 files.
- Matrix: **v030-07** `planned → implemented`. Its ref is six tests of `stand_handler_coverage` plus `cli_commands`'s
  `coverage_replays_a_kept_check_and_lists_the_handlers_it_left`.
- **v030-06** stays `implemented`. Its ref is widened to `stand_actions`' witness ·
  `stand_replay.rs::a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes`, the owed test (CARRY
  `cry-sxrivdk3`), which now passes in both layout modes. Moving it to `verified` is the wrap's.

**Next:** the OPERATOR PASS, entries **17** (hygiene), **18** (push) and **19** (the CI read), then
`/andromeda-wrap-session`.

## Files changed
New (3):
- `packages/dioxus-native-dom/src/reached.rs`
- `packages/escher-driver/src/coverage.rs`
- `tests/blitz-tests/tests/stand_handler_coverage.rs`

Modified (15 listed + 1 recorded):
- dioxus-native-dom: `dioxus_document.rs`, `dioxus_document_tests.rs`, `lib.rs`
- escher-driver: `replay.rs`, `lib.rs`, `json.rs`, `cli.rs`, `cli_tests.rs`
- seven_guis: `src/session_host.rs`, `tests/cli_commands.rs`, `tests/common/mod.rs`, `tests/host_log.rs`
- blitz-tests: `tests/stand_replay.rs`, `tests/common/mod.rs`
- `docs/capabilities.md`
- recorded: `packages/dioxus-native-dom/src/element_id.rs`

The scope read says `scope: clean — changed 19 · listed 18 · recorded 1 (companion 0 · mechanical 0 · in-intent 1 ·
widening 0)`. The step-0 sweep is in `.andromeda/cache/chunks/2026-11-08-handler-coverage/evidence/sweep.md`.

## Deviations from the plan, with their justifications
1. **The reached record reads `element_listener_ids`, not `element_facts`** (step 2).
   - Scope-record line: `` - `packages/dioxus-native-dom/src/element_id.rs` · in-intent · serves step 2 · self ``.
   - Why: `element_facts` is `#[cfg(feature = "accessibility")]`, but step 1 makes the record ungated. The wasm build
     (`seven_guis --no-default-features --features hybrid`) compiles dioxus-native-dom with no `accessibility`
     feature, so an ungated call to `element_facts` would not build there.
   - The new crate-private reader is one vdom walk on every build, giving each element's stable id and its current
     listeners, which is what `element_facts` reads for those two fields.
   - `cargo check -p dioxus-native-dom --no-default-features --locked` compiles. Its 4 dead-code warnings
     (`OwnerScope` fields, `SignalRegistry` readers) stood before this change, and none involves the new code.
2. **Cells' rows in `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` are partly generated** (step 8).
   - Cells' boot screen reads 678 handlers, 676 of them grid-cell `click`s.
   - The stated rows hold the named handlers (`back-btn` `click`, `cells-grid` `keydown`). The grid's cells follow from
     `grid_cells()`, row by row from `cell-A0` to `cell-Z25`, and the drives' clicked cells are stated as
     `CELLS_CLICKED_BY_DRIVES` (`cell-A0`, `cell-C0`).
   - Check (a) holds the table equal to the graph's own reading on a fresh boot, in both modes.
   - `UNEXERCISED_AFTER_DRIVES` covers all 7 `GRAPH_DRIVES` tasks: the two Home-opened tasks, Temperature Converter
     and Circle Drawer, are included, opened from Home as `stand_graph` opens them.
   - Measured with a throwaway target, `tests/blitz-tests/tests/zz_measure_coverage.rs`, deleted before the gates. Each
     row was reconciled with its drives by hand. CRUD's `crud-person-3` is the row Create adds; the Cells editor's
     `keydown`, `input` and `blur` are all reached.
3. **`coverage_needles()` reads the CRUD screen, not the Counter** (step 11).
   - `host_log`'s run is a `coverage crud <check>` over a CRUD session kept through the binary, with a `type` of the
     sentinel and a row click.
   - Why: CRUD's screen holds names and a typed field, so the run's readings of 0 name needles and 0 typed-text
     occurrences test something; on the Counter they would hold trivially.
   - The Counter run stands in `cli_commands`, as step 11 asks.
4. **`replay_each` is `pub(crate)`, and its hook takes `&Session`** (step 3). Nothing outside the crate calls it.
5. **The role's refusal of a `--session` or `--record` word among its paths is usage.** That follows from step 6's
   «any other shape is usage» and «takes no `--session` and no `--record`».
6. **Unit tests beyond the plan's list.**
   - `json.rs`: `a_coverage_line_keys_each_handler_and_leaves_out_an_absent_owner`.
   - `cli_tests.rs`: `the_coverage_role_reads_its_checks_refusing_any_it_cannot_read` (a missing file, a non-check,
     one past the bound, a check of another app, a failing boot and an accepted run of two checks).
   - The scratch files go under the workspace `target/tmp`, through `CARGO_MANIFEST_DIR`, as the crate's other unit
     tests do. No new env read.

## Gate guard lines
No `gate.py guard` entry is in the block. Entry 8 is a `git diff --quiet` probe: green, nothing moved.

## Surfaced / stuck
None.

## Smoke (P3)
**Smoke ✓**, by hand, with the real `target/debug/escher-session`:
- `start counter --session target/tmp/smk-cov-2229 --record target/tmp/smk-cov-rec-2229`: exit 0.
- `click --id counter-increment`: exit 0.
- `stop`: exit 0, the host gone.
- `coverage counter …/check-1.json` answers `{"unexercised":[{"id":"back-btn","event":"click"}],"checks":1,"held":1}`,
  exit 0.
- A missing path is refused `malformed`, exit 1, with no path in the line.
- `coverage nowhere …`: exit 2.

`target/tmp/smk-cov-rec-2229` (the kept check, owner-only, under the gitignored `target/`) is left standing: the
host's deny rule refuses a recursive `rm`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `escher-session` host (counter, pid 3081409) | this run (the P3 smoke's `start`) | terminated — ended by `stop`; the census reads none |
| `escher-session` hosts and clients of entries 7 and 14 | a gate entry (the tool) | terminated — census: none running |
| `cargo` | a gate entry | terminated — census: none running |

Census: `gate.py census --comm escher-session --comm seven_guis_native --comm cargo` → `census: none running`
(trail `.andromeda/runs/2026-11-08T21-57-04-implement/census-2026-11-08-handler-coverage.json`).

## Decisions & corrections
- **Guard refusal 1:** a `cat >>` heredoc append to `stand_replay.rs` was refused by the Bash guard («a cat/tee
  heredoc with a file target»). Re-issued once through the Edit tool.
- **Guard refusal 2:** a recursive `rm` in the smoke's compound was refused by the host deny rule, and nothing in the
  call ran. Re-issued without it in fresh directories; the target is named above.
- **A wakeup armed against the body's rule:** a `ScheduleWakeup` was armed while the backgrounded block ran, against
  the rule that its notification is the wake. It was cancelled at once (`stop: true`) before it fired.
- **The pin sweep, run first (testing.md 2026-10-10):** besides the plan's known `cli_tests.rs` pin, it found
  `cli_commands.rs:683-684`'s usage fragments. They still held and were extended. No recurrence of the learning.
