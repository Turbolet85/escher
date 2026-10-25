# tests · CI Integration — category read

## Carried
- (.andromeda/test-plan-detail/ci-integration.md:8) The flight booker's mode becoming a real `select` moves `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` and touches `stand_act_select`, each a member of the one-line a11y leg, which must stay green with its fourteen files.
  > a11y (`--test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids --test accessibility_states --test accessibility_label --test accessibility_description --test accessibility_merged_states --test accessibility_active_descendant_unread --test inert_subtree --test select_interaction --test stand_act_select`, still one line, fourteen files
- (.andromeda/test-plan-detail/ci-integration.md:8) A stand check the chunk moves or adds keeps every leg it already runs in; a re-pinned a11y-leg stand file runs in both the a11y and the stand-repeat legs.
  > each joining file keeping every leg it already ran in, so `stand_act_select` runs in both the a11y and stand-repeat legs as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` do
- (.andromeda/test-plan-detail/ci-integration.md:8) Every `stand_*.rs` file the chunk adds or changes (Cells served to the driver, the select, a load on the stand) joins the stand-repeat leg by its prefix and must pass three rounds; the file count moves from 48 if a file is added.
  > stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 48 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay, `stand_lint` since 2026-10-19-visual-lint `stand_load_wait` since 2026-10-20-load-wait, `stand_act_select` and `stand_snapshot_parts`
- (.andromeda/test-plan-detail/ci-integration.md:9) A new blitz-tests target outside the `stand_` prefix reaches stand-repeat only as a literal `--test` word in ci-leg.sh, pinned by `test_stand_repeat_leg_runs_three_rounds`; `select_interaction` stays among the ten.
  > ten named targets as literal `--test` words, `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction` and `text_extent`, each file keeping its name (.github/scripts/ci-leg.sh:54-58), `test_stand_repeat_leg_runs_three_rounds` naming the glob and the ten words (.github/scripts/test_ci_workflows.py:361-365)
- (.andromeda/test-plan-detail/ci-integration.md:10) The seven_guis process checks the chunk moves (`cli_flow` with `flows/flight.sh`, and `host_binary` · `host_log` · `mcp_server` · `mcp_log` · `cli_commands` if `start`/`serve`/`mcp` boot a new task) run three rounds under a zero-failure, no-retry budget.
  > then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and, since that chunk, `recording`, `test_stand_repeat_leg_runs_three_rounds` pinning both words beside the ten; the first red command ends the leg with its status — no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`
- (.andromeda/test-plan-detail/ci-integration.md:29) A moved or new check that reads a sink's capture (a host or `mcp` role booting a new task) counts by target, and the chunk must read green under the workspace leg, not only per package.
  > so a check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
- (.andromeda/test-plan-detail/ci-integration.md:31) Stand checks over the new screens (Cells' parts and lint, the select's options with its client rect as their `bounds`) must pass under both bodies of the bounds reader, the workspace and the per-package build.
  > `writing-mode` selects which of two bodies of the engine's bounds reader `BaseDocument::physical_unrounded_geometry` is compiled — `visible_region` and every snapshot `bounds` read through it — so the same stand check runs over a different reader under each runner
- (.andromeda/test-plan-detail/ci-integration.md:42) The chunk's CI read moves the per-OS tallies from this last recorded reading, every job attributed per target.
  > re-read with the `a11y` leg's fourteen files on CI#40134371519, the pre-CI commit `4cbf0a4c9` (green 17/17): linux 206 · 1105 · 0 · 12; macos and coverage 187 · 1102 · 0 · 12; windows 187 · 1057 · 0 · 9; the a11y job 14 · 84 · 0 · 0; the stand repeat 198 · 837 · 0 · 21
- (.andromeda/test-plan-detail/ci-integration.md:43) A per-job tally the chunk reads from a CI log must strip both spellings of the escape.
  > A per-job tally read from a CI job log through `gh run view --log` strips the ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`: the log spells the escape as the two literal characters `^[`, so a strip of `\x1b\[[0-9;]*m` alone leaves every cargo `Running tests/{name}.rs` line unattributed
- (.andromeda/test-plan-detail/ci-integration.md:63) The local `fast` leg's workspace count moves from this baseline and is re-counted per target.
  > re-counted with `stand-repeat`'s ten named targets: 206 result lines, 1105 passed · 0 failed · 12 ignored
- (.andromeda/test-plan-detail/ci-integration.md:65) The a11y leg's per-file counts the select change moves (stand_keyboard, stand_contrast, stand_accessibility_ids, stand_act_select) are re-read from `target/ci-logs/a11y.log`, exit 0 required.
  > `ci-leg.sh a11y` fourteen files, exit 0, 14 result lines · 84 passed · 0 failed · 0 ignored (accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 5 · stand_keyboard 14 · accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5, `target/ci-logs/a11y.log`
- (.andromeda/test-plan-detail/ci-integration.md:65) The doc leg is part of the chunk's done gate and must exit 0.
  > `ci-leg.sh doc` — `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked` — exit 0, 0 errors
- (.andromeda/test-plan-detail/ci-integration.md:71) The coverage leg's per-crate floors must still hold after the chunk (no driver change is intended, so escher-driver's 97 must not drop).
  > `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor script's status
- (.andromeda/test-plan.md:11) The cold-agent pipe's contract tests `test_cold_agent.py` move with the new goals and run in the fast `ci-scripts` leg.
  > unit-tested with `python3 -m unittest discover -s .github/scripts` as the fast `ci-scripts` leg (§4)
- (.andromeda/test-plan.md:187) The live cold-agent run is fired once on the operator host and never enters CI.
  > one live run on the operator host, never part of CI (no CI secret)
- (.andromeda/test-plan.md:372) CI runs on the build branch, so the chunk's push is read on `build/escher-0.2.0`.
  > **Platform:** GitHub Actions; tests run on PRs and pushes to main, v0.* and build/** — escher's build branch included
- (.andromeda/test-plan.md:386) The local pre-push gate must pass before the chunk is done.
  > **Local pre-push gate:** `bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts, stopping at the first red
- (.andromeda/test-plan.md:389) New stand screens (Cells, the select) are checked under the bundled font alone, so they measure the same on every runner.
  > except the headless-stand checks, which use only the bundled DejaVu Sans with system fonts off and so measure the same on any runner, in the `-p blitz-tests` package-alone build included
- (.andromeda/test-plan.md:393) Unix-gated process checks the chunk adds or moves widen the macOS–windows gap, attributed in the tallies.
  > linux runs more than macOS by the doc-test result lines, and macOS more than windows by the unix-gated binaries; ios and android build only
- (.andromeda/test-plan.md:412) Every stand check the chunk adds or moves holds the zero-failure budget.
  > the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the ten named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)

## Bindings
- **a11y leg ↔ a11y plan §Bootstrap and the stand's tables**: the select moves `common::tab_order`, `ENABLED_TAB_ORDER` and the contrast rows; the leg's fourteen files and per-file counts must move with them (.andromeda/test-plan-detail/ci-integration.md:8), (.andromeda/test-plan-detail/ci-integration.md:65)
- **stand-repeat leg ↔ §10 flakiness budget**: a new `stand_*.rs` or a new named target joins the three-round, no-retry leg, `test_ci_workflows.py` pinning the named words (.andromeda/test-plan-detail/ci-integration.md:9), (.andromeda/test-plan.md:412)
- **ci-scripts leg ↔ §3 cold-agent pipe**: `test_cold_agent.py` moves with the pipe's new goals and is gated by the fast leg (.andromeda/test-plan.md:11), (.andromeda/test-plan.md:386)
- **live run ↔ security plan §Surfaces**: the run stays off CI with no CI secret, on the operator's login, its second carriage of new screens a question for the founder's word (.andromeda/test-plan.md:187)
- **engine features by runner ↔ obs plan (host_log, mcp_log)**: a host or `mcp` role booting a new task is read by target under both builds (.andromeda/test-plan-detail/ci-integration.md:29)
- **coverage leg ↔ §10 coverage floors**: the three per-crate floors hold over the chunk's tree (.andromeda/test-plan-detail/ci-integration.md:71)
