proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 54
    section: "## 4. Unit Test Strategy"
    change: "Where line 54 opens the reading with `the CI-scripts leg runs 108 tests over eight test files`, lead instead with the new reading: `the CI-scripts leg runs 110 tests over eight test files, test_ci_workflows.py's 35 (+2, `test_only_stand_repeat_frees_disk` and `test_fast_leg_ends_on_the_font_embolden_witness`), the other seven unmoved, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md and green at CI#41333172440 on `45c333f1`;` then keep the 108 reading, its seven per-file counts and its `Ran 108` as the next history entry, opening `108, with test_ci_workflows.py's 33, ...`."
    sidecar: "unit-test-strategy: CI-scripts leg 108 -> 110 tests, test_ci_workflows.py 33 -> 35; the 108 reading kept as history."
    rationale: "The report's Counts, `ran-count`, names `unit-test-strategy.md:54` x2 as changed, 108 -> 110, by the two named tests in test_ci_workflows.py; eight test files stand, no Python file being new in Changes."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 20
    section: "## 4. Unit Test Strategy"
    change: "In the load counter's list, after ``is_noop` is the inner provider's`, add: `, and since 2026-11-01-upstream-sync-ui-graph a failed load is forwarded through `NetHandler::error`, so a failed stylesheet reaches the document and stops blocking render (`a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it`)`."
    sidecar: "unit-test-strategy: settle.rs's load counter pinned forwarding NetHandler::error to the document."
    rationale: "The report's Counts, `harness-lib`, names `unit-test-strategy.md:20` as changed for the forward of `error`; Symbols states `CountedHandler` forwards it, and Coverage names this test."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 20
    section: "## 4. Unit Test Strategy"
    change: "In the contrast reader's list, `three fixture tests` becomes `four fixture tests`, and after `an element with no text reads `None`` add: `, and, since 2026-11-01-upstream-sync-ui-graph, a blended ancestor reads `None` and an unblended one its pair (`a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`)`."
    sidecar: "unit-test-strategy: contrast.rs's fixture tests three -> four, the blended ancestor reading None."
    rationale: "The report's New text lists `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair` at contrast.rs:304-315, and `ws-tally` counts `text_contrast`'s 1; line 20's `three fixture tests` no longer holds."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:310
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 53
    section: "## 4. Unit Test Strategy"
    change: "Directly after `(.github/scripts/test_ci_workflows.py:305-309);` insert: `since 2026-11-01-upstream-sync-ui-graph `test_fast_leg_ends_on_the_font_embolden_witness` pins the `embolden` leg, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, last in `FAST_LEGS` (.github/scripts/test_ci_workflows.py:361-375);`."
    sidecar: "unit-test-strategy: test_ci_workflows.py pins the fast leg ending on the font-embolden witness."
    rationale: "The report's Harness / gate surface names the new leg and this test, and Coverage reads the `embolden` leg as pinned by it; the file's CI entry lists what the test file pins."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 53
    section: "## 4. Unit Test Strategy"
    change: "Directly after `(.github/scripts/test_ci_workflows.py:305-309);` insert: `since 2026-11-01-upstream-sync-ui-graph `test_only_stand_repeat_frees_disk` pins the free-disk step to `Stand checks repeated` alone, `matrix_test` holding none (.github/scripts/test_ci_workflows.py:110-115);`."
    sidecar: "unit-test-strategy: test_ci_workflows.py pins the free-disk step to Stand checks repeated alone."
    rationale: "The report's Harness / gate surface names this test beside the removal of `matrix_test`'s free-disk step, and its Retired bullet (2) keeps `Stand checks repeated`'s step standing."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (mutator, restyle narrowing):** since 2026-11-01-upstream-sync-ui-graph, upstream's `fe33b778b`, `mutator.rs` holds 39 tests of `restyle_for_attribute_change` and `restyle_for_child_list_change`, read through `check_incremental_restyle` and `assert_matches_fresh`, an incremental snapshot against a fresh one: changes in a hidden subtree then unhidden, class, id, attribute and style changes, child insertion, removal and moves, `nth-child` and sibling rules, pseudo-elements and `focus-within` (packages/blitz-dom/src/mutator.rs:2326-2887)`."
    sidecar: "unit-test-strategy: new item, mutator.rs's 39 restyle-narrowing tests."
    rationale: "The report's `ws-tally` counts `fe33b778b` at 39 and New text lists them at mutator.rs; the category holds what unit tests cover, per module, and has no row for them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (mutator, a custom widget's attributes):** since 2026-11-01-upstream-sync-ui-graph, upstream's `a26c6ddeb`, `custom_widget_replays_initial_attributes` pins `ElementData::set_custom_widget` replaying each existing attribute to `Widget::attribute_changed` (packages/blitz-dom/src/mutator.rs:1656-1727)`."
    sidecar: "unit-test-strategy: new item, the custom widget's attribute replay test."
    rationale: "The report's `ws-tally` counts `a26c6ddeb` at 1, New text lists the test, and Symbols states the replay."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:213
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (document, `<picture>`):** since 2026-11-01-upstream-sync-ui-graph, upstream's `dee854c76`, `picture_display_tests` holds one test, `picture_is_not_display_contents` (packages/blitz-dom/src/document.rs:4061-4094)`."
    sidecar: "unit-test-strategy: new item, the picture display test."
    rationale: "The report's Symbols names `picture_display_tests` at document.rs:4061-4094, and `ws-tally` counts `dee854c76` at 1."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-dom (document, an inline box with only block children):** since 2026-11-01-upstream-sync-ui-graph, upstream's `f76777766`, `inline_with_only_block_children_tests` holds four tests of `Node::is_inline_with_only_block_children`: it is detected, and it follows child edits, text edits and `display: contents` descendant edits (packages/blitz-dom/src/document.rs:4096-4296)`."
    sidecar: "unit-test-strategy: new item, the four inline-with-only-block-children tests."
    rationale: "The report's Coverage row for `is_inline_with_only_block_children` names `document.rs` 4, and New text lists the four tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-net:** since 2026-11-01-upstream-sync-ui-graph, upstream's `f6edb9701`, the crate's inline `#[cfg(test)] mod tests` holds three tests through a `NetHandler`: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`; two of them bind `127.0.0.1:0` by `std::net::TcpListener::bind`, test-only, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:465-546; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534)`."
    sidecar: "unit-test-strategy: new item, blitz-net's first unit tests and their two test-only binds."
    rationale: "The report's Coverage row for `NetHandler::error` names `unit blitz-net 3`, New text lists them, and Symbols states the two test-only binds; the category has no blitz-net row."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:103
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 100
    section: "## 4. Unit Test Strategy"
    change: "New item: `- **blitz-paint (layers):** since 2026-11-01-upstream-sync-ui-graph, upstream's `6fca6308d`, `layers.rs`'s inline test module holds three tests of `LayerManager`: `blend_layers_are_not_replaced_by_clips`, `normal_layers_clip_unless_isolation_is_forced` and `disabled_or_exhausted_layers_do_not_pop` (packages/blitz-paint/src/layers.rs:104-181)`."
    sidecar: "unit-test-strategy: new item, layers.rs's three blend-layer tests."
    rationale: "The report's Coverage row for `mix-blend-mode` names `unit layers.rs 3`, and New text lists the three tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A test painting a blended element to a frame, or naming `convert_blend_mode` · searched: `git grep` over `tests/`, only `contrast.rs`'s fixture; `mix-blend-mode` and `isolation` paint on every document, `layers.rs`'s three their one unit cover; its owner the P5 card's — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, blending painted with no frame test and convert_blend_mode named by no test."
    rationale: "The report's Coverage row carries the ✗ for this new path on every document, and Addendum 1 (inputs#I8) says it takes an owner at P5; the file's absence group states no such gap."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A frame test of `render_element` skipping the own shadow, background and border of an inline box with only block children (`paints_own_box`) — the flag read by `document.rs`'s four unit tests, the paint skip on no frame — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, the inline box's paint skip read on no frame."
    rationale: "The report's Coverage row for `is_inline_with_only_block_children` reads `frame ✗` for a new paint path; Symbols states the skip."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A test of its own for upstream's `8b1a0a3ce` (`table_needs_rebuild`), `7a3dd8918` (`init_stylo_prefs`), `ae774d7f4` (sub-pixel `width` · `height`) and `e42ee1f25` (culling on the scaled size) · searched: `git show -U0`, evidence/trial-takes.md; the workspace their witness, `incremental_oracle` 7 unmoved — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, four taken upstream commits with no test of their own."
    rationale: "The report's Coverage names these four as carrying no test of their own, `ws-tally` counting each at 0; each adds a code path, the workspace alone their witness."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:109
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 115
    section: "## 4. Unit Test Strategy"
    change: "New absence row: `- A test of `flush_queued_mounted_events` skipping an element removed since its event was queued — upstream's `element_removed_before_its_mounted_event_is_flushed` left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "unit-test-strategy: absence row, the mounted-event guard with its upstream test left out."
    rationale: "The report's Symbols states the guard and the test left out, `ws-tally` counts `6ef4b532d` at 0, and inputs#I6's third row carries the test's return."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
note: "D-tests-framework, D-tests-obs-harness and D-retired found nothing in this file. Outside it: test-plan.md:30 (blitz-net, no test observed) and test-plan.md:32 (blitz-paint's inline tests, no layers.rs) stand against report.md:103-104, for the body's own pass. Line 20 stands near the 2 000-char limit once both its edits land, so check its length at apply."
