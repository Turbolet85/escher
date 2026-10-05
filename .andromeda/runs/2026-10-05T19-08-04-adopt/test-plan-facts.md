# test-plan — gathered facts

## §Test Scope Summary

### facts-s01.md:202

- `cargo test --workspace` runs on ubuntu with default features (.github/workflows/ci.yml:48-56)
- The matrix runs `test --all --tests` on windows, macos, linux and `build --all` on ios and android (.github/workflows/ci.yml:135-174; .github/workflows/ci.yml:219-221)
- Web Platform Tests run for `css` and `svg` (.github/workflows/wpt.yml:55-56)
- CI Python scripts are unit-tested with `python3 -m unittest discover -s .github/scripts` (.github/workflows/ci.yml:111-116)

### facts-s02.md:165

- the slice is a set of HTML example documents, each built around one rendering feature or a real page (examples/assets/hr.html:192; examples/assets/inline-backgrounds.html:57; examples/assets/iframe_navigation.html:29; examples/assets/object_fit.html:12)

### facts-s03.md:209

- The slice's only file under `tests/` is `tests/stylo_usage.rs`, whose code lines are all commented out (tests/stylo_usage.rs:4-160)

### facts-s04.md:320

- Tests exist only in the browser crates: about_pages (3), browser_history (4), favicon (5), url_suggestions (12 `#[test]` plus 4 `#[tokio::test]`), persistence (10) (apps/browser/src/about_pages.rs:204-239; apps/browser/src/browser_history.rs:178-225; apps/browser/src/favicon.rs:83-112; apps/browser/src/url_suggestions.rs:415-629; apps/browser/persistence/src/lib.rs:341-506)
- observed absent — tests in rdme, bump, examples, accesskit_xplat · searched: `#\[(tokio::)?test\]` over the 86 slice files (hits only in the five browser files above)

### facts-s05.md:233

- 25 `#[test]` functions in the slice: 9 in document.rs, 12 in mutator.rs, 4 in net.rs (packages/blitz-dom/src/document.rs:2859; packages/blitz-dom/src/mutator.rs:1352; packages/blitz-dom/src/net.rs:616)
- Covered behaviours: media type defaults, disabled state, id map, redraw-after-mutation, in-document flags, style-property relayout, rule-tree copy-on-write (packages/blitz-dom/src/mutator.rs:1352-1799); zoom redraw, hover cursor, hover/checked/pseudo/background-image invalidation, `@font-face` alias registration (packages/blitz-dom/src/document.rs:2843-3402); `@font-face` style mapping (packages/blitz-dom/src/net.rs:606-646)

### facts-s06.md:227

- `stylo_to_parley.rs` holds nine unit tests of font feature mapping (packages/blitz-dom/src/stylo_to_parley.rs:525-655)
- `util.rs` holds seven SVG parsing tests compiled only with the `svg` feature (packages/blitz-dom/src/util.rs:180-243)
- `stylo.rs` holds two `#[test]` functions whose bodies are entirely commented out (packages/blitz-dom/src/stylo.rs:1492-1518)

### facts-s07.md:155

- The slice holds one inline unit-test module, in list.rs, covering `marker_for_style` (packages/blitz-dom/src/layout/list.rs:185-237)

### facts-s08.md:200

- In-file tests cover overlay scrollbar opacity, text-input scrolling, and the element state of the `disabled` attribute (packages/blitz-dom/src/node/scrollbar.rs:224-237; packages/blitz-dom/src/node/element.rs:958-1092; packages/blitz-dom/src/node/node.rs:1736-1822)
- One test parses a full HTML document through `DocumentHtmlParser` (packages/blitz-html/src/html_sink.rs:315-333)
- observed absent — any test in blitz-net · searched: `#\[test\]|#\[cfg\(test\)\]` over packages/blitz-net/src/lib.rs (no match)

### facts-s09.md:189

- inline unit tests exist in gradient.rs and css_box.rs (packages/blitz-paint/src/gradient.rs:517-536; packages/blitz-paint/src/kurbo_css/css_box.rs:661-847)
- blitz-test-harness is a dedicated, unpublished headless harness crate for Blitz documents (packages/blitz-test-harness/Cargo.toml:1-4)

### facts-s10.md:236

- tests/dom.rs holds 26 `#[test]` functions covering the JS DOM APIs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/dom.rs:23-786)
- tests/preact.rs holds 2 `#[test]` functions running the vendored Preact TodoMVC example headlessly (packages/blitz-vibey-script/tests/preact.rs:1-2; packages/blitz-vibey-script/tests/preact.rs:102-199)
- observed absent — inline unit test modules · searched: `#\[cfg\(test\)\]|mod tests` over the 32 slice files

### facts-s11.md:209

- `dioxus-native-dom` has three unit tests: `keyed_nodes_do_not_crash`, `touches_reports_all_active_pointers` and `touches_is_empty_when_no_pointers_are_active` (packages/dioxus-native-dom/src/dioxus_document.rs:375-411; packages/dioxus-native-dom/src/events.rs:684-728)
- observed absent — tests in `dioxus-native` and `stylo_taffy` · searched: `#\[test\]|#\[cfg\(test\)\]` over the 21 listed s11 files (matches only in dioxus_document.rs and events.rs)

### facts-s12.md:194

- The crate holds Blitz's integration tests, one file per behavior under `tests/` (tests/blitz-tests/lib.rs:1; tests/blitz-tests/Cargo.toml:3)
- Coverage spans accessibility, layout, text, paint, hit testing, input events, scrolling, style invalidation, DOM mutation leaks, network-driven stylesheet loading and Dioxus integration (tests/blitz-tests/tests/accessibility_roles.rs:1; tests/blitz-tests/tests/incremental_oracle.rs:1; tests/blitz-tests/tests/whitespace_modes.rs:15; tests/blitz-tests/tests/paint_order.rs:1; tests/blitz-tests/tests/pointer_events.rs:1; tests/blitz-tests/tests/touch_events.rs:1; tests/blitz-tests/tests/fragment_navigation.rs:1; tests/blitz-tests/tests/style_property_invalidation.rs:1; tests/blitz-tests/tests/inner_html_leak.rs:1; tests/blitz-tests/tests/link_rel_attribute.rs:1; tests/blitz-tests/tests/stale_node_mapping.rs:1)

