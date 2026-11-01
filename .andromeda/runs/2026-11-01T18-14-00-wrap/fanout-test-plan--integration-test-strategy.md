proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 68
    section: "## 5. Integration Test Strategy"
    change: "In the `select_popup` record replace «One row skips its key press: a key pressed after an ancestor gained `hidden` panics at `packages/blitz-dom/src/node/node.rs:1788`, for any control, with no list opened, the test's comment saying why» with «Every row presses its key, one `harness.press` (tests/blitz-tests/tests/select_popup.rs:513), row 3 red before `BaseDocument::deallocate_anonymous_block` cleared its children's `layout_parent` and green since 2026-11-01-upstream-sync-ui-graph»."
    sidecar: "integration-test-strategy: `select_popup`'s row-3 skip and the key-press panic it recorded retired; every row now presses its key."
    rationale: "Report Retired (1) the key-press panic and (4) `select_popup`'s row-3 skip (report.md:33); report.md:26 reads `select_popup.rs:513` as one `harness.press` for every row; report.md:50 marks `integration-test-strategy.md:68` changed; report.md:139 reads row 3 red before."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:50
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 74
    section: "## 5. Integration Test Strategy"
    change: "In the `select_popup_painted` record replace «the header's weight under `blitz-paint/font-embolden` has the local witness §2 Painted select list names» with «the header's weight under `blitz-paint/font-embolden` is read by the `embolden` step of `fast` (§2 Painted select list), `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` (.github/scripts/ci-leg.sh:28-29), no CI job, macOS exercising the feature there»."
    sidecar: "integration-test-strategy: the `font-embolden` witness is now the `embolden` step of `fast`, the no-local-gate gap retired."
    rationale: "Report Retired (3) the no-local-gate gap for `blitz-paint/font-embolden` (report.md:33); report.md:42 marks `integration-test-strategy.md:74` changed, now the `embolden` step of `fast`; report.md:64 gives the leg's command and its placement."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 70
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in a `<!-- detail: procedure · integration -->` group of its own: «- **Harness ↔ event pipeline, a key past a lost box:** `key_press_after_container_lost_its_box` (3), in both layout modes, presses a key on a control whose anonymous block was dropped, the box taken by `hidden`, by `display:none` in `style` or by CSS alone: `a_key_on_a_button_whose_anonymous_block_was_dropped_does_not_panic`, `a_key_on_a_select_whose_anonymous_block_was_dropped_still_moves_its_choice` and `a_key_on_a_control_in_no_anonymous_block_reads_the_same`, red before the fix at `node/node.rs:1788:34`; focus stays and the key still acts, the limit a11y-plan §5 owns; no `stand_` prefix, so `run all` and `run key_press_after_container_lost_its_box` read it and no repeat leg does (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»."
    sidecar: "integration-test-strategy: new record for `key_press_after_container_lost_its_box`, 3 tests in both layout modes."
    rationale: "report.md:27 names the new check, 3 tests in both layout modes, no `stand_` prefix; report.md:23 the three ways and focus staying; report.md:60 it joins no leg; report.md:114 the select's key still moving its choice; report.md:139 red before; report.md:58 and report.md:81 the limit's owners; arch §Existing Scopes names the check by its own sentence, report.md:95."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in a `<!-- detail: procedure · integration -->` group of its own: «- **Script ↔ DOM, `URLSearchParams`:** `tests/url_search_params.rs` (4), a target of its own by file, runs a page's script over the global class and `URL.prototype.searchParams`: `construction_and_form_encoding`, `mutations_and_live_iteration`, `invalid_inputs_and_receivers` and `url_search_params_are_bidirectionally_live`, `href` and `search` live both ways (packages/blitz-vibey-script/tests/url_search_params.rs:15-95; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»."
    sidecar: "integration-test-strategy: new record for blitz-vibey-script's `url_search_params` target, 4 tests."
    rationale: "report.md:29 names the new target; report.md:17 the surface; report.md:102 tests integ `url_search_params` 4; report.md:346-356 the file and its four tests. Binds with arch §Existing Scopes, report.md:95."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 10
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in a `<!-- detail: procedure · integration -->` group of its own: «- **DOM ↔ resource loading, a failed stylesheet:** `render_blocking_stylesheet` gains 3 on `HtmlDocument`: `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` and `invalid_stylesheet_url_does_not_block_rendering`, a failed, empty or non-CSS sheet ceasing to block render and a bad `href` starting no load; on a harness or driver document the forward of `NetHandler::error` through `CountedHandler` carries it, §4 its unit test, so a failed load ends the wait where it read `NotSettled(Loads)` for good (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»."
    sidecar: "integration-test-strategy: new record for `render_blocking_stylesheet`'s 3 failed-stylesheet tests and the failed load ending the wait."
    rationale: "report.md:14 the failed-stylesheet behaviour; report.md:25 the forward and a failed load ending the wait; report.md:37 `render_blocking_stylesheet` +3; report.md:103 the tests of this path; report.md:381-390 the three tests. Binds with test-plan §4 and arch §Standard Contracts, report.md:49."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:103
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A test painting a blended element to a frame, or naming `convert_blend_mode` · searched: `git grep` over `tests/`, only `contrast.rs`'s fixture; `mix-blend-mode` and `isolation` paint on every document since 2026-11-01-upstream-sync-ui-graph, their tests `layers.rs`' 3 unit tests, its owner named at that wrap's P5»."
    sidecar: "integration-test-strategy: blending painted with no frame test recorded as an absence, its owner at P5."
    rationale: "report.md:15 blend and isolation now paint; report.md:104 the ✗, no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; report.md:172 the ✗ takes an owner at P5, inputs#I8."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A frame test of `render_element`'s skip of a box with `Node::is_inline_with_only_block_children`, its own shadow, background and border: none, its flag read by `document.rs`'s 4 unit tests alone, as measured at 2026-11-01-upstream-sync-ui-graph»."
    sidecar: "integration-test-strategy: the inline-decorations paint skip recorded with no frame test."
    rationale: "report.md:19 the public reader and the paint skip; report.md:106 tests unit `document.rs` 4, `inline_with_only_block_children_tests`, frame ✗."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A test of `flush_queued_mounted_events` skipping an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239): none, upstream's `element_removed_before_its_mounted_event_is_flushed` left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1`, a `CARRY:` on «Upstream sync ahead of determinism and exploration»»."
    sidecar: "integration-test-strategy: the mounted-event guard recorded with no test, upstream's test left out and carried."
    rationale: "report.md:20 the guard and the test left out; report.md:36 `6ef4b532d` adding 0 passed; report.md:80 the test's return and its owner."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 80
    section: "## 5. Integration Test Strategy"
    change: "Add after this line, in the absence group: «- A test of their own for upstream's `8b1a0a3ce` (`table_needs_rebuild`), `7a3dd8918` (`init_stylo_prefs`), `ae774d7f4` (the resolved `width` and `height`) and `e42ee1f25` (culling bounds): none, the workspace their witness, `incremental_oracle` 7 unmoved, as measured at 2026-11-01-upstream-sync-ui-graph»."
    sidecar: "integration-test-strategy: four taken upstream commits recorded with no test of their own, the workspace their witness."
    rationale: "report.md:109 names the four and their witness; report.md:12, report.md:16, report.md:21 and report.md:22 the paths each adds."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:109
note: "D-tests-framework and D-tests-obs-harness hold (cargo test throughout, `scripts/agent-run.sh` unedited, status and verdict shapes unmoved, report.md:64); outside this file, `.andromeda/test-plan.md:30` still reads blitz-net as holding no test where report.md:283-289 lists its new `mod tests` with 3 tests."
