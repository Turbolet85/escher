# Apply — test-plan · part 3 of 4

## Verdicts
- fanout-test-plan--cascade-g4.md 1 · applied · .andromeda/test-plan-detail/test-strategy.md:8#1
- fanout-test-plan--cascade-g4.md 2 · applied · .andromeda/test-plan-detail/test-strategy.md:40#1
- fanout-test-plan--cascade-g4.md 3 · applied · .andromeda/test-plan-detail/test-strategy.md:60#1
- cascade-compare-test-plan.md 16 · applied · .andromeda/test-plan-detail/test-strategy.md:34#1
- fanout-test-plan--cascade-g4.md 4 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:10+#1
- fanout-test-plan--cascade-g4.md 5 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:20#1, .andromeda/test-plan-detail/unit-test-strategy.md:20#2, .andromeda/test-plan-detail/unit-test-strategy.md:20#3
- fanout-test-plan--cascade-g4.md 7 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:53#1
- fanout-test-plan--cascade-g4.md 8 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:54#1, .andromeda/test-plan-detail/unit-test-strategy.md:54#2
- fanout-test-plan--cascade-g4.md 9 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:102+#1
- fanout-test-plan--cascade-g4.md 10 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:102+#2
- cascade-compare-test-plan.md 21 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:102+#3
- fanout-test-plan--cascade-g4.md 11 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:22+#1
- fanout-test-plan--cascade-g4.md 13 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:68#1
- fanout-test-plan--cascade-g4.md 14 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:74#1
- fanout-test-plan--cascade-new-integration-test-strategy.md 1 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:76+#1

## Edits
- line: .andromeda/test-plan-detail/test-strategy.md:8
  old: «a filter or a background image»
  new: «a filter, a background image or a `mix-blend-mode` other than `normal` (`paints_more_than_colour`, through `resolved_style_value`; packages/blitz-test-harness/src/contrast.rs:154-156), `Session::lint`'s `contrast` reading the same with no second formula»
