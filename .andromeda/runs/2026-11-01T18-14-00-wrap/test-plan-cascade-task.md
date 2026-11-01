# Task — test-plan · 2026-11-01-upstream-sync-ui-graph

## The session
Took twelve upstream DioxusLabs/blitz commits by no-commit cherry-pick, upstream read at e42ee1f25, merge base 23b9b26a9, 36 of 57 taken and 21 left out: restyle narrowing, table damage, `<picture>`, failed stylesheets through `NetHandler::error`, `mix-blend-mode` and `isolation` painting, Stylo prefs before the UA sheet, `URLSearchParams`, `CustomWidget` attribute replay, inline decorations, mounted events, sub-pixel computed sizes and culling bounds.
Added escher's own changes: `deallocate_anonymous_block` clears stale `layout_parent` ids, ending the key-press panic; `CountedHandler` forwards `NetHandler::error`, so a failed load ends the wait; `Harness::text_contrast` reads `None` past a blended element.
Added tests: the blitz-tests target `key_press_after_container_lost_its_box`, the blitz-vibey-script target `url_search_params`, unit tests in mutator.rs, document.rs, layers.rs, blitz-net, contrast.rs and settle.rs, 3 more in `render_blocking_stylesheet`, and 2 CI script tests.
Added the `embolden` leg as the last step of the local `fast` gate, with no CI job.
Retired the key-press panic limit, the `matrix_test` free-disk step, the gap that no local gate built `blitz-paint/font-embolden`, `select_popup`'s row-3 skip, the orphan taffy re-export comment and `Node`'s mix-blend-mode TODO.
Measured CI#41333172440 green at 45c333f16, checks 17/17, wall 1369 s, with new per-OS tallies, `run all` 733 · 0 · 12, CI scripts 110, harness lib 24, and Cells at 47,089 bytes where the plan held a stale 47,949.

