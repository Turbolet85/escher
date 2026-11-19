@history_item n=1 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows (137 rows, each verified against the code, guards re-pointed or left out), and so produced every picked row this chunk sits on: decision/test-engine-with-system-fonts, decision/tests-built-with-system-fonts, decision/mobile-targets-built-not-tested, decision/escher-session-no-colour and its guard, decision/default-ua-sheet-via-default-config. Why it matters: a mend that changes the blitz-tests crate's features, its per-file test binaries or the CI legs must keep each row true as the code shows it. The `system-fonts` feature on `blitz-dom` for blitz-tests, which is also on by default for a `cargo test --workspace`, is one of the feature-unification differences the measurement has to name. Merging test files must keep the named guards (`font_family_fallback`, the seven_guis checks' shared reader and the shared Speaker no-ESC assertions) present and passing.

@history_item n=2 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — added decision/unavailable-families-use-default-font, ratified as upstream's, with guard `font_family_fallback`. Why it matters: the guard is a test target. If the mend uses one test binary per leg in place of one per file, the guard's test must keep its name, its assertions and both layout modes, so the row's guard still resolves.

@history_item n=3 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the seven_guis headless stand registers the bundled DejaVu Sans for every generic, with system fonts off on native, through `build_single_font_ctx(DEJAVU_SANS)` and `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature. Why it matters: seven_guis builds with a font feature set different from blitz-tests' `system-fonts`. That is exactly the kind of per-package feature difference the scope suspects of building shared crates under a second hash (`cargo test -p seven_guis` against `--workspace`). The stand's font must not move to save build space.

@history_item n=4 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — moved the stand's font registration to architecture §Standard Contracts → Headless stand. design-system keeps only a pointer, and the WASM bundled-font item and the custom `FontContext` item stay. Why it matters: a mend that touches seven_guis' features or test layout (`examples/seven_guis/tests/`) is checked against architecture's contract, not a design-system value.

@history_item n=5 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted. This is the origin of decision/screenshot-never-skips-text and the two fonts-unavailable contract rows. Why it matters: these rows rely on test builds that resolve fonts (system fonts, or the stand's bundled face). A profile or feature change made to cut `target/` growth must not leave a test build without fonts, or font assertions would pass vacuously or shots would be refused.

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group in the open list, a header heavier than a row) — set embolden behaviour per build: `hint` stays `!FONT_EMBOLDEN_ENABLED`, and a synthesized bold header is heavier than a row on every build, including the emboldening one. This was a direct operator direction after a red on macOS alone. Why it matters: the `embolden` CI leg (`cargo test -p blitz-tests --features blitz-paint/font-embolden`) is a second feature set over the same test targets and one of the hash multipliers to measure. Any mend must keep that leg building and asserting separately.

@history_item n=7 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — re-pointed design-system citations into `ci.yml` and `publish-browser.yml` after the CI files moved, including the ios/android matrix entries behind decision/mobile-targets-built-not-tested. Why it matters: this is the precedent that a chunk moving `ci.yml` lines re-points the master's citations with no claim change. A mend that edits `.github/workflows/ci.yml` carries the same duty, and the mobile targets stay built and untested.

@history_item n=8 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — re-pointed the ios/android matrix citations (237-252 → 328-343) by a measured line map after the CI gate legs rewrote `ci.yml`. Why it matters: this is the precedent for a CI-leg change. This chunk may change a leg's cargo invocation (pinned by `test_ci_workflows.py`), and the master's `ci.yml` citations behind decision/mobile-targets-built-not-tested would need re-pointing by a measured map.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed by the chunk's line moves) — re-pointed citations, including the six lines of `ci.yml`'s free-disk step. Why it matters: CI already has a free-disk step, a prior answer to build-space pressure on the runner. The measurement and any sweep should account for it. Moving `ci.yml` lines again needs the same citation re-point.

@history_item n=10 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — re-pointed design-system citations into the root `Cargo.toml` after members and workspace-dependency entries were inserted. Why it matters: the chunk may edit `Cargo.toml`'s `[profile.*]` (for example, debuginfo for test binaries). This is the precedent that cited root-manifest lines are re-pointed by measured shifts with no claim change.

@history_item n=11 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once, through `DocumentConfig::default()` (`UserAgentStylesheet::default_sheet()`), after upstream #1189. This is the origin of decision/default-ua-sheet-via-default-config, pinned by `default_config_uses_the_default_sheet`. Why it matters: that pinning test lives in the blitz-tests targets. A merge into one binary per leg must keep it, with its assertions unchanged.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the `escher-session` output form: no colour, no colour dependency, no ESC byte, no terminal test, and output that is the same piped or not. This is the origin of decision/escher-session-no-colour. Why it matters: the row's guard is the seven_guis checks' shared reader asserting no ESC byte on every line. Moving or merging `examples/seven_guis/tests/` files must keep that shared reader and its assertion. The rule also bars a new colour dependency.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — extended the no-ESC, no-terminal-test rule to the `mcp` role, asserted on every reply by the shared `Speaker`. Why it matters: the shared `Speaker` is the other half of decision/escher-session-no-colour's guard. A test-layout mend must keep it shared across the moved tests.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — `escher-session` serves six tasks (counter, flight-booker, timer, crud, cells, load), listed in the usage line's `apps:` and the MCP `start` tool's enum, and every stand boot uses the bundled font. Why it matters: this is the origin of the escher-session-tasks rows. The stand tests that pin the served-task list are among the files a one-binary-per-leg mend would move, and their assertions must not change.

@history_item n=15 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native, 1 of 2 and 2 of 2) — moved contract/escher-session-tasks/{cells,counter,crud,flight-booker,load,timer} from six tasks to seven, and added contract/escher-session-tasks/order as the seventh. Why it matters: these seven contract rows bind this chunk as they stand at HEAD. The stand tests in `examples/seven_guis/tests/` and blitz-tests that check the seven served tasks keep their assertions through any merge or move. Duplicated stand-test setup belongs to «Stand-test setup shared», not here, unless the mend moves those files anyway.