### facts-s13.md:205

- The slice is the WPT conformance runner; it runs reftests, checkLayout attr tests, crashtests, testharness.js tests and `.any.js`/`.window.js` tests (wpt/runner/src/test_runners/mod.rs:240-327)
- Unit tests in the slice: 8 in fuzzy.rs, 3 in js_wrapper.rs, 2 in harness_test.rs, 1 in mod.rs (wpt/runner/src/test_runners/fuzzy.rs:146-235; wpt/runner/src/test_runners/js_wrapper.rs:102-132; wpt/runner/src/test_runners/harness_test.rs:294-315; wpt/runner/src/test_runners/mod.rs:408-443)

## §Test Strategy

### facts-s01.md:208

- MSRV is verified by build only, not test (.github/workflows/ci.yml:20-25)
- WPT results are diffed against the main-branch report and summarised per test as gained/lost subtests (.github/workflows/wpt.yml:75-85; .github/scripts/wpt_diff_to_pr.py:116-161)

### facts-s02.md:168

- fixture prose states the expected rendering for a human viewer: frame navigation replaces only the sub-document (examples/assets/iframe_navigation.html:33-36; examples/assets/iframe_navigation.html:58-61)
- fixture comments state what a case checks: correct alpha with no double-paint, and per-run ascent/descent (examples/assets/inline-backgrounds.html:40; examples/assets/inline-backgrounds.html:47; examples/assets/inline-backgrounds.html:84-86)
- the hr fixture's first case is left unstyled to show the UA stylesheet only (examples/assets/hr.html:33-36; examples/assets/hr.html:195)
- full real-page snapshots are paired with reduced variants (examples/assets/google_reduced.html:1-9; examples/assets/gosub_reduced.html:1-12; examples/assets/servo-new-reduced.html:1-51; examples/assets/servo-new-reduced-1.html:1-31)

### facts-s03.md:212

- observed absent — test functions · searched: `#\[test\]` and `#\[cfg\(test` over the 32 slice files

### facts-s04.md:324

- Persistence tests target the inner SQL functions with a freshly migrated in-memory connection rather than the `HistoryStore` wrapper (apps/browser/persistence/src/lib.rs:306-326)
- In-memory history tests call `record_visit_inner` directly on a `VecDeque` (apps/browser/src/browser_history.rs:178-207)
- URL suggestion tests split a synchronous nucleo driver from worker-level tokio tests covering the command state machine (apps/browser/src/url_suggestions.rs:390-406; apps/browser/src/url_suggestions.rs:547-549)

### facts-s05.md:237

- Tests build a `BaseDocument` directly from `DocumentConfig` and construct the DOM through `DocumentMutator` (packages/blitz-dom/src/mutator.rs:1482-1498)
- DOM fixtures are built manually because the HTML parser lives in blitz-html, which the comment says would be a circular dev-dependency (packages/blitz-dom/src/document.rs:2885-2890; packages/blitz-dom/src/document.rs:3345-3350)
- Pipeline tests call `resolve(0.0)` and assert on resulting layout or computed styles (packages/blitz-dom/src/mutator.rs:1693-1718; packages/blitz-dom/src/document.rs:3028-3060)

### facts-s06.md:232

- Tests are inline Rust unit tests next to the code they cover (packages/blitz-dom/src/stylo_to_parley.rs:525-528; packages/blitz-dom/src/util.rs:180-182)

### facts-s07.md:158

- out of slice — the slice states no test strategy beyond the one inline module

### facts-s08.md:205

- Tests are Rust `#[test]` functions inside `#[cfg(test)]` modules next to the code under test (packages/blitz-dom/src/node/scrollbar.rs:224-229; packages/blitz-dom/src/node/element.rs:958-976; packages/blitz-dom/src/node/node.rs:1736-1742)

### facts-s09.md:193

- the harness wraps any blitz_dom Document, such as HtmlDocument or DioxusDocument, with deterministic construction defaults, a pump/tick loop, inspection helpers and input synthesis (packages/blitz-test-harness/src/lib.rs:1-14)
- dom_string produces a stable one-node-per-line tree serialization with geometry, described as suitable for snapshot-style assertions (packages/blitz-test-harness/src/inspect.rs:117-131)
- regression tests are named for the bug they guard, e.g. transposed tall corners and k == 2 NaN (packages/blitz-paint/src/kurbo_css/css_box.rs:684-691; packages/blitz-paint/src/kurbo_css/css_box.rs:756-765)

### facts-s10.md:241

- tests build a `ScriptDocument` from inline HTML, run its scripts, then assert DOM text via selector queries (packages/blitz-vibey-script/tests/dom.rs:8-21)
- events are driven with synthetic click events and `UiEvent::KeyDown` rather than real input (packages/blitz-vibey-script/tests/dom.rs:179-190; packages/blitz-vibey-script/tests/preact.rs:68-100)
- timer tests sleep in real time before polling (packages/blitz-vibey-script/tests/dom.rs:278-280; packages/blitz-vibey-script/tests/dom.rs:299-300)

### facts-s11.md:213

- Tests are Rust `#[cfg(test)]` modules inside the source files (packages/dioxus-native-dom/src/dioxus_document.rs:366-367; packages/dioxus-native-dom/src/events.rs:656-657)
- `keyed_nodes_do_not_crash` is a regression test for a panic when keyed nodes are reordered (packages/dioxus-native-dom/src/dioxus_document.rs:375-378)
- A rustdoc usage example for `DioxusDocument` sits in the doc comment (packages/dioxus-native-dom/src/dioxus_document.rs:41-63)

