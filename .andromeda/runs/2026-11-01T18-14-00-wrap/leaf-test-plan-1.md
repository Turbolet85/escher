group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
- leaf: .claude/docs/tests-summary.md:13   class: omission
  leaf-text: «no clock read but the harness's, no time advanced, no load waited on)»
  source: .andromeda/test-plan.md:135 «the crate-private load counter forwards `NetHandler::error` to the handler it wraps, so a failed stylesheet reaches the document and stops counting as a pending render-blocking resource»
  replace-with: «no clock read but the harness's, no time advanced, no load waited on; the crate-private load counter forwarding `NetHandler::error` to the handler it wraps, so a failed stylesheet reaches the document, stops counting as a pending render-blocking resource and does not read `NotSettled { busy: Loads }`, and the driver's `settle_waiting` returns where it would otherwise have run to `LOAD_WAIT`)»
- leaf: .claude/docs/tests-summary.md:13   class: stale
  leaf-text: «has `opacity` below 1, a filter or a background image, and `contrast_ratio` the WCAG 2.x ratio»
  source: .andromeda/test-plan-detail/test-strategy.md:8 «`None` with no text or where an element on the walk has `opacity` below 1, a filter, a background image or a `mix-blend-mode` other than `normal`»
  replace-with: «has `opacity` below 1, a filter, a background image or a `mix-blend-mode` other than `normal` (`paints_more_than_colour`, through `resolved_style_value`), `Session::lint`'s `contrast` reading the same with no second formula, and `contrast_ratio` the WCAG 2.x ratio»
- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 237 · 0 · 7 over 51 files at 2026-10-31-select-popup, where `run all` reads 727 · 0 · 12»
  source: .andromeda/test-plan.md:180 «`run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 733 · failed 0 · ignored 12»
  replace-with: «`run stand` 237 · 0 · 7 over 51 files at 2026-11-01-upstream-sync-ui-graph, where `run all` reads 733 · 0 · 12, `key_press_after_container_lost_its_box`, with no `stand_` prefix, picked up by `run all` with no script change; `run stand` 237 · 0 · 7 over 51 files at 2026-10-31-select-popup, where `run all` reads 727 · 0 · 12»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «stylesheet loading, Dioxus integration»
  source: .andromeda/test-plan-detail/test-scope-summary.md:247 «upstream's 3 tests, `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` over an empty, a non-CSS and a one-byte body»
  replace-with: «stylesheet loading (`render_blocking_stylesheet` 3, upstream's: `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` over an empty, a non-CSS and a one-byte body, and `invalid_stylesheet_url_does_not_block_rendering`, a bad `href` starting no load), Dioxus integration»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, 5 parts of `SNAPSHOT_TEXT_BUDGET`»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:27 «47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary»
  replace-with: «47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary»
- leaf: .claude/docs/tests-summary.md:21   class: retired
  leaf-text: «one row skipping its key press, a key pressed after an ancestor gained `hidden` panicking at `packages/blitz-dom/src/node/node.rs:1775` for any focused control, no check reading that panic»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:74 «Every row takes its key press through one `harness.press`, the key pressed after an ancestor gained `hidden` among them: it does not panic, focus stays and the key still acts»
  replace-with: «every row taking its key press through one `harness.press`, the key pressed after an ancestor gained `hidden` among them: it does not panic, focus stays and the key still acts»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «each run by the workspace `test` leg, `run all`, by name and as a named `--test` of the `stand-repeat` leg, not by `run stand` or the `a11y` leg»
  source: .andromeda/test-plan-detail/test-scope-summary.md:283 «as the `embolden` leg, the last step of `fast`, with no CI job»
  replace-with: «each run by the workspace `test` leg, `run all`, by name and as a named `--test` of the `stand-repeat` leg, not by `run stand` or the `a11y` leg, `select_popup_painted` also, with `--features blitz-paint/font-embolden`, as the `embolden` leg, the last step of `fast`, with no CI job»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the highlight in no snapshot and no tree its four stated limits — and process telemetry»
  source: .andromeda/test-plan-detail/test-scope-summary.md:73 «a blitz-tests target of 3 tests, each in both layout modes, with no `stand_` prefix, so `run all` runs it and `run stand` does not»
  replace-with: «the highlight in no snapshot and no tree its four stated limits, since 2026-11-01-upstream-sync-ui-graph `key_press_after_container_lost_its_box` (3, its own target with no `stand_` prefix, each test in both layout modes: a key press with focus on a control whose anonymous block was dropped not panicking for `hidden`, `style="display:none"` and CSS alone, a select's choice still moving past the dropped block, and a control in no anonymous block reading the same; run by the workspace `test` leg and `run all`, none of `stand-repeat`'s seventeen named `--test` words and none of the `a11y` leg's fourteen files, so not by `stand-repeat`, `run stand` or the `a11y` leg) — and process telemetry»
- leaf: .claude/docs/tests-summary.md:22   class: omission
  leaf-text: «`blitz-shell`: one inline unit test»
  source: .andromeda/test-plan.md:30 «its `#[cfg(test)] mod tests` pins a load's delivery — `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`»
  replace-with: «`blitz-net`: 3 inline unit tests, `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, the last two each over a test-only `TcpListener::bind` on `127.0.0.1:0`, served by no binary and reached by no caller; `blitz-shell`: one inline unit test»
- leaf: .claude/docs/tests-summary.md:23   class: stale
  leaf-text: «22 inline unit tests, 5 of them in `contrast.rs`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:130 «the crate holds 24, the load counter's fifth `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it` in `settle.rs`»
  replace-with: «24 inline unit tests, 6 of them in `contrast.rs`»
- leaf: .claude/docs/tests-summary.md:23   class: omission
  leaf-text: «an element with no text reading `None`), 6 in `screenshot.rs`»
  source: .andromeda/test-plan-detail/unit-test-strategy.md:26 «a blended ancestor reads `None` and an unblended one its pair»
  replace-with: «an element with no text reading `None`, a blended ancestor reading `None` and an unblended one its pair), 6 in `screenshot.rs`»
- leaf: .claude/docs/tests-summary.md:23   class: stale
  leaf-text: «and 11 in `settle.rs`, the crate's first (the load counter, the settle loop»
  source: .andromeda/test-plan-detail/unit-test-strategy.md:26 «a failed stylesheet reaches the document through the counter's forward of `NetHandler::error` and unblocks it»
  replace-with: «and 12 in `settle.rs`, the crate's first (the load counter, a failed stylesheet reaching the document through the counter's forward of `NetHandler::error` and unblocking it among its five, the settle loop»
- leaf: .claude/docs/tests-summary.md:26   class: retired
  leaf-text: «no local gate builds `blitz-paint/font-embolden`, so a paint change under that OS-gated feature has its local witness in `cargo test»
  source: .andromeda/test-plan-detail/test-strategy.md:40 «a paint change under the OS-gated `blitz-paint/font-embolden` feature has its local witness in the `embolden` step, the last of `ci-leg.sh fast` with no CI job»
  replace-with: «a paint change under the OS-gated `blitz-paint/font-embolden` feature has its local witness in the `embolden` step, the last of `ci-leg.sh fast` with no CI job, `cargo test»
- leaf: .claude/docs/tests-summary.md:26   class: omission
  leaf-text: «`Test (macos)` reads through `apple-font-embolden`.»
  source: .andromeda/test-plan-detail/test-strategy.md:34 «`mix-blend-mode` and `isolation` paint with no frame check — `layers.rs`'s 3 unit tests read the layer manager, and no test paints a blended element to a frame»
  replace-with: «`Test (macos)` reads through `apple-font-embolden`; `mix-blend-mode` and `isolation` paint with no frame check — `layers.rs`'s 3 unit tests read the layer manager, and no test paints a blended element to a frame or names `convert_blend_mode` — and the paint skip of an inline box holding only block children is read by `document.rs`'s 4 unit tests of `is_inline_with_only_block_children`, on no frame, owner of both gaps a `CARRY:` on Upstream sync ahead of determinism and exploration.»
- leaf: .claude/docs/tests-summary.md:27   class: omission
  leaf-text: «(`tests/selection.rs`, since the 2026-10-10 upstream merge) + 2 headless Preact TodoMVC flows»
  source: .andromeda/test-plan-detail/test-scope-summary.md:155 «tests/url_search_params.rs holds 4 `#[test]` functions of the global `URLSearchParams` and `URL.prototype.searchParams`»
  replace-with: «(`tests/selection.rs`, since the 2026-10-10 upstream merge), 4 of the global `URLSearchParams` and `URL.prototype.searchParams` (`tests/url_search_params.rs`) + 2 headless Preact TodoMVC flows»
- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «fast legs fmt · clippy · `test` (`cargo test --workspace --locked`) · CI scripts, then build»
  source: .andromeda/test-plan.md:391 «`bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts → embolden — `embolden` the `blitz-paint/font-embolden` witness»
  replace-with: «fast legs fmt · clippy · `test` (`cargo test --workspace --locked`) · CI scripts · `embolden` (the `blitz-paint/font-embolden` witness, a leg with no CI job of its own, macOS exercising the feature there), then build»
- leaf: .claude/docs/tests-summary.md:29   class: retired
  leaf-text: «whose own free-disk step is skipped on every row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row running `ubuntu-latest`»
  source: .andromeda/test-plan-detail/ci-integration.md:70 «`matrix_test` runs none, `Stand checks repeated`'s the one that frees disk (§9 above), pinned by `test_only_stand_repeat_frees_disk`»
  replace-with: «whose job, `matrix_test`, runs no free-disk step, `Stand checks repeated`'s the one that frees disk, pinned by `test_only_stand_repeat_frees_disk`»
- leaf: .claude/docs/tests-summary.md:29   class: retired
  leaf-text: «`apple-font-embolden` making it red on macOS alone and no local gate building `blitz-paint/font-embolden`, so a local gate block reads green over a defect only macOS shows, a paint change under that feature having its local witness in `cargo test»
  source: .andromeda/test-plan-detail/ci-integration.md:46 «`apple-font-embolden` made it red on macOS alone; a paint change under that feature has its local witness in the `embolden` step of `fast`, its last»
  replace-with: «`apple-font-embolden` making it red on macOS alone, a paint change under that feature having its local witness in the `embolden` step of `fast`, its last, `cargo test»
- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «`test_ci_workflows.py` pins the workflow invariants — every package install bounded among them —»
  source: .andromeda/test-plan-detail/unit-test-strategy.md:59 «`test_only_stand_repeat_frees_disk` pins that only `Stand checks repeated` frees disk»
  replace-with: «`test_ci_workflows.py` pins the workflow invariants — every package install bounded, only `Stand checks repeated` freeing disk (`test_only_stand_repeat_frees_disk`) and `fast` ending on the font-embolden witness (`test_fast_leg_ends_on_the_font_embolden_witness`) among them —»
- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «108 CI-script tests in eight files — 104 with»
  source: .andromeda/test-plan-detail/unit-test-strategy.md:60 «the CI-scripts leg runs 110 tests over eight test files — test_ci_workflows.py's 35»
  replace-with: «110 CI-script tests in eight files, test_ci_workflows.py's 35 — 108 until 2026-11-01-upstream-sync-ui-graph; 104 with»
- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1236 · 0 · 12 over 217 result lines»
  source: .andromeda/test-plan-detail/ci-integration.md:124 «Re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored»
  replace-with: «`cargo test --workspace` 1299 · 0 · 12 over 219 result lines with upstream's twelve commits and the key-press check at 2026-11-01-upstream-sync-ui-graph (from 217 · 1236: +2 result lines — `url_search_params`, `key_press_after_container_lost_its_box` — and +63 passed — the twelve's 58, the new check's 3, `text_contrast`'s 1, the counter's 1; `run stand` 237 passed · 0 failed · 7 ignored over 51 files, `run all` 733 passed · 0 failed · 12 ignored; timings not re-measured); 1236 · 0 · 12 over 217 result lines»
- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «read now on CI#41235503227 at 2026-10-31-select-popup under the same floors escher-driver 98.04 %, dioxus-native-dom 86.14 %, blitz-test-harness 96.03 %, the 97 % floor holding»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:17 «dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, those two inferred from counts, not from the lcov»
  replace-with: «read now on CI#41333172440 at 2026-11-01-upstream-sync-ui-graph under the same floors escher-driver 98.04 %, dioxus-native-dom 86.12 %, blitz-test-harness 96.06 %, those two inferred from counts, not from the lcov, the 97 % floor holding»
