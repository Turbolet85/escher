group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:14   class: stale
  leaf-text: «fifteen named targets outside that prefix (`inert_subtree` · `accessibility_states` · `scrolled_box_absolute_position` · `hit_clipped_at_scrolling_box` · `password_caret_click` · `transformed_client_rect` · `accessibility_active_descendant_unread` · `virtual_clock_engine` · `select_interaction` · `text_extent` · `visible_region_transformed` · `hit_non_invertible_transform` · `open_state` · `checkable_change_event` · `text_range_no_change_event`)»
  source: .andromeda/test-plan-detail/ci-integration.md:9 «seventeen named targets as literal `--test` words»
  replace-with: «seventeen named targets outside that prefix (`inert_subtree` · `accessibility_states` · `scrolled_box_absolute_position` · `hit_clipped_at_scrolling_box` · `password_caret_click` · `transformed_client_rect` · `accessibility_active_descendant_unread` · `virtual_clock_engine` · `select_interaction` · `text_extent` · `visible_region_transformed` · `hit_non_invertible_transform` · `open_state` · `checkable_change_event` · `text_range_no_change_event` · `select_popup` · `select_popup_painted`)»

- leaf: .claude/docs/commands.md:29   class: omission
  leaf-text: «`cargo test -p {crate}` — one crate's unit tests»
  source: .andromeda/test-plan-detail/ci-integration.md:34 «a paint change under that feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`»
  replace-with: «`cargo test -p {crate}` — one crate's unit tests · `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` — the local witness for a paint change under `blitz-paint/font-embolden`, which no local gate builds and CI's macOS leg alone exercises, reading on linux the header and row numbers `Test (macos)` reads through `apple-font-embolden`»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`click --id <id>` ·»
  source: .andromeda/architecture-detail/standard-contracts.md:191 «`click`'s help says a click on a select opens its list»
  replace-with: «`click --id <id>` (a click on a select opens its list and a second closes it; an option of a closed select lies under it and is refused `covered`: click the select first, then the option) ·»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «(on a select, the text chooses the first option whose label starts with it and, on a `multiple` select, its match alone)»
  source: .andromeda/architecture-detail/standard-contracts.md:203 «`focus_select` also closes any list, and `type` on a single select fires no DOM focus events, a text opening on a space typing nothing there»
  replace-with: «(on a select, single or `multiple`, which it focuses with no click, closing any list, the text chooses the first option whose label starts with it and, on a `multiple` select, its match alone; a text opening on a space types nothing on any select)»

- leaf: .claude/docs/commands.md:60   class: stale
  leaf-text: «4 tests, unix only, the fourth serving `cells`»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:35 «`host_log` holds 5 tests and `mcp_log` 3»
  replace-with: «5 tests, unix only, the fifth opening and choosing in a select's list through the host at `trace` with 0 option needles, the fourth serving `cells`»

- leaf: .claude/docs/commands.md:60   class: tree
  leaf-text: «`tests/flows/record_checks.sh` run through the binary»
  source: .andromeda/architecture-detail/occupied-resources.md:53 «`examples/seven_guis/tests/flows/record_checks.sh`»
  replace-with: «`examples/seven_guis/tests/flows/record_checks.sh` run through the binary»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_act_select --test stand_snapshot_parts»
  source: .andromeda/test-plan-detail/test-scope-summary.md:210 «`stand_act_popup`:** 12 tests, the sixteenth `stand_act_*` file, each in both layout modes, driving the select list through a held driver session»
  replace-with: «--test stand_act_select --test stand_act_popup --test stand_snapshot_parts»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «also run by the `a11y` leg, a named `--test` of its fourteen;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:210 «`stand_act_popup`:** 12 tests, the sixteenth `stand_act_*` file, each in both layout modes, driving the select list through a held driver session»
  replace-with: «also run by the `a11y` leg, a named `--test` of its fourteen; since 2026-10-31-select-popup `stand_act_popup`, 12 tests, driving the select list through a held session: a click opening it and a click on a row choosing, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option and a button under the list refused `covered`, a row past the scrollport `off-screen` until `scroll` brings it in, `type` leaving no list open, a shot of a row cropping to it and leaving the list open, the lint, the open screen under its budget with every actionable element keyed, two clicks on one point opening then closing, and a read after a step with no `advance`, not among the `a11y` leg's fourteen;»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «a click then a key choosing an option,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:179 «a click then a key choosing an option, the click opening the select's list and the key moving or choosing in it»
  replace-with: «a click then a key choosing an option, the click opening the select's list and the key moving or choosing in it,»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «`stand_lint`, 8 tests, on `Session::lint`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:176 «`stand_lint` 9, declaring `mod fixtures;` and `mod session_common;`»
  replace-with: «`stand_lint`, 9 tests, on `Session::lint`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «its planted overlap two rows of a `multiple` select reading exactly `fx-row-a` × `fx-row-b`,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:216 «an open list's rows read no overlap on the stand, in both layout modes»
  replace-with: «its planted overlap two rows of a `multiple` select reading exactly `fx-row-a` × `fx-row-b`, an open list's rows reading no overlap on the stand,»

- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «`ArrowDown` · `ArrowUp` · `Home` · `End` choosing an enabled option,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:24 «`enter_and_space_open_the_list_and_a_released_space_does_nothing_more`»
  replace-with: «`ArrowDown` · `ArrowUp` · `Home` · `End` choosing an enabled option, Enter and Space opening the list and a released Space doing nothing more,»

- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «the `a11y` leg, by name and by `run all`, not by `run stand`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:234 «`select_popup`:** 25 tests, its own target with no `stand_` prefix, each in both layout modes»
  replace-with: «the `a11y` leg, by name and by `run all`, not by `run stand`); and `cargo test -p blitz-tests --locked --test select_popup --test select_popup_painted` — a single select's list the engine paints (since 2026-10-31-select-popup), each its own target, both layout modes: `select_popup`, 25 tests, its opening, key and pointer moves, choosing, closing, inert, disabled, hidden and removed selects, the tree's `expanded`, clips, groups, the twenty-row cap, hostile markup, `size` on a `multiple` select, redraw requests and a label's click focusing a single select and opening nothing; `select_popup_painted`, 6 tests, the list on frames: the highlight and a disabled row against their constants, the plain contrast pair, painting past clips and `opacity`, a closed frame byte-equal after a close with no choice, a header darker than a row by weight and a thumb taking no hit; run by the workspace `test` leg, `run all`, by name and as named `--test`s of the `stand-repeat` leg, not by `run stand` or the `a11y` leg»

- leaf: .claude/docs/services/blitz-test-harness.md:19   class: stale
  leaf-text: «(every scrolling box that holds the node, then the viewport)»
  source: .andromeda/test-plan.md:135 «every scrolling box that holds the node, then the viewport, a row of an open select list moving the list's `scroll_y` alone»
  replace-with: «(every scrolling box that holds the node, then the viewport, a row of an open select list moving the list's `scroll_y` alone)»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «(`STATE_ROWS`, 18: 17 pass, 1 exempt)»
  source: .andromeda/architecture-detail/existing-scopes.md:115 «(`STATE_ROWS`, 20: 19 pass, 1 exempt»
  replace-with: «(`STATE_ROWS`, 20: 19 pass, 1 exempt)»