### facts-s12.md:198

- Two styles: direct `HtmlDocument` + `DocumentConfig` construction, and the `blitz_test_harness::Harness` wrapper over `HtmlDocument` and `DioxusDocument` (tests/blitz-tests/tests/comment_layout.rs:14-25; tests/blitz-tests/tests/harness_smoke.rs:1-3)
- Differential oracle: the same fixture and mutation sequence runs in an incremental and a non-incremental document, comparing every node's `final_layout()`, `layout_children` and paint tree after each step and asserting no damage remains (tests/blitz-tests/tests/incremental_oracle.rs:1-19; tests/blitz-tests/tests/incremental_oracle.rs:314-347)
- Pixel tests render to a CPU buffer and sample pixels; box-shadow expectations come from Chromium renders with a 1px antialiasing tolerance (tests/blitz-tests/tests/paint_order.rs:12-29; tests/blitz-tests/tests/outset_box_shadow_shape.rs:6; tests/blitz-tests/tests/outset_box_shadow_shape.rs:82-88)
- Regression tests document the bug and the real-world page that exposed it (tests/blitz-tests/tests/stale_dirty_descendants.rs:1-20; tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8)
- WPT script tests are ported to Rust when the WPT runner cannot run them (tests/blitz-tests/tests/oof_dynamic_cb.rs:7-12)
- Deterministic pseudo-random template swaps (an LCG) run 1000 steps to shake out slab/ElementId reuse bugs (tests/blitz-tests/tests/stale_node_mapping.rs:176-217)
- Pure-restyle paths are driven by `:hover` rather than attribute mutation because mutations insert full damage and mask under-damaging bugs (tests/blitz-tests/tests/oof_dynamic_cb.rs:143-145; tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:6-9)
- Font-dependent tests skip at runtime with `eprintln!` when no usable font is available (tests/blitz-tests/tests/text_selection_anonymous_block.rs:63-78; tests/blitz-tests/tests/text_selection_anonymous_block.rs:104-109)

### facts-s13.md:209

- Reftests render test and reference to RGBA buffers at 800x600 and compare them exactly, by fuzzy tolerance, or by dify diff (wpt/runner/src/test_runners/ref_test.rs:25-102; wpt/runner/src/test_runners/ref_test.rs:179-211)
- Attr tests check element layout against `data-expected-*` attributes, one subtest per element matching the selector (wpt/runner/src/test_runners/attr_test.rs:71-124)
- Harness tests run real testharness.js and collect per-subtest results (wpt/runner/src/test_runners/harness_test.rs:119-183)
- Partial passes are counted fractionally (pass/total per test) in addition to whole-test counts (wpt/runner/src/main.rs:139-145; wpt/runner/src/main.rs:666-667; wpt/runner/src/main.rs:812-815)
- Failures are bucketed by the first matching feature flag (grid-lanes, subgrid, writing-mode, direction, intrinsic size, calc, float, script) else "other" (wpt/runner/src/main.rs:639-658)

## §Test Harness Contract

### facts-s01.md:212

- The WPT runner is invoked as `cargo build -rp wpt` then `cargo run -rp wpt css svg`, producing `./wpt/output/wptreport.json` (.github/workflows/wpt.yml:53-58)
- The `wpt` cli provides `calc-scores` and `diff --format json` (.github/workflows/wpt.yml:69-80)
- The test file states it runs with `python3 -m unittest discover .github/scripts` (.github/scripts/test_wpt_diff_to_pr.py:2)

### facts-s02.md:174

- out of slice — no harness, runner or test code is in the slice

### facts-s03.md:215

- out of slice — test harness configuration

### facts-s04.md:329

- Tests use the standard `#[test]` harness and `#[tokio::test]` for async worker tests (apps/browser/src/url_suggestions.rs:415; apps/browser/src/url_suggestions.rs:551)
- `drive_worker` queues messages, drops the sender, captures publications, and bounds the run with a 2-second timeout (apps/browser/src/url_suggestions.rs:364-388)
- `make_conn` opens an in-memory sqlite connection and migrates it to latest (apps/browser/persistence/src/lib.rs:322-326)

### facts-s05.md:242

- Tests use the standard `#[cfg(test)]` module and `#[test]` attribute harness (packages/blitz-dom/src/net.rs:606-618)
- observed absent — third-party test frameworks · searched: `proptest|insta::|criterion|mock` over the 15 s05 files

### facts-s06.md:235

- Tests use the built-in Rust `#[test]` attribute with `assert!` and `assert_eq!` (packages/blitz-dom/src/stylo_to_parley.rs:550-564; packages/blitz-dom/src/util.rs:191-200)
- SVG tests are gated on both `test` and the `svg` feature (packages/blitz-dom/src/util.rs:180)

### facts-s07.md:161

- Tests use Rust's built-in harness: `#[cfg(test)] mod tests` with `#[test]` functions and `assert_eq!` (packages/blitz-dom/src/layout/list.rs:185-204)

### facts-s08.md:208

- Tests use the built-in Rust test harness with `assert!`/`assert_eq!` (packages/blitz-dom/src/node/scrollbar.rs:228-236; packages/blitz-dom/src/node/node.rs:1754-1761)
- DOM tests construct `BaseDocument::new(DocumentConfig::default())` and create nodes with `create_node` (packages/blitz-dom/src/node/node.rs:1743-1752)
- Text-input tests build a `TextInputData` laid out at scale 1.0 with fresh parley `FontContext`/`LayoutContext` (packages/blitz-dom/src/node/element.rs:963-974)

### facts-s09.md:198

