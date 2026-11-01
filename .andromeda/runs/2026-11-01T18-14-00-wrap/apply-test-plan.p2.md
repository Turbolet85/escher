# Apply — test-plan · part 2 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:8#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:21#1, .andromeda/test-plan-detail/test-scope-summary.md:21#2, .andromeda/test-plan-detail/test-scope-summary.md:21#3
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:27+#1
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:27+#2
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:55#1
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/test-scope-summary.md:57+#1
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/test-scope-summary.md:94#1
- fanout-test-plan--cascade-g1.md 8 · applied · .andromeda/test-plan-detail/test-scope-summary.md:96+#1
- fanout-test-plan--cascade-g1.md 10 · applied · .andromeda/test-plan-detail/test-scope-summary.md:100#1
- fanout-test-plan--cascade-g1.md 11 · applied · .andromeda/test-plan-detail/test-scope-summary.md:102+#1
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan-detail/test-scope-summary.md:119#1
- fanout-test-plan--cascade-g1.md 14 · applied · .andromeda/test-plan-detail/test-scope-summary.md:132+#1
- fanout-test-plan--cascade-g2.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:157#1
- fanout-test-plan--cascade-g2.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:240#1
- fanout-test-plan--cascade-g3.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:270#1
- fanout-test-plan--cascade-new-test-scope-summary.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:90+#1
- cascade-compare-test-plan.md 15 · applied · .andromeda/test-plan-detail/test-scope-summary.md:206+#1

