## 2026-11-01-upstream-sync-ui-graph — a failed load ends the wait
**Section:** registries/contracts/test-plan/session-lifecycle.md (session-start); §1 (test-scope-summary.md: blitz-test-harness, blitz-test-harness — a failed load ends the wait, the `stand_settle` clause); §3 (test-plan.md: A failed load and `settle`); §4 (test-plan.md and unit-test-strategy.md: blitz-test-harness)
**Change:**
- session-start: after `NotSettled(Loads)` at the bound it now reads that a failed load ends the wait where it would otherwise have run to `LOAD_WAIT`, `settle_waiting` unedited.
- §3, new row: the crate-private load counter forwards `NetHandler::error` to the handler it wraps, so a failed stylesheet reaches the document and stops counting as a pending render-blocking resource, and a harness document does not read `NotSettled { busy: Loads }` for it.
- §1, new record: `CountedHandler`'s forward ends the harness and driver wait, `settle_waiting` returning where it would otherwise have run to `LOAD_WAIT`. The `stand_settle` clause gains that a failed load ends the wait, read by `settle.rs`'s own unit test.
- §1 blitz-test-harness: the crate holds 24, where it read 22: the load counter's fifth, `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it`, and `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`.
- §4: `settle.rs` pins the counter's forward of `NetHandler::error`, the failed stylesheet reaching the document and unblocking it.
**Why:** `f6edb9701`'s failed-stylesheet fix stopped at `CountedHandler` on every harness-built document, `NetHandler::error`'s do-nothing default compiling silently through the wrapper, so a harness or driver session read `NotSettled(Loads)` for good; the forward, red before and green after, closes it. No gate read this, so a later take that extends a trait sweeps that trait's implementers.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the `embolden` step of `fast` and the font-embolden gap closed
**Section:** §1 (test-scope-summary.md: `select_popup_painted`); §2 (test-strategy.md: Painted select list); §5 (integration-test-strategy.md: Paint ↔ harness); §9 (test-plan.md: Local pre-push gate; ci-integration.md: The `embolden` leg, Red on `Test (macos)`)
**Change:**
- §9 Local pre-push gate: `fast` reads fmt → clippy → test → ci-scripts → embolden (was ending at ci-scripts); `embolden` is `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, a leg with no CI job of its own, macOS exercising the feature there.
- §9, new record: the leg is in `LEGS` and last in `FAST_LEGS`, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`.
- Retired, in ci-integration.md and test-strategy.md: «no local gate builds `blitz-paint/font-embolden`», and with it the row's reading that a local gate block goes green over a defect only macOS shows and its owner, a CARRY on Upstream sync ahead of the UI graph. The local witness of a paint change under the feature is now the `embolden` step.
- §5 and §1: `select_popup_painted`'s header weight under the feature is read locally by that step; the check also runs as the `embolden` leg.
**Why:** the chunk made the font-embolden witness a pinned step of `fast`, with a test holding it there, so the gap and its owner's line have nothing left to hold.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the key-press panic retired and its check
**Section:** §1 (test-scope-summary.md: blitz-dom (select) — the key press past a dropped block, `key_press_after_container_lost_its_box`); §2 (test-strategy.md: Directory pattern); §5 (integration-test-strategy.md: Harness ↔ event pipeline); §7 (test-data-fixtures.md: Lost-box control pages); §9 (ci-integration.md: Legs reaching `key_press_after_container_lost_its_box`)
**Change:**
- §1, was «the key press `select_popup` skips», a press panicking at `position_in_containing_block` for any focused control, no check reading it, owner a CARRY on «Upstream sync ahead of the UI graph»; now `select_popup` presses the key in every row, and a key press on a focused control whose anonymous block was dropped does not panic, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still naming the block; focus stays and the key still acts.
- §5: was one row skipping its key press, the test's comment saying why; now every row takes it through one `harness.press`.
- §1, new record: the target, 3 tests in both layout modes, for `hidden`, `style="display:none"` and CSS alone, a select's choice still moving past the dropped block, a control in no anonymous block reading the same.
- §9, new record: it runs in the workspace `test` leg and `run all`, no line of either script naming it; it has no `stand_` prefix and is none of `stand-repeat`'s seventeen words, so neither `stand-repeat` nor `run stand` runs it, and it is none of the `a11y` leg's fourteen files.
- §7, new row: its `format!`-built pages, `in_anonymous_block`, `in_no_anonymous_block`, `SELECT`, the enum `Lost`, `boot(page, incremental)`.
- §2: blitz-tests reads 157 `.rs` files and 156 test targets (was 156 and 155), this check the latest.
**Why:** the chunk ended the panic in the engine, its check and `select_popup`'s row 3 red before; the founder chose it in answer F1 A, by dialog in the escher overseer session, 2026-11-01, his label relayed verbatim by the operator (inputs#I3). That F1 A says focus stays and keys still act is the operator's reading of his fork text on that answer, not a new word of his (inputs#I10).
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the matrix job's free-disk step gone
**Section:** §4 (unit-test-strategy.md: CI workflows and leg script); §9 (ci-integration.md: The matrix job holds no free-disk step)
**Change:**
- §9, was «The matrix job's free-disk step never fires», skipped on every matrix row because its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest`; now `matrix_test` runs no free-disk step, pinned by `test_only_stand_repeat_frees_disk`.
- §4 names that pin.
**Why:** the chunk removed the step, and `Test (android)` read no such step in the run.
**Kept:** `Stand checks repeated`'s free-disk step stands, the one that frees disk.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the workspace tally and `run all`
**Section:** §3 (test-plan.md: Proof, the newest reading); §9 (ci-integration.md: the per-OS tallies, the local baseline's re-count)
**Change:**
- Per-OS, new reading on CI#41333172440 at `45c333f16`, green 17/17: linux 219 · 1299 · 0 · 12 (was 217 · 1236 · 0 · 12); macos and coverage 200 · 1296 · 0 · 12 (was 198 · 1233 · 0 · 12); windows 200 · 1248 · 0 · 9 (was 198 · 1185 · 0 · 9).
- Each of the four test jobs moved +2 result lines (`url_search_params`, `key_press_after_container_lost_its_box`) and +63 passed: the twelve's 58, the new check's 3, `text_contrast`'s 1, the counter's 1. Linux equals `target/ci-logs/test.log`, the other rows by the same arithmetic, not test by test.
- Local baseline: 219 result lines, 1299 passed · 0 failed · 12 ignored, +2 and +63 as above, timings not re-measured.
- `run all` reads 733 · 0 · 12 (was 727 · 0 · 12), the new check picked up with no script change.
**Why:** these are the chunk's measured readings: upstream's twelve commits, the founder's answer F2 A, by dialog in the escher overseer session, 2026-11-01, his label relayed verbatim by the operator (inputs#I3), with the key-press check and the chunk's two harness tests.
**Kept:** the a11y job 14 · 114 · 0 · 0, the stand repeat 228 · 1155 · 0 · 21 and `run stand` 237 · 0 · 7 over 51 files, all unmoved; the earlier readings stand as history, the new ones after them.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — line citations re-pointed after the take
**Section:** §1 to §10 (test-plan.md; ci-integration.md, mocking-stubbing-discipline.md, quality-gates-coverage-targets.md, test-data-fixtures.md, test-scope-summary.md, test-strategy.md, unit-test-strategy.md, integration-test-strategy.md)
**Change:** line numbers only, the claims those citations carry unmoved, for citations into `ci-leg.sh`, `ci.yml`, `test_ci_workflows.py`, blitz-dom's `document.rs`, `mutator.rs`, `net.rs`, `stylo.rs`, `node/node.rs` and `node/element.rs`, blitz-paint's `render.rs` and `render/background.rs`, blitz-traits' `net.rs`, blitz-test-harness's `settle.rs` and `contrast.rs`, `dioxus_document.rs`, `select_popup.rs` and `render_blocking_stylesheet.rs`.
**Why:** the take and the chunk's edits moved the cited lines.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the `url_search_params` target
**Section:** §1 (test-plan.md and test-scope-summary.md: blitz-vibey-script); §5 (test-plan.md: Script ↔ DOM; integration-test-strategy.md: Script ↔ DOM, `URLSearchParams`); §7 (test-data-fixtures.md: Inline HTML)
**Change:**
- §1 and §5 name tests/url_search_params.rs for the global `URLSearchParams` and `URL.prototype.searchParams`.
- test-scope-summary.md reads 4 `#[test]` functions there.
- §5, new record: `construction_and_form_encoding`, `mutations_and_live_iteration`, `invalid_inputs_and_receivers`, `url_search_params_are_bidirectionally_live`.
- §7, new row: `check(script)` builds a page with `format!` around each of the 4 tests' scripts.
**Why:** `91ef65e21` brought the class and its 4 tests, one of the twelve of the founder's answer F2 A, relayed verbatim by the operator (inputs#I3).
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `render_blocking_stylesheet` on a stylesheet that stops blocking
**Section:** §1 (test-scope-summary.md: tests/blitz-tests — `render_blocking_stylesheet` on a stylesheet that fails); §5 (integration-test-strategy.md: DOM ↔ resource loading, a stylesheet that stops blocking); §7 (test-data-fixtures.md: Stylesheet answers)
**Change:**
- §1 and §5, new records of the 3 tests: `failed_stylesheet_unblocks_rendering`; `empty_or_non_css_stylesheet_unblocks_rendering`, over an empty, an HTML and a lone `0xFF`-byte body; `invalid_stylesheet_url_does_not_block_rendering`, a bad `href` starting no load.
- §7, new row: those three cases, stated in the check.
**Why:** `f6edb9701` makes a failed, empty or non-CSS stylesheet stop blocking render and brought these tests, one of the twelve of the founder's answer F2 A, relayed verbatim by the operator (inputs#I3).
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — blitz-net's unit tests and test-only loopback binds
**Section:** §1 (test-plan.md and test-scope-summary.md: blitz-net); §4 (unit-test-strategy.md: blitz-net); §7 (test-data-fixtures.md: Loopback listeners); §8 (mocking-stubbing-discipline.md: Net handler)
**Change:**
- §1, was «no test — observed absent», searched over `packages/blitz-net/src/lib.rs`; now its `#[cfg(test)] mod tests` holds 3: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors`, `http_error_body_is_not_delivered_as_css`, the last two each over a test-only `TcpListener::bind` on `127.0.0.1:0`, served by no binary and reached by no caller.
- test-scope-summary.md and §4 gain records of the same.
- §7, new row: no fixture file, test-only TCP on loopback.
- §8, new row: a test-local `Handler` implements `NetHandler` beside the module's `fetch` helper returning `Result<Bytes, String>`; the network is no stand-in in the two that bind.
**Why:** the take brought the crate these tests, so the claim of absence no longer holds, and the plan's reading of one bind measured two.
**Kept:** the `file:` read, through `Url::to_file_path` off wasm, still has no test named for it.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the `keyboard_compat.rs` exclusion through this take
**Section:** §10 (quality-gates-coverage-targets.md: Coverage floors)
**Change:**
- The exclusion's owner: was the `.andromeda/residuals.md` line whose owner is Epoch 3's «Upstream sync ahead of the UI graph»; now that line's owner is «Upstream sync ahead of determinism and exploration».
- New sentence: the take of upstream/main `e42ee1f25`, thirty-six of 57 taken and twenty-one left out over merge base `23b9b26a9`, leaves `286874cc1` out, so `keyboard_compat.rs` stays and the exclusion stands.
**Why:** `286874cc1` is among the twenty-one left out; the delta moved to that owner by the operator's word on the phase card (inputs#I6).
**Kept:** the three floors and `coverage_floor.py` unchanged.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the coverage readings
**Section:** §10 (quality-gates-coverage-targets.md: Coverage floors)
**Change:**
- Read at CI#41333172440 (was CI#41235503227): dioxus-native-dom 2791/3241 = 86.12 % (was 2791/3240 = 86.14 %); blitz-test-harness 999/1040 = 96.06 % (was 968/1008 = 96.03 %), those two inferred from counts, not from the lcov.
- `coverage floor: 3 of 3 crates at or above their floors`.
**Why:** the take moved those two crates' line counts; the report reads the moved figures by inference, not from the lcov.
**Kept:** escher-driver 4959/5058 = 98.04 %, unmoved, and the floors.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — Cells' bytes corrected
**Section:** §1 (test-scope-summary.md: Served pages and the stand's unit tests); §7 (test-data-fixtures.md: Stand screen over the budget)
**Change:**
- The stand's Cells screen reads 47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET` (was 47,949 bytes).
- §7's qualifier «as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed» goes; it now reads through the binary.
**Why:** measured through the binary at the base and on this tree alike, so the take moved nothing and 47,949 had been stale since an earlier chunk. A plan quoting a flagged-old reading re-reads it.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — blitz-dom's unit tests from the take
**Section:** §1 (test-scope-summary.md: blitz-dom (document, mutator, net, form)); §4 (unit-test-strategy.md: blitz-dom (restyle, `mutator.rs`), blitz-dom (`document.rs`, `<picture>` and the inline flag)); §7 (test-data-fixtures.md: Restyle trees)
**Change:**
- §1: 83 `#[test]` functions (was 38), `document::` 24 (was 19), document.rs 15 (was 10) with `picture_display_tests` 1 and `inline_with_only_block_children_tests` 4, mutator.rs 52 (was 12) with upstream's 39 restyle tests and `custom_widget_replays_initial_attributes`.
- §4, new items: the 39 restyle tests comparing an incremental document against a fresh one through `check_incremental_restyle`; `CustomWidget`'s replay test; `picture_is_not_display_contents`; the four inline-flag tests, the flag detected and following child, text and `display: contents` descendant edits.
- §7, new row: trees built through `DocumentMutator` by `div`, `parent_with_children`, `hidden_parent_with_children`, `focus_tree`, `parent_with_text_and_children`, beside `PSEUDO_CSS`, held against a fresh snapshot by `assert_matches_fresh`.
**Why:** `fe33b778b` 39, `a26c6ddeb` 1, `dee854c76` 1 and `f76777766` 4, of the twelve of the founder's answer F2 A, relayed verbatim by the operator (inputs#I3).
**Kept:** net.rs 4 and form.rs 3, unmoved.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the CI scripts' count
**Section:** §1 (test-scope-summary.md: CI Python scripts); §4 (unit-test-strategy.md: CI workflows and leg script)
**Change:**
- The `ci-scripts` leg reads 110 tests (was 108) in eight files, test_ci_workflows.py's 35 (was 33).
- §4 names both new pins, `test_only_stand_repeat_frees_disk` and `test_fast_leg_ends_on_the_font_embolden_witness`; green at CI#41333172440 on `45c333f16` (was CI#40214713710 on `55bf3425`).
**Why:** the chunk added those two tests, one holding the removed free-disk step out and one holding `fast`'s last step.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — blending and the inline box's paint skip read on no frame
**Section:** §1 (test-plan.md and test-scope-summary.md: blitz-paint; test-scope-summary.md: blitz-paint — blending, blitz-dom (layout) — an inline box holding only block children); §2 (test-strategy.md: Pixel tests); §4 (unit-test-strategy.md: Test file location, blitz-paint's `layers.rs`)
**Change:**
- §1 and §4 add `layers.rs` to blitz-paint's inline tests: `blend_layers_are_not_replaced_by_clips`, `normal_layers_clip_unless_isolation_is_forced`, `disabled_or_exhausted_layers_do_not_pop`.
- New gap: `mix-blend-mode` and `isolation` paint on every document, and no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, resting on those 3 unit tests.
- New gap: `render_element` skips the own shadow, background and border of an inline box holding only block children, on no frame; `Node::is_inline_with_only_block_children` is held by 4 unit tests.
- §1 names both gaps' owner: a `CARRY:` on «Upstream sync ahead of determinism and exploration».
**Why:** the report measured no frame test of either. The owner is the operator's placement, the halt's Q2 (inputs#I10).
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — upstream's commits with no test here
**Section:** §1 (test-scope-summary.md: blitz-dom, blitz-paint — upstream commits with no test of their own; dioxus-native-dom — a second upstream test absent)
**Change:**
- New record: `8b1a0a3ce` (table damage), `7a3dd8918` (Stylo prefs before the UA sheet), `ae774d7f4` (sub-pixel computed sizes) and `e42ee1f25` (culling bounds) hold no test of their own, the workspace their witness, `incremental_oracle` 7 among its checks.
- New record: upstream's `element_removed_before_its_mounted_event_is_flushed` is left out and returns with `286874cc1`, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test of its own.
**Why:** the four commits brought no test; the left-out test reads 1 mounted event on Dioxus 0.7 where it asserts 0.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `text_contrast` past a blend
**Section:** §2 (test-strategy.md: Harness-driven tests); §3 (test-plan.md: `text_contrast` past a blend); §4 (unit-test-strategy.md: blitz-test-harness)
**Change:**
- `text_contrast`'s `None` list gains an element on the walk whose `mix-blend-mode` is other than `normal` (`paints_more_than_colour`, through `resolved_style_value`), beside `opacity` below 1, a filter and a background image.
- `Session::lint`'s `contrast` reads the same, no second formula, and the `None` moves no stand row.
- §4: `contrast.rs`'s fixture tests read four (was three), `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair` the new one.
**Why:** `6fca6308d` made `mix-blend-mode` paint, and the chunk made `Harness::text_contrast` read `None` past a blend; no stand row moves.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/