- Harness construction: from_html, from_html_with, from_component, from_vdom, wrap; constructors pump once, wrap does not (packages/blitz-test-harness/src/harness.rs:59-92)
- Harness core: into_inner, base, base_mut, time, pump, tick, dispatch, dispatch_recorded, set_viewport_size (packages/blitz-test-harness/src/harness.rs:94-184)
- pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded do not pump (packages/blitz-test-harness/src/harness.rs:113-138)
- input helpers: click, click_at, mouse_down_at, mouse_up_at, move_mouse_to, drag, tap, tap_at, touch_down, touch_move, touch_up, wheel_at, press, press_with, type_text, ime — each pumps after dispatch (packages/blitz-test-harness/src/input.rs:95-232)
- synthesized pointer events set page, screen and client coordinates to the same values (packages/blitz-test-harness/src/input.rs:18-27)
- key_event uses Code::Unidentified and Location::Standard and fills text only for pressed character keys (packages/blitz-test-harness/src/input.rs:75-93)
- inspection helpers: query, node, query_all, layout_rect, layout_rect_of, center_of, text_content, attr, hit, hit_node, focused, hovered, dom_string (packages/blitz-test-harness/src/inspect.rs:26-131)

### facts-s10.md:246

- `doc_from_html` constructs with the default `DocumentConfig` and calls `execute_scripts`; `text_of_selector` reads text content (packages/blitz-vibey-script/tests/dom.rs:8-21)
- preact helpers: load_todomvc, resolve, query, query_all, text_of, enter_key, click, add_todo (packages/blitz-vibey-script/tests/preact.rs:16-100)
- virtual time and `without_timer_thread` are intended for embedders driving timers manually, e.g. test runners (packages/blitz-vibey-script/src/clock.rs:10-14; packages/blitz-vibey-script/src/document.rs:108-133)
- dev-dependency blitz-dom enables `system-fonts` so text inputs shape real text in the selection tests (packages/blitz-vibey-script/Cargo.toml:42-48)

### facts-s11.md:218

- The dioxus-native-dom tests use the `dioxus` crate as a dev-dependency (packages/dioxus-native-dom/Cargo.toml:43-44; packages/dioxus-native-dom/src/dioxus_document.rs:370)
- The document test builds a `DioxusDocument` with `DocumentConfig::default()`, calls `initial_build`, and drives updates with `mark_dirty` and `poll(None)` (packages/dioxus-native-dom/src/dioxus_document.rs:394-404)

### facts-s12.md:208

- `Harness::from_html(html)` and `Harness::from_html_with(html, HarnessOptions)` build an HTML-backed harness; `Harness::from_component(fn)` and `Harness::from_vdom(vdom, options)` build Dioxus-backed ones (tests/blitz-tests/tests/harness_smoke.rs:10; tests/blitz-tests/tests/pointer_events.rs:8-17; tests/blitz-tests/tests/harness_smoke.rs:104; tests/blitz-tests/tests/stale_node_mapping.rs:49-56)
- `pump()` applies pending changes after a mutation through `base_mut().mutate()` (tests/blitz-tests/tests/dir_attribute.rs:72-79; tests/blitz-tests/tests/oof_dynamic_cb.rs:37-43)
- `dispatch_recorded([UiEvent, ...])` returns the list of dispatched event names (tests/blitz-tests/tests/touch_events.rs:40-44)
- The harness crate exports a `pointer_event(id, x, y, button, buttons, mods)` builder (tests/blitz-tests/tests/touch_events.rs:6; tests/blitz-tests/tests/touch_events.rs:12-21)
- `layout_rect(selector)` returns a rect with `x`, `y`, `width`, `height`; `center_of` returns an `(x, y)` tuple (tests/blitz-tests/tests/harness_smoke.rs:16-21)

### facts-s13.md:216

- Invocation: `WPT_DIR` env var, optional suite arguments, `--verbose`/`-v`, `--run-quarantined`, `--list` (wpt/runner/src/main.rs:240-250; wpt/runner/src/main.rs:461-479)
- Outputs: `wpt_expectations.txt` and `wptreport.json` in the output directory (wpt/runner/src/main.rs:832-847)
- Status set: PASS, FAIL, TIMEOUT, SKIP, CRASH (wpt/runner/src/main.rs:104-122)

## §Bootstrap phases

(no fact block)

## §Unit Test Strategy

### facts-s01.md:217

- test_wpt_diff_to_pr.py covers `format_lines` ordering/alignment/markers, `render` headline counts and empty diff, and `splice` idempotent replacement (.github/scripts/test_wpt_diff_to_pr.py:51-88)
- out of slice — Rust unit tests in the crates

### facts-s02.md:177

- out of slice — no unit test code is in the slice

### facts-s03.md:218

- observed absent — unit tests · searched: `#\[test\]` and `#\[cfg\(test` over the 32 slice files

### facts-s04.md:334

- About-page URL parsing: known paths, unknown rejection, round-trip (apps/browser/src/about_pages.rs:204-239)
- History fold, non-consecutive revisit, truncation to the cap, elapsed-label buckets (apps/browser/src/browser_history.rs:178-225)
- Favicon decode acceptance and rejection cases (apps/browser/src/favicon.rs:83-112)
- Suggestions: empty query, literal-first, search-last, case-insensitive and fuzzy matching, cap of six history rows, URL dedup, ranking (apps/browser/src/url_suggestions.rs:415-545)
- Persistence: schema bootstrap, migration validation, round trip, ordering, clear, fold, NULL-only favicon patching, prune cap (apps/browser/persistence/src/lib.rs:341-506)

### facts-s05.md:246

- `stylo_to_fontique_style` is unit-tested for Italic, `Oblique(0,0)` → Normal, single angle and range-uses-min cases (packages/blitz-dom/src/net.rs:616-645)
- Disabled-state toggling is tested on a node created without a tree (packages/blitz-dom/src/mutator.rs:1379-1413)

### facts-s06.md:239

