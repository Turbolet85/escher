# Codebase Research — 2026-10-24-stand-requirement-sweep

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (41 lines, read in full) — 1 Session Addition
  (2026-10-06, `timeout` exit 124) not applicable: no live leg here runs under `timeout`; `.claude/rules/testing.md`
  (auto-loaded, in full) — 3 additions applied: 2026-10-10 (search `tests/` and the owning crate for a table's name
  before calling a check unaffected), 2026-10-15 (assert a table's computed sum, never a hand literal), 2026-10-19 (a
  `cfg(unix)` gate moved → `cargo check --tests --target x86_64-pc-windows-gnu`).
- **Platform issues consulted:** none — the one CI bullet is a cancelled run, closed against its successor run's green
  over the same code tree; no failure signature to search.
- **External inputs:** none — every fact this chunk turns on lives in this repository.

## Files inspected
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` (60-613) — the 0.1.0 record: `WITNESSES: [(&str, &[Witness]); 16]`
  (`:236`), `Subject` `AllFour | Task | Fixture` (`:214-222`), `source()`'s `include_str!` map (`:532-576`),
  `every_requirement_names_its_witness` pinning length 16, the v010 row order and each `fn {test}(` (`:579-613`).
- `.github/scripts/ci-leg.sh` (1-60) — `test` = `cargo test --workspace --locked`; `coverage` floors at `:46`;
  `stand-repeat` = the `stand_*.rs` glob + eight named seven_guis targets, three rounds (`:48-60`).
- `.github/scripts/test_ci_workflows.py` (323-368) — `test_stand_repeat_leg_runs_three_rounds` asserts named words
  (`stand_settle`, `cli_flow`, `mcp_server`, `recording`, `stand_session_recording`, `stand_replay`), not the full list;
  `test_coverage_leg_reaches_the_floor_on_each_crate` asserts the three floor lines and `3 of 3`.
- `.github/scripts/test_blitz_tests_targets.py` (full) — pins the manifest's one `[[test]]` table, not the target list.
- `examples/seven_guis/tests/host_log.rs`, `mcp_log.rs` (grep) — both drive `lint`, a `snapshot` part (`{"part":1}`,
  answering `"parts":1`) and, in their recording tests, `serve --record` / `mcp --record` at `trace`, 0 needles.
- `examples/seven_guis/tests/recording.rs` (grep) — `record_checks.sh`'s clients write nothing to stdout/stderr at the
  default level (`:233`).
- `packages/escher-driver/src/lint.rs` (195-304, 478-504) — the defect loop and `truncated()`.
- `packages/blitz-dom/src/text_extent.rs` (full) — the limits stated in the doc (`:22-24`).
- `tests/blitz-tests/tests/stand_lint.rs` (246-282) — the CRUD-row test filters `Clipped | OffScreen` (`:263-267`).
- `escher-0.2.0/chunks/2026-10-19-visual-lint/report.md` (237-241) — «Untested reading».
- CI#40105170961 coverage job log (`gh run view --job 120906459770 --log`) — the per-crate and per-file readings.

## Graph impact (from the code-graph query)
- **text_extent** — 7 call sites: 1 non-test, `escher_driver::lint::truncated` @ `packages/escher-driver/src/lint.rs:487`;
  6 in `tests/blitz-tests/tests/text_extent.rs` (`:40`, `:65`, `:71`, `:92`, `:97`, `:106`) — a fixture for C1 adds
  callers in a new check only; no signature changes (trace `.andromeda/runs/2026-10-24T15-06-04-phase/tree-query-2026-10-24-stand-requirement-sweep.json`,
  rust plane rebuilt 8803 nodes / 53848 edges).

## Patterns detected
- **A requirement-to-check record held by the code it names** (`stand_requirement_sweep.rs:236-613`): rows of
  `(Subject, file, test)`, each found at compile time through `include_str!`, so a renamed test turns the record red.
- **Witness placement by file name** (`ci-leg.sh:50-53`): `stand-repeat` selects by the `stand_` prefix; a file outside
  it runs in `test` alone.
- **Coverage at HEAD** (CI#40105170961): escher-driver 4918/5019 = 97.99 % · dioxus-native-dom 2740/3202 = 85.57 % ·
  blitz-test-harness 949/1008 = 94.15 %, `3 of 3 crates at or above their floors`; blitz-dom (no floor) 15652/19817 =
  78.98 % (derived: the report rows under `packages/blitz-dom/src/`, summed), its 0.2.0 files `clock.rs` 90.00 % ·
  `node/range.rs` 99.25 % · `node/select.rs` 100 % · `text_extent.rs` 100 %.
- **The `truncated` equality** (`lint.rs:269-293`, `:483-503`): `truncated` fires iff `text_extent(e).text` (area > 0) is
  not within the clip region cut by `e`'s padding box on the axes `e` clips; independent of the `clipped`/`off-screen`
  arm. So: off-screen + text → `truncated` too; clipped + text → `truncated` only when the text box leaves the region.

## Conventions to follow
- **One file, one target, both layout modes** (`.claude/rules/testing.md` Integration; `for incremental in [false, true]`).
- **Stand boot** through `seven_guis::stand` / `fixtures::boot` with `stand::options(incremental)` — bundled DejaVu Sans,
  no font skip (`tests/blitz-tests/tests/fixtures/mod.rs`).
- **Assertion messages** carry a mode, a task and a row/step index only (`stand_requirement_sweep.rs:1-11`, `stand_lint.rs:1-9`).

## New files to create
- none — at P4 C1 lands in the two checks that own its readers (`stand_lint`, `text_extent`), one behaviour per file, no new target

## Files to modify
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` — the 0.2.0 rows of the record and their existence check
- `.github/scripts/ci-leg.sh` — stand-repeat's named list of the ten witnesses (the founder's word, P4)
- `.github/scripts/test_ci_workflows.py` — the stand-repeat pin, with the leg
- `examples/seven_guis/tests/host_log.rs` — the start --record clients' own stderr at trace
- `tests/blitz-tests/tests/stand_lint.rs` — C1: the `truncated` co-reading beside `clipped` and `off-screen` on an in-file page
- `tests/blitz-tests/tests/text_extent.rs` — C1: the reader's three stated limits (RTL, vertical, a transformed box)
- `tests/blitz-tests/tests/stand_act_spans.rs` — a paged part read in the sink-capturing child
- `tests/blitz-tests/tests/fixtures/mod.rs` — the over-budget page, now read by two checks
- `tests/blitz-tests/tests/stand_snapshot_parts.rs` — reads the over-budget page from the fixtures module

## Sweeps
- `stand_requirement_sweep|WITNESSES|every_requirement_names_its_witness` over the tracked tree (`git grep`, runs and
  chunk folders excluded): 30 hits · 0 changed · 30 no-change (spec masters, sidecars, docs and metrics carrying counts —
  the wrap's to reconcile; the only code pin is the file's own `:588-595`).
- The witness inventory (`/tmp/…/scratchpad/witnesses.py`, the matrix refs of v020-02…09 + C1): 32 rows · every named
  test present · 10 integration files outside `stand-repeat` (listed in scope.md) + 3 unit tests.
- `--record` over `examples/seven_guis/tests/`: 21 hits · 0 changed · 21 no-change (launch and usage sites; the stderr
  readings at `trace` sit in `host_log.rs`/`mcp_log.rs`, the client's at default level in `recording.rs:233`).

## Open questions
- Where do the 0.2.0 rows live — `WITNESSES` widened, a second table beside it, or a file of their own? → blocks: plan-decision
- How does a witness outside `stand-repeat` get inside — the leg's selection widened by name, or a `stand_*` carrier? → blocks: plan-decision
- The two unmeasured stderr readings (a paged screen's part through a host; the `start --record` client at `trace`):
  measure or state as limits? → blocks: plan-decision
