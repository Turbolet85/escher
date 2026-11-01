proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 8
    section: "## 2. Test Strategy"
    change: "replace «a filter or a background image» in the contrast reader's `None` clause with «a filter, a background image or, since 2026-11-01-upstream-sync-ui-graph, a `mix-blend-mode` other than `normal` (`paints_more_than_colour`, through `resolved_style_value`), `Session::lint`'s `contrast` reading the same with no second formula»"
    sidecar: "test-strategy: `text_contrast` reads `None` past a `mix-blend-mode` other than `normal`, `Session::lint` the same."
    rationale: "report.md:24 adds the condition and states `Session::lint` reads the same; report.md:44 marks `test-strategy.md:8` changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:24
    fact: F16
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 40
    section: "## 2. Test Strategy"
    change: "replace «no local gate builds `blitz-paint/font-embolden`, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reads» with «a paint change under the OS-gated `blitz-paint/font-embolden` feature has its local witness in the `embolden` step, since 2026-11-01-upstream-sync-ui-graph the last of `ci-leg.sh fast` with no CI job, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reads»"
    sidecar: "test-strategy: the «no local gate builds font-embolden» gap retired, the witness now the `embolden` step of `fast`."
    rationale: "report.md:33 retires the gap as item (3); report.md:42 marks `test-strategy.md:40` changed, now the `embolden` step of `fast`; report.md:64 states the leg, last in `FAST_LEGS`, with no CI job."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 60
    section: "## 2. Test Strategy"
    change: "replace «156 `.rs` files, 155 test targets and the unbuilt `all.rs`, as measured at» with «157 `.rs` files, 156 test targets and the unbuilt `all.rs`, since 2026-11-01-upstream-sync-ui-graph added `key_press_after_container_lost_its_box.rs`, its own auto-discovered target with no manifest line; 156 and 155 as measured at»; the rest of the line stands"
    sidecar: "test-strategy: blitz-tests' file and target counts move by the one new check, to 157 and 156."
    rationale: "report.md:8 lists the one new file under `tests/blitz-tests/tests/`, report.md:29 names it a new test target with `test_blitz_tests_targets.py` unedited and green, report.md:30 no manifest moved; the figure is the line's 156 and 155 plus that one, the report stating no fresh `ls`."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 8
    section: "## 4. Unit Test Strategy"
    change: "follow the line with the bullet «- **Test file location, blitz-paint's `layers.rs`:** since 2026-11-01-upstream-sync-ui-graph `layers.rs` holds 3 unit tests in its `#[cfg(test)] mod tests`: `blend_layers_are_not_replaced_by_clips`, `normal_layers_clip_unless_isolation_is_forced` and `disabled_or_exhausted_layers_do_not_pop` (packages/blitz-paint/src/layers.rs:104-181)»"
    sidecar: "unit-test-strategy: `layers.rs`'s 3 unit tests join the inline test locations."
    rationale: "report.md:104 reads `layers.rs` 3 at the unit tier; the file's New text, `layers.rs` 104-181, names the module and the three tests."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:296
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 20
    section: "## 4. Unit Test Strategy"
    change: "after «`is_noop` is the inner provider's» add «and, since 2026-11-01-upstream-sync-ui-graph, `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it` holds its forward of `NetHandler::error`»; replace «three fixture tests» with «four fixture tests»; replace «and an element with no text reads `None`» with «an element with no text reads `None`, and `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`»"
    sidecar: "unit-test-strategy: `settle.rs` pins the counter's forward of `NetHandler::error`; `contrast.rs` holds a fourth fixture test, the blended ancestor."
    rationale: "report.md:25 states the forward; report.md:39 marks `unit-test-strategy.md:20` changed for it; report.md:36 counts `text_contrast`'s 1 new test; the New text names both tests at report.md:310 and :314."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:39
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 207
    section: "## 4. Unit Test Strategy"
    change: "replace «`settle.rs` pins the load counter, the settle loop» with «`settle.rs` pins the load counter, its forward of `NetHandler::error` among it, the settle loop»"
    sidecar: "test-plan §4: the harness broad line names the load counter's forward of `NetHandler::error`."
    rationale: "report.md:39 marks `test-plan.md:207` changed for the forward of `error`; this is the broad line unit-test-strategy's blitz-test-harness entity details."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 53
    section: "## 4. Unit Test Strategy"
    change: "before «; `InstallScriptTest` runs the CI install script» add «; since 2026-11-01-upstream-sync-ui-graph `test_only_stand_repeat_frees_disk` pins that only `Stand checks repeated` frees disk (.github/scripts/test_ci_workflows.py:110-115) and `test_fast_leg_ends_on_the_font_embolden_witness` that `fast` ends on the font-embolden witness (.github/scripts/test_ci_workflows.py:361-375)»"
    sidecar: "unit-test-strategy: test_ci_workflows.py's two new pins, the free-disk step and `fast`'s last leg."
    rationale: "report.md:64 names the two tests under the harness surface; report.md:38 gives their lines; report.md:143-144 reads the two criteria they hold as met."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:38
    fact: F14
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 54
    section: "## 4. Unit Test Strategy"
    change: "replace «the CI-scripts leg runs 108 tests over eight test files — test_ci_workflows.py's 33,» with «the CI-scripts leg runs 110 tests over eight test files — test_ci_workflows.py's 35,»; before «as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md» add «as measured at 2026-11-01-upstream-sync-ui-graph and green at CI#41333172440 on `45c333f16`; 108, with test_ci_workflows.py's 33,»; the older readings stand"
    sidecar: "unit-test-strategy: the CI-scripts count moves 108 to 110, test_ci_workflows.py's 33 to 35."
    rationale: "report.md:38 moves `ran-count` 108 to 110 by the two named tests and marks `unit-test-strategy.md:54` ×2 changed; report.md:67 gives the green run; 35 is the file's 33 plus those two."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 82
    section: "## 4. Unit Test Strategy"
    change: "follow the line with the bullet «- **blitz-dom (restyle, `mutator.rs`):** since 2026-11-01-upstream-sync-ui-graph `mutator.rs` holds upstream's 39 restyle tests from `fe33b778b`, comparing an incremental document against a fresh one through `check_incremental_restyle` over hidden subtrees, siblings, pseudo-elements and focus-within (packages/blitz-dom/src/mutator.rs:2177-2887), beside `custom_widget_replays_initial_attributes` (packages/blitz-dom/src/mutator.rs:1656-1727)»"
    sidecar: "unit-test-strategy: mutator.rs's 39 restyle tests from `fe33b778b` and the `CustomWidget` replay test."
    rationale: "report.md:36 counts `fe33b778b` 39 and `a26c6ddeb` 1; the New text lists the helper and tests at report.md:219-266 and the replay test at report.md:213."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:36
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 82
    section: "## 4. Unit Test Strategy"
    change: "follow the line with the bullet «- **blitz-dom (`document.rs`, the take's modules):** since 2026-11-01-upstream-sync-ui-graph `picture_display_tests`, one test `picture_is_not_display_contents` (packages/blitz-dom/src/document.rs:4061-4094), and `inline_with_only_block_children_tests`, four tests of the flag following child, text and `display: contents` descendant edits (packages/blitz-dom/src/document.rs:4096-4296)»"
    sidecar: "unit-test-strategy: document.rs's `picture_display_tests` and `inline_with_only_block_children_tests`."
    rationale: "report.md:36 counts `dee854c76` 1 and `f76777766` 4; report.md:106 reads the reader's 4 unit tests; the New text lists both modules at report.md:195-201."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:195
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 14
    section: "## 5. Integration Test Strategy"
    change: "follow the line with the bullet «- **Script ↔ DOM, `URLSearchParams` (blitz-vibey-script):** since 2026-11-01-upstream-sync-ui-graph tests/url_search_params.rs (4) covers the global `URLSearchParams` and `URL.prototype.searchParams`: `construction_and_form_encoding`, `mutations_and_live_iteration`, `invalid_inputs_and_receivers` and `url_search_params_are_bidirectionally_live` (packages/blitz-vibey-script/tests/url_search_params.rs:15-35; packages/blitz-vibey-script/tests/url_search_params.rs:37-59; packages/blitz-vibey-script/tests/url_search_params.rs:61-78; packages/blitz-vibey-script/tests/url_search_params.rs:80-95)»"
    sidecar: "integration-test-strategy: blitz-vibey-script's `url_search_params` target, 4 tests."
    rationale: "report.md:102 reads `url_search_params` 4 at the integration tier; report.md:29 names the target; the New text lists the four at report.md:349-355."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 256
    section: "## 5. Integration Test Strategy"
    change: "replace «tests/selection.rs covers the script Selection API;» with «tests/selection.rs covers the script Selection API; tests/url_search_params.rs the global `URLSearchParams` and `URL.prototype.searchParams`;»"
    sidecar: "test-plan §5: the Script ↔ DOM broad line names `url_search_params`."
    rationale: "report.md:29 adds the target and report.md:17 the surface; this is the broad line integration-test-strategy's Script ↔ DOM entity details."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:29
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 68
    section: "## 5. Integration Test Strategy"
    change: "replace «One row skips its key press: a key pressed after an ancestor gained `hidden` panics at `packages/blitz-dom/src/node/node.rs:1788`, for any control, with no list opened, the test's comment saying why» with «Every row takes its key press through one `harness.press`: since 2026-11-01-upstream-sync-ui-graph a key pressed after an ancestor gained `hidden` no longer panics, focus staying and the key still acting»; the count (25) stands"
    sidecar: "integration-test-strategy: `select_popup`'s row-3 skip retired with the key-press panic."
    rationale: "report.md:33 retires the panic as (1) and the skip as (4); report.md:26 reads `select_popup.rs:513` one `harness.press` for every row; report.md:50 marks `integration-test-strategy.md:68` changed; report.md:23 keeps focus and the key's action; New text adds one line and no test to the file."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:50
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 74
    section: "## 5. Integration Test Strategy"
    change: "replace «has the local witness §2 Painted select list names» with «is read locally by the `embolden` step of `ci-leg.sh fast`, as §2 Painted select list names»"
    sidecar: "integration-test-strategy: `select_popup_painted`'s font-embolden witness is now the `embolden` step of `fast`."
    rationale: "report.md:42 marks `integration-test-strategy.md:74` changed, each now the `embolden` step of `fast`; report.md:33 retires the gap as (3)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 26
    section: "## 7. Test Data & Fixtures"
    change: "replace «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/counts.md» with «47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary at the base and on this tree, as measured at 2026-11-01-upstream-sync-ui-graph (`evidence/counts.md`); 47,949 bytes as read before `cells-grid` took its `id` and `aria_label`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/counts.md»"
    sidecar: "test-data-fixtures: Cells' reading corrected to 47,089 bytes, 47,949 stale since an earlier chunk."
    rationale: "report.md:87 measures 47,089 bytes, 740 lines, 5 parts at the base and on this tree, the take moving nothing, and disposes amend all three, `test-data-fixtures.md:26` among them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:87
note: "F9 (`render_blocking_stylesheet`) has no picked carrier in this group, for the new-detail pass; test-plan.md:92's «mutations insert full damage» sits on no picked entity of this group and the report states no damage move for it, so it is unproposed and worth the §2 reader's eye; the blend no-frame-test gap under D-tests-coverage (report.md:104) is F4 in test-scope-summary, its owner P5's; D-tests-framework and D-tests-obs-harness did not fire here."