- Feature-mapping tests assert exact OpenType tag and value pairs per `font-variant-*` input (packages/blitz-dom/src/stylo_to_parley.rs:550-635)
- A test asserts `font-feature-settings` entries come after variant-derived ones so they win (packages/blitz-dom/src/stylo_to_parley.rs:637-654)
- SVG tests assert intrinsic width, height and aspect ratio for viewBox, absolute, percentage, unit and non-numeric dimensions (packages/blitz-dom/src/util.rs:184-242)

### facts-s07.md:164

- Four unit tests check list markers: disc, decimal, lower-alpha (including `aa.`/`ab.` past 26), upper-alpha (packages/blitz-dom/src/layout/list.rs:200-236)
- observed absent — tests in the other seven slice files · searched: `#\[test\]` over the 8 slice files

### facts-s08.md:213

- `opacity_holds_through_the_fade_delay_then_fades_out` asserts scrollbar opacity at fixed durations (packages/blitz-dom/src/node/scrollbar.rs:228-236)
- Five text-input scroll tests cover no-scroll for short text, following the caret on a single line, vertical-only multiline scroll, clamping and bubbling of `scroll_by`, and no scroll when text fits (packages/blitz-dom/src/node/element.rs:976-1091)
- Four tests assert `DISABLED`/`ENABLED` element state for a button with `disabled` (empty or `"false"` value), an `<a>` with `disabled`, and a bare button (packages/blitz-dom/src/node/node.rs:1742-1821)
- Two text-input tests assert only inside an `if` on the measured layout size, so they pass without asserting when the text does not overflow (packages/blitz-dom/src/node/element.rs:997-1007; packages/blitz-dom/src/node/element.rs:1038-1044)

### facts-s09.md:207

- unit tests sit in #[cfg(test)] mod tests blocks inside the source file (packages/blitz-paint/src/gradient.rs:517-536; packages/blitz-paint/src/kurbo_css/css_box.rs:661-775)
- two #[test] functions sit at file top level outside the tests module (packages/blitz-paint/src/kurbo_css/css_box.rs:777-847)
- a helper assert_solves checks start_angle numerically against its defining equation across a grid of inputs (packages/blitz-paint/src/kurbo_css/css_box.rs:665-682; packages/blitz-paint/src/kurbo_css/css_box.rs:767-774)

### facts-s10.md:252

- observed absent — unit tests inside source files · searched: `#\[cfg\(test\)\]|mod tests` over the 32 slice files

### facts-s11.md:222

- Touch-data tests build `BlitzPointerEvent` values directly and check `touches`/`touches_changed` counts and coordinates (packages/dioxus-native-dom/src/events.rs:684-728)

### facts-s12.md:215

- observed absent — in-file `#[cfg(test)]` unit-test modules · searched: `#\[cfg` over the 61 slice files
- out of slice — unit tests inside the engine crates

### facts-s13.md:221

- fuzzy.rs tests parse named, spaced, positional and per-reference ranges, invalid input, metas from HTML, tolerance selection and buffer diff (wpt/runner/src/test_runners/fuzzy.rs:146-235)
- js_wrapper.rs tests META block parsing, absence of the GLOBAL block for `.window.js`, and exclusion of worker-only `.any.js` (wpt/runner/src/test_runners/js_wrapper.rs:102-132)
- mod.rs tests the timeout quarantine file's validity against a list of known reasons (wpt/runner/src/test_runners/mod.rs:408-443)

## §Integration Test Strategy

### facts-s01.md:221

- `tests/blitz-tests` is a workspace member (Cargo.toml:22)
- out of slice — its contents

### facts-s02.md:180

- out of slice — no integration test code is in the slice

### facts-s03.md:221

- `tests/stylo_usage.rs` sketches styling a DOM with Stylo (stylist, stylesheet, traversal) entirely in comments (tests/stylo_usage.rs:55-160)

### facts-s04.md:341

- observed absent — integration tests outside `src` · searched: `#\[(tokio::)?test\]` over the 86 slice files; every hit sits inside a `#[cfg(test)] mod tests` in `src`

### facts-s05.md:250

- `load_resource` is driven with a fabricated `ResourceLoadResponse` to pin the `@font-face` override load path (packages/blitz-dom/src/document.rs:3336-3401)
- Hover invalidation tests drive `set_hover_to` and `resolve` and compare computed styles before/after (packages/blitz-dom/src/document.rs:3028-3060)

### facts-s06.md:244

- out of slice — no integration tests are in the s06 files

### facts-s07.md:168

- out of slice — no integration tests are in this slice

### facts-s08.md:219

- `parses_some_html` parses an HTML string into a `BaseDocument` via html5ever and calls `print_tree`, with no assertion (packages/blitz-html/src/html_sink.rs:315-333)
- out of slice — crate-level `tests/` directories

### facts-s09.md:212

- harness input routes through the document's real event-dispatch pipeline without a window (packages/blitz-test-harness/src/lib.rs:11-12; packages/blitz-test-harness/src/input.rs:1-5)
- dispatch_recorded drives events against the underlying BaseDocument, bypassing document-specific handling such as Dioxus VirtualDom forwarding (packages/blitz-test-harness/src/harness.rs:132-170)

### facts-s10.md:255

- dom.rs covers inline scripts, document order, tree mutation, attributes, selectors, innerHTML, click listeners, bubbling, microtasks, timers, requestAnimationFrame, input value, checkbox events, DOMContentLoaded/load, on-event properties, wrapper identity, style, modifier state, hidden, selection offsets, interface globals (packages/blitz-vibey-script/tests/dom.rs:23-578)
- dom.rs covers CSSOM rules, CSSOM restyle, font-face and keyframes rules (packages/blitz-vibey-script/tests/dom.rs:580-703)
- dom.rs tests `fetch()` through a custom ScriptFetcher and the window error event path (packages/blitz-vibey-script/tests/dom.rs:705-786)
- the selection test uses an explicit 800×600 viewport at scale 1.0 (packages/blitz-vibey-script/tests/dom.rs:540-548)

### facts-s11.md:225