## Reached files
- packages/blitz-paint/src/blend.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js · merge surface
- packages/blitz-vibey-script/tests/url_search_params.rs · merge surface
- tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
- packages/blitz-dom/assets/default.css · merge surface
- packages/blitz-dom/src/document.rs · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/layout/damage.rs · merge surface
- packages/blitz-dom/src/lib.rs
- packages/blitz-dom/src/mutator.rs · merge surface
- packages/blitz-dom/src/net.rs · merge surface
- packages/blitz-dom/src/node/custom_widget.rs
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/node/node.rs · merge surface
- packages/blitz-dom/src/resolved_style.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-net/src/lib.rs · merge surface
- packages/blitz-paint/src/layers.rs · merge surface
- packages/blitz-paint/src/lib.rs · merge surface
- packages/blitz-paint/src/render.rs · merge surface
- packages/blitz-paint/src/render/background.rs
- packages/blitz-paint/src/render/box_shadow.rs
- packages/blitz-test-harness/src/contrast.rs
- packages/blitz-test-harness/src/settle.rs
- packages/blitz-traits/src/net.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · merge surface
- tests/blitz-tests/tests/render_blocking_stylesheet.rs · merge surface
- tests/blitz-tests/tests/select_popup.rs
- wpt/runner/timeout-quarantine.txt · merge surface
- .github/workflows/ci.yml
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- Workspace tally, ci-integration.md:88, a new reading after its history rows: linux 219 · 1299 · 0 · 12, macos and coverage 200 · 1296 · 0 · 12, windows 200 · 1248 · 0 · 9; +2 result lines and +63 passed; a11y 14 · 114 · 0 · 0 and repeat 228 · 1155 · 0 · 21 unmoved.
- `run all` reading: 727 · 0 · 12 moves to 733 · 0 · 12, +3 the new check and +3 `render_blocking_stylesheet`; test-plan.md:177 is the line this reading follows; test-plan.md:176's offsets and other citation digits ending in 727 stay as they are.
- CI scripts count, test-scope-summary.md:8 and unit-test-strategy.md:54: 108 moves to 110; ci-scripts citations into test_ci_workflows.py are re-pointed.
- blitz-test-harness library count: 22 moves to 24; ci-integration.md:105 holds it only as history; test-scope-summary.md:100's count of `settle.rs` inline tests on the load counter, which stands at 4, moves +1.
- Load counter's sentences, test-plan.md:207 and unit-test-strategy.md:20: `settle.rs` now also pins that the counter forwards `NetHandler::error`; other `settle.rs` citation digits are re-pointed.
- Load wait and settle, test-scope-summary.md:157 and the §3 pump semantics: a failed, empty, non-CSS or bad-`href` stylesheet stops blocking render, so a harness or driver session no longer reads `NotSettled(Loads)` for good or runs to `LOAD_WAIT` on it; `settle_waiting` itself is unedited.
- The `fast` leg's order, test-plan.md:388 ×2: fmt → clippy → test → ci-scripts → embolden; ci-integration.md:71 and :98-110 and unit-test-strategy.md:55-56 name the command or an old run and do not change.
- The `font-embolden` gap, ci-integration.md:34, test-strategy.md:40 and integration-test-strategy.md:74: retired, each now the `embolden` step of `fast`.
- The `matrix_test` free-disk step, ci-integration.md:58 ×2: removed; ci-integration.md:46 and :52 name `Stand checks repeated`'s step, which stands, its watch at 3 green runs with the retirement condition met and no cure shown.
- The key-press panic limit, test-scope-summary.md:55 ×2 and §1: retired; the owner pointer on that line, and those on ci-integration.md:34 and quality-gates-coverage-targets.md:15, are re-pointed to the owner the card names or to nothing.
- `select_popup`, integration-test-strategy.md:68: no longer skips row 3; `select_popup.rs:513` is one `harness.press` for every row.
- `text_contrast`'s `None` conditions, test-strategy.md:8 and test-plan.md:131: gain an element on the walk with a `mix-blend-mode` other than `normal`; other lines that only name the reader do not change, and no `stand_contrast` row moves.
- `blitz-net` absence in §1 (test-plan.md:30 and its detail): the no-test claim and its search are now false, since `packages/blitz-net/src/lib.rs` holds a `#[cfg(test)] mod tests` of 3 tests.
- Inventories in §1 and §4 that list where inline tests live: blitz-paint now has `layers.rs` tests beside gradient.rs and css_box.rs; `document.rs` and `mutator.rs` hold new modules and tests through the take.
- blitz-vibey-script's test inventory, §1, §2 and §5 Script ↔ DOM: a new `tests/url_search_params.rs` beside dom.rs, selection.rs and preact.rs.
- Upstream line in quality-gates-coverage-targets.md:15: its existing text is the last take's history, and a new sentence for this take goes beside it: e42ee1f25, 36 of 57 taken, 21 left out; the left-out sites there ×2 are this fact's class.
- Coverage readings, quality-gates-coverage-targets.md:16 ×3: dioxus-native-dom 86.12 % (2791/3241), blitz-test-harness 96.06 % (999/1040), escher-driver 98.04 % unmoved; floors held 3 of 3, the two moved inferred from counts and not from lcov.
- Cells size, test-scope-summary.md:270 and test-data-fixtures.md:26: 47,949 bytes is stale since an earlier chunk; measured 47,089 bytes, 740 lines, 5 parts; amend both.
- Line citations into the edited engine files (document.rs, mutator.rs, node/node.rs, node/element.rs, net.rs, stylo.rs, render.rs, render/background.rs, layout/damage.rs, dioxus_document.rs, contrast.rs, settle.rs): insertions and removals move the digits of citations past them, including the commented-out `stylo.rs` tests and the `render.rs` and `background.rs` paint comments.
- Line citations into the CI files: `ci-leg.sh` gained lines at 8-9 and 28-29, so its cited lines move; `ci.yml` lost its lines at 392-398 at the base, so citations past that point move, the matrix's `:411-415` among them; `test_ci_workflows.py` gained lines at 110-116 and 361-376.
- CI workflow invariants in §4: test_ci_workflows.py now also pins that only `Stand checks repeated` frees disk and that `fast` ends on the font-embolden witness.
- Stand-repeat's named targets in §9 Legs and §10 Flakiness budget: the count is read and stays seventeen, since the new check joins no leg; a sentence naming the new check must not raise it.
- `<picture>` behaviour: the UA sheet no longer sets `display: contents` on it, every select rule staying; any text relying on the old rule moves.
- `Harness`-visible engine behaviour through the take: restyle narrowing on attribute and child-list edits, sub-pixel resolved `width` and `height`, inline boxes with only block children skipping their own paint; text stating these readings or their damage assumptions can go stale, among them §2's pure-restyle paths note that mutations insert full damage.
- `CustomWidget`: `set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget`; test text naming that widget flow may need it.
- WPT quarantine, §7 and §4: two WPT files join `timeout-quarantine.txt` at lines 105 and 107; test-plan.md:315 names the file and states no count, so only a stated count would move.

