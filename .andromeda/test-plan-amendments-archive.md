# test-plan — archived amendment originals

Writer = wrap P7 and registry.py migrate --apply · read by NO loop skill · cold history, never cited for current truth.

## Registry migration (U35) — 2026-10-05

<!-- U35 · test-plan.md · ## 3. Test Harness Contract · sha256 056247b9235428b12f583116fe7d6092b0390d81f8a1af3dcb1bbd0b4e6a33ee -->

## 3. Test Harness Contract

**Runners and invocation:**

- **WPT in CI:** invoked as `cargo build -rp wpt` then `cargo run -rp wpt css svg`, producing `./wpt/output/wptreport.json` (.github/workflows/wpt.yml:53-58); the `wpt` cli provides `calc-scores` and `diff --format json` (.github/workflows/wpt.yml:69-80)
- **WPT runner CLI:** `WPT_DIR` env var, optional suite arguments, `--verbose`/`-v`, `--run-quarantined`, `--list` (wpt/runner/src/main.rs:240-250; wpt/runner/src/main.rs:461-479)
- **WPT outputs:** `wpt_expectations.txt` and `wptreport.json` in the output directory (wpt/runner/src/main.rs:832-847)
- **WPT status set:** PASS, FAIL, TIMEOUT, SKIP, CRASH (wpt/runner/src/main.rs:104-122)
- **Python script tests:** the test file states it runs with `python3 -m unittest discover .github/scripts` (.github/scripts/test_wpt_diff_to_pr.py:2)
- **Rust tests:** the standard built-in harness — `#[cfg(test)]` modules with `#[test]` functions and `assert!`/`assert_eq!` (packages/blitz-dom/src/net.rs:606-618; packages/blitz-dom/src/stylo_to_parley.rs:550-564; packages/blitz-dom/src/util.rs:191-200; packages/blitz-dom/src/layout/list.rs:185-204; packages/blitz-dom/src/node/scrollbar.rs:228-236; packages/blitz-dom/src/node/node.rs:1754-1761), with `#[tokio::test]` for async worker tests (apps/browser/src/url_suggestions.rs:415; apps/browser/src/url_suggestions.rs:551)
- **Feature gating:** SVG tests are gated on both `test` and the `svg` feature (packages/blitz-dom/src/util.rs:180)
- **Third-party test frameworks:** observed absent · searched: `proptest|insta::|criterion|mock` over the 15 files of slice s05

**blitz-test-harness (`Harness`):**

- **Construction:** from_html, from_html_with, from_component, from_vdom, wrap; constructors pump once, wrap does not (packages/blitz-test-harness/src/harness.rs:59-92). `Harness::from_html(html)` and `Harness::from_html_with(html, HarnessOptions)` build an HTML-backed harness; `Harness::from_component(fn)` and `Harness::from_vdom(vdom, options)` build Dioxus-backed ones (tests/blitz-tests/tests/harness_smoke.rs:10; tests/blitz-tests/tests/pointer_events.rs:8-17; tests/blitz-tests/tests/harness_smoke.rs:104; tests/blitz-tests/tests/stale_node_mapping.rs:49-56)
- **Core:** into_inner, base, base_mut, time, pump, tick, dispatch, dispatch_recorded, set_viewport_size (packages/blitz-test-harness/src/harness.rs:94-184)
- **Pump semantics:** pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded do not pump (packages/blitz-test-harness/src/harness.rs:113-138); `pump()` applies pending changes after a mutation through `base_mut().mutate()` (tests/blitz-tests/tests/dir_attribute.rs:72-79; tests/blitz-tests/tests/oof_dynamic_cb.rs:37-43)
- **Recorded dispatch:** `dispatch_recorded([UiEvent, ...])` returns the list of dispatched event names (tests/blitz-tests/tests/touch_events.rs:40-44)
- **Input helpers:** click, click_at, mouse_down_at, mouse_up_at, move_mouse_to, drag, tap, tap_at, touch_down, touch_move, touch_up, wheel_at, press, press_with, type_text, ime — each pumps after dispatch (packages/blitz-test-harness/src/input.rs:95-232); synthesized pointer events set page, screen and client coordinates to the same values (packages/blitz-test-harness/src/input.rs:18-27); key_event uses Code::Unidentified and Location::Standard and fills text only for pressed character keys (packages/blitz-test-harness/src/input.rs:75-93); the crate exports a `pointer_event(id, x, y, button, buttons, mods)` builder (tests/blitz-tests/tests/touch_events.rs:6; tests/blitz-tests/tests/touch_events.rs:12-21)
- **Inspection helpers:** query, node, query_all, layout_rect, layout_rect_of, center_of, text_content, attr, hit, hit_node, focused, hovered, dom_string (packages/blitz-test-harness/src/inspect.rs:26-131); `layout_rect(selector)` returns a rect with `x`, `y`, `width`, `height`; `center_of` returns an `(x, y)` tuple (tests/blitz-tests/tests/harness_smoke.rs:16-21)

**Crate-local test helpers:**

- **apps/browser:** `drive_worker` queues messages, drops the sender, captures publications, and bounds the run with a 2-second timeout (apps/browser/src/url_suggestions.rs:364-388); `make_conn` opens an in-memory sqlite connection and migrates it to latest (apps/browser/persistence/src/lib.rs:322-326)
- **blitz-dom node tests:** DOM tests construct `BaseDocument::new(DocumentConfig::default())` and create nodes with `create_node` (packages/blitz-dom/src/node/node.rs:1743-1752); text-input tests build a `TextInputData` laid out at scale 1.0 with fresh parley `FontContext`/`LayoutContext` (packages/blitz-dom/src/node/element.rs:963-974)
- **blitz-vibey-script:** `doc_from_html` constructs with the default `DocumentConfig` and calls `execute_scripts`; `text_of_selector` reads text content (packages/blitz-vibey-script/tests/dom.rs:8-21); preact helpers: load_todomvc, resolve, query, query_all, text_of, enter_key, click, add_todo (packages/blitz-vibey-script/tests/preact.rs:16-100); virtual time and `without_timer_thread` are intended for embedders driving timers manually, e.g. test runners (packages/blitz-vibey-script/src/clock.rs:10-14; packages/blitz-vibey-script/src/document.rs:108-133); dev-dependency blitz-dom enables `system-fonts` so text inputs shape real text in the selection tests (packages/blitz-vibey-script/Cargo.toml:42-48)
- **dioxus-native-dom:** the tests use the `dioxus` crate as a dev-dependency (packages/dioxus-native-dom/Cargo.toml:43-44; packages/dioxus-native-dom/src/dioxus_document.rs:370); the document test builds a `DioxusDocument` with `DocumentConfig::default()`, calls `initial_build`, and drives updates with `mark_dirty` and `poll(None)` (packages/dioxus-native-dom/src/dioxus_document.rs:394-404)

> NOT YET MEASURED — the reading reached the test runners and the in-process harness only; no product boot, status, cleanup or logs command, status endpoint shape, log format, PID file or test-data bootstrap mechanism of a 5-command contract was gathered

### Bootstrap phases (derive for route / setup-project)

- **coverage-tooling-install:** coverage tooling is recorded absent — see `## 9. CI Integration`.

---
