# Task — test-plan · 2026-10-27-upstream-sync-engine-debts

## The session
Upstream sync: eleven upstream DioxusLabs/blitz commits taken by no-commit cherry-pick onto the fork's build branch, from upstream main `ebacd68bd` with the merge base `23b9b26a9` unmoved. Sixteen commits were left out. Three conflict hunks were resolved by hand, and no dependency, lock, `deny.toml` or manifest changed.
Engine behaviour changed in every document. `:open` now matches a `details` or `dialog` carrying `open`. Every box-generating `::before`/`::after` votes in flow classification. A text input that is focused before its editor exists enables IME when the editor is built. The windowed shell routes redraw requests through its event loop.
blitz-vibey-script changed in three ways. A `file:` URL now loads only for a `file:` document (`document_may_load`, a narrowing). CSS properties are accessors on `CSSStyleDeclaration`. The runtime and the element bindings were split into module trees (`runtime/`, seven Rust modules and nine JS bootstrap scripts; `dom/element/`, six modules). This was a move: no public `ScriptRuntime` name changed.
Tests added: the blitz-tests target `pseudo_element_flow_classification` (3 tests); 3 tests in blitz-vibey-script `tests/dom.rs` (27 → 30); 1 dioxus-native-dom lib test (81 → 82); 1 test in `apps/browser/src/tab.rs`. The WPT pin moved and `publish-browser.yml` (upstream-only) gained 15 lines.
Measured: the workspace tally moved 208 · 1117 · 0 · 12 → 209 · 1125 · 0 · 12; `run all` 630 → 633 · 0 · 12; `run stand` is unmoved at 205 · 0 · 7 over 50 files; the a11y leg is unmoved at 14 · 85 · 0 · 0; `fast`, `doc` and `audit` are green; fork CI#40487650912 is green, 17/17.
Nothing was retired. escher-driver, escher-telemetry, `ci-leg.sh`, `agent-run.sh`, `cold-agent.sh` and the CI scripts are byte-unchanged.

## Reached files
- .github/workflows/publish-browser.yml · merge surface
- apps/browser/src/main.rs · merge surface
- apps/browser/src/tab.rs · merge surface
- wpt/WPT_COMMIT · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-vibey-script/src/document.rs · merge surface
- packages/blitz-vibey-script/src/fetch.rs · merge surface
- packages/blitz-vibey-script/src/lib.rs · merge surface
- packages/blitz-vibey-script/src/runtime.rs · merge surface
- packages/blitz-vibey-script/src/dom/element.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/attributes.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/form_controls.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/geometry.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/markup.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/scrolling.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/selectors.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/css.rs · merge surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/fetch.rs · merge surface
- packages/blitz-vibey-script/src/runtime/modules.rs · merge surface
- packages/blitz-vibey-script/src/runtime/timers.rs · merge surface
- packages/blitz-vibey-script/src/runtime/window.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/{css,dom_exception,fetch,fonts,interfaces,node_constants,style,stylesheets,window}.js · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/all.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface

## Touches
- The blitz-tests directory count: 147 → 148 `.rs` files and 146 → 147 test targets, from the new `pseudo_element_flow_classification`. The standing site is `test-plan-detail/test-strategy.md:54`.
- The blitz-vibey-script `tests/dom.rs` test count: 27 → 30 `#[test]`. One test is from the `file:` narrowing commit (`cee6b8731`) and one from the CSS-accessor commit (`efdc5472a`). The file gained lines at 455-488 and 859-950, so the stated range `:23-824` in `test-plan-detail/test-scope-summary.md:68` moves too.
- The dioxus-native-dom lib test count: 81 → 82 unit tests. The file count of nine is unchanged, because the new test sits in `dioxus_document.rs`. The standing site is `test-plan-detail/test-scope-summary.md:74`.
- The per-OS CI rows at `test-plan-detail/ci-integration.md:43`, now on CI#40487650912:
  - Test [default features]: 209 · 1125 · 0 · 12
  - Test (macos) and Coverage report: 190 · 1122 · 0 · 12
  - Test (windows): 190 · 1076 · 0 · 9
  - Accessibility (a11y) tests (14 · 85 · 0 · 0) and Stand checks repeated (204 · 864 · 0 · 21): unmoved
