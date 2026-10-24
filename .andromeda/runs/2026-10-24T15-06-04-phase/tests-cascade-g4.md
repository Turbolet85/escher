# tests · cascade read · group 4 of 4

## Carried
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:14) A new fixture check that needs a network in flight uses `HeldNet` from the reached `fixtures/mod.rs`, never a new network mock, and `HeldNet` keeps leaving `is_noop` unoverridden so the harness counts its loads.
  > `HeldNet`, escher's one held network stand-in since 2026-10-19-shared-fixtures, in place of `stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider` and `stand_boot`'s `RecordingNetProvider`: `fetch` keeps each request's url and handler, `held()` and `urls()` (request order) read them, and `answer(body)` answers the last held request through `NetHandler::bytes` and returns its url; it does not override `is_noop`, so the harness counts its loads
- (.andromeda/test-plan.md:350) The body's rule for the network stand-in binds every fixture check the sweep adds.
  > `HeldNet` is escher's one held network stand-in; it does not override `is_noop`, so the harness counts its loads
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20) Every fixture check the sweep adds boots offline on the engine's no-op net, navigation and shell providers, adding no stand-in of its own for them.
  > | Net, navigation, shell | no-op provider implementations |
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) A sweep witness that reads the Timer or moves time does so only through `TimerTicks`, the driver's `advance` and `timer_step`, so it stays deterministic across stand-repeat's three rounds.
  > settle neither advances the timer nor waits on a tick not yet delivered; an undelivered source holds elapsed at 0.0 s; the driver moves that time by `advance`, which moves the session's clock — the harness's `VirtualClock`, the clock every harness-built document reads — by exactly `ms` inside one settled step, then hands `ms` to the step
- (.andromeda/test-plan.md:352) The body's time rule binds the same witnesses.
  > `TimerTicks`, provided by every headless-stand boot, replaces the timer's `futures_timer::Delay`; settle neither advances the timer nor waits on a tick not yet delivered
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:43) The CI scripts the chunk reaches (`test_ci_workflows.py`, `coverage_floor.py`, `test_coverage_floor.py`) and the build configuration take no mocking library.
  > - Mocking libraries in CI and build configuration · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
- (.andromeda/test-plan.md:362) The body's observed absence holds on the changed CI scripts.
  > Observed absent, each by its search: a mocking library in CI, the build configuration and the apps
- (.andromeda/test-plan.md:358) Every fixture check that reads text (the `truncated` co-reading, the RTL and vertical `text_extent` limits) uses the pinned bundled font with no skip path.
  > - The headless-stand checks keep a real but pinned font — the bundled DejaVu Sans woff2, decoded through seven_guis' native `dioxus-native` `woff` feature, with system fonts off and no skip
- (.andromeda/test-plan.md:387) Witnesses moved into stand-repeat measure the same under its `-p blitz-tests` build only because they hold the bundled font with system fonts off.
  > except the headless-stand checks, which use only the bundled DejaVu Sans with system fonts off and so measure the same on any runner, in the `-p blitz-tests` package-alone build included
- (.andromeda/test-plan-detail/ci-integration.md:8) Every witness the record names runs inside the stand-repeat leg, whose selection by `stand_*.rs` prefix in the reached `ci-leg.sh` is pinned by `test_stand_repeat_leg_runs_three_rounds`, so a widened selection or a new `stand_*` carrier moves that pin with it.
  > stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 48 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay, `stand_lint` since 2026-10-19-visual-lint `stand_load_wait` since 2026-10-20-load-wait, `stand_act_select` and `stand_snapshot_parts`, `test_stand_repeat_leg_runs_three_rounds` naming it (.github/scripts/test_ci_workflows.py:359)
- (.andromeda/test-plan-detail/ci-integration.md:9) The leg's seven_guis half (`host_log`, `mcp_log`, `recording` among the eight) and its no-retry ending stay as they are whatever the selection becomes.
  > — then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and, since that chunk, `recording`, `test_stand_repeat_leg_runs_three_rounds` pinning both words; the first red command ends the leg with its status — no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`;
- (.andromeda/test-plan.md:374) The body's leg rule binds any change to stand-repeat's selection.
  > the `stand-repeat` leg runs the stand checks and seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no `--ignored`, no env var
- (.andromeda/test-plan-detail/ci-integration.md:15) A widened stand-repeat selection keeps the `Stand checks repeated` job restoring the test job's cache without saving.
  > the test job saves under `shared-key: workspace-test`, which the `a11y` job and, since 2026-10-14-quality-gates, the `stand-repeat` job restore with `save-if: false`
- (.andromeda/test-plan.md:378) The body's cache rule binds the same job.
  > the test job saves under `shared-key: workspace-test`, which the `a11y` and `stand-repeat` jobs restore with `save-if: false`
- (.andromeda/test-plan-detail/ci-integration.md:28) The stderr re-measurement through the reached `host_log` and `mcp_log` counts sink lines by target, and is read green under both the per-package and the workspace build.
  > so a check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
- (.andromeda/test-plan-detail/ci-integration.md:29) The 0.2.0 additions (`lint`, `snapshot`'s `part`, `--record`) are measured the way 0.1.0's sweep measured the host and its `mcp` role, under both builds.
  > re-read at 2026-10-14-stand-requirement-sweep, the workspace-built host prints that `focussed node` line twice (6 lines / 978 B, the `mcp` role 11 lines) and the host built alone none beyond its own (4 / 742 B, the `mcp` role 9), `host_log` and `mcp_log` green under both builds
- (.andromeda/test-plan-detail/ci-integration.md:30) A bounds-reading fixture witness moved into stand-repeat (`transformed_client_rect`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, the `text_extent` transformed-box reading) runs over the per-package body of the bounds reader there and must pass under both builds.
  > so the same stand check runs over a different reader under each runner;
- (.andromeda/test-plan.md:389) The body's runner rule binds the same witnesses and the stderr checks.
  > so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green, and each body of the engine's bounds reader holds a check
- (.andromeda/test-plan-detail/ci-integration.md:31) A sweep witness that compares a shot byte for byte holds it on one host only, never across OSes.
  > `stand_screenshot_repeat` reads byte-identical shots under the per-package and the workspace build on one host — the shot's bytes differ across OSes (CI#39225375577, macOS aarch64 against linux x86_64) —
- (.andromeda/test-plan-detail/ci-integration.md:69) The coverage leg in the reached `ci-leg.sh` ends with the reached `coverage_floor.py` and exits with its status; a floor re-read, raised or added for blitz-dom goes through that one chained command and keeps the script's exits and its crate-directory-only output.
  > then, chained, `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor script's status — since 2026-10-14-quality-gates, with no threshold until then: the script (stdlib) sums `LF`/`LH` over the `SF` records under each crate directory, prints one line per crate and the verdict `coverage floor: N of N crates at or above their floors` (exit 0) or `coverage floor: K of N crates under their floors` (exit 1), exits 2 on a crate with no record or a malformed argument, and prints no source path but the crate directory (§10)
