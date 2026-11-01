# Apply — test-plan · part 4 of 4

## Verdicts
- fanout-test-plan--cascade-g4.md 15 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:26#1
- cascade-compare-test-plan.md 9 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:25+#1
- cascade-compare-test-plan.md 10 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:29+#1
- cascade-compare-test-plan.md 11 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:35+#1
- cascade-compare-test-plan.md 12 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:45+#1
- cascade-compare-test-plan.md 13 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:45+#2
- fanout-test-plan--cascade-g5.md 1 · applied · .andromeda/test-plan-detail/ci-integration.md:13+#1
- fanout-test-plan--cascade-g5.md 3 · applied · .andromeda/test-plan-detail/ci-integration.md:34#1
- fanout-test-plan--cascade-g5.md 4 · applied · .andromeda/test-plan-detail/ci-integration.md:58#1
- fanout-test-plan--cascade-g5.md 5 · applied · .andromeda/test-plan-detail/ci-integration.md:88+#1
- fanout-test-plan--cascade-g5.md 6 · applied · .andromeda/test-plan-detail/ci-integration.md:110+#1
- cascade-compare-test-plan.md 1 · applied · .andromeda/test-plan-detail/ci-integration.md:19+#1
- fanout-test-plan--cascade-g5.md 7 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15#1, .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15+#1
- fanout-test-plan--cascade-g5.md 8 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16#1
- cascade-compare-test-plan.md 8 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19+#1

