# tests · cascade read · group 4 of 4

## Carried
- (.andromeda/test-plan-detail/ci-integration.md:8) The refactor must leave every leg the chunk re-runs green as the legs define them: the `doc` leg over the workspace, and `stand-repeat`, whose `stand_*.rs` glob takes the edited `stand_requirement_sweep` and the re-read `stand_lint`, each kept in its file and name so the glob still holds it.
  > each of the thirteen linux ci.yml jobs (twelve until 2026-10-14-quality-gates) runs `bash .github/scripts/ci-leg.sh {leg}`
  (.andromeda/test-plan-detail/ci-integration.md:8)
  > doc (`cargo doc --workspace --no-deps`)
  (.andromeda/test-plan-detail/ci-integration.md:8)
  > stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 50 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay, `stand_lint` since 2026-10-19-visual-lint `stand_load_wait` since 2026-10-20-load-wait, `stand_act_select`, `stand_snapshot_parts`, `stand_cells_served` and `stand_load_page`
  (.andromeda/test-plan-detail/ci-integration.md:10)
  > the first red command ends the leg with its status — no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`
  (.andromeda/test-plan.md:377)
  > - **Legs:** each linux ci.yml job runs `bash .github/scripts/ci-leg.sh {leg}`, every cargo leg `--locked` but `examples/wasm_hello` (no `Cargo.lock`); the dev host runs the same script, so every linux leg is host-reproducible; the `stand-repeat` leg runs the stand checks, ten named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no `--ignored`, no env var
  (.andromeda/test-plan.md:387)
  > - **Local pre-push gate:** `bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts, stopping at the first red (.github/scripts/ci-leg.sh:80-85)
- (.andromeda/test-plan-detail/ci-integration.md:29) The two touched tests and `stand_lint` must be read green under both the workspace build (the `fast` leg) and the `-p blitz-tests` build (`stand-repeat`, `agent-run.sh`), since the lint's bounds-based findings read through a different body of the engine's bounds reader under each.
  > the same check file read green under `-p blitz-tests` and red under the workspace leg, so a check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
  (.andromeda/test-plan-detail/ci-integration.md:31)
  > `writing-mode` selects which of two bodies of the engine's bounds reader `BaseDocument::physical_unrounded_geometry` is compiled — `visible_region` and every snapshot `bounds` read through it — so the same stand check runs over a different reader under each runner
  (.andromeda/test-plan.md:392)
  > - **Engine features by runner:** `cargo test --workspace --locked` and `cargo test -p blitz-tests --locked` build the same test file against different engine features — `tracing` and `writing-mode` on in the workspace build only — so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green, and each body of the engine's bounds reader holds a check
- (.andromeda/test-plan-detail/ci-integration.md:63) The `fast` leg's workspace test count the chunk re-runs must read this last recorded tally unmoved, since the chunk adds, removes and renames no test, and the `doc` leg it re-runs must exit 0 as recorded.
  > re-counted with the served Cells and load page: 208 result lines, 1117 passed · 0 failed · 12 ignored — +2 result lines (`stand_cells_served`, `stand_load_page`) and +12 passed (`stand_cells_served` 4, `stand_load_page` 2, seven_guis' three stand unit tests, `host_log` 1, `stand_keyboard` 1, `stand_snapshot` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md (a `ci-leg.sh fast` run's tally probe)
  (.andromeda/test-plan-detail/ci-integration.md:65)
  > `ci-leg.sh doc` — `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked` — exit 0, 0 errors, no output-filename collision, under the line-tables-only profile
  (.andromeda/test-plan.md:399)
  > - **Local baseline (the reference the fork's CI is compared against):** the dev host, the dev profile with `debug = "line-tables-only"`, workspace default features, every cargo command with `--locked`, no `opt-level` rewrite; cold is the first run after `cargo clean`, entries in sequence in one `target/`, warm an immediate re-run
- (.andromeda/test-plan-detail/ci-integration.md:71) The coverage leg's floor check over escher-driver must still pass with `lint.rs::findings` split into helpers, each new helper line reached by the lint's existing checks.
  > then, chained, `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor script's status
  (.andromeda/test-plan.md:401)
  > - **Coverage:** the `coverage` leg runs `cargo llvm-cov` over the workspace, `keyboard_compat.rs` excluded, into `target/coverage/lcov.info` and a per-file report, then `coverage_floor.py` over the per-crate floors (§10), the leg exiting with the floor script's status; the script prints no source path but the crate directory
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) escher-driver must stay at or above its 97 % line floor over the refactored `lint.rs`, its last reading 97.99 %, while the edit to the `#[path]` child `mcp_tests.rs` moves no record of the reading.
  > three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16)
  > A `*_tests.rs` `#[path]` test-child file is no record of the per-crate reading — the coverage job's per-file report lists none, `mcp_tests.rs` and `schema_tests.rs` included — so a test module moved out of line moves its crate's reading with no line of code changed
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16)
  > read again with the served Cells and load page, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %, `coverage floor: 3 of 3 crates at or above their floors`
  (.andromeda/test-plan.md:411)
  > - **Coverage floors:** the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %; both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it; a `#[path]` test-child file is no record of the per-crate reading
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22) The refactored `stand_requirement_sweep` test and the re-read `stand_lint` must stay green in every one of `stand-repeat`'s three rounds with no retry.
  > the stand checks hold a zero-failure budget, on the founder's «Повторное задание, ноль сбоев (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label): the `stand-repeat` leg and its job `Stand checks repeated` run the 50 `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry
  (.andromeda/test-plan.md:413)
  > - **Flakiness budget:** the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the ten named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)

## Bindings
- **ci-integration Legs ↔ quality-gates flakiness budget**: the `stand-repeat` leg's `stand_*.rs` glob (.andromeda/test-plan-detail/ci-integration.md:8) is the population the zero-failure budget runs three times (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22), so the edited `stand_requirement_sweep` must keep its file name to stay in both.
- **ci-integration Legs ↔ ci-integration engine features by runner**: `stand-repeat` runs `cargo test -p blitz-tests --locked` (.andromeda/test-plan-detail/ci-integration.md:8) while the `fast` leg runs the workspace build (.andromeda/test-plan.md:387), so the two legs are the two builds whose green readings are not interchangeable (.andromeda/test-plan-detail/ci-integration.md:29).
- **ci-integration coverage procedure ↔ quality-gates coverage floors**: the leg exits with `coverage_floor.py`'s status (.andromeda/test-plan-detail/ci-integration.md:71) over escher-driver's 97 % floor (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14), which `lint.rs`'s helpers move and `mcp_tests.rs` does not (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16).
- **ci-integration local baseline ↔ ci-integration Legs**: the recorded workspace tally and `doc` exit 0 (.andromeda/test-plan-detail/ci-integration.md:63; .andromeda/test-plan-detail/ci-integration.md:65) are what the re-run `fast` and `doc` legs (.andromeda/test-plan-detail/ci-integration.md:8) must read again unmoved.
