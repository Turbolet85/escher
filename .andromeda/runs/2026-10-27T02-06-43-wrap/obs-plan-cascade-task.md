# Task — obs-plan · 2026-10-26-codebase-health

## The session
Refactored for complexity only, with no behaviour change (v030-01): three over-ceiling functions were split into private helpers so that each now reads under the cognitive-complexity ceiling of 15. The audit's population over 15 went from 88 to 85.
escher-driver `lint.rs`: `findings` (signature unchanged, its one non-test caller `Session::lint` untouched) now walks a new private `readings` per entry, with a new private `placement` for the off-screen and clipped match; `ordered`, `overlaps` and the inline tests moved down 16 lines with their bodies unchanged.
escher-driver `mcp_tests.rs`: the listing test `the_listing_is_the_twelve_verbs_of_the_table` now calls three new private `#[track_caller]` helpers (`assert_described`, `assert_schema`, `assert_property`); it still holds 16 `#[test]`, 15 on unix.
blitz-tests `stand_requirement_sweep.rs`: the 0.2.0 sweep test now calls new private helpers `assert_0_2_0_row`, `assert_0_2_0_witness` and `inside_stand_repeat` (which now holds the `BLITZ_TESTS` and `SEVEN_GUIS` prefix constants); every message is unchanged.
Measured: a silence census reading 0 over the three files (the first census `mcp_tests.rs` appears in); escher-driver still has 2 `info_span!` sites, 0 events and 0 `#[instrument]`; escher-driver's coverage moved 4918/5019 to 4930/5031 = 97.99 %; CI#40434241601 green 17/17 with per-job tallies equal to the base run.
Nothing added or retired: no public symbol, verb, field, carrier, span, log site, env var, crate, module, dependency, CI leg or script changed.

## Reached files
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/mcp_tests.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- escher-0.3.0/chunks/2026-10-26-codebase-health/scope.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/research.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/inputs/
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/complexity.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/counts.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/row-controls.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/operator-pass.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson

## Touches
- The silence line for the visual lint (escher-driver module `lint`, `Session::lint`, `Finding` · `Defect` · `Reason` · `Bounds`): still silent after the split, with the census reading 0 over `lint.rs`. Its citation `packages/escher-driver/src/lint.rs:176-230` sits above the first hunk at `:273` and has not moved.
- Line citations into `packages/escher-driver/src/lint.rs` at or after base line 273 have moved: `ordered` `:299` → `:315`, `overlaps` `:334` → `:350`, inline `mod tests` `:527` → `:543` (and the span `:526-604` → `:542-620`). The file went from 604 to 620 lines. Citations above `:273` have not moved.
- Line citations into `packages/escher-driver/src/mcp_tests.rs` after base line 480 have moved +17: `env!("CARGO_MANIFEST_DIR")` `:633` → `:650`, `:754-785` → `:771-802`, `:615-717` → `:632-734`, whole file `:1-786` → `:1-803`. `env!("CARGO_PKG_VERSION")` `:247`, `:167-196` and the listing test head `:399` have not moved.
- Line citations into `tests/blitz-tests/tests/stand_requirement_sweep.rs` after base line 1017 have moved: the span from the test to the file's end `:1015-1090` → `:1015-1109`, and the file went from 1090 to 1109 lines. Spans ending at or before `:1013` and the test head `:1016` have not moved.
- The silence group for escher-driver's out-of-line test files (`cli_tests.rs`, `recording_tests.rs`, `recording_off_unix_tests.rs` and the seven_guis shared module) does not name `mcp_tests.rs`, which now has a census reading of 0.
- The silence claim for `stand_requirement_sweep` (installs no subscriber, reads no env var, prints nothing, its readings going into assertions only) still holds with the new private helpers. Its `WITNESSES` / `WITNESSES_020` tables and leg check are unchanged.
- escher-driver's tracing inventory was re-read: 2 `info_span!` sites (`command`, `tool_call`), 0 events, 0 `#[instrument]`, no subscriber, no env read. The span fields are unchanged.
- The obs checks are green after the change: `stand_act_spans`, `stand_mcp_tools`, `host_log` and `mcp_log` under both builds, and `stand_lint` 7 in both layout modes. The sink's needle readings are unchanged.
- escher-driver's env-read set is unchanged. `mcp_tests.rs` still holds 2 `env!` compile-time reads (`CARGO_MANIFEST_DIR`, `CARGO_PKG_VERSION`), and the `recording` unit tests' `CARGO_MANIFEST_DIR` read is untouched.
- Module and target counts the doc's ordinals rest on are unchanged: escher-driver has seventeen private modules, blitz-tests 147 `.rs` files and 146 targets, and the `session_common` and `fixtures` readers are unchanged since no file was added.
- The `Finding` / `Reason` `Debug` claims and the rule that a lint finding is written only by `to_json` are unchanged, because `Session::lint` and every reader in `lint.rs` were left untouched.
- Test tallies were re-read unchanged: escher-driver 100 unit tests, stand checks 205 · 0 · 7, stand-repeat 204 · 864 · 0 · 21, a11y 14 · 85 · 0 · 0, workspace 208 · 1117 · 0 · 12, CI scripts 108.
- The coverage report artifact uploaded by CI now reads escher-driver at 4930/5031 = 97.99 % (was 4918/5019) against the 97 % floor; dioxus-native-dom and blitz-test-harness are unchanged. Only a site that quotes the reading moves.
- The stand-repeat leg's membership: a sentence crediting `stand_requirement_sweep`'s leg check alone with holding which targets the leg runs is disproved. The leg check reads `ci-leg.sh` whole, so `select_interaction` dropped from stand-repeat alone stays green, and membership is held jointly with `.github/scripts/test_ci_workflows.py` `test_stand_repeat_leg_runs_three_rounds`. This gap predates the chunk.
- The complexity reading (`rust-code-analysis-cli` 0.0.25) is a dev-host reading with no CI leg and no log or artifact. Its 88 → 85 count is held by no master.

