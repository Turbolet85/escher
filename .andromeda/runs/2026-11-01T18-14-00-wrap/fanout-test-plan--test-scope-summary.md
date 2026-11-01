proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: "## 1. Test Scope Summary"
    change: "In the CI Python scripts record read `110 tests in eight test files (§4; 108 until 2026-11-01-upstream-sync-ui-graph, which added `test_only_stand_repeat_frees_disk` and `test_fast_leg_ends_on_the_font_embolden_witness` to test_ci_workflows.py; 104 with test_cold_agent.py's 33;` where it reads `108 tests in eight test files (§4; 104 with test_cold_agent.py's 33;`, the rest of the line as it stands."
    sidecar: "§1 CI Python scripts: 108 → 110 tests, the two new test_ci_workflows.py cases named, 108 kept as history."
    rationale: "The report moves `ran-count` at this line, 108 → 110, changed (report.md:38), the two tests listed at report.md:185-186."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 21
    section: "## 1. Test Scope Summary"
    change: "In the blitz-dom (document, mutator, net, form) record read `83 `#[test]` functions: 24 `document::` tests (19 until 2026-11-01-upstream-sync-ui-graph, which took `picture_display_tests` 1 and `inline_with_only_block_children_tests` 4; 16 until 2026-10-12-headless-screenshot …` and `— 15 in document.rs and,` and `52 in mutator.rs (12 until that chunk, which took the restyle narrowing's 39 and `custom_widget_replays_initial_attributes`, behind `custom-widget`)`, where it reads 38, 19, 10 and 12; net.rs's 4 and form.rs's 3 unmoved."
    sidecar: "§1 blitz-dom test count: 38 → 83, document 19 → 24, document.rs 10 → 15, mutator.rs 12 → 52, by the take's tests."
    rationale: "The tally names fe33b778b 39, dee854c76 1, f76777766 4 and a26c6ddeb 1 (report.md:36), and New text lists them in document.rs and mutator.rs (report.md:195-201, 213, 224-266); the figures are that arithmetic over this record's own 38, no per-file run stated in the report, and the Counts bullet names no row for this line."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-dom (mutator) — restyle narrowing:** upstream's 39 tests in mutator.rs read `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` against a fresh restyle (`check_incremental_restyle`, `assert_matches_fresh`): attribute, class, style, insertion and removal in a hidden subtree then unhidden; descendants and later siblings restyled by a class, id or style change; `nth-child` and sibling rules on insertion and removal, across text; a moved child; `::before` and `::after` created, removed and recoloured; `:focus-within` ancestors on a removed or moved focused subtree (packages/blitz-dom/src/mutator.rs:689-807; packages/blitz-dom/src/mutator.rs:2177-2887) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: the restyle narrowing's 39 mutator.rs tests."
    rationale: "fe33b778b replaces the blanket restyle and brings 39 tests (report.md:11, report.md:36), listed by name at report.md:224-266; no record of this file names them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-dom — `<picture>`:** `picture_display_tests`' `picture_is_not_display_contents` pins that a `picture` is no longer `display: contents`, upstream's rule gone from `default.css` and every select rule staying (packages/blitz-dom/src/document.rs:4061-4094) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: the `<picture>` display test."
    rationale: "dee854c76 and its one test (report.md:13, report.md:195-196); no record of this file names it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-dom — `CustomWidget`'s attribute replay:** `custom_widget_replays_initial_attributes`, behind `custom-widget`, pins `ElementData::set_custom_widget` replaying each existing attribute to `Widget::attribute_changed` with no old value (packages/blitz-dom/src/mutator.rs:1656-1727; packages/blitz-dom/src/node/element.rs:902-906) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: the custom widget's attribute replay test."
    rationale: "a26c6ddeb and its one test (report.md:18, report.md:36, report.md:213); no record of this file names it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:18
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-dom — an inline box over block children:** `inline_with_only_block_children_tests` 4 read `Node::is_inline_with_only_block_children` detected and following child, text and `display: contents` descendant edits (packages/blitz-dom/src/document.rs:4096-4296); `render_element`'s skip of such a box's own shadow, background and border (`paints_own_box`) is read on no frame — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: the inline-with-block-children reader's 4 unit tests and its paint skip's missing frame."
    rationale: "The new public reader has unit tests and its paint skip has none on a frame, ✗ at report.md:106; the report names no owner for this gap."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **The engine — behaviours taken with no test of their own:** `table_needs_rebuild` in `compute_layout_damage` (`8b1a0a3ce`), `init_stylo_prefs` before the UA sheet (`7a3dd8918`), the resolved `width` and `height` reading sub-pixel (`ae774d7f4`) and `render_element`'s culling bounds on the scaled size (`e42ee1f25`): each holds no test of its own, the workspace run its witness, `incremental_oracle` 7 unmoved — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: four taken upstream behaviours with no test of their own, the workspace their witness."
    rationale: "Coverage of new surfaces names these four commits as testless, the workspace their witness (report.md:109), each with 0 in the tally (report.md:36)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:109
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 55
    section: "## 1. Test Scope Summary"
    change: "Replace the record with `- **blitz-dom (select) — a key press past a dropped anonymous block:** `select_popup` presses a key in every row, the row where an ancestor gains `hidden` among them, a key there no longer panicking since `BaseDocument::deallocate_anonymous_block` clears `layout_parent` on each child still naming the block; read by `select_popup`'s row 3 and `key_press_after_container_lost_its_box` (below), both red before at `node/node.rs:1788:34` (tests/blitz-tests/tests/select_popup.rs:513; packages/blitz-dom/src/document.rs:1493-1503); the limit that stays: a focused control that no longer shows keeps focus and takes keys — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; owner of the limit: its line in `.andromeda/residuals.md` and the `CARRY:` on «A11y re-read»`"
    sidecar: "§1: the key-press panic and select_popup's row-3 skip retired; the check reading the fix and the limit that stays stated, the owner re-pointed."
    rationale: "Retired (1) and (4) name the panic and the skip (report.md:33), `test-scope-summary.md:55` ×2 changed and its owner row re-pointed (report.md:50, report.md:62); the fix at report.md:23 and report.md:26, red before at report.md:139, the limit's owner at report.md:81."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 88
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-net:** 3 inline tests in its `#[cfg(test)] mod tests`, upstream's: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, the last two each binding `std::net::TcpListener` on `127.0.0.1:0`, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:465-546; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: blitz-net's first 3 inline tests and their two test-only loopback binds."
    rationale: "f6edb9701's tests in blitz-net (report.md:28, report.md:103, report.md:283-289); this file holds no blitz-net record, and the body's says none exist (see note)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-paint — `mix-blend-mode` and `isolation`:** `layers.rs` holds 3 inline tests of the blend layers — `blend_layers_are_not_replaced_by_clips`, `normal_layers_clip_unless_isolation_is_forced` and `disabled_or_exhausted_layers_do_not_pop` (packages/blitz-paint/src/layers.rs:104-181); no fork or upstream test paints a blended element to a frame or names `convert_blend_mode` (packages/blitz-paint/src/blend.rs:5-26) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; the frame gap's owner: named at this wrap's P5 (inputs#I8)`"
    sidecar: "§1 new record: blend and isolation painted on every document, 3 unit tests and no frame test, the gap's owner due at P5."
    rationale: "A new paint path on every document with ✗ at the frame tier (report.md:104); the operator's word gives the ✗ an owner at P5 (report.md:168, report.md:172)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary"
    change: "In the blitz-test-harness record read `4 on the load counter (5 since 2026-11-01-upstream-sync-ui-graph)`, and append after its last `report.md`: `; 24 since 2026-11-01-upstream-sync-ui-graph, one more in each of `settle.rs` — `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it`, `CountedHandler` forwarding `NetHandler::error` (packages/blitz-test-harness/src/settle.rs:121-125; packages/blitz-test-harness/src/settle.rs:456-473) — and `contrast.rs` — `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`, `text_contrast` reading `None` past a `mix-blend-mode` other than `normal` (packages/blitz-test-harness/src/contrast.rs:154-156; packages/blitz-test-harness/src/contrast.rs:304-315) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 blitz-test-harness: 22 → 24 unit tests, the load counter's fifth and contrast's new `None` row named."
    rationale: "`harness-count` 22 → 24 and `test-scope-summary.md:100` changed, +1 on the load counter (report.md:39); the two tests at report.md:310 and report.md:314, the behaviours at report.md:24-25."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 119
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **blitz-vibey-script — `URLSearchParams`:** tests/url_search_params.rs, its own target, holds 4 `#[test]` functions — `construction_and_form_encoding`, `mutations_and_live_iteration`, `invalid_inputs_and_receivers` and `url_search_params_are_bidirectionally_live` — over the global class and `URL.prototype.searchParams`, `href` and `search` live both ways (packages/blitz-vibey-script/tests/url_search_params.rs:15-95) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: blitz-vibey-script's url_search_params target, 4 tests."
    rationale: "New test target and surface (report.md:17, report.md:29, report.md:102), its tests listed at report.md:349-355; no record of this file names it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 132
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **dioxus-native-dom — a second upstream test absent:** upstream's `element_removed_before_its_mounted_event_is_flushed`, its commit `6ef4b532d` taken, is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0, so the skip in `flush_queued_mounted_events` holds no test of its own (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); it returns with `286874cc1` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; owner: a `CARRY:` on «Upstream sync ahead of determinism and exploration»`"
    sidecar: "§1 new record: 6ef4b532d's upstream test left out, its guard untested, the return carried."
    rationale: "A new guard with its one test left out and 0 tests in the tally (report.md:20, report.md:36), the row and its owner at report.md:80; line 132 names the sibling absence."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 157
    section: "## 1. Test Scope Summary"
    change: "After `until the check answers it, its `@import` included` insert `, a failed load ending the wait since 2026-11-01-upstream-sync-ui-graph (`CountedHandler`, the harness record above)`, the rest of the line as it stands."
    sidecar: "§1 stand_settle's load row: a failed load now ends the wait."
    rationale: "`load-wait` `test-scope-summary.md:157` changed, a failed load ends the wait (report.md:49), `settle_waiting` unedited (report.md:25); kept short for the line's 2 000-char bound, the detail on line 100's record."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:49
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 204
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **tests/blitz-tests — `render_blocking_stylesheet` on a stylesheet that fails:** upstream's 3 more tests, `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` over an empty, a non-CSS and a one-byte body, and `invalid_stylesheet_url_does_not_block_rendering`, a bad `href` starting no load (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: render_blocking_stylesheet's 3 failed-stylesheet tests."
    rationale: "f6edb9701 and `run all`'s +3 (report.md:14, report.md:37, report.md:103), listed at report.md:383-390; this file names the target nowhere."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:37
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 228
    section: "## 1. Test Scope Summary"
    change: "New record after this line: `- **tests/blitz-tests — `key_press_after_container_lost_its_box`:** 3 tests, its own auto-discovered target with no `stand_` prefix, each in both layout modes, a control's box taken away by `hidden`, by `display:none` in `style` and by CSS alone: a key on a button whose anonymous block was dropped does not panic, a key on a select past its dropped block still moves its choice, and a control in no anonymous block reads the same; run by the workspace `test` leg, `run all` and by name, not by `run stand`, the `stand-repeat` leg or the `a11y` leg (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "§1 new record: key_press_after_container_lost_its_box, 3 tests, the panic fix's check."
    rationale: "The new check (report.md:27, report.md:105), its readings at report.md:23 and report.md:114, its legs at report.md:37 and report.md:60, its tests at report.md:375-380; the check is named nowhere (report.md:95)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 240
    section: "## 1. Test Scope Summary"
    change: "Read `run as `select_popup` is, a named `--test` of the `stand-repeat` leg, and with `blitz-paint/font-embolden` as `ci-leg.sh`'s `embolden` leg, the last of `fast`, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (.github/scripts/ci-leg.sh:28-29; .github/scripts/test_ci_workflows.py:361-375);` where it reads `run as `select_popup` is, a named `--test` of the `stand-repeat` leg;`, the rest of the line as it stands."
    sidecar: "§1 select_popup_painted: also run with font-embolden as the `embolden` step of `fast`."
    rationale: "The harness surface adds the leg running this target (report.md:64), pinned (report.md:108, report.md:143); the record's list of what runs the target is otherwise short by it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 270
    section: "## 1. Test Scope Summary"
    change: "Read `a screen of 47,089 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`` where it reads `a screen of 47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET``, and end the closing provenance `the two counts at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the screen's bytes at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "§1 stand_cells_served: Cells' screen 47,949 → 47,089 bytes, a stale reading corrected, not a move of this take."
    rationale: "Spec claims disproved: 47,089 bytes, 740 lines, 5 parts measured through the binary, the take moving nothing, disposition amend all three, this line among them (report.md:87)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:87
note: "D-tests-framework and D-tests-obs-harness hold no drift for this file (no new runner; `agent-run.sh` and status shape unmoved, report.md:64); outside this file, the body's test-plan.md:30 «blitz-net: no test — observed absent» and test-plan.md:32 «inline unit tests exist in gradient.rs and css_box.rs» no longer hold (report.md:103-104, report.md:283-298), and the owner of the inline paint skip's frame gap is unnamed."
