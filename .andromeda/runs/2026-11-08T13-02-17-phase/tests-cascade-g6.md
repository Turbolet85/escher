@carried n=1 label=test-plan/test-data-fixtures fact=fct-6hjdlvz3
why: the chunk edits the recording and replay of a kept check, so the committed checks must still replay green and read byte-equal to what the binary records, or be re-recorded through record_checks.sh in the same chunk.
quote: the two committed checks — `counter-cli.json` (8 lines) and `counter-mcp.json` (11 lines), the stand Counter's scripted command-line and MCP sessions as `escher-session` records them through `record_checks.sh` (ids, names, values, snapshot text and diffs; no typed text, host path, name, secret or `screenshot` step), embedded as bytes by `stand_replay` and seven_guis `recording`
@carried n=1 run=2 label=test-plan fact=fct-wzfum7l4
quote: the two committed checks, the stand Counter's scripted command-line and MCP sessions as `escher-session` records them through `record_checks.sh`, holding no typed text, host path, secret or `screenshot` step, embedded as bytes by `stand_replay` and seven_guis `recording`
@carried n=2 label=test-plan/test-data-fixtures fact=fct-y2fxawue
why: the read's minimal fixture for a covered or off-screen target must reuse the shared pages of tests/fixtures/mod.rs where they serve, and a new reader of a shared page is recorded in its reader list.
quote: `many_items`: `ITEMS` (400) buttons under `#items`, ids `item-NNN`, a screen over the snapshot text budget, read by `stand_snapshot_parts` and `stand_act_spans`
@carried n=2 run=2 label=test-plan fact=fct-adt6dvk7
quote: `covered` (`fx-under` under the `z-index: 5` overlay `fx-cover`; tests/blitz-tests/tests/fixtures/mod.rs:46-62) and `many_items` (`ITEMS`, 400, buttons under `#items` with the ids `item-NNN`, a screen whose text passes `SNAPSHOT_TEXT_BUDGET`; tests/blitz-tests/tests/fixtures/mod.rs:64-86); a page with one reader stays in its check's file
@carried n=3 label=test-plan/test-data-fixtures fact=fct-zkjxcvz6
why: the read's owner-served case (a select serving its options and groups) must be proven on a select page, and a select page the read's check also reads moves from its check's own file into the shared fixtures.
quote: Select list pages (`rsx!`, each in its check's own file) | `select_popup`'s `page`, `stated_page` (an author's `aria-expanded` and `open`), `clipped_page`, `long_page`, `sized_page` (`size` on a `multiple` select), `focus_moved_page` (a `:focus` rule moving the select's box) and `chosen_disabled_page` (a disabled chosen option); `select_popup_painted`'s `page`, `clipped_page`, `grouped_page` and `long_page`
@carried n=4 label=test-plan/test-data-fixtures fact=fct-3qvwds5o
why: the read draws on the unpaged UI graph and answers the whole screen, so its answer on a fresh Cells screen must be held within the answer bound in both layout modes, the unpaged limit stated beside it.
quote: `graph` on a fresh `start cells`: 293,869 bytes, the whole screen in one answer, `graph` taking no argument — the stated limit «An unpaged graph», owned by residual `res-7cc5bi4k`, target next; held within the answer bound in both layout modes by `stand_graph`'s `the_whole_cells_answer_is_within_the_answer_bound`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
@carried n=5 label=test-plan/test-data-fixtures fact=fct-zbl26ely
why: the chunk edits docs/capabilities.md and the schema's verb table, so a new verb or cause must be named in docs/capabilities.md and its counts in words must match the tables the pin reads by path, the ci-scripts leg green.
quote: no fixture file: `test_capability_statement.py` reads the files it pins by path from the repository root — `docs/capabilities.md`, `README.md` and escher-driver's `schema.rs`, `refusal.rs` and `error.rs` — beside an inline `NUMBER_WORDS` table (one … twenty), writing nothing
@carried n=6 label=test-plan/ci-integration fact=fct-mrjxzany
why: a new stand check for the read joins the stand-repeat leg by its prefix, moving its 52 files, and a new seven_guis process check or named target must be added to the leg and its pin test_stand_repeat_leg_runs_three_rounds, every round green with no retry.
quote: stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 52 files
@carried n=6 run=2 label=test-plan fact=fct-nymiowfp
quote: the `stand-repeat` leg runs the stand checks, seventeen named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no `--ignored`, no env var
@carried n=7 label=test-plan/ci-integration fact=fct-prdavblg
why: the read's command line is read through host_log and mcp_log under both the per-package and the workspace build, so its check must count lines by target and be green under both builds.
quote: so a check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green
@carried n=7 run=2 label=test-plan fact=fct-he2hzpm5
quote: so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green
@carried n=8 label=test-plan/ci-integration fact=fct-qjdo4c7k
why: the chunk's new unit, stand and process checks move the per-OS tallies, which must be re-read at its CI run from this base, each move attributed by name and the unix-gated ones absent from windows.
quote: Re-read on CI#42371051720, the pre-CI commit `e26f1a207` (green 17/17): linux 220 · 1310 · 0 · 12; macos and coverage 201 · 1307 · 0 · 12; windows 201 · 1259 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved; ios and android build only
@carried n=8 run=2 label=test-plan/ci-integration fact=fct-3kqdhqos
quote: a tally move is attributed by grepping the test's own name, beside the ANSI strip of the per-OS tallies above
@carried n=8 run=3 label=test-plan/ci-integration fact=fct-kjlei6et
quote: Re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored
@carried n=8 run=4 label=test-plan fact=fct-eu6eymmp
quote: linux runs more than macOS by the doc-test result lines, and macOS more than windows by the unix-gated binaries; ios and android build only
@carried n=9 label=test-plan/ci-integration fact=fct-na5rpohe
why: the read's new code in escher-driver, and any reader P3 adds in dioxus-native-dom, must keep the coverage leg's per-crate floors green.
quote: `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor script's status
@carried n=9 run=2 label=test-plan fact=fct-k6zjxg3m
quote: then `coverage_floor.py` over the per-crate floors (§10), the leg exiting with the floor script's status
@binding n=1
**test-data-fixtures ↔ ci-integration**: a covered, off-screen or owner-served fixture the read's stand check reads joins tests/fixtures/mod.rs when a second check reads it, and that stand check joins the stand-repeat leg by its prefix, so the fixture's reader list and the leg's file count move together in one record (test-plan/test-data-fixtures fct-y2fxawue) (test-plan/test-data-fixtures fct-zkjxcvz6) (test-plan/ci-integration fct-mrjxzany)
@binding n=2
**test-data-fixtures ↔ the chunk's recording and replay edits**: recording the read as a step of a kept check must leave the two committed checks replaying green and byte-equal to what record_checks.sh records, re-recorded in the same chunk if the format moves (test-plan/test-data-fixtures fct-6hjdlvz3)
@binding n=3
**test-data-fixtures ↔ the capability statement**: a verb or cause the verb table gains is named as a code span in docs/capabilities.md and counted in words against the schema.rs tables the pin reads by path (test-plan/test-data-fixtures fct-zbl26ely)
@binding n=4
**ci-integration ↔ ci-integration**: every test the chunk adds moves the fast-leg count and the per-OS tallies from their last readings, attributed by test name, with the unix-gated session and command-line checks absent from windows (test-plan/ci-integration fct-qjdo4c7k) (test-plan/ci-integration fct-kjlei6et) (test-plan/ci-integration fct-3kqdhqos)
@binding n=5
**ci-integration ↔ the read's answer bound**: the read's whole-screen answer on Cells is held within the answer bound in both layout modes, as the graph's is, and its log check counts lines by target under both builds (test-plan/test-data-fixtures fct-3qvwds5o) (test-plan/ci-integration fct-prdavblg)