- `keyed_nodes_do_not_crash` runs a real `VirtualDom` against a `DioxusDocument` through 100 inserts, then checks that `<main>` has 100 children (packages/dioxus-native-dom/src/dioxus_document.rs:394-410)

### facts-s12.md:219

- All tests are Cargo integration tests in `tests/blitz-tests/tests/` with `#[test]` functions, depending only on dev-dependencies (tests/blitz-tests/lib.rs:1; tests/blitz-tests/Cargo.toml:13)
- Layout assertions read `final_layout()` location/size, `scrollable_overflow_rect`, `scroll_width()`/`scroll_height()` (tests/blitz-tests/tests/display_contents.rs:76-86; tests/blitz-tests/tests/inline_box_scrollable_overflow.rs:32-40)
- Style assertions read computed values via `primary_styles()` (tests/blitz-tests/tests/style_property_invalidation.rs:45-56)
- Restyle-avoidance is asserted by comparing computed-style pointers before and after (tests/blitz-tests/tests/resize_restyle.rs:39-44; tests/blitz-tests/tests/resize_restyle.rs:77-94)
- Leak checks compare `doc.tree().len()` and anonymous-block counts across repeated operations (tests/blitz-tests/tests/anonymous_block_leak.rs:23-76; tests/blitz-tests/tests/inner_html_leak.rs:11-43)

### facts-s13.md:226

- A unit test evaluates `TESTDRIVER_VENDOR_JS` in a real `ScriptDocument` and asserts the `unsupported_feature` message it emits (wpt/runner/src/test_runners/harness_test.rs:294-310)

## §E2E Test Strategy

### facts-s01.md:225

- WPT reftests are run with a Thai font installed because some reftests depend on Thai glyph widths (.github/workflows/wpt.yml:43-50)
- Successful PR WPT runs have their results posted into the PR description (.github/workflows/wpt-post-results.yml:43-49)

### facts-s02.md:183

- the iframe fixtures carry click-through instructions and expected outcomes instead of assertions (examples/assets/iframe_navigation.html:33-36; examples/assets/iframe_navigation.html:42-45; examples/assets/iframe_navigation.html:67-70)
- observed absent — automated assertions · searched: `\bassert|expect\(` over the 21 s02 files

### facts-s03.md:224

- `rowspan.html` is a minimal reproduction page stating its expected rendering in text (examples/rowspan.html:15-16)
- `paint_bench` is a manual benchmark of paint and rasterize phases (examples/paint_bench.rs:1-7)
- `screenshot` renders a page to a PNG file for inspection (examples/screenshot.rs:107-143)

### facts-s04.md:344

- observed absent — end-to-end or UI-driving tests · searched: `#\[(tokio::)?test\]` over the 86 slice files; no test launches a window or document

### facts-s05.md:254

- out of slice — end-to-end tests

### facts-s06.md:247

- out of slice — no end-to-end tests are in the s06 files

### facts-s07.md:171

- out of slice — no end-to-end tests are in this slice

### facts-s08.md:223

- out of slice — no end-to-end tests are in the listed files

### facts-s09.md:216

- a paint comment states never-scrolled containers paint no scrollbar thumbs, keeping them out of static reftest screenshots (packages/blitz-paint/src/render.rs:715-720)
- paint comments reference WPT cases by path (packages/blitz-paint/src/render/background.rs:370-373; packages/blitz-paint/src/render/background.rs:590)

### facts-s10.md:261

- preact.rs loads `examples/preact/index.html` with a `file:` base URL and drives add, toggle, filter, destroy and clear-completed flows (packages/blitz-vibey-script/tests/preact.rs:12-34; packages/blitz-vibey-script/tests/preact.rs:127-199)

### facts-s11.md:228

- out of slice — no end-to-end tests in these files

### facts-s12.md:226

- `harness_smoke.rs` is described as end-to-end smoke tests for the harness covering document construction, inspection and input synthesis for `HtmlDocument` and `DioxusDocument` (tests/blitz-tests/tests/harness_smoke.rs:1-3)
- A Dioxus counter component is clicked through the harness and its rendered text asserted (tests/blitz-tests/tests/harness_smoke.rs:89-113)
- out of slice — windowed or browser-driven end-to-end runs

### facts-s13.md:229

- out of slice — no end-to-end test of the runner itself is shown

## §Test Data & Fixtures

### facts-s01.md:229

- WPT tests are cloned at the commit in `./wpt/WPT_COMMIT` (.github/workflows/wpt.yml:51-52)
- The script test uses an inline `ENTRIES` list covering changed, added and removed entries (.github/scripts/test_wpt_diff_to_pr.py:8-48)
- examples/assets holds static HTML pages for rendering CSS features and reduced real-world pages (examples/assets/clip-path.html:193; examples/assets/bbc_reduced.html:6; examples/assets/bottom_only.html:6)

### facts-s02.md:187

- page snapshots name their origins: google.com, graphite.art and servo.org (examples/assets/google.html:6; examples/assets/graphite.html:7; examples/assets/servo-new.html:18)
- fixtures reference sibling images square.png, wide.png, tall.png and gosub-logo.svg, which are not s02 files (examples/assets/object_fit.html:13-15; examples/assets/noscript.html:6; examples/assets/iframe_page_b.html:18; examples/assets/gosub.html:101)
- several fixtures load images and styles from remote hosts at render time (examples/assets/newservo.html:4; examples/assets/newservo.html:8; examples/assets/servo-new-reduced.html:16; examples/assets/graphite_software_overview.html:1641)
- the iframe page A fixture links to does_not_exist.html as a deliberate broken link (examples/assets/iframe_page_a.html:25)

### facts-s03.md:229