- (.andromeda/test-plan.md:398) The body's coverage rule binds the same leg.
  > then `coverage_floor.py` over the per-crate floors (§10), the leg exiting with the floor script's status; the script prints no source path but the crate directory
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) The three floors read at or over their values at the chunk's end; they are the founder's chosen gate, so moving one is a gate change the chunk raises rather than decides.
  > three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 % — on the founder's «3 крейта, на 1 пункт ниже (Рекомендую)» (2026-10-14, relayed verbatim by the overseer as his chosen option label)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) Any edit to the coverage leg keeps the `keyboard_compat.rs` exclusion that `test_ci_workflows.py` pins.
  > the exclusion is pinned by `test_coverage_leg_reaches_the_floor_on_each_crate` asserting `--ignore-filename-regex` among the leg's words
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) Both exclusion lines stand through the chunk, since escher has not taken upstream's retirement of the file.
  > so the file stays in the tree and both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand, the three floors and `coverage_floor.py` unchanged
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) A test the sweep adds as a `#[path]` child moves no crate's reading, and escher-driver holds under one point over its floor, so the chunk's end reads it again.
  > A `*_tests.rs` `#[path]` test-child file is no record of the per-crate reading
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) escher-driver's last measured margin over its floor is the reading the chunk's end compares against.
  > since 2026-10-20-load-wait it reads 4695/4794 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors
- (.andromeda/test-plan.md:408) The body's floors bind the chunk's end.
  > the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22) "Inside stand-repeat" means the zero-failure budget: every witness the record names runs three rounds with no retry.
  > the `stand-repeat` leg and its job `Stand checks repeated` run the 48 `stand_*.rs` files and seven_guis' eight process checks three times with no retry
- (.andromeda/test-plan.md:410) The body's flakiness budget binds every witness the record names.
  > the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files and seven_guis' process checks three times with no retry (§9 Legs)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:28) v020-02's witness is the mutation tier as defined here; any mutation run the chunk makes reads each unit's test command green first and skips cargo-mutants' baseline.
  > with `--baseline=skip`, each unit's mutant test command first read green on the unmutated tree
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:28) The mutation tier never becomes a CI leg or a stand-repeat witness; the record names it as a dev-host witness.
  > Run on the dev host alone, never in CI
- (.andromeda/test-plan.md:412) The body's mutation rule binds v020-02's row of the record.
  > it runs on the dev host alone, never in CI

## Bindings
- **Mocking & stubbing ↔ Quality gates (flakiness)**: the fixture checks the sweep adds hold the network with `HeldNet` and time with `TimerTicks`, `advance` and `timer_step`, which is what lets them run three rounds with no retry, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:14), (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) ↔ (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22)
- **CI integration (legs) ↔ Quality gates (flakiness)**: stand-repeat's selection in `ci-leg.sh` and its pin in `test_ci_workflows.py` are the budget's scope, so moving a fixture witness inside it moves both together, (.andromeda/test-plan-detail/ci-integration.md:8) ↔ (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22)
- **CI integration (engine features by runner) ↔ CI integration (legs)**: a fixture witness moved from the `test` leg (workspace build) into stand-repeat (`-p blitz-tests`) then runs under both feature sets, and the stderr checks `host_log` and `mcp_log` count by target under both, (.andromeda/test-plan-detail/ci-integration.md:30) ↔ (.andromeda/test-plan-detail/ci-integration.md:9)
- **CI integration (coverage) ↔ Quality gates (coverage floors)**: the leg's chained `coverage_floor.py` command is where the floors live, so a floor re-read, raised or added for blitz-dom changes the leg, its `keyboard_compat` exclusion pin and the gate together, (.andromeda/test-plan-detail/ci-integration.md:69) ↔ (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14)
- **Quality gates (mutation tier) ↔ CI integration (legs)**: v020-02's witness stays a dev-host run outside every leg, so the record names it apart from the stand-repeat witnesses, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:28) ↔ (.andromeda/test-plan.md:374)
- **Mocking (observed absence) ↔ CI integration**: the CI scripts the chunk edits stay free of a mocking library, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:43) ↔ (.andromeda/test-plan-detail/ci-integration.md:69)
- **Mocking (real font kept) ↔ CI integration (fonts)**: fixture checks that read text hold the bundled font, which makes them read the same under stand-repeat's package-alone build, (.andromeda/test-plan.md:358) ↔ (.andromeda/test-plan.md:387)
