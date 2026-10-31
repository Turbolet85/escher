group: architecture+security-plan+test-plan-1
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
- leaf: .claude/docs/services/escher-driver.md:11   class: omission
  leaf-text: «`is_masked_input`, the test the recorder reads to keep a masked `type` as its stand-in)»
  source: .andromeda/architecture-detail/standard-contracts.md:18 «`dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs`, and `is_multiple_select`, called by `closed_select_of` and `execute.rs` `locate`»
  replace-with: «`is_masked_input`, the test the recorder reads to keep a masked `type` as its stand-in, and, since 2026-10-31-engine-debts, `closed_select_of`, which `lint.rs` reads, and `is_multiple_select`, which `execute.rs` `locate` reads, both public and re-exported because this crate has no blitz-dom dependency to name `local_name!` with)»
- leaf: .claude/docs/services/escher-driver.md:17   class: omission
  leaf-text: «`truncated` reads blitz-dom's `BaseDocument::text_extent`»
  source: .andromeda/architecture-detail/standard-contracts.md:213 «The `overlap` reading leaves out a closed select's options: `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role»
  replace-with: «`overlap` leaves out a closed select's options — `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's, and `overlaps` skips such an entry on either side, a `multiple` select's rows linted as usual; `truncated` reads blitz-dom's `BaseDocument::text_extent`»
- leaf: .claude/docs/services/escher-driver.md:18   class: retired
  leaf-text: «a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows, a measured limit `off-screen`'s meaning states»
  source: .andromeda/architecture-detail/standard-contracts.md:177 «under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view»
  replace-with: «under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view, a measured limit `off-screen`'s meaning states»
- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «a select's option is not clicked — a `click` on one is refused `covered`»
  source: .andromeda/architecture-detail/established-decisions.md:78 «an option of a closed single select is not clicked (a driver `click` on one is refused `covered`, under a group too)»
  replace-with: «an option of a closed single select is not clicked — a `click` on one is refused `covered`, under a group too, while a `click` on a `multiple` select's row is accepted and chooses it»
- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «Otherwise they click the centre of the element's snapshot `bounds` plus the viewport's scroll»
  source: .andromeda/architecture-detail/standard-contracts.md:190 «`type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead»
  replace-with: «Otherwise they click the centre of the element's snapshot `bounds` plus the viewport's scroll, but for a `type` on a `multiple` select (`Target` carries `multiple`), which does not click, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead (`BaseDocument::clear_select_typeahead`, its one non-test caller), without which a second `type` joins the first's text, and a text opening on a space types nothing, since no label can match and the Space would flip a row»
- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «on a select the text is the engine's typeahead, choosing the first option whose label starts with it, as `type`'s help says»
  source: .andromeda/architecture-detail/standard-contracts.md:179 «says that on a select the text chooses the first option whose label starts with it and, on a `multiple` select, that the text chooses its match alone»
  replace-with: «on a select the text is the engine's typeahead, choosing the first option whose label starts with it and, on a `multiple` select, its match alone, as `type`'s help says»
- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «`press` presses its key on what holds focus (shift when asked)»
  source: .andromeda/architecture-detail/standard-contracts.md:192 «`press` carries no Ctrl, so a set of rows of a `multiple` select that are not adjacent cannot be built through the driver, and its help states it»
  replace-with: «`press` presses its key on what holds focus (shift when asked; it carries no Ctrl, so a set of rows of a `multiple` select that are not adjacent cannot be built through the driver, as its help states)»
- leaf: .claude/docs/services/escher-driver.md:35   class: retired
  leaf-text: «`off-screen`'s meaning, one — a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows;»
  source: .andromeda/architecture-detail/standard-contracts.md:177 «under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view»
  replace-with: «`off-screen`'s meaning, one — under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view;»
- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows, 2 tests in both layout modes»
  source: .andromeda/architecture-detail/existing-scopes.md:112 «its snapshot `bounds` read where it shows and a control in a scrolling box under a zoomed world reached where it shows, 3 tests in both layout modes»
  replace-with: «a `click` on a translated button accepted and landing, its snapshot `bounds` read where it shows and a control in a scrolling box under a zoomed world reached where it shows, 3 tests in both layout modes»
- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «a disabled select and an unchosen option refused, 5 tests in both layout modes»
  source: .andromeda/architecture-detail/existing-scopes.md:112 «and an option under a closed select's group refused `covered` — 10 tests in both layout modes»
  replace-with: «a disabled select and an unchosen option refused, a `multiple` select's name, role and rows each read with its own bounds, a click on a row accepted and choosing it, a press with Shift spanning the rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered`, 10 tests in both layout modes»
- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «`stand_lint` (7, declaring»
  source: .andromeda/test-plan-detail/test-scope-summary.md:158 «`stand_lint` 8, declaring `mod fixtures;` and `mod session_common;`»
  replace-with: «`stand_lint` (8, declaring»
- leaf: .claude/docs/services/escher-driver.md:62   class: retired
  leaf-text: «`[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint reading a select's options at its client rect as overlapping, an open driver defect in `lint.rs`'s `overlaps` owned by a `.andromeda/residuals.md` line (`the_stand_at_boot_reads_its_measured_findings`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:159 «the lean tasks at boot read their measured findings, `[0, 0, 0, 0]`, the lint skipping a closed select's options»
  replace-with: «`[0, 0, 0, 0]`, the lint skipping a closed select's options (`the_stand_at_boot_reads_its_measured_findings`); the pair of overlapping rows it plants on a `multiple` select reads exactly `fx-row-a` × `fx-row-b`»
- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «`stand_cells_served` (4, on `stand::boot_cells`»
  source: .andromeda/architecture-detail/existing-scopes.md:135 «`stand_cells_served`, 5 tests in both layout modes»
  replace-with: «`stand_cells_served` (5, on `stand::boot_cells`»
- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «the lint at boot reading its measured findings) and `stand_load_page` (2, on `stand::boot_load`:»
  source: .andromeda/architecture-detail/existing-scopes.md:135 «and the keyboard walking to a cell and committing a value read in the first part; `stand_load_page`, 3 tests in both layout modes»
  replace-with: «the lint at boot reading its measured findings, the keyboard walking to a cell and committing a value read in the first part — `press` walking and opening the editor, `type` supplying the characters and `press` committing) and `stand_load_page` (3, on `stand::boot_load`:»
- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «and the page keyed with no lint finding at boot)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:234 «and the page keyed with no lint finding at boot, and its rows asserted beside its load wait by the file's third test»
  replace-with: «and the page keyed with no lint finding at boot, and its rows asserted beside its load wait by the file's third test)»