- servo.html is a snapshot of the servo.org home page referencing a local `servo.css` (examples/assets/servo.html:7; examples/assets/servo.html:25; examples/assets/servo.html:332)
- servo_header_reduced.html reduces the navbar with `<base href="https://servo.org" />` (examples/assets/servo_header_reduced.html:5-18)
- servo_reduced.html reduces a flex wrapping case (examples/assets/servo_reduced.html:4-8)
- Visual fixtures: text-decoration.html, shadow.html, svg.html, svg_size.html (examples/assets/text-decoration.html:150; examples/assets/shadow.html:118; examples/assets/svg.html:3-13; examples/assets/svg_size.html:5)
- The Preact TodoMVC page and a Core DOM APIs reference page sit in `examples/preact` (examples/preact/index.html:6; examples/preact/core_dom_apis.html:6)

### facts-s04.md:347

- Fixture helpers build entries from URL strings, with titles or fixed timestamps (apps/browser/src/url_suggestions.rs:348-362; apps/browser/persistence/src/lib.rs:328-339; apps/browser/src/browser_history.rs:170-176)
- Test URLs use the `.test` TLD (apps/browser/src/browser_history.rs:181; apps/browser/persistence/src/lib.rs:377-379)
- A 1x1 PNG is encoded in-test for favicon checks (apps/browser/src/favicon.rs:75-81)

### facts-s05.md:257

- Style fixtures are injected as inline user-agent stylesheets (packages/blitz-dom/src/document.rs:2983-2985; packages/blitz-dom/src/document.rs:3274-3277)
- Test viewports are fixed sizes (800x600 and 400x300, scale 1.0, Light) (packages/blitz-dom/src/mutator.rs:1673; packages/blitz-dom/src/document.rs:2893)
- The embedded bullet font is used as a valid font payload (packages/blitz-dom/src/document.rs:3366-3381)

### facts-s06.md:250

- SVG test inputs are inline byte-string literals (packages/blitz-dom/src/util.rs:186; packages/blitz-dom/src/util.rs:193)
- Test helpers `pairs` and `feature_settings` build and flatten feature lists (packages/blitz-dom/src/stylo_to_parley.rs:530-548)

### facts-s07.md:174

- Unit tests build `ListStyleType` values inline with a `list_style` helper (packages/blitz-dom/src/layout/list.rs:196-198)

### facts-s08.md:226

- Test inputs are inline string literals, including generated multi-line text (packages/blitz-html/src/html_sink.rs:319; packages/blitz-dom/src/node/element.rs:986; packages/blitz-dom/src/node/element.rs:1019-1022)
- observed absent — fixture files or loaders · searched: `fixture` over the 16 listed files (no match)

### facts-s09.md:220

- HarnessOptions carries width, height, scale, color_scheme, an optional base_url and an optional net_provider for sub-resources (packages/blitz-test-harness/src/harness.rs:11-21)
- harness documents always use HtmlProvider as the HTML parser provider (packages/blitz-test-harness/src/harness.rs:36-51)

### facts-s10.md:264

- test HTML is inline raw strings per test (packages/blitz-vibey-script/tests/dom.rs:25-36)
- the fetch test uses base URL `http://example.test/dir/page.html` and a fixed JSON body for `/data.json` (packages/blitz-vibey-script/tests/dom.rs:710-720; packages/blitz-vibey-script/tests/dom.rs:740-742)
- the E2E fixture is the vendored example at `../../examples/preact` relative to the crate manifest (packages/blitz-vibey-script/tests/preact.rs:12-14)

### facts-s11.md:231

- `finger_event(id, x, y)` builds a finger `BlitzPointerEvent` fixture (packages/dioxus-native-dom/src/events.rs:663-682)
- The keyed-nodes test uses a shared `Rc<RefCell<HashMap<usize, usize>>>` as app props (packages/dioxus-native-dom/src/dioxus_document.rs:379-394)

### facts-s12.md:231

- Fixtures are inline HTML string constants or `format!`-built pages (tests/blitz-tests/tests/incremental_oracle.rs:371-420; tests/blitz-tests/tests/dir_attribute.rs:12-29)
- Image load results are injected directly into `background_images` layers as `ImageData::Raster` or `ImageData::Svg` with `Status::Ok` (tests/blitz-tests/tests/background_size.rs:31-43; tests/blitz-tests/tests/svg_background_size.rs:37-46)
- Benchmark pages are generated in code: ~2000-node realistic, 5000-item stress, 50 nested stacking contexts, ~40k-node large page (tests/blitz-tests/tests/paint_tree_bench.rs:59-98; tests/blitz-tests/tests/paint_tree_bench.rs:208-221)
- An external page can be supplied through `PAINT_TREE_BENCH_HTML`, with stylesheets inlined (tests/blitz-tests/tests/paint_tree_bench.rs:258-269)

### facts-s13.md:232

- Test inputs come from the WPT checkout at `WPT_DIR` (wpt/runner/src/main.rs:463-470)
- The quarantine list is `timeout-quarantine.txt`, two directories above `src/test_runners`, embedded at compile time (wpt/runner/src/test_runners/mod.rs:32)
- Fuzzy unit tests use inline HTML and byte-array fixtures (wpt/runner/src/test_runners/fuzzy.rs:184-188; wpt/runner/src/test_runners/fuzzy.rs:231-232)

## §Mocking & Stubbing Discipline

### facts-s01.md:234

- The script's `--dry-run` prints instead of calling the GitHub API, and tests call `render` with `run_url=None` (.github/scripts/wpt_diff_to_pr.py:204-206; .github/scripts/test_wpt_diff_to_pr.py:67)
- observed absent — mocking libraries · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

### facts-s02.md:193

- out of slice — no mock or stub is in the slice

### facts-s03.md:236

- observed absent — mocks or stubs · searched: `mock|Mock|stub|fake` over the 32 slice files

### facts-s04.md:352

- Worker tests inject a capturing `publish` closure instead of a Dioxus signal (apps/browser/src/url_suggestions.rs:374-378)
- The synchronous nucleo driver uses a no-op notify closure (apps/browser/src/url_suggestions.rs:398)
- observed absent — mocking libraries · searched: `mockall|mock` (case-insensitive) over the 86 slice files