- The local workspace tally at `test-plan-detail/ci-integration.md:64`: 208 · 1117 · 0 · 12 → 209 · 1125 · 0 · 12. The +8 passed split as the new target 3, `tests/dom.rs` 3, dioxus-native-dom 1 and `apps/browser/src/tab.rs` 1.
- The agent-run proof chain's `run all` reading: 630 → 633 · 0 · 12. The +3 are the new target's tests, a file with no `stand_` prefix that `run all` picks up with no script change. `run stand` reads 205 · 0 · 7 over 50 `stand_*.rs` files, unmoved. The 0.1.0 reading of 396 at `test-plan.md:172` stays as measured then.
- Citation rows into `packages/blitz-vibey-script/tests/dom.rs`, whose lines moved:
  - `test-plan.md:96` and `:143`
  - `test-plan-detail/integration-test-strategy.md:8`
  - `test-plan-detail/mocking-stubbing-discipline.md:19`
  - `test-plan-detail/test-data-fixtures.md:25` and `:40`
- Citations into the cherry-picked files, whose line numbers shifted where text was added above them:
  - blitz-dom `node/element.rs`: +10 lines at 488-489 and 543-550
  - blitz-dom `mutator.rs`: +8 lines at 368-371 and 520-523
  - blitz-dom `stylo.rs`: +1 line at 510
  - blitz-dom `layout/construct.rs`: added at 313-317, 860 and 867-872
  - blitz-vibey-script `document.rs`: added at 16, 163-169, 210-211 and 222
  - dioxus-native-dom `dioxus_document.rs`: added at 725-759
  - `apps/browser/src/tab.rs`: added at 130-136, 177-182 and 294-343
  - `.github/workflows/publish-browser.yml`: added at 126 and 153-166
- Citations into blitz-vibey-script `src/runtime.rs` and `src/dom/element.rs`: the cited code moved into the new `runtime/` and `dom/element/` modules. `runtime.rs` keeps 151 lines.
- The test-scope-summary absence of inline unit test modules in blitz-vibey-script source files, searched over the 32 files of slice s10. The split adds 13 Rust modules. The added tests count no inline module in the crate's sources, so the absence holds, but the file count it names is stale.
- The Script ↔ DOM boundary (`fetch()` through a custom ScriptFetcher, the window error event path). The default fetcher still takes only `file:` and `data:`, and now refuses a `file:` load for a non-`file:` document by `fetch()`, module `import` and `<script src>`.
- The uncaught-JS-error path the window error event tests reach. Its `tracing::error!` moved whole into `report_js_error` (`runtime/events.rs:21`), feature-gated, under engine target `blitz_vibey_script`.
- Upstream's unbuilt `tests/all.rs`: it gains a `mod` line at `:54` and stays `test = false`. The fork's one-target-per-file layout holds: the new file runs as its own target with no `Running tests/all.rs` line, and `BlitzTestsTargetsTest` is green inside `fast`.
- The apps/browser test inventory in test-scope-summary: `tab.rs` now holds a test module beside about_pages, browser_history, favicon, url_suggestions and persistence.
- The dioxus-native-dom test inventory in test-scope-summary: the document's tests gain an IME-on-mount test.
- The WPT fixture-source pin: `wpt/WPT_COMMIT` moved `9d8aceec0dd…` → `d4b721171c1…`. The rule naming the file is unmoved, and the pin a 2026-10-10 run read at `test-plan-detail/test-strategy.md:46` stays as measured then.
- The keyboard_compat coverage exclusion, re-read: `keyboard_compat.rs` stays, and both `cargo llvm-cov` lines in `ci-leg.sh` keep its exclusion. The floor and the exclusion are unmoved.
- The owner at `test-plan-detail/quality-gates-coverage-targets.md:15` that a master names. `.andromeda/residuals.md` lines 35 (the left-out upstream delta, now +6 commits) and 36 (the merge surfaces) move their owner to Epoch 3's «Upstream sync ahead of the UI graph».
- The upstream-merge records at `test-plan-detail/test-scope-summary.md:14` and `:98` (the 2026-10-17 merge at `23b9b26a9`). They were re-read, unmoved as measured then; the merge base is unchanged.
- The pipeline-test invariant that pipeline tests run `for incremental in [false, true]`. The new target does not assert both layout modes, and `incremental_oracle` stays green.
- Standing gate readings re-read and green:
  - `fast`, `doc` and `audit` (advisories ok, licenses ok)
  - the CI-scripts leg (`Ran 108 tests`, OK)
  - `cargo test -p seven_guis --locked` (12 lines, 41 passed)
  - blitz-dom lib (106 passed)
  - the 23-target integration run (125 passed)
  - the windowed `seven_guis_native` boot smoke (exit 124, `service.name=seven_guis`)

