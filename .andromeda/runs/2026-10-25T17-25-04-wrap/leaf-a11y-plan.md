group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «the 18 control rows beside a painted-pixel witness (pass 14, fail 0»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:12 «and the record reads pass 13 (`back-btn` ×4, `#333333` on `#f0f0f0`, 11.09; `flight-type`, `#1a1a1a` on `#ffffff`, 17.40, 15px 400;»
  replace-with: «the 17 control rows beside a painted-pixel witness (pass 13, fail 0»

- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «so the five accent controls read `#ffffff` on `#3a5ce5` at 5.47»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:12 «and the four accent controls `counter-increment`, `flight-book`, `timer-reset` and `crud-create`, each `#ffffff` on `#3a5ce5` at 5.47»
  replace-with: «so the four accent controls read `#ffffff` on `#3a5ce5` at 5.47»

- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «19 `STATE_ROWS` (the colour states reached by input, hover and active included: pass 18, exempt 1)»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:12 «`STATE_ROWS`, 18 colour states reached by input — return-chosen 2 (`flight-type` and `flight-return-date`, each 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — pass 17»
  replace-with: «18 `STATE_ROWS` (the colour states reached by input, hover and active included: pass 17, exempt 1)»

- leaf: .claude/docs/a11y-summary.md:22   class: stale
  leaf-text: «the flight booker's mode buttons state `aria-pressed`, its date inputs `aria-invalid`;»
  source: .andromeda/a11y-plan.md:198 «the stand states no `aria-pressed`, the flight booker's mode a `select` whose options state its choice by `selected`»
  replace-with: «the flight booker's date inputs state `aria-invalid`, and the stand states no `aria-pressed` — the flight booker's mode is one `select#flight-type`, whose options state its choice by `selected`;»

- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «each `selected` from its chosen state and with its select's client rect as its `bounds`;»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:15 «so the stand's flight booker at boot reads one `overlap` (`flight-type-one-way` × `flight-type-return`) in both layout modes»
  replace-with: «each `selected` from its chosen state and with its select's client rect as its `bounds`, so the driver's visual lint reads a select's options as overlapping — the stand's flight booker at boot one `overlap` (`flight-type-one-way` × `flight-type-return`), an open defect owned by a `.andromeda/residuals.md` line;»

- leaf: .claude/docs/a11y-summary.md:25   class: stale
  leaf-text: «asserts the 15 stand controls' roles and non-empty names»
  source: .andromeda/a11y-plan-detail/screen-reader-support.md:17 «on the stand the 14 interactive controls carry an HTML-AAM role and a non-empty accessible name»
  replace-with: «asserts the 14 stand controls' roles and non-empty names»

- leaf: .claude/docs/a11y-summary.md:25   class: stale
  leaf-text: «`crud-surname`, `crud-create`, the three other tasks' unchanged»
  source: .andromeda/a11y-plan-detail/cognitive-accessibility.md:9 «and the flight booker's to four, `back-btn`, `flight-type`, `flight-start`, `flight-book` (`flight-return-date` joining in `ENABLED_TAB_ORDER`, five stops)»
  replace-with: «`crud-surname`, `crud-create`, and the flight booker's to four, `back-btn`, `flight-type`, `flight-start`, `flight-book`, the two other tasks' unchanged»

- leaf: .claude/docs/a11y-summary.md:25   class: stale
  leaf-text: «(20 rows, `counter-increment`, `flight-return`, `flight-book` and `crud-create` changing the screen)»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:38 «(16 rows, `counter-increment`, `flight-book` and `crud-create` changing the screen; the flight booker's `select#flight-type` takes no activation key)»
  replace-with: «(16 rows, `counter-increment`, `flight-book` and `crud-create` changing the screen; the flight booker's `select#flight-type` takes no activation key)»

- leaf: .claude/docs/a11y-summary.md:25   class: stale
  leaf-text: «the flight booker after `flight-return`, six stops;»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:98 «the flight booker after the return flight is chosen on `select#flight-type` by ArrowDown, five stops»
  replace-with: «the flight booker after the return flight is chosen on `select#flight-type` by ArrowDown, five stops;»

- leaf: .claude/docs/a11y-summary.md:25   class: omission
  leaf-text: «through nine author ids added by `id:` lines alone (until then 2 · 3 · 676);»
  source: .andromeda/a11y-plan.md:142 «the pages the stand serves beside the lean four hold no Tab-order, keyboard-activation or contrast row, so SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button»
  replace-with: «through nine author ids added by `id:` lines alone (until then 2 · 3 · 676), and the load page the stand serves reads none either (`stand_load_page`); the pages served beside the lean four (Cells, the load page) hold no Tab-order, keyboard-activation or contrast row, so SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button, an open gap owned by a `.andromeda/residuals.md` line;»

- leaf: .claude/docs/a11y-summary.md:14   class: omission
  leaf-text: «reading the focused node back per step, in both layout modes;»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:101 «and `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows` pins its arrows moving the choice in both layout modes, `stand_keyboard` holding 15 tests»
  replace-with: «reading the focused node back per step, in both layout modes, and pins the flight booker's `select#flight-type` (named `Flight type` by `aria-label`, its second Tab stop) moving its choice by its arrows (`the_flight_type_select_moves_its_choice_by_its_arrows`, 15 tests);»

- leaf: .claude/docs/a11y-summary.md:32   class: stale
  leaf-text: «`stand_contrast` measures the stand's 18 control rows (2026-10-13-stand-contrast-harness)»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3 «the stand's control rows are measured by `stand_contrast` (18 at 2026-10-13-stand-contrast-harness, 17 now — §6)»
  replace-with: «`stand_contrast` measures the stand's control rows (18 at 2026-10-13-stand-contrast-harness, 17 now)»

- leaf: .claude/docs/a11y-summary.md:34   class: stale
  leaf-text: «15 stand controls named»
  source: .andromeda/a11y-plan-detail/screen-reader-support.md:17 «on the stand the 14 interactive controls carry an HTML-AAM role and a non-empty accessible name»
  replace-with: «14 stand controls named»

- leaf: .claude/rules/a11y.md:35   class: stale
  leaf-text: «the flight booker after `flight-return`, six stops;»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:98 «the flight booker after the return flight is chosen on `select#flight-type` by ArrowDown, five stops»
  replace-with: «the flight booker after the return flight is chosen on `select#flight-type` by ArrowDown, five stops (its boot `tab_order` four, `back-btn`, `flight-type`, `flight-start`, `flight-book`);»

- leaf: .claude/rules/a11y.md:35   class: omission
  leaf-text: «reading the focused node back per step in both layout modes»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:101 «and `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows` pins its arrows moving the choice in both layout modes, `stand_keyboard` holding 15 tests»
  replace-with: «reading the focused node back per step in both layout modes, and pins the flight booker's `select#flight-type` moving its choice by its arrows (`the_flight_type_select_moves_its_choice_by_its_arrows`, 15 tests)»

- leaf: .claude/rules/a11y.md:35   class: omission
  leaf-text: «so a control enabled after boot needs its place in `ENABLED_TAB_ORDER`;»
  source: .andromeda/a11y-plan.md:142 «the pages the stand serves beside the lean four hold no Tab-order, keyboard-activation or contrast row, so SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button»
  replace-with: «so a control enabled after boot needs its place in `ENABLED_TAB_ORDER`, while the pages the stand serves beside the lean four (Cells, the load page) hold no Tab-order, keyboard-activation or contrast row — SC 2.1.1 and 1.4.3 unasserted on the load page's `load-fetch` Button, an open gap owned by a `.andromeda/residuals.md` line;»

- leaf: .claude/rules/a11y.md:38   class: stale
  leaf-text: «the 18 stand control rows, 14 `TEXT_ROWS` (the CRUD list rows and the text outside the controls) and 19 `STATE_ROWS`»
  source: .andromeda/a11y-plan.md:64 «the 17 control rows, beside a painted-pixel witness of each measured background, and, since 2026-10-14-stand-a11y-assertions, 14 `TEXT_ROWS`»
  replace-with: «the 17 stand control rows, 14 `TEXT_ROWS` (the CRUD list rows and the text outside the controls) and 18 `STATE_ROWS`»

- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «runs fourteen files (84 tests)»
  source: .andromeda/a11y-plan-detail/ci-integration.md:10 «14 files and 85 tests (`stand_keyboard` 15)»
  replace-with: «runs fourteen files (85 tests)»

- leaf: .claude/rules/a11y.md:26   class: omission
  leaf-text: «and its options as its children, each with its select's client rect as its `bounds`.»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:15 «so the stand's flight booker at boot reads one `overlap` (`flight-type-one-way` × `flight-type-return`) in both layout modes»
  replace-with: «and its options as its children, each with its select's client rect as its `bounds` — so the driver's visual lint reads a select's options as overlapping, the stand's flight booker at boot one `overlap` (`flight-type-one-way` × `flight-type-return`), an open defect owned by a `.andromeda/residuals.md` line.»

- leaf: .claude/rules/a11y.md:28   class: omission
  leaf-text: «the three non-lean tasks (Temperature Converter, Circle Drawer with its dialog open included, Cells — keyed by `id:` lines alone)»
  source: .andromeda/a11y-plan-detail/screen-reader-support.md:39 «The check also returns nothing on the load page the stand serves beside the lean four (`stand_load_page::the_load_page_is_keyed_and_reads_no_finding_at_boot`, in both layout modes)»
  replace-with: «the three non-lean tasks (Temperature Converter, Circle Drawer with its dialog open included, Cells — keyed by `id:` lines alone) and the load page the stand serves (`stand_load_page`)»
