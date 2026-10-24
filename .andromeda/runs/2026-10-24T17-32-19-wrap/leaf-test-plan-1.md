group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 195 · 0 · 7 over 48 files with the paged snapshot at 2026-10-20-large-screen-in-parts, unmoved at 2026-10-24-upstream-sync-polish-and-ship»
  source: .andromeda/test-plan.md:189 «`run stand` 197 · 0 · 7 over 48 files, as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md»
  replace-with: «`run stand` 197 · 0 · 7 over 48 files at 2026-10-24-stand-requirement-sweep (+2 passed, `stand_requirement_sweep` +1 and `stand_lint` +1; the ten named `stand-repeat` targets carry no `stand_` prefix, so the agent harness reaches them through `run all` and `run {name}` alone), 195 · 0 · 7 with the paged snapshot at 2026-10-20-large-screen-in-parts, unmoved at 2026-10-24-upstream-sync-polish-and-ship»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «since 2026-10-19-visual-lint `text_extent` (3: an engine check»
  source: .andromeda/test-plan-detail/test-scope-summary.md:22 «by blitz-tests' `text_extent` 6, both layout modes, on HTML pages booted with the stand's options»
  replace-with: «since 2026-10-19-visual-lint `text_extent` (6: an engine check»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «its own auto-discovered target, run by the workspace `test` leg, `run text_extent` and `run all`, not by `run stand`; its stated limits — RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect — asserted by no check)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:23 «its stated limits are asserted by three of those tests under `-p blitz-tests` and `--workspace` — RTL text read as a left-to-right line from the content box»
  replace-with: «its own auto-discovered target, run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), `run text_extent` and `run all`, not by `run stand`; its stated limits asserted by three of those tests — RTL text read as a left-to-right line from the content box, vertical text as a horizontal line, a transformed box's lines read at its client rect, 2 × its transform-blind `offset_rect` about the same centre, the lines unscaled)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «an engine check with no `stand_` prefix, not selected by `run stand`, declaring no shared module»
  source: .andromeda/test-plan-detail/test-scope-summary.md:43 «an engine check on a parsed page with no `stand_` prefix, so not selected by `run stand` though run by the `stand-repeat` leg (a named `--test` of its list), declaring no shared module»
  replace-with: «an engine check with no `stand_` prefix, not selected by `run stand` though run by the `stand-repeat` leg (a named `--test` of its list), declaring no shared module»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and `inert_subtree` (4: Tab and Shift+Tab skipping an inert subtree, a hit passing through it, the tree holding no node for it, removing `inert` restoring all three), each in both layout modes, run by the workspace `test` leg and per package, not by the `a11y` leg»
  source: .andromeda/test-plan-detail/test-scope-summary.md:100 «`accessibility_active_descendant_unread` and `inert_subtree` also by the `stand-repeat` leg (named `--test`s of its list), `accessibility_merged_states` not»
  replace-with: «and `inert_subtree` (4: Tab and Shift+Tab skipping an inert subtree, a hit passing through it, the tree holding no node for it, removing `inert` restoring all three), each in both layout modes, run by the workspace `test` leg and per package — `accessibility_active_descendant_unread` and `inert_subtree` also by the `stand-repeat` leg (named `--test`s of its list), `accessibility_merged_states` not — not by the `a11y` leg»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_requirement_sweep` (2: through»
  source: .andromeda/test-plan-detail/test-scope-summary.md:118 «since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` 3, a stand check `run stand` picks up by its prefix, declaring `mod common;` and `mod session_common;`»
  replace-with: «`stand_requirement_sweep` (3: through»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and `WITNESSES`, 16 requirement rows each naming a witness test found in source)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:119 «— and `WITNESSES_020`, the 0.2.0 record: nine rows v020-01 to v020-09 in order, 46 witnesses, v020-01 and v020-02 holding `Unit` witnesses alone»
  replace-with: «and `WITNESSES`, 16 requirement rows each naming a witness test found in source; and `WITNESSES_020`, the 0.2.0 record — nine rows v020-01 to v020-09 in order, 46 witnesses — its third test asserting each named test found in its file, every non-`Unit` blitz-tests witness a `stand_*` file or one of `STAND_REPEAT_NAMED` (10), every seven_guis witness one of `STAND_REPEAT_SEVEN_GUIS` (8), and `ci-leg.sh` holding the `stand_*.rs` glob and `--test {name}` for all 18, a crate's `Unit` witness run by the workspace `test` leg outside `stand-repeat`)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_lint` (6, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb»
  source: .andromeda/test-plan-detail/test-scope-summary.md:122 «`stand_lint` 7, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, in one test, on the `lint` verb through `Session::run`»
  replace-with: «`stand_lint` (7, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on `Session::lint` in process and, in one test, on the `lint` verb»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «a clipped or off-screen node holding text also reading `truncated` asserted by no check)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:124 «a clipped or off-screen node holding text also reading `truncated` is asserted by `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it`, in both layout modes»
  replace-with: «a clipped or off-screen node holding text also reading `truncated` asserted by `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` on its in-file page `reading_page` — a nowrap line past its `overflow: hidden` box read clipped and truncated, a paragraph passing its box by padding alone read clipped alone, a text wholly right of the viewport read truncated and off-screen — the `lint` verb answering the same)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «`select_interaction` (7, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix, run by the workspace `test` leg, by name and by `run all`, not by `run stand`;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:125 «its own auto-discovered target with no `stand_` prefix (run by the workspace `test` leg, the `stand-repeat` leg — a named `--test` of its list —, by name and by `run all`, not by `run stand`)»
  replace-with: «`select_interaction` (7, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix, run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand`;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «a `snapshot` with `part` 1 among its 35 calls per mode, its line `verb="snapshot"` alone;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:109 «36 calls per mode — one `snapshot` with `part` 1 and, on the shared `many_items` page held as `paged`, one with `part` 2 read as `Outcome::Part` with `parts` at least 2»
  replace-with: «a `snapshot` with `part` 1 and, on the shared `many_items` page, one with `part` 2 read as `Outcome::Part` among its 36 calls per mode, each line `verb="snapshot"` alone, the page's 400 `item-NNN` ids and its buttons' names joining the needles;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «and `host_log` (2, 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:15 «`tests/host_log.rs` (3 tests, unix only; 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`, and the third reading the clients of a recording session»
  replace-with: «and `host_log` (3, unix only; 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`, and the third reading the clients of a recording session — `start crud --record` through `stop`, a part read and a `lint` among them, each a run of the binary at `RUST_LOG=trace` — 0 id, name, sentinel, recording-path and recorded needles over their own stderr)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «still reading 0 needles on stderr at `trace`, its count (2 each) unmoved;»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:21 «still reading 0 needles on stderr at `trace`; `host_log` holds 3 tests and `mcp_log` 2. A paged `part` read through the `escher-session` host's own sink stays unmeasured»
  replace-with: «still reading 0 needles on stderr at `trace`, `host_log` holding 3 tests and `mcp_log` 2; a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next), the in-process capture reading one (`stand_act_spans`);»

- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «three rounds of the 48 `stand_*.rs` files and seven_guis' eight process checks (47 before `stand_snapshot_parts`;»
  source: .andromeda/test-plan-detail/ci-integration.md:9 «ten named targets as literal `--test` words, `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`»
  replace-with: «three rounds of the 48 `stand_*.rs` files, ten named blitz-tests targets outside the `stand_` prefix as literal `--test` words (`inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction`, `text_extent`) and seven_guis' eight process checks (the ten since 2026-10-24-stand-requirement-sweep, `test_stand_repeat_leg_runs_three_rounds` naming the glob and the ten words; 47 before `stand_snapshot_parts`;»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1098 · 0 · 12 over 205 result lines with the paged snapshot at 2026-10-20-large-screen-in-parts (»
  source: .andromeda/test-plan-detail/ci-integration.md:62 «re-counted with `stand-repeat`'s ten named targets: 206 result lines, 1105 passed · 0 failed · 12 ignored — no new result line and +6 passed (`text_extent` 3, `stand_requirement_sweep` 1»
  replace-with: «`cargo test --workspace` 1105 · 0 · 12 over 206 result lines with `stand-repeat`'s ten named targets at 2026-10-24-stand-requirement-sweep (from 206 · 1099 with upstream's `hit_test_dropped_nodes` at 2026-10-24-upstream-sync-polish-and-ship: no new result line and +6 passed — `text_extent` 3, `stand_requirement_sweep` 1, `stand_lint` 1, `host_log` 1; `run stand` 197 passed · 0 failed · 7 ignored over 48 files, `run all` not re-counted; timings not re-measured); 1098 · 0 · 12 over 205 result lines with the paged snapshot at 2026-10-20-large-screen-in-parts (»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «escher-driver 98.07 % from 2026-10-19-test-splits and 97.93 % since 2026-10-19-visual-lint, which added lint.rs, the 97 % floor holding, 3 of 3 crates at or above their floors»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «read again with the stand checks' named list, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008»
  replace-with: «escher-driver 98.07 % from 2026-10-19-test-splits and 97.93 % since 2026-10-19-visual-lint, which added lint.rs, and read again with the stand checks' named list at 2026-10-24-stand-requirement-sweep escher-driver 97.99 %, dioxus-native-dom 85.57 %, blitz-test-harness 94.15 % (equal to CI#40105170961), the 97 % floor holding, 3 of 3 crates at or above their floors»

- leaf: .claude/docs/tests-summary.md:43   class: omission
  leaf-text: «the 48 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (with `stand_snapshot_parts` CI#39410865604's»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry (§9 Legs; with the ten CI#40114977936 read 198 · 837 · 0 · 21, +30 · +153»
  replace-with: «the 48 `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks run three times with no retry (with the ten CI#40114977936's `Stand checks repeated` reading 198 · 837 · 0 · 21, the local leg `stand-repeat: 3 of 3 rounds green`; with `stand_snapshot_parts` CI#39410865604's»
