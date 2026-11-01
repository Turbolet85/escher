proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: "## 1. Test Scope Summary"
    change: "In the CI Python scripts line, read `108 tests in eight test files (§4; 104 with test_cold_agent.py's 33;` as `110 tests in eight test files (§4; 108 until 2026-11-01-upstream-sync-ui-graph added two to test_ci_workflows.py; 104 with test_cold_agent.py's 33;`; the rest of the line stands."
    sidecar: "test-scope-summary: the CI scripts count moves from 108 to 110."
    rationale: "The report's ran-count row names this line as changed, 108 to 110, by test_fast_leg_ends_on_the_font_embolden_witness and test_only_stand_repeat_frees_disk."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:38
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 21
    section: "## 1. Test Scope Summary"
    change: "In the blitz-dom (document, mutator, net, form) line, read `38 #[test] functions: 19 document:: tests (16 until` as `83 #[test] functions (38 until 2026-11-01-upstream-sync-ui-graph): 24 document:: tests (19 until that chunk, which took picture_display_tests 1 and inline_with_only_block_children_tests 4; 16 until`, read `10 in document.rs and` as `15 in document.rs (10 until that chunk) and`, and read `12 in mutator.rs` as `52 in mutator.rs (12 until that chunk, which took upstream's 39 restyle tests and custom_widget_replays_initial_attributes)`; net.rs's 4 and form.rs's 3 stand, keeping the line's backticks."
    sidecar: "test-scope-summary: blitz-dom's group count moves from 38 to 83, document.rs from 10 to 15 and mutator.rs from 12 to 52."
    rationale: "The report's tally gives fe33b778b 39, dee854c76 1, a26c6ddeb 1 and f76777766 4 passed, and its New text lists picture_display_tests and inline_with_only_block_children_tests in document.rs and the rest in mutator.rs; 83 is that sum over the entity's own 38, f6edb9701's 6 falling outside this group's files."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "New record after this entity: `- **blitz-dom (layout) — an inline box holding only block children:** the public reader `Node::is_inline_with_only_block_children` is held by 4 unit tests in document.rs's `inline_with_only_block_children_tests`; the paint skip it drives, `render_element` leaving out such a box's own shadow, background and border, has no frame test (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/document.rs:4096-4296) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "test-scope-summary: records that the inline-box paint skip has no frame test."
    rationale: "The report's Coverage of new surfaces row reads unit document.rs 4 and frame ✗ for this reader and its paint skip."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:106
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "New record after this entity: `- **blitz-dom, blitz-paint — upstream commits with no test of their own:** `8b1a0a3ce` (table damage), `7a3dd8918` (Stylo prefs before the UA sheet), `ae774d7f4` (sub-pixel computed sizes) and `e42ee1f25` (culling bounds) brought no test of their own, the workspace their witness, `incremental_oracle` at 7 unmoved — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "test-scope-summary: records the four upstream commits that brought no test of their own."
    rationale: "The report names these four commits as having no test of their own, the workspace their witness and incremental_oracle 7 unmoved."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:109
    fact: F6
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 55
    section: "## 1. Test Scope Summary"
    change: "Replace this entity with: `- **blitz-dom (select) — the key press past a dropped block:** `select_popup` presses the key in every row, its row where an ancestor gains `hidden` among them, no skip left; a key press on a focused control whose anonymous block was dropped no longer panics, `BaseDocument::deallocate_anonymous_block` clearing each child's stale `layout_parent`, and focus stays and the key still acts (tests/blitz-tests/tests/select_popup.rs:513; packages/blitz-dom/src/document.rs:1493-1503) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`; the claim that no check reads the panic and the owner pointer both go, the owed thing landed."
    sidecar: "test-scope-summary: retires the key-press panic and select_popup's row-3 skip, and drops the line's owner pointer."
    rationale: "The report's Retired bullet names the panic and the row-3 skip, its panic row marks this line changed, retired together, and its owner row answers the pointer with nothing where the owed thing landed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 55
    section: "## 1. Test Scope Summary"
    change: "New record after this entity: `- **blitz-dom (select) — `key_press_after_container_lost_its_box`:** a blitz-tests target of 3 tests in both layout modes, no `stand_` prefix so `run all` alone moves: a key press with focus on a control whose anonymous block was dropped does not panic for `hidden`, `style=display:none` and CSS alone, a select's choice still moves past the dropped block, and a control in no anonymous block reads the same (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "test-scope-summary: adds the check key_press_after_container_lost_its_box."
    rationale: "The report's New check bullet and its acceptance row state the target, its 3 tests, both modes and the three ways, its New text giving the three tests' lines."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "In the blitz-paint line, read `inline unit tests exist in gradient.rs and css_box.rs` as `inline unit tests exist in gradient.rs, css_box.rs and, since 2026-11-01-upstream-sync-ui-graph, layers.rs`, its citation group gaining packages/blitz-paint/src/layers.rs:104-181; the rest of the line stands."
    sidecar: "test-scope-summary: blitz-paint's inline unit tests now include layers.rs."
    rationale: "The report's tally gives 6fca6308d 3 passed and its New text lists layers.rs's mod tests with three tests, so the two-file list is now short."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:295
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "New record after this entity: `- **blitz-paint — blending:** no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, a grep over `tests/` finding only `contrast.rs`'s fixture, so `mix-blend-mode` and `isolation` painting rest on the 3 `layers.rs` unit tests (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/layers.rs:104-181); this wrap's P5 card names the gap's owner — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "test-scope-summary: records that blended painting has no frame test, the gap's owner named at P5."
    rationale: "The report's Coverage row reads unit layers.rs 3 and ✗ on any blended frame or test naming convert_blend_mode, and its addendum gives the ✗ an owner at P5."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 32
    section: "## 1. Test Scope Summary"
    change: "In the blitz-paint broad line, read `inline unit tests exist in gradient.rs and css_box.rs` as `inline unit tests exist in gradient.rs, css_box.rs and layers.rs`; the rest of the line stands."
    sidecar: "test-plan §1: the blitz-paint broad line now names layers.rs."
    rationale: "The broad line repeats the two-file list the detail corrects; the report's New text lists layers.rs's mod tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:295
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary"
    change: "At the end of the blitz-test-harness line, append: `; 24 since 2026-11-01-upstream-sync-ui-graph, one more in `settle.rs`, the load counter's fifth, `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it` (packages/blitz-test-harness/src/settle.rs:456-473), and one more in `contrast.rs`, `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair` (packages/blitz-test-harness/src/contrast.rs:304-315) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`; the earlier readings stand as their history."
    sidecar: "test-scope-summary: the harness lib moves from 22 to 24 and the load counter's tests from 4 to 5."
    rationale: "The report's harness-count row reads 22 to 24 and marks this line's load-counter count changed, +1; the tally names the counter's 1 and text_contrast's 1."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary"
    change: "New record after this entity: `- **blitz-test-harness — a failed load ends the wait:** `CountedHandler` forwards `NetHandler::error`, so a failed load ends the harness and driver wait: `settle_waiting`, unedited, returns where it would otherwise have run to `LOAD_WAIT`, and a failed stylesheet no longer reads `NotSettled(Loads)` for good (packages/blitz-test-harness/src/settle.rs:121-125) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "test-scope-summary: records that a failed load now ends the harness and driver wait."
    rationale: "The report's CountedHandler bullet states the forward, the wait's new end and that the contracts owe the sentence."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:25
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 119
    section: "## 1. Test Scope Summary"
    change: "At the end of the blitz-vibey-script line, append: `; tests/url_search_params.rs, since 2026-11-01-upstream-sync-ui-graph, holds 4 `#[test]` functions of the global `URLSearchParams` and `URL.prototype.searchParams` (packages/blitz-vibey-script/tests/url_search_params.rs:15-95) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "test-scope-summary: blitz-vibey-script's inventory gains tests/url_search_params.rs."
    rationale: "The report's Crates / modules bullet names the new test target, its Coverage row reads integ url_search_params 4, and its New text lists the four tests; the tests' names are F8, integration-test-strategy's."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 40
    section: "## 1. Test Scope Summary"
    change: "In the blitz-vibey-script broad line, read `tests/selection.rs the script Selection API and tests/preact.rs runs` as `tests/selection.rs the script Selection API, tests/url_search_params.rs the global `URLSearchParams` and `URL.prototype.searchParams`, and tests/preact.rs runs`; the rest of the line stands."
    sidecar: "test-plan §1: the blitz-vibey-script broad line now names tests/url_search_params.rs."
    rationale: "The broad line lists the crate's test files and the report adds a fourth target."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:29
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 132
    section: "## 1. Test Scope Summary"
    change: "New record after this entity: `- **dioxus-native-dom — a second upstream test absent:** upstream's `element_removed_before_its_mounted_event_is_flushed` is left out, because it reads 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1`, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test of its own (packages/dioxus-native-dom/src/dioxus_document.rs:235-239) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`"
    sidecar: "test-scope-summary: records the left-out upstream mounted-event test and the guard with no test of its own."
    rationale: "The report's 6ef4b532d row states the left-out test and its reading, its tally counts that commit 0 tests, and the card's row has it return with 286874cc1."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 30
    section: "## 1. Test Scope Summary"
    change: "In the blitz-net broad line, replace the `no test — observed absent` claim and its search with `three inline unit tests in lib.rs's `#[cfg(test)] mod tests`, over two test-only `TcpListener::bind` calls on `127.0.0.1:0`, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:465-546)`; the clause that its `file:` read has no test named for it stands, and the closing provenance becomes `as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "test-plan §1: blitz-net holds 3 inline unit tests; the observed-absent claim is withdrawn."
    rationale: "The report's New text lists blitz-net's mod tests with three tests, its Coverage row reads unit blitz-net 3 and its bind row names the two loopback binds, so the line's absence and its search are false; none of the three tests names the file read."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:283
note: "F3 has no picked entity to sit beside, so it stays with the new-detail pass while its false body line test-plan.md:30 is proposed here; report.md:49 and report.md:87 mark test-scope-summary.md:157 (load wait) and test-scope-summary.md:270 (Cells 47,089 bytes) to change, and neither line is in this group's listing, so neither is proposed here and both need another pass to carry them."