## New facts
- F1 · log-coverage · escher-driver's out-of-line test file `mcp_tests.rs` is silent: 0 print, tracing and subscriber sites. Its only env reads are the 2 compile-time `env!` (`CARGO_MANIFEST_DIR`, `CARGO_PKG_VERSION`). This chunk's census is the first one it appears in.
- F2 · log-coverage · `lint.rs`'s new private helpers `readings` (an entry's defect and reason pairs in push order) and `placement` (the off-screen and clipped match) hold no print, log site, env read or span, and the census reads 0 over the file.
- F3 · log-coverage · the new private test helpers `assert_described`, `assert_schema` and `assert_property` (`mcp_tests.rs`) and `assert_0_2_0_row`, `assert_0_2_0_witness` and `inside_stand_repeat` (`stand_requirement_sweep.rs`) print and log nothing. Their assertion messages name a row and an index only.

## Untouched
- escher-telemetry was not edited: `init`, the allowlist formatter, the scrub sets, the closed-span line and the chaining panic hook.
- The subscriber install sites in `examples/seven_guis/src/main.rs`, `examples/seven_guis/src/session_host.rs` and `packages/escher-driver/src/cli.rs`, and the MCP server's protocol output, have no edit.
- The `command` and `tool_call` span definitions in `packages/escher-driver/src/execute.rs` and `mcp.rs` and their eight and two fields are unchanged; no span or field was added.
- The agent-run harness log (`scripts/agent-run.sh`) and the cold-agent pipe log, verdict fields and `server.log` counting (`scripts/cold-agent.sh`) are unchanged.
- The engine and upstream log sites, the `tracing` feature wiring and the values logged as-is in blitz-dom, blitz-net, blitz-shell, dioxus-native, the upstream apps and the WPT runner are unchanged: nothing under `packages/blitz-dom/` was edited and no `Cargo.toml` changed.
- Metric coverage (in-process timings, counters, the FPS overlay, WPT scores) is unchanged; no metric was added.
- Error capture and panic hooks, including blitz-vibey-script's JS error surface and the WPT runner's hook, are unchanged.
- CI artifact handling (the leg log on failure, the coverage report on success, 7-day retention), `apt-install.sh` and the publish builds are unchanged: no CI workflow, leg or script was edited.
- The served stand (`seven_guis::stand`, `boot_cells`, `boot_load`, `LoadNet`), the load wait, the select checks, the parts check and the session-recording silence groups are unchanged; this session reached none of their files.