## Edits
- line: .andromeda/test-plan-detail/test-scope-summary.md:8
  old: «108 tests in eight test files (§4;»
  new: «110 tests in eight test files, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md (§4;»
- line: .andromeda/test-plan-detail/test-scope-summary.md:21
  old: «38 `#[test]` functions: 19 `document::` tests (16 until»
  new: «83 `#[test]` functions, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md: 24 `document::` tests (16 until»
- line: .andromeda/test-plan-detail/test-scope-summary.md:21
  old: «— 10 in document.rs and,»
  new: «— 15 in document.rs (`picture_display_tests` 1 and `inline_with_only_block_children_tests` 4 among them, packages/blitz-dom/src/document.rs:4061-4094; packages/blitz-dom/src/document.rs:4096-4296) and,»
- line: .andromeda/test-plan-detail/test-scope-summary.md:21
  old: «, 12 in mutator.rs, 4 in net.rs (»
  new: «, 52 in mutator.rs (upstream's 39 restyle tests and `custom_widget_replays_initial_attributes` among them, packages/blitz-dom/src/mutator.rs:2326-2887; packages/blitz-dom/src/mutator.rs:1656-1727), 4 in net.rs (»
- after: .andromeda/test-plan-detail/test-scope-summary.md:27
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-dom (layout) — an inline box holding only block children:** the public reader `Node::is_inline_with_only_block_children` is held by 4 unit tests in document.rs's `inline_with_only_block_children_tests`; the paint skip it drives, `render_element` leaving out such a box's own shadow, background and border, has no frame test (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/document.rs:4096-4296) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; owner: a `CARRY:` on «Upstream sync ahead of determinism and exploration»»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/test-scope-summary.md:27
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-dom, blitz-paint — upstream commits with no test of their own:** `8b1a0a3ce` (table damage), `7a3dd8918` (Stylo prefs before the UA sheet), `ae774d7f4` (sub-pixel computed sizes) and `e42ee1f25` (culling bounds) hold no test of their own, the workspace their witness, `incremental_oracle` 7 among its checks (packages/blitz-dom/src/layout/damage.rs:442-458; packages/blitz-dom/src/document.rs:87-102; packages/blitz-dom/src/resolved_style.rs:468) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:55
  old: «the key press `select_popup` skips:** `select_popup` skips the key press in its one row where an ancestor gains `hidden`, because a key press there panics at `position_in_containing_block` for any focused control, list or no list, the control's anonymous block dropped; no check reads that panic (tests/blitz-tests/tests/select_popup.rs:478-516; packages/blitz-dom/src/node/node.rs:1788) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; owner: a CARRY on «Upstream sync ahead of the UI graph»»
  new: «the key press past a dropped block:** `select_popup` presses the key in every row, its row where an ancestor gains `hidden` among them; a key press on a focused control whose anonymous block was dropped does not panic, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still naming the block, and focus stays and the key still acts (tests/blitz-tests/tests/select_popup.rs:513; packages/blitz-dom/src/document.rs:1493-1503) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:57
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-dom (select) — `key_press_after_container_lost_its_box`:** a blitz-tests target of 3 tests, each in both layout modes, with no `stand_` prefix, so `run all` runs it and `run stand` does not: a key press with focus on a control whose anonymous block was dropped does not panic for `hidden`, `style="display:none"` and CSS alone, a select's choice still moves past the dropped block, and a control in no anonymous block reads the same (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/test-scope-summary.md:90
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-net:** 3 inline unit tests in its `#[cfg(test)] mod tests`, `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, over two test-only `TcpListener::bind` calls on `127.0.0.1:0`, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:513-520; packages/blitz-net/src/lib.rs:522-530; packages/blitz-net/src/lib.rs:532-545) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:94
  old: «inline unit tests exist in gradient.rs and css_box.rs (packages/blitz-paint/src/gradient.rs:517-536; packages/blitz-paint/src/kurbo_css/css_box.rs:661-847)»
  new: «inline unit tests exist in gradient.rs, css_box.rs and layers.rs (packages/blitz-paint/src/gradient.rs:517-536; packages/blitz-paint/src/kurbo_css/css_box.rs:661-847; packages/blitz-paint/src/layers.rs:104-181)»
- after: .andromeda/test-plan-detail/test-scope-summary.md:96
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-paint — blending:** no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, a grep over `tests/` finding only `contrast.rs`'s fixture, so `mix-blend-mode` and `isolation` painting rest on the 3 `layers.rs` unit tests (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/layers.rs:104-181) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; owner: a `CARRY:` on «Upstream sync ahead of determinism and exploration»»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:100
  old: «(packages/blitz-test-harness/src/contrast.rs:203-324) — as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md»
  new: «(packages/blitz-test-harness/src/contrast.rs:203-324) — as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md; the crate holds 24, the load counter's fifth `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it` in `settle.rs` (packages/blitz-test-harness/src/settle.rs:456-473) and `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair` in `contrast.rs` (packages/blitz-test-harness/src/contrast.rs:304-315) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:102
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-test-harness — a failed load ends the wait:** `CountedHandler` forwards `NetHandler::error`, so a failed load ends the harness and driver wait: `settle_waiting` returns where it would otherwise have run to `LOAD_WAIT`, and a failed stylesheet does not read `NotSettled(Loads)` for good (packages/blitz-test-harness/src/settle.rs:121-125) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:119
  old: «packages/blitz-vibey-script/tests/preact.rs:102-199)»
  new: «packages/blitz-vibey-script/tests/preact.rs:102-199); tests/url_search_params.rs holds 4 `#[test]` functions of the global `URLSearchParams` and `URL.prototype.searchParams` (packages/blitz-vibey-script/tests/url_search_params.rs:15-95) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:132
  anchor: «- **dioxus-native-dom — an upstream test absent:** upstream's `task_polled_during_render_can_access_document`, its commit `286874cc1` not taken, has no counterpart in the fork's dioxus_document.rs test module — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  add: «- **dioxus-native-dom — a second upstream test absent:** upstream's `element_removed_before_its_mounted_event_is_flushed` is left out, because it reads 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1`, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test of its own (packages/dioxus-native-dom/src/dioxus_document.rs:235-239) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- line: .andromeda/test-plan-detail/test-scope-summary.md:157
  old: «a load in flight reads `NotSettled(Loads)` until the check answers it, its `@import` included,»
  new: «a load in flight reads `NotSettled(Loads)` until the check answers it, its `@import` included (a failed load ends the wait, read by `settle.rs`'s own unit test),»
- after: .andromeda/test-plan-detail/test-scope-summary.md:206
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **tests/blitz-tests — `render_blocking_stylesheet` on a stylesheet that fails:** upstream's 3 tests, `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` over an empty, a non-CSS and a one-byte body, and `invalid_stylesheet_url_does_not_block_rendering`, a bad `href` starting no load (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:240
  old: «run as `select_popup` is, a named `--test` of the `stand-repeat` leg;»
  new: «run as `select_popup` is, a named `--test` of the `stand-repeat` leg, and, with `--features blitz-paint/font-embolden`, as the `embolden` leg, the last step of `fast`, with no CI job (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29);»
- line: .andromeda/test-plan-detail/test-scope-summary.md:270
  old: «a screen of 47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`)»
  new: «a screen of 47,089 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»

## Patterns
[[pattern]]
id = "tp2-01"
fixed = '108 tests in eight test files'

[[pattern]]
id = "tp2-02"
fixed = '38 `#[test]` functions'

[[pattern]]
id = "tp2-03"
fixed = '10 in document.rs'

[[pattern]]
id = "tp2-04"
fixed = '12 in mutator.rs'

[[pattern]]
id = "tp2-05"
fixed = 'a key press there panics at'

[[pattern]]
id = "tp2-06"
fixed = 'skips the key press in its one row'

[[pattern]]
id = "tp2-07"
fixed = 'inline unit tests exist in gradient.rs and css_box.rs'

[[pattern]]
id = "tp2-08"
fixed = '47,949 bytes'