## New facts
- F1 · test-scope-summary · tests/blitz-tests holds upstream's target `pseudo_element_flow_classification`, with 3 tests that every box-generating `::before`/`::after` votes in flow classification (the `display: contents` filter dropped). Its `mod` line is in the unbuilt `all.rs`.
- F2 · integration-test-strategy · `pseudo_element_flow_classification` is an exception to the both-layout-modes rule: the file asserts neither mode pair, and `incremental_oracle` green beside it is the layout-identity witness.
- F3 · ci-integration · `pseudo_element_flow_classification` is reached by the workspace `test` leg and by `run all` / `run {name}` only. It has no `stand_` prefix, is not one of the `stand-repeat` leg's ten named targets, and is not in the a11y leg.
- F4 · integration-test-strategy · `tests/dom.rs` `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` pin `document_may_load`: a `file:` URL loads by `fetch()`, module `import` and `<script src>` only for a `file:` document. Its `FetchError` refusal is witnessed on the linux, macOS and Windows CI legs.
- F5 · integration-test-strategy · `tests/dom.rs` `style_property_accessors` pins CSS properties as accessors on `CSSStyleDeclaration`, their names from Stylo's property list (`runtime/css.rs` `css_property_names`, `bootstrap/style.js` `installStyleProperties`).
- F6 · test-data-fixtures · `tests/dom.rs` `temp_dir_with_files` writes a directory of sibling files under `std::env::temp_dir()` at test time for the `file:`-load tests. It is test data on disk beside the in-code seeds, and a second test-only `TMPDIR` read beside escher-driver's client unit test.
- F7 · test-scope-summary · dioxus-native-dom's `input_focused_on_mount_enables_ime` (in `dioxus_document.rs`) pins that a text input focused before its editor is constructed enables IME (`set_ime_enabled(true)`) at that construction.
- F8 · test-scope-summary · `apps/browser/src/tab.rs` holds a test module with one test, `tab_lens_survives_webview_props_replacement`: tab lenses survive a webview props replacement.
- F9 · test-scope-summary · `:open` matching on a `details` or `dialog` carrying `open` (`ElementData::flush_open_state` setting `ElementState::OPEN`, written on construction and on an `open` attribute write or removal) has no test. `details_element` and the a11y leg read green beside it.
- F10 · test-scope-summary · the windowed shell's redraw route through its event loop holds no check of its own. It is read only by the windowed `seven_guis_native` boot smoke and by `focus_redraw`.
- F11 · test-scope-summary · upstream's dioxus-native-dom test `task_polled_during_render_can_access_document` is absent from the fork: it was dropped from a conflict hunk because its commit `286874cc1` was not taken.

## Untouched
- The blitz-test-harness API, settle, screenshot and contrast readers and their unit tests: the crate is not reached and its constructors, helpers and options are unchanged.
- escher-driver's unit tests, the session, MCP, recording and replay checks, `session_common`, `fixtures/mod.rs` and the committed Counter checks: escher-driver and escher-telemetry are byte-unchanged.
- The stand checks, their shared tables (`controls`, `tab_order`, `ENABLED_TAB_ORDER`, `REGION_NAMES`), the contrast rows and the stand figures: no stand file changed and `run stand` is unmoved at 205 · 0 · 7 over 50 files.
- The agent-run and cold-agent contracts, their contract tests and the cold-agent live-run verdict: `scripts/agent-run.sh`, `scripts/cold-agent.sh` and their tests are unchanged.
- The CI legs, the fast/slow split, caches, failure logs, package installs and `stand-repeat`'s three-round zero-failure budget: `ci.yml`, `ci-leg.sh` and `apt-install.sh` are unchanged.
- The coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the mutation tier: `coverage_floor.py` and `.config/nextest.toml` are unchanged.
- Third-party test frameworks and the test-that 0.6.0 dev-dependency: no package entered, left or moved.
- The mocking stand-ins `HeldNet`, `LateNet`, `LoadNet` and `TimerTicks`, and the time discipline of the session checks: none is reached.
- The `CARGO_TARGET_TMPDIR` rows (`test-plan.md:149`, `test-plan-detail/integration-test-strategy.md:25`, `test-plan-detail/test-strategy.md:14`): another variable sharing the `TMPDIR` token, unmoved.
- The stand log format, the sink's target allowlist and the needle counts: escher-telemetry is unchanged, and `host_log` and `mcp_log` are green under both builds.
- Rows naming `boot_load`, `LOAD_ANSWER_DELAY` or `type-select`: the design-system stand-row move touches no test-plan claim.
- The owner-generic row at `test-plan-detail/test-scope-summary.md:139`: another chunk's owner, unmoved.
- The WPT runner, its CLI, outputs, scoring and unit tests: `wpt/runner/` is not reached; only the pin file moved.
