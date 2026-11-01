# Cascade compare — test-plan · wrap · 2026-11-01-upstream-sync-ui-graph
E items 62 · held 26 · lost at classify 8 · lost at strong 4 · beside a placed fact 2 · cascade only 3 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### ci-integration#2.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "- **Legs reaching `key_press_after_container_lost_its_box`:** the blitz-tests target `key_press_after_container_lost_its_box` runs in the workspace `test` leg and in `agent-run.sh`'s `run all`, each taking it as one more target with no line of its script naming it; its 3 tests run in both layout modes; it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it, and it is none of the `a11y` leg's fourteen files (§9 Legs above) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "§9: new record of the legs that reach key_press_after_container_lost_its_box, in the form of the pseudo_element_flow_classification record."
    rationale: "The new check holds no stand_ prefix, so run all alone moves (report.md:27); it joins no named leg, the seventeen unmoved (report.md:60), and the workspace tally counts its result line (report.md:36) while a11y and repeat stay unmoved (report.md:36)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27

### integration-test-strategy#3.1 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in a `<!-- detail: procedure · integration -->` group of its own: «- **Script ↔ DOM, `URLSearchParams`:** `tests/url_search_params.rs` (4), a target of its own by file, runs a page's script over the global class and `URL.prototype.searchParams`: `construction_and_form_encoding`, `mutations_and_live_iteration`, `invalid_inputs_and_receivers` and `url_search_params_are_bidirectionally_live`, `href` and `search` live both ways (packages/blitz-vibey-script/tests/url_search_params.rs:15-95; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»."
    sidecar: "integration-test-strategy: new record for blitz-vibey-script's `url_search_params` target, 4 tests."
    rationale: "report.md:29 names the new target; report.md:17 the surface; report.md:102 tests integ `url_search_params` 4; report.md:346-356 the file and its four tests. Binds with arch §Existing Scopes, report.md:95."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102

### integration-test-strategy#1.1 · lost at classify · beside a placed fact F8 (near 14, integration-test-strategy#2.1) · .andromeda/test-plan-detail/integration-test-strategy.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 10
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in a `<!-- detail: procedure · integration -->` group of its own: «- **DOM ↔ resource loading, a failed stylesheet:** `render_blocking_stylesheet` gains 3 on `HtmlDocument`: `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` and `invalid_stylesheet_url_does_not_block_rendering`, a failed, empty or non-CSS sheet ceasing to block render and a bad `href` starting no load; on a harness or driver document the forward of `NetHandler::error` through `CountedHandler` carries it, §4 its unit test, so a failed load ends the wait where it read `NotSettled(Loads)` for good (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»."
    sidecar: "integration-test-strategy: new record for `render_blocking_stylesheet`'s 3 failed-stylesheet tests and the failed load ending the wait."
    rationale: "report.md:14 the failed-stylesheet behaviour; report.md:25 the forward and a failed load ending the wait; report.md:37 `render_blocking_stylesheet` +3; report.md:103 the tests of this path; report.md:381-390 the three tests. Binds with test-plan §4 and arch §Standard Contracts, report.md:49."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:103

### integration-test-strategy#11.1 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:80
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A test painting a blended element to a frame, or naming `convert_blend_mode` · searched: `git grep` over `tests/`, only `contrast.rs`'s fixture; `mix-blend-mode` and `isolation` paint on every document since 2026-11-01-upstream-sync-ui-graph, their tests `layers.rs`' 3 unit tests, its owner named at that wrap's P5»."
    sidecar: "integration-test-strategy: blending painted with no frame test recorded as an absence, its owner at P5."
    rationale: "report.md:15 blend and isolation now paint; report.md:104 the ✗, no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; report.md:172 the ✗ takes an owner at P5, inputs#I8."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104

### integration-test-strategy#11.1 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:80
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A frame test of `render_element`'s skip of a box with `Node::is_inline_with_only_block_children`, its own shadow, background and border: none, its flag read by `document.rs`'s 4 unit tests alone, as measured at 2026-11-01-upstream-sync-ui-graph»."
    sidecar: "integration-test-strategy: the inline-decorations paint skip recorded with no frame test."
    rationale: "report.md:19 the public reader and the paint skip; report.md:106 tests unit `document.rs` 4, `inline_with_only_block_children_tests`, frame ✗."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106

### integration-test-strategy#11.1 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:80
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A test of `flush_queued_mounted_events` skipping an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239): none, upstream's `element_removed_before_its_mounted_event_is_flushed` left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1`, a `CARRY:` on «Upstream sync ahead of determinism and exploration»»."
    sidecar: "integration-test-strategy: the mounted-event guard recorded with no test, upstream's test left out and carried."
    rationale: "report.md:20 the guard and the test left out; report.md:36 `6ef4b532d` adding 0 passed; report.md:80 the test's return and its owner."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20

