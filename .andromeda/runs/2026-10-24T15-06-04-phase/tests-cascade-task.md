# Task — tests · 2026-10-24-stand-requirement-sweep

## The chunk
This chunk is a test sweep: it re-proves 0.2.0 requirements v020-01 to v020-09 headless, on the 7GUIs stand or on a minimal fixture beside it, and writes a stated record that names, for each requirement, the task or fixture and the check that proves it. Each named check is itself checked to exist.
Where the record finds a gap, it adds one new check per behaviour, run in both layout modes (incremental false and true). It also adds checks on fixtures for readings of the visual lint that nothing asserts yet: a clipped or off-screen node holding text also reads truncated, and the text extent reader reads RTL and vertical text as LTR lines and a transformed box at its client rect.
Every witness in the record must run inside the stand-repeat leg (three rounds, zero failures allowed). A fixture check whose file name does not start with stand_ either gets the leg's selection widened or has its proof carried by a stand_ check.
The coverage leg's per-crate floors must still pass at the chunk's end. A floor may be re-read, raised, or added for blitz-dom.
The sink-installing escher-session host and its mcp role are measured again: their stderr at trace, under both builds, must hold no id, no name and no typed text for the 0.2.0 additions (the lint verb, the snapshot part argument, --record recording).

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- .github/scripts/ci-leg.sh · conflict surface
- .github/scripts/test_ci_workflows.py · conflict surface
- .github/scripts/coverage_floor.py
- .github/scripts/test_coverage_floor.py
- packages/escher-driver/src/lint.rs · conflict surface
- packages/blitz-dom/src/text_extent.rs · conflict surface

## Needs
- The stand checks' layout and naming: one test target file per behaviour under tests/blitz-tests/tests, both layout modes in every pipeline check, and new stand_ files picked up with no script change.
- The blitz-tests target pin (BlitzTestsTargetsTest over tests/blitz-tests/Cargo.toml: autotests not off, one [[test]] table, all, not built). A new target file has to keep this pin true.
- The shared modules the sweep's checks read: common (the stated tables and helpers, never a second role or name mapping), session_common (hold, act, lint and linted, snapshot_part and part, refused, the screenshot builders, and the counts of who reads it) and fixtures (boot, hold, the shared pages and HeldNet; a page with one reader stays in its own file).
- The 0.1.0 sweep record's form, which this chunk extends or sits beside: a stated table of requirement to witness check, each named test checked to exist, read by run stand.
- The agent-run contract. run stand selects stand_*.rs files by prefix; a non-stand fixture check is run by name or by run all; the stated pass, fail and ignore counts and file counts get re-counted when stand checks are added.
- The flakiness budget, which binds the stand-repeat leg and the Stand checks repeated job: the stand_*.rs files plus the eight seven_guis process targets, three rounds, no retry, no --ignored, no env var. The job restores the test cache without saving.
- The coverage leg and floors. cargo llvm-cov runs over the workspace with keyboard_compat.rs excluded, then coverage_floor.py runs over per-crate floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90). A #[path] test-child file is no record of a crate's reading. Changing a floor changes a CI gate.
- The mutation tier, which is v020-02's witness: cargo-mutants with nextest under the mutants profile, --baseline=skip, each unit's test command read green first on the unmutated tree, run on the dev host only and never in CI.
- The CI leg invariants that test_ci_workflows.py pins on ci-leg.sh and ci.yml: each leg runs through ci-leg.sh, --locked, failure-log upload, and ci-leg.sh's legs under a cargo shim. These bind any change to stand-repeat's selection or to the floors.
- The stand log format and stderr measurement. Two record classes (event lines and closed-span lines). The command and tool_call spans carry fixed words and counts only; lint and a part read record verb and cause alone. Needles must be 0 for ids, names, a typed sentinel and a shot's first 64 base64 characters, at trace, for the host built alone and with the workspace.
- Engine features differ by runner: tracing and writing-mode are on only in the workspace build. A check reading a sink capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green. This binds host_log and mcp_log through both builds.
- Process-lifecycle and command-line checks: unix-gated targets, every wait bounded, a guard that kills and reaps a held host, child streams sent to null unless read, a pipe drained while a sink-installing host runs, and state directories under target/tmp.
- Session recording and replay as 0.2.0 witnesses: --record on start, the host role and mcp; the two committed Counter checks under tests/blitz-tests/checks, replayed by stand_replay and kept fresh byte for byte by seven_guis recording, holding no screenshot step.
- The visual lint as a witness, through Session::lint and the lint verb: six defects in Defect::ALL order; findings hold ids, closed words and numbers only; nothing changes. The scope summary states that the text extent reader's and the select's limits are asserted by no check, and the truncated co-reading is unasserted too.
- The text extent reader's stated limits (RTL and vertical text read as LTR horizontal lines, a transformed box at its client rect, None for a node that is not an inline root). A change here is an engine change for every document; a change to lint findings moves a driver answer on every carrier, so either is a founder fork.
- The non-stand fixture witnesses outside stand-repeat today: inert_subtree, text_extent, password_caret_click, select_interaction, virtual_clock_engine, transformed_client_rect, scrolled_box_absolute_position, hit_clipped_at_scrolling_box, accessibility_states, accessibility_active_descendant_unread.
- The headless harness the witnesses run on: from_vdom through seven_guis::stand::options (800x600, scale 1.0, bundled font, no net), settle, screenshot and screenshot_rect, text_contrast, and the VirtualClock that tick and advance_time move and settle never moves.
- Fonts: stand checks use the bundled DejaVu Sans with system fonts off and take no skip path; a shot or lint with no sans-serif family is refused fonts-unavailable. This binds any new fixture check that reads text.
- Wall time in session checks: no sleep and no wall-clock read, except stand_virtual_clock and stand_load_wait. Time moves by the driver's advance and the stand's timer_step only.
- Test data: nothing is seeded. Fixtures are rsx! components and inline literals in code, booted through fixtures::boot, and compiled into each target by agent-run boot.
- Stand-ins: HeldNet is the one held network stand-in (the harness counts its loads); TimerTicks replaces the Timer's delay. A new fixture check uses these rather than a new mock.
- Pixel tests: a shot's PNG matches the harness raster and is byte-identical across runs on one host but not across OSes, so a check compared byte for byte across OSes holds no screenshot step. Per-OS tallies differ because the unix-gated binaries run only on unix.
- The escher-driver and dioxus-native-dom unit tests count toward their crate floors; escher-driver's verbs, refusals and lint are read by integration checks, not unit tests.