## New facts
- F1 · test-scope-summary · `key_press_after_container_lost_its_box` is a blitz-tests target of 3 tests in both layout modes, no `stand_` prefix so `run all` alone moves; a key press with focus on a control whose anonymous block was dropped does not panic for `hidden`, `style=display:none` and CSS alone, a select's choice still moves past the dropped block, and a control in no anonymous block reads the same.
- F2 · test-scope-summary · A failed load now ends the harness and driver wait: `CountedHandler` forwards `NetHandler::error` and `settle_waiting` returns where it would otherwise have run to `LOAD_WAIT`.
- F3 · test-scope-summary · `blitz-net` holds 3 inline unit tests over two test-only `TcpListener::bind` calls on `127.0.0.1:0`, served by no binary and reached by no caller: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`.
- F4 · test-scope-summary · No fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; a grep over `tests/` finds only `contrast.rs`'s fixture, so `mix-blend-mode` and `isolation` painting rest on the 3 `layers.rs` unit tests, and P5 names the gap's owner.
- F5 · test-scope-summary · `is_inline_with_only_block_children` and its paint skip have no frame test; the reader is held by 4 unit tests in `document.rs`.
- F6 · test-scope-summary · Upstream commits 8b1a0a3ce, 7a3dd8918, ae774d7f4 and e42ee1f25 brought no test of their own, the workspace their witness, with `incremental_oracle` at 7 unmoved.
- F7 · test-scope-summary · Upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out because it reads 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with 286874cc1, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test of its own.
- F8 · integration-test-strategy · blitz-vibey-script's `url_search_params` target holds 4 tests of the global `URLSearchParams` and `URL.prototype.searchParams`: `construction_and_form_encoding`, `mutations_and_live_iteration`, `invalid_inputs_and_receivers` and `url_search_params_are_bidirectionally_live`.
- F9 · integration-test-strategy · `render_blocking_stylesheet` adds `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` and `invalid_stylesheet_url_does_not_block_rendering`.
- F10 · unit-test-strategy · `mutator.rs` holds upstream's 39 restyle tests from fe33b778b, comparing an incremental document against a fresh one through `check_incremental_restyle` over hidden subtrees, siblings, pseudo-elements and focus-within, beside `custom_widget_replays_initial_attributes`.
- F11 · unit-test-strategy · `document.rs` adds `picture_display_tests`, one test `picture_is_not_display_contents`, and `inline_with_only_block_children_tests`, four tests of the flag following child, text and `display: contents` descendant edits.
- F12 · unit-test-strategy · `layers.rs` holds 3 unit tests: `blend_layers_are_not_replaced_by_clips`, `normal_layers_clip_unless_isolation_is_forced` and `disabled_or_exhausted_layers_do_not_pop`.
- F13 · unit-test-strategy · `contrast.rs` adds `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`, and `settle.rs` adds `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it`.
- F14 · unit-test-strategy · test_ci_workflows.py adds `test_only_stand_repeat_frees_disk` and `test_fast_leg_ends_on_the_font_embolden_witness`.
- F15 · ci-integration · `ci-leg.sh` holds a leg `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, in `LEGS` and last in `FAST_LEGS`, with no CI job, macOS exercising the feature in CI.
- F16 · test-strategy · `text_contrast` reads `None` past an element on its walk with a `mix-blend-mode` other than `normal` (`paints_more_than_colour`, through `resolved_style_value`), and `Session::lint`'s `contrast` reads the same with no second formula.

## Untouched
- apps/browser's test inventories and helpers: no file under apps/ was reached.
- escher-driver's own unit-test inventory, wire grammar, verbs and refusals: packages/escher-driver has no diff.
- escher-telemetry's tests and the stand log format and its measured line counts: packages/escher-telemetry has no diff, no log site or `tracing` site was added, and `stand_act_spans` still reads eight fields.
- examples/seven_guis, its stand boots, pinned options, `LoadNet`, `TimerTicks`, process checks and flows: no diff; `cargo test -p seven_guis` still reads 12 · 44.
- The agent-run contract, scripts/agent-run.sh, its verbs, exit grammar, events and state, and `run stand` at 237 · 0 · 7 over 51 files: the script is unedited and the new check has no `stand_` prefix.
- The cold-agent pipe, its verdict, proof and run: scripts/cold-agent.sh is not reached; the word isolation at test-plan.md:219 and unit-test-strategy.md:69 is another sense than CSS `isolation`.
- Third-party test frameworks, test-that and the crate dependency lists: no Cargo.toml, Cargo.lock or deny.toml moved.
- `BlitzTestsTargetsTest` and the blitz-tests manifest rule: test_blitz_tests_targets.py and the manifest are unedited, and the new target is picked up by its file.
- The coverage floor values 97, 83 and 90 % and coverage_floor.py: unmoved; only the readings moved.
- The capability statement's pin, test_capability_statement.py and docs/capabilities.md: neither edited; the stale fifteen there is not read by that pin and is not this take's move.
- `HeldNet`, `LateNet` and `LoadNet`: their files are not reached, and `HeldNet` still answers through `bytes`.
- The shared check modules tests/common, tests/session_common and tests/fixtures: none edited; the new check holds its own boot helper.
- The WPT workflow, scoring and runner sources (wpt.yml, wpt/runner/src): not reached.
- The mutation tier and .config/nextest.toml: not reached and run on the dev host alone.
- The committed checks under tests/blitz-tests/checks and the Preact example fixtures: not reached.
