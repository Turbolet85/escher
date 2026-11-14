# test-plan · the rows that bind 2026-11-13-local-files-for-local-documents — picked 39 of 599 (16 by every draw) · rule any · pin 687

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
decision/wpt-conformance-runner · WPT conformance runner — Web Platform Tests conformance is run by the project's own WPT runner, covering reftests, checkLayout attr tests, crashtests, testharness.js tests and .any.js and .window.js tests.
decision/wpt-run-on-host-on-fork · WPT run on the host on the fork — The CI wpt job runs css and svg only on the upstream repository; on the fork, Web Platform Tests for css and svg are run on the host.
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/failed-load-ends-settle-wait · Failed load ends the settle wait — A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good. · why: the harness test a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it pins it, and the driver's load wait settles through the same harness settle
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/fork-wpt-reading-report-only · Fork WPT reading is report-only — A conformance reading on the fork's dev host is report-only and grades no criterion.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
contract/harness-settle/notsettled · The harness's settle · NotSettled — Returned when work is still due after settling, naming the Busy class that holds it; otherwise Settled.
decision/one-test-target-per-file · One test target per file — blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false. · why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target. · guard: BlitzTestsTargetsTest
decision/wpt-conformance-report-only · WPT conformance is report-only — A WPT conformance reading is report-only and grades no criterion.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/parsed-markup-pinned-with-real-parser · Parsed markup pinned where the real parser runs — Parsed markup is pinned in blitz-tests, where the harness has the real HTML parser, not in dioxus-native-dom unit tests. · why: the unit tests' default DocumentConfig carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level
contract/harness-construction/from-html-with · Harness construction · Harness::from_html_with — Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
contract/harness-settle/busy-loads · Harness settle · NotSettled { busy: Loads } — After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.
contract/harness-settle/failed-load · Harness settle · failed load — A failed stylesheet load reaches the document and stops counting as a pending render-blocking resource, so settle does not read NotSettled { busy: Loads } for it.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
decision/wpt-suites-run-in-ci · WPT suites run in CI — CI builds the WPT runner in release mode and runs its css and svg suites, producing a JSON WPT report that the wpt cli then scores and diffs.
contract/agent-run-selection/all · agent-run run selection · all — all runs the whole package, with no --test.
contract/agent-run-selection/name · agent-run run selection · {name} — {name} runs one file, matching ^[a-z0-9_]+$ with tests/blitz-tests/tests/{name}.rs present; any other selection exits 2.
contract/agent-run-selections/all · agent-run selections · run all — Runs every reader of the blitz-tests crate, the stand readers included; there is no `run fixtures` selection.
contract/agent-run-selections/file · agent-run selections · run {file} — Runs one named non-stand reader.
contract/agent-run-verbs/boot · agent-run verbs · boot — boot recreates target/agent-run/ with a fresh events.jsonl, builds every blitz-tests test binary (--locked --no-run, output to run.log), writes status.json (booted, boot_ts, head) and emits the boot event; exit 0 ready, 1 failed.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/harness-settle/settle-pass-limit · Harness settle · SETTLE_PASS_LIMIT — SETTLE_PASS_LIMIT (64) passes with none quiet make settle return Render, Loads or Layout, read off the last pass.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/wpt-runner-cli/wpt-dir · WPT runner CLI · WPT_DIR — The WPT runner reads the WPT_DIR environment variable.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
contract/harness-settle/pass-limit · The harness's settle · pass limit — Work for ever ends after exactly the settle pass limit of polls, reading Render.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/file-urls-only-for-file-documents · file: URLs loadable only from a file: document — A `file:` URL is loaded by `fetch()`, a module `import` or `<script src>` only for a `file:` document, and refused with a `FetchError` otherwise. · guard: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
decision/load-wait-bound · Acting steps wait out loads in flight — An acting step that meets a load in flight waits for it up to `LOAD_WAIT`, five seconds of wall time, settling again every 10 ms and moving no virtual time; at the bound it ends not settled with `busy` `Loads`. · guard: stand_load_wait

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/wpt-reftests-thai-font · WPT reftests run with a Thai font — WPT reftests are run with a Thai font installed. · why: some reftests depend on Thai glyph widths

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
contract/wpt-runner-env/wpt-dir · WPT runner environment · WPT_DIR — The runner reads its test inputs from the WPT checkout at WPT_DIR.
decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
decision/wpt-pinned-commit · WPT pinned to a recorded commit — WPT tests are cloned and the conformance suites run at the commit the WPT_COMMIT file pins.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/wpt-harness-timeout-multiplier · WPT harness timeout multiplier — The WPT harness applies a timeout multiplier to its test timeouts. · why: Heavy interpolation suites run longer on loaded CI machines.

## session-lifecycle · key of ## 3. Test Harness Contract
contract/driver-session/load-wait · Driver session · LOAD_WAIT — A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.

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