- line: .andromeda/test-plan-detail/test-strategy.md:40
  old: «no local gate builds `blitz-paint/font-embolden`, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests»
  new: «a paint change under the OS-gated `blitz-paint/font-embolden` feature has its local witness in the `embolden` step, the last of `ci-leg.sh fast` with no CI job (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29), `cargo test -p blitz-tests»
- line: .andromeda/test-plan-detail/test-strategy.md:60
  old: «156 `.rs` files, 155 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the latest `select_popup.rs`»
  new: «157 `.rs` files, 156 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md, the latest `key_press_after_container_lost_its_box.rs`, then `select_popup.rs`»
- line: .andromeda/test-plan-detail/test-strategy.md:34
  old: «tests/blitz-tests/tests/password_painted_masked.rs:41-82)»
  new: «tests/blitz-tests/tests/password_painted_masked.rs:41-82); `mix-blend-mode` and `isolation` paint with no frame check — `layers.rs`'s 3 unit tests read the layer manager, and no test paints a blended element to a frame or names `convert_blend_mode` — and the paint skip of an inline box holding only block children is read by `document.rs`'s 4 unit tests of `is_inline_with_only_block_children`, on no frame, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:10
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · unit -->»
  add: «»
  add: «- **Test file location, blitz-paint's `layers.rs`:** `layers.rs` holds 3 unit tests in its `#[cfg(test)] mod tests`: `blend_layers_are_not_replaced_by_clips`, `normal_layers_clip_unless_isolation_is_forced` and `disabled_or_exhausted_layers_do_not_pop` (packages/blitz-paint/src/layers.rs:104-181)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:20
  old: «`is_noop` is the inner provider's»
  new: «`is_noop` is the inner provider's, and a failed stylesheet reaches the document through the counter's forward of `NetHandler::error` and unblocks it (`a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it`)»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:20
  old: «three fixture tests»
  new: «four fixture tests»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:20
  old: «and an element with no text reads `None`»
  new: «an element with no text reads `None`, and a blended ancestor reads `None` and an unblended one its pair (`a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`)»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:53
  old: «; `InstallScriptTest` runs the CI install script»
  new: «; `test_only_stand_repeat_frees_disk` pins that only `Stand checks repeated` frees disk (.github/scripts/test_ci_workflows.py:110-115) and `test_fast_leg_ends_on_the_font_embolden_witness` that `fast` ends on the font-embolden witness (.github/scripts/test_ci_workflows.py:361-375); `InstallScriptTest` runs the CI install script»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:54
  old: «the CI-scripts leg runs 108 tests over eight test files — test_ci_workflows.py's 33,»
  new: «the CI-scripts leg runs 110 tests over eight test files — test_ci_workflows.py's 35,»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:54
  old: «as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md (`ci-scripts` `Ran 108`) and green at CI#40214713710 on `55bf3425`»
  new: «as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md and green at CI#41333172440 on `45c333f16`»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:102
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **blitz-dom (restyle, `mutator.rs`):** `mutator.rs` holds upstream's 39 restyle tests, comparing an incremental document against a fresh one through `check_incremental_restyle` over hidden subtrees, siblings, pseudo-elements and focus-within (packages/blitz-dom/src/mutator.rs:2177-2887), beside `custom_widget_replays_initial_attributes`, `CustomWidget`'s replay of each existing attribute (packages/blitz-dom/src/mutator.rs:1656-1727)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:102
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **blitz-dom (`document.rs`, `<picture>` and the inline flag):** `picture_display_tests` holds one test, `picture_is_not_display_contents` (packages/blitz-dom/src/document.rs:4061-4094), and `inline_with_only_block_children_tests` four, the flag detected and following child, text and `display: contents` descendant edits (packages/blitz-dom/src/document.rs:4096-4296)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:102
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **blitz-net:** the crate's inline `#[cfg(test)] mod tests` holds three tests through a `NetHandler`: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`; two of them bind `127.0.0.1:0` by `std::net::TcpListener::bind`, test-only, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:465-546; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:22
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Script ↔ DOM, `URLSearchParams` (blitz-vibey-script):** tests/url_search_params.rs (4) covers the global `URLSearchParams` and `URL.prototype.searchParams`: `construction_and_form_encoding`, `mutations_and_live_iteration`, `invalid_inputs_and_receivers` and `url_search_params_are_bidirectionally_live` (packages/blitz-vibey-script/tests/url_search_params.rs:15-35; packages/blitz-vibey-script/tests/url_search_params.rs:37-59; packages/blitz-vibey-script/tests/url_search_params.rs:61-78; packages/blitz-vibey-script/tests/url_search_params.rs:80-95)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:68
  old: «One row skips its key press: a key pressed after an ancestor gained `hidden` panics at `packages/blitz-dom/src/node/node.rs:1788`, for any control, with no list opened, the test's comment saying why (tests/blitz-tests/tests/select_popup.rs:202-1182; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  new: «Every row takes its key press through one `harness.press`, the key pressed after an ancestor gained `hidden` among them: it does not panic, focus stays and the key still acts (tests/blitz-tests/tests/select_popup.rs:202-1182; tests/blitz-tests/tests/select_popup.rs:513; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the row's key press at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:74
  old: «has the local witness §2 Painted select list names»
  new: «is read locally by the `embolden` step of `ci-leg.sh fast`, as §2 Painted select list names»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:76
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **DOM ↔ resource loading, a stylesheet that stops blocking:** `render_blocking_stylesheet` reads it by three tests — `failed_stylesheet_unblocks_rendering`, a failed load; `empty_or_non_css_stylesheet_unblocks_rendering`, over an empty body, an HTML body and a lone `\xff` byte; and `invalid_stylesheet_url_does_not_block_rendering`, a bad `href` starting no load (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "tp3-01"
fixed = 'a filter or a background image'

[[pattern]]
id = "tp3-02"
fixed = 'no local gate builds'

[[pattern]]
id = "tp3-03"
fixed = '156 `.rs` files, 155 test targets'

[[pattern]]
id = "tp3-04"
fixed = 'three fixture tests'

[[pattern]]
id = "tp3-05"
fixed = 'runs 108 tests'

[[pattern]]
id = "tp3-06"
fixed = '`ci-scripts` `Ran 108`'

[[pattern]]
id = "tp3-07"
fixed = 'ancestor gained `hidden` panics'

[[pattern]]
id = "tp3-08"
fixed = 'has the local witness §2 Painted select list names'
