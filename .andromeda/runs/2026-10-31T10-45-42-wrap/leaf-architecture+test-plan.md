group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:14   class: stale
  leaf-text: «ten named targets outside that prefix (`inert_subtree` · `accessibility_states` · `scrolled_box_absolute_position` · `hit_clipped_at_scrolling_box` · `password_caret_click` · `transformed_client_rect` · `accessibility_active_descendant_unread` · `virtual_clock_engine` · `select_interaction` · `text_extent`)»
  source: .andromeda/test-plan-detail/ci-integration.md:9 «`select_interaction`, `text_extent`, `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event`, each file keeping its name»
  replace-with: «fifteen named targets outside that prefix (`inert_subtree` · `accessibility_states` · `scrolled_box_absolute_position` · `hit_clipped_at_scrolling_box` · `password_caret_click` · `transformed_client_rect` · `accessibility_active_descendant_unread` · `virtual_clock_engine` · `select_interaction` · `text_extent` · `visible_region_transformed` · `hit_non_invertible_transform` · `open_state` · `checkable_change_event` · `text_range_no_change_event`)»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «(on a select, the text chooses the first option whose label starts with it)»
  source: .andromeda/architecture-detail/standard-contracts.md:179 «says that on a select the text chooses the first option whose label starts with it and, on a `multiple` select, that the text chooses its match alone»
  replace-with: «(on a select, the text chooses the first option whose label starts with it and, on a `multiple` select, its match alone)»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «`stand_lint`, 7 tests»
  source: .andromeda/test-plan-detail/test-scope-summary.md:158 «`stand_lint` 8, declaring `mod fixtures;` and `mod session_common;`, in both layout modes»
  replace-with: «`stand_lint`, 8 tests»

- leaf: .claude/docs/commands.md:61   class: retired
  leaf-text: «`[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line —,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:159 «the lean tasks at boot read their measured findings, `[0, 0, 0, 0]`, the lint skipping a closed select's options»
  replace-with: «`[0, 0, 0, 0]`, the lint skipping a closed select's options, its planted overlap two rows of a `multiple` select reading exactly `fx-row-a` × `fx-row-b`,»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «`stand_act_select`, 5 tests»
  source: .andromeda/test-plan-detail/test-scope-summary.md:161 «`stand_act_select` 10, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes»
  replace-with: «`stand_act_select`, 10 tests»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «a disabled select and an unchosen option refused (a `click` on an option `covered`), also run by the `a11y` leg»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:44 «`stand_act_select` drives a `multiple` select through the driver, in both layout modes — its name, role and rows read each with its own bounds, an accepted row click choosing the row»
  replace-with: «a disabled select and an unchosen option refused (a `click` on an option `covered`), and since 2026-10-31-engine-debts a `multiple` select — its name, role and rows read each with its own bounds, an accepted row click choosing the row, a Shift press spanning rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered` —, also run by the `a11y` leg»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «`stand_cells_served`, 4 tests»
  source: .andromeda/test-plan-detail/test-scope-summary.md:234 «`stand_cells_served` 5, on the stand's own Cells (`stand::boot_cells`»
  replace-with: «`stand_cells_served`, 5 tests»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «47,949 bytes in 740 lines, 5 parts)»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:24 «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts of `SNAPSHOT_TEXT_BUDGET`»
  replace-with: «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts)»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «the lint at boot reading its measured findings — and `stand_load_page`, 2 tests»
  source: .andromeda/architecture-detail/existing-scopes.md:135 «and the keyboard walking to a cell and committing a value read in the first part; `stand_load_page`, 3 tests in both layout modes»
  replace-with: «the lint at boot reading its measured findings, and the keyboard walking to a cell and committing a value read in the first part — and `stand_load_page`, 3 tests»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «and the page keyed with no lint finding at boot;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:234 «and the page keyed with no lint finding at boot, and its rows asserted beside its load wait by the file's third test»
  replace-with: «and the page keyed with no lint finding at boot, and its rows asserted beside its load wait by the file's third test;»

- leaf: .claude/docs/commands.md:62   class: stale
  leaf-text: «the engine's single-choice `select` model on a controlled Dioxus page, 7 tests»
  source: .andromeda/architecture-detail/existing-scopes.md:112 «and `optgroup` under a single and a `multiple` select — 18 tests in both layout modes»
  replace-with: «the engine's `select` model, single and `multiple`, with `optgroup`, on a controlled Dioxus page, 18 tests»

- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «an app's `selected` write re-seeding the choice; run by the workspace `test` leg»
  source: .andromeda/architecture-detail/existing-scopes.md:112 «a re-seed, Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, its keys and typed text, its clicks choosing, flipping and spanning»
  replace-with: «an app's `selected` write re-seeding the choice, and since 2026-10-31-engine-debts Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, its keys and typed text, its clicks choosing, flipping and spanning, a disabled one taking no click and no key, `multiple` removed settling the set to one option, the chosen row's and a plain row's ratios, a seeded set stepped and spanned from its last chosen row, and `optgroup` under a single and a `multiple` select; run by the workspace `test` leg»

- leaf: .claude/docs/commands.md:63   class: retired
  leaf-text: «reading outside its `visible_region`, the measured `off-screen` limit, both layout modes»
  source: .andromeda/test-plan-detail/test-scope-summary.md:148 «a target inside a translated scrolling box reading inside its `visible_region`, in both layout modes»
  replace-with: «reading inside its `visible_region`, both layout modes»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «`run text_extent` and `run all`, not by `run stand`)»
  source: .andromeda/architecture-detail/existing-scopes.md:151 «blitz-tests' engine-fact targets, each one file and one target, in both layout modes: `visible_region_transformed`, 6 tests»
  replace-with: «`run text_extent` and `run all`, not by `run stand`); and `cargo test -p blitz-tests --locked --test visible_region_transformed --test hit_non_invertible_transform --test open_state --test checkable_change_event --test text_range_no_change_event` — the engine-fact targets (since 2026-10-31-engine-debts), each one file and one target, both layout modes, each a named `--test` of the `stand-repeat` leg's list: `BaseDocument::visible_region` reading each clipping box through its transforms, 6 tests; a box under a non-invertible transform taking no hit and its invertible twin hit where it shows, 2 tests; `open` read from the attribute at construction, a write and a removal through the mutator moving it, and a summary's click and Enter flipping its details, 4 tests; a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone, and a label's click reading as its control's, 3 tests; typing in a text input, and a range's key and pointerdown, firing `input` and no `change`, 2 tests»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «5 tests (2 until 2026-10-14-stand-a11y-assertions)»
  source: .andromeda/architecture-detail/existing-scopes.md:113 «`stand_contrast`, 7 tests in both layout modes (2 until 2026-10-14-stand-a11y-assertions; two of them the load page's ratios and Cells' ratios»
  replace-with: «7 tests (2 until 2026-10-14-stand-a11y-assertions; two of them the load page's ratios and Cells' ratios)»