### integration-test-strategy#11.1 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:80
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A test of their own for upstream's `8b1a0a3ce` (`table_needs_rebuild`), `7a3dd8918` (`init_stylo_prefs`), `ae774d7f4` (the resolved `width` and `height`) and `e42ee1f25` (culling bounds): none, the workspace their witness, `incremental_oracle` 7 unmoved, as measured at 2026-11-01-upstream-sync-ui-graph»."
    sidecar: "integration-test-strategy: four taken upstream commits recorded with no test of their own, the workspace their witness."
    rationale: "report.md:109 names the four and their witness; report.md:12, report.md:16, report.md:21 and report.md:22 the paths each adds."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:109

### mocking-stubbing-discipline#1.10 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 19
    section: "## 8. Mocking & Stubbing Discipline"
    change: Add this table row after the settle unit tests' row, | Net handler (blitz-net unit tests) | since 2026-11-01-upstream-sync-ui-graph a test-local `Handler` implements `NetHandler` in `blitz-net`'s `#[cfg(test)] mod tests`, beside the module's `fetch` helper returning `Result<Bytes, String>`, over three tests (`delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors`, `http_error_body_is_not_delivered_as_css`); the network itself is no stand-in in the two that bind, each a `std::net::TcpListener` on `127.0.0.1:0`, served by no binary and reached by no caller | (packages/blitz-net/src/lib.rs:471-486; packages/blitz-net/src/lib.rs:488-511; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534) |
    sidecar: test-plan §8 gains a row for blitz-net's test-local `Handler`, the `NetHandler` stand-in of its three new unit tests, two of which bind real loopback listeners.
    rationale: The report adds `struct Handler`, `impl NetHandler for Handler` and `fetch` in blitz-net's new test module (report.md:283-289) and names its two test-only binds (report.md:28), and §8's table lists each hand-written stand-in by concern yet holds no row for this one; report.md:48 keeps rows 15-16 unchanged, so the row is an addition and edits neither.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:283