## Not needed
- The WPT runner, its conformance scoring, reftests, quarantine and the wpt.yml and wpt-post-results workflows. They run on upstream only and grade no requirement here.
- The apps/browser tests (about pages, history, favicon, URL suggestions, persistence) and their tokio worker helpers.
- blitz-vibey-script's tests (DOM, Selection, Preact TodoMVC) and its virtual timer clock.
- The cold-agent run pipe, its verdict and test_cold_agent.py. They belong to v020-11 and the later Cold-agent bar entry.
- The stand's Tab-order, Enter and Space activation, and contrast tables (tab_order, ENABLED_TAB_ORDER, TEXT_ROWS, STATE_ROWS) being re-asserted after 0.2.0's engine changes, and the a11y leg. These are owned by the later A11y re-read entry.
- The blitz-dom style-bridge, list-marker, text-transform and SVG parsing unit tests, and the commented-out stylo tests.
- The incremental_oracle differential oracle, the randomized stale_node_mapping stress and the pure-restyle hover technique, beyond the general rule that both layout modes run.
- The box-shadow and gradient pixel expectations against Chromium renders, and the paint_tree_bench benchmark with PAINT_TREE_BENCH_HTML.
- The CI install script apt-install.sh, its bounds, and the WPT diff and area-changes script tests.
- The windows, macos, ios and android matrix, the MSRV build and the wasm example build, beyond the fact that the unix-gated targets do not run on windows.
- The human-read example fixtures under examples/assets and examples/preact, and the manual screenshot example.
- The dioxus-native-dom bridge_tests, touch-data and keyed-nodes unit tests, and the element_id re-render unit tests.
- Third-party test frameworks and test-that's verify_that! for upstream's accessibility_label and accessibility_description checks.