### facts-s05.md:262

- Hand-written `ShellProvider` fakes count redraw requests (packages/blitz-dom/src/mutator.rs:1540-1549; packages/blitz-dom/src/document.rs:2849-2857)
- Tests needing real font metrics skip with an `eprintln!` when text measures 0x0 without `system-fonts` (packages/blitz-dom/src/document.rs:2918-2936)

### facts-s06.md:254

- observed absent — mocks or fixtures · searched: `mock|fixture` over the 17 listed s06 files

### facts-s07.md:177

- observed absent — mocks or fixtures · searched: `mock|fixture` over the 8 slice files

### facts-s08.md:230

- observed absent — mocks, fakes or stubs · searched: `mock|fake|stub` over the 16 listed files (no match)

### facts-s09.md:224

- dispatch_recorded installs a RecordingHandler EventHandler that records each dispatched DOM event name (packages/blitz-test-harness/src/harness.rs:144-169)
- observed absent — mock or stub types · searched: `mock|Mock|stub|fake` over the 32 listed s09 files

### facts-s10.md:269

- a test-local `MapFetcher` implements `ScriptFetcher`, returning a NotFound IO error for unknown paths (packages/blitz-vibey-script/tests/dom.rs:710-720; packages/blitz-vibey-script/tests/dom.rs:745)
- no-op provider implementations exist for net, navigation and shell (packages/blitz-traits/src/net.rs:165-173; packages/blitz-traits/src/navigation.rs:14-20; packages/blitz-traits/src/shell.rs:65-66)
- debug_timer swaps in a zero-cost dummy timer when `enable` is off (packages/debug_timer/src/lib.rs:70-82; packages/debug_timer/src/lib.rs:109-123)

### facts-s11.md:235

- observed absent — mocks, stubs or fakes · searched: `mock|stub|fake` over the 21 listed s11 files

### facts-s12.md:237

- `RecordingNetProvider` records requested URLs instead of fetching (tests/blitz-tests/tests/link_rel_attribute.rs:14-24)
- `ManualNetProvider` holds requests and handlers so the test delivers responses when it chooses (tests/blitz-tests/tests/render_blocking_stylesheet.rs:16-30; tests/blitz-tests/tests/render_blocking_stylesheet.rs:62-66)
- `RecordingShell` implements `ShellProvider` and records `set_ime_enabled` calls (tests/blitz-tests/tests/interaction_state_teardown.rs:27-35)
- `NoopEventHandler` is passed to `EventDriver` (tests/blitz-tests/tests/scrollbar_drag.rs:3; tests/blitz-tests/tests/scrollbar_drag.rs:35)
- `Probe`/`SizedProbe` stub widgets implement `Widget` (tests/blitz-tests/tests/custom_widget_layout.rs:12-24)
- `NullScenePainter` stands in for a renderer in timings (tests/blitz-tests/tests/paint_tree_bench.rs:162)

### facts-s13.md:237

- WPT's stock `testharnessreport.js` and `testdriver-vendor.js` are replaced by runner-provided versions (wpt/runner/src/test_runners/harness_test.rs:18-81; wpt/runner/src/test_runners/harness_test.rs:99-104)
- Network is replaced by `WptNetProvider`, which serves files from the local WPT directory (wpt/runner/src/net_provider.rs:16-27; wpt/runner/src/net_provider.rs:70-78)
- Navigation uses `DummyNavigationProvider` and base URL `http://dummy.local` (wpt/runner/src/main.rs:568-569)
- Timers run on virtual time without the background timer thread (wpt/runner/src/test_runners/mod.rs:121-128)

## §CI Integration

### facts-s01.md:238

- Tests run in GitHub Actions on PRs and pushes to main/v0.* (.github/workflows/ci.yml:3-8)
- WPT runs on PRs and pushes to main, publishes a step summary on PRs, and uploads the diff artifact (.github/workflows/wpt.yml:3-7; .github/workflows/wpt.yml:81-93)
- observed absent — coverage tooling · searched: `coverage|tarpaulin|llvm-cov|codecov` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

### facts-s02.md:196

- out of slice — no CI configuration is in the slice

### facts-s03.md:239

- out of slice — CI configuration

### facts-s04.md:357

- A test comment states the worker timeout exists so a regression would not hang CI (apps/browser/src/url_suggestions.rs:380-382)
- out of slice — CI workflow files

### facts-s05.md:266

- out of slice — CI configuration

### facts-s06.md:257

- out of slice — no CI configuration is in the s06 files

### facts-s07.md:180

- out of slice — no CI configuration is in this slice

### facts-s08.md:233

- out of slice — no CI configuration is in the listed files

### facts-s09.md:228

- out of slice — no CI configuration is among these files

### facts-s10.md:274

- out of slice — no CI configuration is among these files

### facts-s11.md:238

- out of slice — no CI configuration in these files

### facts-s12.md:245

- Benchmark tests are `#[ignore]` and run with `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture` (tests/blitz-tests/tests/paint_tree_bench.rs:3; tests/blitz-tests/tests/paint_tree_bench.rs:225-227; tests/blitz-tests/tests/paint_tree_bench.rs:262-263; tests/blitz-tests/tests/paint_tree_bench.rs:341-342)
- Font-dependent assertions rely on the `system-fonts` feature, stated to be on by default when testing the whole workspace (tests/blitz-tests/tests/br_trailing_line.rs:11-13)
- out of slice — CI workflow files

### facts-s13.md:243

- A comment states heavy interpolation suites run longer "on loaded CI machines", motivating the harness timeout multiplier (wpt/runner/src/test_runners/harness_test.rs:30-34)
- out of slice — CI workflow configuration

## §Quality Gates & Coverage Targets

(no fact block)

## §Test Anti-Patterns

(no fact block)

## §Test Decisions Log

(no fact block)