### test-data-fixtures#3.4 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 25
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Lost-box control pages (`format!`-built, in their check's own file) | `key_press_after_container_lost_its_box`'s `in_anonymous_block(control, sheet)` and `in_no_anonymous_block(control)`, with its `SELECT` constant and the enum `Lost` of the three ways a control loses its box (`hidden`, an inline `display:none`, CSS alone), each page booted as a `Harness<HtmlDocument>` by `boot(page, incremental)` in both layout modes | (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:17-18; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:21-28; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:32-39; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:41-48; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:50-58) |"
    sidecar: "§7 data seeds: new row for the code-built pages of `key_press_after_container_lost_its_box` (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:27 adds the check, report.md:23 names the three ways, and report.md:359-380 lists its page builders and boot; each page is test data this table holds a row for by kind and no row names them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:359

### test-data-fixtures#3.8 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 29
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Inline HTML (`URLSearchParams` script tests) | `check(script)` in blitz-vibey-script's `url_search_params` target builds a page with `format!` around each of its 4 tests' scripts | (packages/blitz-vibey-script/tests/url_search_params.rs:4-13) |"
    sidecar: "§7 data seeds: new row for the `format!`-built page of blitz-vibey-script's `url_search_params` target (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:29 adds the target, report.md:102 counts its 4 tests, and report.md:346-356 lists `check`; the standing script-test row names only dom.rs's raw strings and file-load page."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:347

### test-data-fixtures#3.14 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:35
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 35
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Restyle trees (mutator unit tests) | trees built in code through `DocumentMutator` by `div`, `parent_with_children`, `hidden_parent_with_children`, `focus_tree` and `parent_with_text_and_children`, beside the inline sheet `PSEUDO_CSS`, each result held against a fresh document by `assert_matches_fresh` | (packages/blitz-dom/src/mutator.rs:2189-2192; packages/blitz-dom/src/mutator.rs:2218-2242; packages/blitz-dom/src/mutator.rs:2304-2311; packages/blitz-dom/src/mutator.rs:2313-2318; packages/blitz-dom/src/mutator.rs:2664-2665; packages/blitz-dom/src/mutator.rs:2764-2776; packages/blitz-dom/src/mutator.rs:2800-2812) |"
    sidecar: "§7 data seeds: new row for the restyle trees and `PSEUDO_CSS` of `mutator.rs`'s new unit tests (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:36 counts `fe33b778b`'s 39 tests and report.md:211-266 lists their tree builders, the sheet and the comparison; no row of this table holds them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:221

### test-data-fixtures#3.24 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 45
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Stylesheet answers (`render_blocking_stylesheet`) | three cases stated in the check: a failed load, answered bodies looped over empty bytes, an HTML document and a lone `0xFF` byte, and a bad `href`, which starts no load | (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107) |"
    sidecar: "§7 data seeds: new row for the failed, empty, non-CSS and bad-`href` stylesheet cases of `render_blocking_stylesheet` (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:14 states the behaviour the cases read and report.md:381-390 lists the three new tests with the byte strings; the standing network rows hold only the fixed fetch body and `LoadNet`'s bytes."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:388

### test-data-fixtures#3.24 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 45
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Loopback listeners (blitz-net unit tests) | no fixture file: in `#[cfg(test)] mod tests`, `fetch(url)` returns a `Result<Bytes, String>` through the module's `NetHandler` `Handler`, and `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css` each call `std::net::TcpListener::bind` on `127.0.0.1:0`, test-only TCP on loopback, served by no binary and reached by no caller (arch §Occupied Resources) | (packages/blitz-net/src/lib.rs:471-486; packages/blitz-net/src/lib.rs:488-511; packages/blitz-net/src/lib.rs:522-530; packages/blitz-net/src/lib.rs:532-545) |"
    sidecar: "§7 data seeds: new row for blitz-net's two test-only loopback listeners and its `fetch` helper (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:28 states the two test-only binds and owes their registration as test-only, and report.md:281-289 lists the module; this table holds network test data and no row names them. Relation: arch §Occupied Resources."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28

### test-scope-summary#12.1 · lost at classify · beside a placed fact F3 (near 88, test-scope-summary#12.1) · .andromeda/test-plan-detail/test-scope-summary.md:88
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 88
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-net:** 3 inline tests in its `#[cfg(test)] mod tests`, upstream's: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, the last two each binding `std::net::TcpListener` on `127.0.0.1:0`, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:465-546; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: blitz-net's first 3 inline tests and their two test-only loopback binds."
    rationale: "f6edb9701's tests in blitz-net (report.md:28, report.md:103, report.md:283-289); this file holds no blitz-net record, and the body's says none exist (see note)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28

### test-scope-summary#23.1 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:204
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 204
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **tests/blitz-tests — `render_blocking_stylesheet` on a stylesheet that fails:** upstream's 3 more tests, `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` over an empty, a non-CSS and a one-byte body, and `invalid_stylesheet_url_does_not_block_rendering`, a bad `href` starting no load (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: render_blocking_stylesheet's 3 failed-stylesheet tests."
    rationale: "f6edb9701 and `run all`'s +3 (report.md:14, report.md:37, report.md:103), listed at report.md:383-390; this file names the target nowhere."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:37

### test-strategy#5.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:34
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 34
    section: "## 2. Test Strategy"
    change: Append after the line's closing citation «; since 2026-11-01-upstream-sync-ui-graph `mix-blend-mode` and `isolation` paint with no frame check — `layers.rs`'s 3 unit tests read the layer manager, and no test paints a blended element to a frame or names `convert_blend_mode` — and the paint skip of an inline box holding only block children is read by `document.rs`'s 4 unit tests of `is_inline_with_only_block_children`, on no frame, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md».
    sidecar: Test Strategy, Pixel tests — states that blending, isolation and the inline paint skip hold no frame check; the blending gap takes its owner at P5 (inputs#I8).
    rationale: Paint paths this chunk adds carry no test at the pixel level §2 describes, by the report's Coverage of new surfaces rows for `mix-blend-mode` and `is_inline_with_only_block_children`, each marked frame ✗; Addendum 1 gives the blending gap an owner at P5, so the file states the gap and names no owner.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104

### unit-test-strategy#14.1 · lost at classify · beside a placed fact F12 (near 8, unit-test-strategy#1.1) · .andromeda/test-plan-detail/unit-test-strategy.md:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (mutator, restyle narrowing):** since 2026-11-01-upstream-sync-ui-graph, upstream's `fe33b778b`, `mutator.rs` holds 39 tests of `restyle_for_attribute_change` and `restyle_for_child_list_change`, read through `check_incremental_restyle` and `assert_matches_fresh`, an incremental snapshot against a fresh one: changes in a hidden subtree then unhidden, class, id, attribute and style changes, child insertion, removal and moves, `nth-child` and sibling rules, pseudo-elements and `focus-within` (packages/blitz-dom/src/mutator.rs:2326-2887)`."
    sidecar: "unit-test-strategy: new item, mutator.rs's 39 restyle-narrowing tests."
    rationale: "The report's `ws-tally` counts `fe33b778b` at 39 and New text lists them at mutator.rs; the category holds what unit tests cover, per module, and has no row for them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:36

### unit-test-strategy#14.1 · lost at classify · beside a placed fact F10 (near 82, unit-test-strategy#11.1) · .andromeda/test-plan-detail/unit-test-strategy.md:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (mutator, a custom widget's attributes):** since 2026-11-01-upstream-sync-ui-graph, upstream's `a26c6ddeb`, `custom_widget_replays_initial_attributes` pins `ElementData::set_custom_widget` replaying each existing attribute to `Widget::attribute_changed` (packages/blitz-dom/src/mutator.rs:1656-1727)`."
    sidecar: "unit-test-strategy: new item, the custom widget's attribute replay test."
    rationale: "The report's `ws-tally` counts `a26c6ddeb` at 1, New text lists the test, and Symbols states the replay."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:213

### unit-test-strategy#14.1 · lost at classify · beside a placed fact F11 (near 82, unit-test-strategy#11.1) · .andromeda/test-plan-detail/unit-test-strategy.md:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (document, `<picture>`):** since 2026-11-01-upstream-sync-ui-graph, upstream's `dee854c76`, `picture_display_tests` holds one test, `picture_is_not_display_contents` (packages/blitz-dom/src/document.rs:4061-4094)`."
    sidecar: "unit-test-strategy: new item, the picture display test."
    rationale: "The report's Symbols names `picture_display_tests` at document.rs:4061-4094, and `ws-tally` counts `dee854c76` at 1."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:13

### unit-test-strategy#14.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (document, an inline box with only block children):** since 2026-11-01-upstream-sync-ui-graph, upstream's `f76777766`, `inline_with_only_block_children_tests` holds four tests of `Node::is_inline_with_only_block_children`: it is detected, and it follows child edits, text edits and `display: contents` descendant edits (packages/blitz-dom/src/document.rs:4096-4296)`."
    sidecar: "unit-test-strategy: new item, the four inline-with-only-block-children tests."
    rationale: "The report's Coverage row for `is_inline_with_only_block_children` names `document.rs` 4, and New text lists the four tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106

### unit-test-strategy#14.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-net:** since 2026-11-01-upstream-sync-ui-graph, upstream's `f6edb9701`, the crate's inline `#[cfg(test)] mod tests` holds three tests through a `NetHandler`: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`; two of them bind `127.0.0.1:0` by `std::net::TcpListener::bind`, test-only, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:465-546; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534)`."
    sidecar: "unit-test-strategy: new item, blitz-net's first unit tests and their two test-only binds."
    rationale: "The report's Coverage row for `NetHandler::error` names `unit blitz-net 3`, New text lists them, and Symbols states the two test-only binds; the category has no blitz-net row."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:103

### unit-test-strategy#14.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-paint (layers):** since 2026-11-01-upstream-sync-ui-graph, upstream's `6fca6308d`, `layers.rs`'s inline test module holds three tests of `LayerManager`: `blend_layers_are_not_replaced_by_clips`, `normal_layers_clip_unless_isolation_is_forced` and `disabled_or_exhausted_layers_do_not_pop` (packages/blitz-paint/src/layers.rs:104-181)`."
    sidecar: "unit-test-strategy: new item, layers.rs's three blend-layer tests."
    rationale: "The report's Coverage row for `mix-blend-mode` names `unit layers.rs 3`, and New text lists the three tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104

### unit-test-strategy#16.4 · lost at strong · .andromeda/test-plan-detail/unit-test-strategy.md:115
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A test painting a blended element to a frame, or naming `convert_blend_mode` · searched: `git grep` over `tests/`, only `contrast.rs`'s fixture; `mix-blend-mode` and `isolation` paint on every document, `layers.rs`'s three their one unit cover; its owner the P5 card's — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, blending painted with no frame test and convert_blend_mode named by no test."
    rationale: "The report's Coverage row carries the ✗ for this new path on every document, and Addendum 1 (inputs#I8) says it takes an owner at P5; the file's absence group states no such gap."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104

### unit-test-strategy#16.4 · lost at strong · .andromeda/test-plan-detail/unit-test-strategy.md:115
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A frame test of `render_element` skipping the own shadow, background and border of an inline box with only block children (`paints_own_box`) — the flag read by `document.rs`'s four unit tests, the paint skip on no frame — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, the inline box's paint skip read on no frame."
    rationale: "The report's Coverage row for `is_inline_with_only_block_children` reads `frame ✗` for a new paint path; Symbols states the skip."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106

### unit-test-strategy#16.4 · lost at strong · .andromeda/test-plan-detail/unit-test-strategy.md:115
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A test of its own for upstream's `8b1a0a3ce` (`table_needs_rebuild`), `7a3dd8918` (`init_stylo_prefs`), `ae774d7f4` (sub-pixel `width` · `height`) and `e42ee1f25` (culling on the scaled size) · searched: `git show -U0`, evidence/trial-takes.md; the workspace their witness, `incremental_oracle` 7 unmoved — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, four taken upstream commits with no test of their own."
    rationale: "The report's Coverage names these four as carrying no test of their own, `ws-tally` counting each at 0; each adds a code path, the workspace alone their witness."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:109

### unit-test-strategy#16.4 · lost at strong · .andromeda/test-plan-detail/unit-test-strategy.md:115
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A test of `flush_queued_mounted_events` skipping an element removed since its event was queued — upstream's `element_removed_before_its_mounted_event_is_flushed` left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, the mounted-event guard with its upstream test left out."
    rationale: "The report's Symbols states the guard and the test left out, `ws-tally` counts `6ef4b532d` at 0, and inputs#I6's third row carries the test's return."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20

## Held

- ci-integration#1.1
- ci-integration#12.1
- ci-integration#13.1
- ci-integration#5.1
- ci-integration#9.1
- integration-test-strategy#10.1
- integration-test-strategy#9.1
- quality-gates-coverage-targets#2.1
- test-data-fixtures#3.5
- test-scope-summary#1.1
- test-scope-summary#13.1
- test-scope-summary#14.1
- test-scope-summary#16.1
- test-scope-summary#17.3
- test-scope-summary#19.1
- test-scope-summary#2.3
- test-scope-summary#2.4
- test-scope-summary#27.1
- test-scope-summary#29.1
- test-scope-summary#34.1
- test-scope-summary#7.1
- test-strategy#1.1
- test-strategy#6.1
- test-strategy#9.1
- unit-test-strategy#3.1
- unit-test-strategy#7.1

## Cascade only

- integration-test-strategy#2.1
- unit-test-strategy#1.1
- unit-test-strategy#11.1
