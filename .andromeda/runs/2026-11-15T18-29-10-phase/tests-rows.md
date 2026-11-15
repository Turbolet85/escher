# test-plan · the rows that bind 2026-11-15-taffy-from-upstream-git — picked 97 of 605 (57 by every draw) · rule any · pin 1162

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.
decision/wpt-conformance-runner · WPT conformance runner — Web Platform Tests conformance is run by the project's own WPT runner, covering reftests, checkLayout attr tests, crashtests, testharness.js tests and .any.js and .window.js tests.
decision/wpt-run-on-host-on-fork · WPT run on the host on the fork — The CI wpt job runs css and svg only on the upstream repository; on the fork, Web Platform Tests for css and svg are run on the host.
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/key-after-dropped-anonymous-block · Key press after a dropped anonymous block — A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts. · why: key_press_after_container_lost_its_box pins no panic, focus staying and the key still moving a select's choice, in both layout modes
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/box-shadow-chromium-expectations · Box-shadow expectations from Chromium — Box-shadow pixel expectations come from Chromium renders, with a 1px antialiasing tolerance.
decision/font-tests-skip-without-font · Pre-existing font tests skip without a font — The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/fork-wpt-reading-report-only · Fork WPT reading is report-only — A conformance reading on the fork's dev host is report-only and grades no criterion.
decision/incremental-differential-oracle · Incremental layout differential oracle — Incremental layout is checked by running one fixture and mutation sequence in an incremental and a non-incremental document, comparing every node's final layout, layout children and paint tree after each step and asserting no damage remains. · guard: incremental_oracle
decision/msrv-verified-by-build · MSRV verified by build only — The minimum supported Rust version is verified by build only, not by test.
decision/pixel-tests-on-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels.
decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.
decision/shot-png-deterministic-per-host · Shot PNG deterministic on one host — A shot's PNG equals the harness's raster and is byte-identical across runs on one host.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
decision/blitz-dom-fixtures-built-manually · blitz-dom fixtures built without the HTML parser — blitz-dom tests build their DOM fixtures manually through the document mutator rather than by parsing HTML. · why: the HTML parser lives in blitz-html, which would be a circular dev-dependency
decision/randomized-template-swap-stress · Randomized template-swap stress — Deterministic pseudo-random template swaps driven by an LCG run 1000 steps. · why: to shake out slab and ElementId reuse bugs · guard: stale_node_mapping

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/frames-compared-whole · Frames compared whole — A shot's PNG is compared whole, equal to the harness's raster and byte-identical across runs on one host. · guard: stand_act_screenshot, stand_screenshot_repeat
decision/headless-screenshot · Headless screenshot from the harness — The harness paints a shot with no display through the CPU image renderer, composited over opaque white, at the harness's own time with no tick or pump.
decision/pixel-tests-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels, box-shadow expectations taken from Chromium renders with a 1px antialiasing tolerance.
decision/shot-refused-without-fonts · Shot refused without a usable font — A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path. · guard: stand_act_screenshot, stand_mcp_tools, font_reading_tests
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/wpt-conformance-report-only · WPT conformance is report-only — A WPT conformance reading is report-only and grades no criterion.
decision/one-test-target-per-file · One test target per file — blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false. · why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target. · guard: BlitzTestsTargetsTest
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/scrolled-box-reads-where-it-stands · A scrolled box reads where it stands — For a box that is itself scrolled, layout_rect, layout_rect_of and center_of read where the box stands at every scroll offset of its own, in both layout modes, as the client bounding rect and the snapshot's bounds do. · why: ratified · guard: scrolled_box_absolute_position
decision/wpt-suites-run-in-ci · WPT suites run in CI — CI builds the WPT runner in release mode and runs its css and svg suites, producing a JSON WPT report that the wpt cli then scores and diffs.
contract/harness-construction/from-html-with · Harness construction · Harness::from_html_with — Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/harness-screenshot/fonts-unavailable · Harness screenshot · ScreenshotError::FontsUnavailable — A document whose can_paint_text reads false gives FontsUnavailable, with nothing painted.
contract/harness-screenshot/screenshot · Harness screenshot · screenshot — screenshot(&mut self) -> Result<Raster, ScreenshotError> resolves at the harness's own time with no tick and no pump, paints the viewport at its scale and device size and composites over opaque white.
contract/harness-screenshot/screenshot-rect · Harness screenshot · screenshot_rect — screenshot_rect(PixelRect) paints like screenshot at the harness's own time, the rectangle clipped to the viewport; an empty rectangle is EmptyRect.
contract/stand-boot/boot-load · seven_guis headless stand · stand::boot_load — stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
contract/wpt-cli/calc-scores · the wpt cli · calc-scores — The wpt cli provides a calc-scores command, which CI invokes on the WPT report.
contract/wpt-cli/diff-format-json · the wpt cli · diff --format json — The wpt cli provides a diff command with a JSON output format, which CI invokes.
contract/wpt-runner-outputs/wptreport-json · WPT runner outputs · wptreport.json — The WPT runner writes wptreport.json in its output directory.
decision/vibey-selection-tests-shape-real-text · Vibey-script selection tests shape real text — blitz-vibey-script's dev-dependency on blitz-dom enables system-fonts. · why: so text inputs shape real text in the selection tests
contract/agent-run-selection/all · agent-run run selection · all — all runs the whole package, with no --test.
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/all · agent-run selections · run all — Runs every reader of the blitz-tests crate, the stand readers included; there is no `run fixtures` selection.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/agent-run-verbs/boot · agent-run verbs · boot — boot recreates target/agent-run/ with a fresh events.jsonl, builds every blitz-tests test binary (--locked --no-run, output to run.log), writes status.json (booted, boot_ts, head) and emits the boot event; exit 0 ready, 1 failed.
contract/agent-run-verbs/run · agent-run verbs · run — run {selection} runs cargo test -p blitz-tests --locked with merged output to run.log, truncated at the run's start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
contract/harness-construction/from-html · Harness construction · Harness::from_html — Harness::from_html(html) builds an HTML-backed harness and pumps once.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/harness-core/pump · Harness core · pump — pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
contract/harness-inspect/center-of · Harness inspection helpers · center_of — center_of returns an (x, y) tuple.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-cells · seven_guis headless stand · stand::boot_cells — stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
contract/wpt-runner-cli/wpt-dir · WPT runner CLI · WPT_DIR — The WPT runner reads the WPT_DIR environment variable.
contract/wpt-runner-outputs/wpt-expectations-txt · WPT runner outputs · wpt_expectations.txt — The WPT runner writes wpt_expectations.txt in its output directory.
contract/wpt-status/crash · WPT status set · CRASH — CRASH is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/fail · WPT status set · FAIL — FAIL is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/pass · WPT status set · PASS — PASS is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/skip · WPT status set · SKIP — SKIP is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/timeout · WPT status set · TIMEOUT — TIMEOUT is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/blitz-tests-single-test-target · blitz-tests manifest holds one test target — The blitz-tests manifest does not set `autotests` to false and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: the fork's standing two-line difference from upstream's manifest · guard: BlitzTestsTargetsTest
decision/ci-legs-through-leg-script · CI legs run through the leg script — Each linux CI job runs its leg through `ci-leg.sh`, the leg present in the script's list, with a failure-log upload and no perl rewrite. · guard: CiWorkflowTest in test_ci_workflows.py

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/blitz-tests-manifest-difference · blitz-tests manifest's difference from upstream — The blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: The fork's standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/start-in-state-reads-as-by-hand-path · Start in a state held to its by-hand path — A session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot and takes the next call as the by-hand session does; an unknown name is refused with the session reading as boot. · guard: stand_start_in_state

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/wpt-reftests-thai-font · WPT reftests run with a Thai font — WPT reftests are run with a Thai font installed. · why: some reftests depend on Thai glyph widths

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/select-list-painted-by-paint-scene · Select list painted by the scene painter alone — The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own. · why: the harness screenshot, the session's screenshot of an open-list row in stand_act_popup and select_popup_painted all read the open list with no shell present

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/wpt-pinned-commit · WPT pinned to a recorded commit — WPT tests are cloned and the conformance suites run at the commit the WPT_COMMIT file pins.
contract/wpt-runner-env/wpt-dir · WPT runner environment · WPT_DIR — The runner reads its test inputs from the WPT checkout at WPT_DIR.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/graph-drives-confirm-every-write · Shared graph drives exercise every stand write — The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag. · why: stand_graph's every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run and stand_handler_coverage both run GRAPH_DRIVES and read no flag

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/font-metric-tests-never-skip · Font-metric tests never skip — A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping. · why: The skip came from the fixture's no-namespace element, not from missing system fonts. · guard: hover_state_tests
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello, which has no Cargo.lock
decision/ci-legs-host-reproducible · Host-reproducible CI legs — Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
decision/embolden-leg-local-only · Embolden witness leg without a CI job — The font-embolden witness leg runs locally and has no CI job of its own. · why: macOS exercises the feature in CI
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/local-pre-push-gate · Local pre-push gate — The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.
decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
decision/local-baseline · Local baseline for CI comparison — The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run. · why: It is the reference the fork's CI is compared against.
decision/wpt-harness-timeout-multiplier · WPT harness timeout multiplier — The WPT harness applies a timeout multiplier to its test timeouts. · why: Heavy interpolation suites run longer on loaded CI machines.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/clippy-legs · Clippy over the workspace and upstream blitz-dom — The clippy leg runs over the workspace, then, only when green, over upstream's blitz-dom no-default-features line with warnings denied.
decision/dependency-audit-advisories-and-licenses · Dependency audit — The audit leg checks advisories and licenses with cargo-deny over the whole locked workspace with all features.
decision/embolden-local-witness · Local witness for font-embolden paint — A paint change under the OS-gated font-embolden feature has its local witness in the embolden leg, the last of the fast legs, which has no CI job. · why: macOS exercises the feature in CI, and the leg's command reproduces macOS's numbers on linux. · guard: test_fast_leg_ends_on_the_font_embolden_witness; select_popup_painted
decision/msrv-1-91 · Minimum supported Rust version — CI builds the workspace with Rust 1.91 as the minimum supported version.
decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the tier-0 witnesses and the five tree-reading checks, each file witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap. · why: Chosen over running the tier-0 witnesses alone.
decision/blitz-dom-requests-through-stamping-constructor · blitz-dom builds requests through one stamping constructor — blitz-dom builds a network request only through its one stamping constructor, enforced by a crate-level clippy disallowed-methods config and one expect, so the clippy leg fails on any other construction. · why: Upstream's blitz-dom has no such clippy config, a new merge surface. · guard: clippy leg

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64. · guard: stand_session_recording
decision/two-clicks-without-advance-double-click · Two clicks with no advance are a double click — Under the virtual clock, two clicks with no `advance` between them are a double click. · why: Ratified. · guard: stand_virtual_clock

# test-plan · its absences, names and overviews, whole

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
> NOT YET MEASURED — the reading gathered no tier justification, critical-path list or coverage triggers for this summary

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
> NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
absence/windowed-end-to-end-tests — An automated test that launches a window

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
absence/mocking-library — A mocking library

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
> NOT YET MEASURED — a performance budget: none is set

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-test-decisions-log · ## 12. Test Decisions Log
> NO RECORDED INTENT

## session-lifecycle · key of ## 3. Test Harness Contract
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is not part of the five-command agent-run contract, which starts and binds nothing.