## Edits
- line: .andromeda/test-plan-detail/test-data-fixtures.md:26
  old: «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/counts.md»
  new: «47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/counts.md»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:25
  anchor: «| Select list pages (`rsx!`, each in its check's own file) | `select_popup`'s `page`, `stated_page` (an author's `aria-expanded` and `open`), `clipped_page`, `long_page`, `sized_page` (`size` on a `multiple` select), `focus_moved_page` (a `:focus` rule moving the select's box) and `chosen_disabled_page` (a disabled chosen option); `select_popup_painted`'s `page`, `clipped_page`, `grouped_page` and `long_page`; `stand_act_popup`'s `long_page`, held through `hold_long` | (tests/blitz-tests/tests/select_popup.rs:32-107; tests/blitz-tests/tests/select_popup.rs:612-622; tests/blitz-tests/tests/select_popup.rs:649-671; tests/blitz-tests/tests/select_popup.rs:853-864; tests/blitz-tests/tests/select_popup.rs:973-989; tests/blitz-tests/tests/select_popup.rs:1088-1099; tests/blitz-tests/tests/select_popup.rs:1120-1130; tests/blitz-tests/tests/select_popup_painted.rs:22-34; tests/blitz-tests/tests/select_popup_painted.rs:189-209; tests/blitz-tests/tests/select_popup_painted.rs:255-267; tests/blitz-tests/tests/select_popup_painted.rs:299-310; tests/blitz-tests/tests/stand_act_popup.rs:168-196) |»
  add: «| Lost-box control pages (`format!`-built, in their check's own file) | `key_press_after_container_lost_its_box`'s `in_anonymous_block(control, sheet)` and `in_no_anonymous_block(control)`, with its `SELECT` constant and the enum `Lost` of the three ways a control loses its box (`hidden`, `style="display:none"`, CSS alone), each page booted as a `Harness<HtmlDocument>` by `boot(page, incremental)` in both layout modes | (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:17-18; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:21-28; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:32-39; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:41-48; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:50-58) |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:29
  anchor: «| Files on disk (script tests) | `temp_dir_with_files` writes a directory of sibling files under `std::env::temp_dir()` at test time for the two `file:`-load tests — test data on disk beside the in-code seeds, through a test-only `TMPDIR` read (arch §Occupied Resources) | (packages/blitz-vibey-script/tests/dom.rs:900-909) |»
  add: «| Inline HTML (`URLSearchParams` script tests) | `check(script)` in blitz-vibey-script's `url_search_params` target builds a page with `format!` around each of its 4 tests' scripts | (packages/blitz-vibey-script/tests/url_search_params.rs:4-13) |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:35
  anchor: «| Style fixtures | inline user-agent stylesheets | (packages/blitz-dom/src/document.rs:3708-3711; packages/blitz-dom/src/document.rs:4003-4007) |»
  add: «| Restyle trees (mutator unit tests) | trees built in code through `DocumentMutator` by `div`, `parent_with_children`, `hidden_parent_with_children`, `focus_tree` and `parent_with_text_and_children`, beside the inline sheet `PSEUDO_CSS`, each incremental snapshot held against a fresh one by `assert_matches_fresh` | (packages/blitz-dom/src/mutator.rs:2189-2192; packages/blitz-dom/src/mutator.rs:2218-2242; packages/blitz-dom/src/mutator.rs:2304-2311; packages/blitz-dom/src/mutator.rs:2313-2318; packages/blitz-dom/src/mutator.rs:2664-2665; packages/blitz-dom/src/mutator.rs:2764-2776; packages/blitz-dom/src/mutator.rs:2800-2812) |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:45
  anchor: «| Network answer (stand load page) | `stand::boot_load`'s stand-in `LoadNet` answers every fetch of the fixed URL `http://stand.test/load.css` with the fixed bytes `LOADED_CSS` (`.load-body .load-done { display: block; }`) from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms); every other stand boot keeps `net_provider: None`; read by `stand_load_page` and the stand unit test `the_load_page_click_leaves_a_load_in_flight`; the load page is also booted by `stand_keyboard`'s `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `stand_contrast`'s `the_load_page_meets_its_ratios` | (examples/seven_guis/src/stand.rs:154-175; tests/blitz-tests/tests/stand_load_page.rs:35-111; tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_contrast.rs:670-700) |»
  add: «| Stylesheet answers (`render_blocking_stylesheet`) | three cases stated in the check: a failed load; answered bodies looped over empty bytes, an HTML document and a lone `0xFF` byte; and a bad `href`, which starts no load | (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107) |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:45
  anchor: «| Network answer (stand load page) | `stand::boot_load`'s stand-in `LoadNet` answers every fetch of the fixed URL `http://stand.test/load.css` with the fixed bytes `LOADED_CSS` (`.load-body .load-done { display: block; }`) from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms); every other stand boot keeps `net_provider: None`; read by `stand_load_page` and the stand unit test `the_load_page_click_leaves_a_load_in_flight`; the load page is also booted by `stand_keyboard`'s `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `stand_contrast`'s `the_load_page_meets_its_ratios` | (examples/seven_guis/src/stand.rs:154-175; tests/blitz-tests/tests/stand_load_page.rs:35-111; tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_contrast.rs:670-700) |»
  add: «| Loopback listeners (blitz-net unit tests) | no fixture file: `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, in the crate's `#[cfg(test)] mod tests`, each bind a `std::net::TcpListener` on `127.0.0.1:0`, test-only TCP on loopback, served by no binary and reached by no caller (arch §Occupied Resources); their stand-in handler is §8's | (packages/blitz-net/src/lib.rs:522-530; packages/blitz-net/src/lib.rs:532-545) |»
- after: .andromeda/test-plan-detail/ci-integration.md:13
  anchor: «»
  next: «<!-- detail: values · ci -->»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **The `embolden` leg:** `ci-leg.sh` holds a leg `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, in `LEGS` and last in `FAST_LEGS`, with no CI job, macOS exercising the feature in CI; `test_fast_leg_ends_on_the_font_embolden_witness` pins it (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29; .github/scripts/test_ci_workflows.py:361-375) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/test-plan-detail/ci-integration.md:19
  anchor: «»
  next: «<!-- detail: values · ci -->»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **Legs reaching `key_press_after_container_lost_its_box`:** the blitz-tests target `key_press_after_container_lost_its_box` runs in the workspace `test` leg and in `agent-run.sh`'s `run all`, each taking it as one more target with no line of its script naming it; its 3 tests run in both layout modes; it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it, and it is none of the `a11y` leg's fourteen files (§9 Legs above) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- line: .andromeda/test-plan-detail/ci-integration.md:34
  old: «made it red on macOS alone, and no local gate builds `blitz-paint/font-embolden` (owner: a CARRY on Upstream sync ahead of the UI graph), so a local gate block reads green over a defect only macOS shows; a paint change under that feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced macOS's two numbers on linux»
  new: «made it red on macOS alone; a paint change under that feature has its local witness in the `embolden` step of `fast`, its last (§9 The `embolden` leg), whose command reproduced macOS's two numbers on linux»
- line: .andromeda/test-plan-detail/ci-integration.md:58
  old: «**The matrix job's free-disk step never fires:** the free-disk step at .github/workflows/ci.yml:392-394 is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest` (`completed · skipped` on CI#41190771843's `Test (android)`) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md»
  new: «**The matrix job holds no free-disk step:** `matrix_test` runs none, `Stand checks repeated`'s the one that frees disk (§9 above), pinned by `test_only_stand_repeat_frees_disk` (.github/scripts/test_ci_workflows.py:110-115) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md»
- after: .andromeda/test-plan-detail/ci-integration.md:88
  anchor: « Re-read on CI#41235503227, the final HEAD `715798ce2` (green 17/17): linux 217 · 1236 · 0 · 12; macos and coverage 198 · 1233 · 0 · 12; windows 198 · 1185 · 0 · 9; the a11y job 14 · 114 · 0 · 0; the stand repeat 228 · 1155 · 0 · 21 — +3 result lines and +55 passed on linux, macos and coverage over CI#41192433140, +3 · +53 on windows, no new result line and +2 passed on the a11y job, +9 · +147 on the stand repeat — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md»
  add: « Re-read on CI#41333172440, the pre-CI commit `45c333f16` (green 17/17): linux 219 · 1299 · 0 · 12; macos and coverage 200 · 1296 · 0 · 12; windows 200 · 1248 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved — +2 result lines (`url_search_params`, `key_press_after_container_lost_its_box`) and +63 passed on each of the four test jobs over CI#41235503227: the twelve's 58 (`fe33b778b` 39 · `dee854c76` 1 · `f6edb9701` 6 · `6fca6308d` 3 · `91ef65e21` 4 · `a26c6ddeb` 1 · `f76777766` 4), the new check's 3, `text_contrast`'s 1, the counter's 1; linux equal to `target/ci-logs/test.log`, the other rows by the same arithmetic, not test by test — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md»
- after: .andromeda/test-plan-detail/ci-integration.md:110
  anchor: « re-counted with the five new targets: 214 result lines, 1181 passed · 0 failed · 12 ignored — +5 result lines (`visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2) and +56 passed (those 17, 31 more in the edited checks, blitz-dom's lib 6, blitz-shell's lib 1, blitz-vibey-script's `dom` 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (a `ci-leg.sh fast` run's tally probe); re-counted with the select list's three new targets: 217 result lines, 1236 passed · 0 failed · 12 ignored — +3 result lines (`select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12) and +55 passed (those 43, blitz-dom's lib `select_list` 6, `stand_keyboard` +2, `stand_lint` +1, `cli_commands`, `host_log` and `mcp_log` +1 each), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md (CI#41235503227's linux job)»
  add: « Re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored — +2 result lines (`url_search_params`, `key_press_after_container_lost_its_box`) and +63 passed (the twelve's 58, the new check's 3, `text_contrast`'s 1, the counter's 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md (`target/ci-logs/test.log`, equal to CI#41333172440's linux job)»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
  old: «whose owner is Epoch 3's «Upstream sync ahead of the UI graph» — as measured»
  new: «whose owner is «Upstream sync ahead of determinism and exploration» — as measured»
- after: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
  anchor: « Upstream retires `keyboard_compat.rs` in `286874cc1` (Dioxus 0.8.0-alpha.3), a commit escher has not taken, left out of the take of upstream/main `ebacd68bd`, so the file stays in the tree and both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand (`test -f …keyboard_compat.rs && grep -c …` over `.github/scripts/ci-leg.sh` reads 2), the three floors and `coverage_floor.py` unchanged; the exclusion lifts when escher takes that move, owned by the `.andromeda/residuals.md` line of the left-out upstream delta, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md.»
  add: « The take of upstream/main `e42ee1f25`, thirty-six of 57 taken and twenty-one left out over merge base `23b9b26a9`, leaves `286874cc1` out, so `keyboard_compat.rs` stays and the exclusion stands, the three floors and `coverage_floor.py` unchanged — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md.»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
  old: «dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at CI#41235503227, read in escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md)»
  new: «dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, those two inferred from counts, not from the lcov, `coverage floor: 3 of 3 crates at or above their floors` (as measured at CI#41333172440, read in escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md)»
- after: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
  anchor: «| Document, network (settle unit tests) | a `Scripted` `Document` whose `poll` answers from a script and counts its calls, and a `Held` `NetProvider` that keeps each handler until the test takes it | (packages/blitz-test-harness/src/settle.rs:213-248; packages/blitz-test-harness/src/settle.rs:323-380) |»
  add: «| Net handler (blitz-net unit tests) | a test-local `Handler` implements `NetHandler` in `blitz-net`'s `#[cfg(test)] mod tests`, beside the module's `fetch` helper returning `Result<Bytes, String>`, over three tests (`delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors`, `http_error_body_is_not_delivered_as_css`); the network itself is no stand-in in the two that bind a loopback listener (§7 Loopback listeners) | (packages/blitz-net/src/lib.rs:471-486; packages/blitz-net/src/lib.rs:488-511; packages/blitz-net/src/lib.rs:513-520; packages/blitz-net/src/lib.rs:522-530; packages/blitz-net/src/lib.rs:532-545) |»

## Patterns
[[pattern]]
id = "tp4-01"
fixed = '47,949 bytes in 740 lines'

[[pattern]]
id = "tp4-02"
fixed = 'no local gate builds `blitz-paint/font-embolden`'

[[pattern]]
id = "tp4-03"
fixed = 'skipped on every matrix row'

[[pattern]]
id = "tp4-04"
regex = 'whose owner is Epoch 3.s «Upstream sync ahead of the UI graph»'

[[pattern]]
id = "tp4-05"
fixed = '2791/3240 = 86.14 %'
