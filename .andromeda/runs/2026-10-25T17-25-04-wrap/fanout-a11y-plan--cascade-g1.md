proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 18
    section: "## 1. A11y Scope Summary"
    change: "Append to the Dioxus crates entity: ; re-read at 2026-10-25-cold-agent-bar, which moved no dependency (`Cargo.toml`, `Cargo.lock` and `deny.toml` unchanged, `LoadNet` on `blitz-traits`, already a seven_guis dependency): the stand graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still, the `escher-session` host now serving six tasks included — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "a11y-scope-summary · Dioxus crates: re-read at 2026-10-25-cold-agent-bar, no dependency moved, still no platform adapter in the stand graph"
    rationale: "report Changes → Dependencies (none, the git diff probe 0) and Outcome → Gates (the `cargo tree … accesskit_(xplat|winit)` count green); the task's Untouched says the adapter's absence was measured again"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 18
    section: "## 1. A11y Scope Summary"
    change: "New entity after the Dioxus crates entity: - **Served pages** — since 2026-10-25-cold-agent-bar two served pages stand beside the four lean tasks, Cells (`seven_guis::stand::boot_cells`, mounting `Task::Cells` in `TaskShell`) and the load page (`seven_guis::stand::boot_load`, its `load-fetch` Button and `load-done` Paragraph), both served by `escher-session`; neither is a `LeanTask`, so neither carries a Tab-order, keyboard-activation or contrast row, and their a11y reach is their own checks — keyed, `unkeyed_actionable` empty, the load page's lint 0 (examples/seven_guis/src/stand.rs:131-135; examples/seven_guis/src/stand.rs:141-152; tests/blitz-tests/tests/stand_cells_served.rs:91-133; tests/blitz-tests/tests/stand_load_page.rs:70-84) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "a11y-scope-summary · new entity Served pages: Cells and the load page beside the lean four, with no Tab-order, activation or contrast row, reached by their own checks"
    rationale: "report Changes → Symbols / APIs (`boot_cells`, `boot_load`, the load page's ids) and Coverage of new surfaces (a11y n/a for Cells, a11y keyed and lint 0 for the load page, no keyboard or contrast row as it is no `LeanTask`)"
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 9
    section: "## 2. A11y Strategy"
    change: "Re-anchor the citation `tests/blitz-tests/tests/stand_accessibility_ids.rs:363-410` to the Counter refresh test's current lines, one line later past the line this chunk added at :181; the claim itself is unchanged"
    sidecar: "a11y-strategy · tree lifecycle: the `stand_accessibility_ids` citation re-anchored past the line added at :181"
    rationale: "report New text, by line → `stand_accessibility_ids.rs` added 1 line at 181, above the cited range; the companion re-pin moves only the control count, so the Counter claim holds"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 8
    section: "## 5. Keyboard Navigation"
    change: "Re-anchor the citation `tests/blitz-tests/tests/stand_keyboard.rs:677-718` (the disabled-attribute fixture rows) to that test's current lines, since the file gained lines above it (7-15 · 45 · 208 · 214 · 220-221 · 249-310) and lost the mode buttons' rows; the focusability claim itself is unchanged"
    sidecar: "keyboard-navigation · focus order: the `stand_keyboard` disabled-attribute citation re-anchored"
    rationale: "report New text, by line → `stand_keyboard.rs` added 76 lines in 6 ranges above the cited range; the task's Untouched holds the engine's focusability rules"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 12
    section: "## 5. Keyboard Navigation"
    change: "Re-anchor the citation `tests/blitz-tests/tests/stand_keyboard.rs:677-718` to that test's current lines, as in the focus-order entity; the disabled-ness claim itself is unchanged"
    sidecar: "keyboard-navigation · disabled-ness: the `stand_keyboard` citation re-anchored"
    rationale: "report New text, by line → `stand_keyboard.rs` added lines 7-15 · 45 · 208 · 214 · 220-221 · 249-310, all above the cited range"
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 25
    section: "## 5. Keyboard Navigation"
    change: "Re-anchor the citation `tests/blitz-tests/tests/stand_keyboard.rs:638-665` (a click focusing each stand button) to that test's current lines, since the file gained lines above it; on the stand the flight booker's two options are now among the options a driver `click` is refused `covered` on (of 77 snapshot nodes a click is accepted on 68 and refused on 9)"
    sidecar: "keyboard-navigation · focus restoration: the `stand_keyboard` click-focus citation re-anchored; the stand's select options refused `covered`"
    rationale: "report New text, by line → `stand_keyboard.rs` added ranges above 638; report Counts F6 (68 · 9, the two options refused `covered`)"
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "Replace «(20 rows, `counter-increment`, `flight-return`, `flight-book` and `crud-create` changing the screen)» with «(16 rows since 2026-10-25-cold-agent-bar, `BUTTONS` 10 → 8 as the flight booker's two mode buttons `flight-one-way` and `flight-return` were retired for `select#flight-type`, which takes no activation key; `counter-increment`, `flight-book` and `crud-create` changing the screen; 20 rows before it)», and re-anchor the citation `tests/blitz-tests/tests/stand_keyboard.rs:155-211` to the activation test's current lines (the file gained lines inside it at 208 and 214 and above it at 7-15 and 45)"
    sidecar: "keyboard-navigation · keyboard handling: Enter and Space activation rows 20 → 16, `flight-return` retired; citation re-anchored"
    rationale: "report Retired (the flight booker's two `aria-pressed` mode buttons) and Counts F7 (the activation table 20 → 16 rows, `BUTTONS` 10 → 8), stale site keyboard-navigation.md:38"
    basis: ".andromeda/a11y-plan-detail/keyboard-navigation.md:38"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 39
    section: "## 5. Keyboard Navigation"
    change: "Re-anchor the citations `tests/blitz-tests/tests/stand_keyboard.rs:344-349` and `tests/blitz-tests/tests/stand_keyboard.rs:365-396` (the four rows of `crud-update` and `crud-delete`) to those tests' current lines, since the file gained lines above them; the 4-row claim itself is unchanged"
    sidecar: "keyboard-navigation · keyboard handling: the enabled-later activation citations re-anchored"
    rationale: "report New text, by line → `stand_keyboard.rs` added 7-15 · 45 · 208 · 214 · 220-221 · 249-310, all above 344"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 62
    section: "## 5. Keyboard Navigation"
    change: "Re-anchor the citation `tests/blitz-tests/tests/stand_keyboard.rs:815-842` (Tab reaching the buttons under `inert: false`) to that test's current lines, since the file gained lines above it; the falsy-attribute claim itself is unchanged"
    sidecar: "keyboard-navigation · framework exposure: the `stand_keyboard` inert-false citation re-anchored"
    rationale: "report New text, by line → `stand_keyboard.rs` added ranges above 815; the task's Untouched holds the bridge's 28 boolean attributes"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 75
    section: "## 5. Keyboard Navigation"
    change: "Re-anchor the citation `tests/blitz-tests/tests/stand_keyboard.rs:720-780` (the CRUD list keys) to that test's current lines, since the file gained lines above it; the CRUD list claim itself is unchanged"
    sidecar: "keyboard-navigation · CRUD shortcuts: the `stand_keyboard` list-keys citation re-anchored"
    rationale: "report New text, by line → `stand_keyboard.rs` added ranges above 720; the task's Untouched holds `crud-list` and its rows"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 96
    section: "## 5. Keyboard Navigation"
    change: "Re-anchor the citation `tests/blitz-tests/tests/stand_snapshot_state.rs:329-365` (the Tab and Shift+Tab focus check) to that test's current lines, since the file gained lines above it (6-8 · 121-131 · 168 · 182) and its flight-pressed test was rewritten as `exactly_the_chosen_flight_type_reads_selected`; the CRUD Tab claim itself is unchanged"
    sidecar: "keyboard-navigation · harness: the `stand_snapshot_state` Tab check citation re-anchored"
    rationale: "report New text, by line → `stand_snapshot_state.rs` added 6-8 · 121-131 · 168 · 182 above 329; Symbols / APIs → renamed tests"
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 97
    section: "## 5. Keyboard Navigation"
    change: "Replace «Enter and Space on each focused stand button read as its click (20 rows)» with «Enter and Space on each focused stand button read as its click (16 rows since 2026-10-25-cold-agent-bar, the flight booker's two mode buttons retired; 20 before it)», and re-anchor its `tests/blitz-tests/tests/stand_keyboard.rs` citations (the file gained 7-15 · 45 · 208 · 214 · 220-221 · 249-310) and its `tests/blitz-tests/tests/common/mod.rs:47-63` and `:93-113` citations (that file gained 25 · 52 · 66 · 72 · 143) to the current lines"
    sidecar: "keyboard-navigation · harness: `stand_keyboard`'s activation rows 20 → 16; its `stand_keyboard` and shared-module citations re-anchored"
    rationale: "report Retired and Counts F7 (stale site keyboard-navigation.md:97); report New text, by line for both files"
    basis: ".andromeda/a11y-plan-detail/keyboard-navigation.md:97"
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 98
    section: "## 5. Keyboard Navigation"
    change: "Replace «the flight booker after `flight-return`, six stops: `back-btn`, `flight-one-way`, `flight-return`, `flight-start`, `flight-return-date`, `flight-book`» with «the flight booker, since 2026-10-25-cold-agent-bar, after the return flight is chosen on `select#flight-type` by ArrowDown, five stops: `back-btn`, `flight-type`, `flight-start`, `flight-return-date`, `flight-book` (its boot `tab_order` `back-btn` → `flight-type` → `flight-start` → `flight-book`; until then six stops through the two retired mode buttons)», add after the set-difference clause «— the select alone stands in the tables, its two options taking no focus», and re-anchor the citations `tests/blitz-tests/tests/stand_keyboard.rs:213-247`, `:311-342`, `:398-416` and `tests/blitz-tests/tests/common/mod.rs:65-91` to the current lines (lines added at 220-221 · 249-310 and at 66 · 72)"
    sidecar: "keyboard-navigation · harness: the flight booker's enabled Tab order is five stops through `flight-type`; the select alone in the tables; citations re-anchored"
    rationale: "report Counts F2 (the Tab order through `flight-type`, five enabled stops reached by Tab and ArrowDown; stale site keyboard-navigation.md:98), Retired, and Deviations (step 4's tables hold the select alone, its options taking no focus)"
    basis: ".andromeda/a11y-plan-detail/keyboard-navigation.md:98"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 100
    section: "## 5. Keyboard Navigation"
    change: "New entity after the select key and typeahead check: - Since 2026-10-25-cold-agent-bar the engine's single-choice select model is exercised on a lean stand task: the flight booker's flight type is a focusable `select#flight-type` (named `Flight type` by `aria-label`, its second Tab stop), and `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows` pins its arrows moving the choice in both layout modes, `stand_keyboard` holding 15 tests (tests/blitz-tests/tests/stand_keyboard.rs:249-309) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "keyboard-navigation · harness: new entity, the stand's flight type select pinned by `stand_keyboard`'s arrow test"
    rationale: "report Symbols / APIs (the new test `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows`), Counts F7 (its tests 14 → 15) and Coverage of new surfaces (`select#flight-type`, Tab stop and arrows)"
    fact: F1
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 11
    section: "## 6. Visual Design Verification"
    change: "Replace «identical in both layout modes — 18 rows.» with «identical in both layout modes — 17 rows since 2026-10-25-cold-agent-bar, when the flight booker's two mode buttons gave way to `select#flight-type` (18 before it).»"
    sidecar: "visual-design-verification · contrast: `stand_contrast`'s control rows 18 → 17"
    rationale: "report Retired and Counts F5 (control rows 18 → 17)"
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 12
    section: "## 6. Visual Design Verification"
    change: "Restate the record: since 2026-10-25-cold-agent-bar it reads pass 13 (`back-btn` ×4, `#333333` on `#f0f0f0`, 11.09; `flight-type`, `#1a1a1a` on `#ffffff`, 17.40, 15px 400, in place of the retired `flight-return` 9.33 and accent `flight-one-way`; `flight-start` 17.40; `crud-filter`, `crud-name` and `crud-surname`, `#000000` on `#ffffff`, 21.00; and the four accent controls `counter-increment`, `flight-book`, `timer-reset` and `crud-create`, each `#ffffff` on `#3a5ce5` at 5.47), fail 0, exempt-inactive 3, no-text 1 (tally 14 · 0 · 3 · 1 before it); the 4.39 clause names the five accent controls of that day, `flight-one-way` among them; the painted-pixel clause reads all 16 measured rows; and `STATE_ROWS` reads 18 colour states — return-chosen 2 (`flight-type` 17.40 and `flight-return-date` 17.40, where it was `flight-return` 5.47 · `flight-one-way` 9.33 · `flight-return-date` 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — pass 17 and exempt 1 (19 · 18 · 1 before it), the lowest pass still 5.11; `TEXT_ROWS` unchanged at 14"
    sidecar: "visual-design-verification · contrast: the record reads pass 13 with `flight-type` 17.40 and four accent controls; `STATE_ROWS` 19 → 18, return-chosen 2"
    rationale: "report Retired and Counts F5 (tally 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1, `STATE_ROWS` 19 → 18, the pairs that left and the `flight-type` pair; stale site visual-design-verification.md:12); Outcome (stand_contrast reads the select at 17.40)"
    basis: ".andromeda/a11y-plan-detail/visual-design-verification.md:12"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 13
    section: "## 6. Visual Design Verification"
    change: "Re-anchor the `tests/blitz-tests/tests/stand_contrast.rs` citations (49-132, 476-507, 509-559, 163-182, 201-274, 294-406, 572-600, 602-630, 632-656) to the current lines, since the file gained lines at 12 · 68-69 · 279 · 295 · 299-300 · 413-414 · 417-423 · 479 · 595 · 634 · 651 inside and above them and lost the mode buttons' rows; the `contrast.rs` citations and the hover and active claims are unchanged"
    sidecar: "visual-design-verification · contrast: the `stand_contrast` citations re-anchored"
    rationale: "report New text, by line → `stand_contrast.rs` added 20 lines in 11 ranges; the task's Untouched holds `text_contrast`, `contrast_ratio` and the thresholds"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "Replace «the `a11y` leg reads 84 tests over 14 targets» with «the `a11y` leg reads 85 tests over 14 targets since 2026-10-25-cold-agent-bar (`stand_keyboard` 14 → 15; 84 before it)», add after «in both layout modes» «— still none since 2026-10-25-cold-agent-bar, though the lint at boot is no longer 0: the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return`), the other three 0 (the entity beside)», and re-anchor the citations `tests/blitz-tests/tests/stand_lint.rs:174-314` and `:415-442` to the current lines (lines added at 7-9 · 277-286 · 288-289 · 296 · 300-306 · 308-311 · 428-434)"
    sidecar: "visual-design-verification · contrast: the a11y leg 84 → 85 tests; the lean lint at boot reads one `overlap`, no contrast finding; `stand_lint` citations re-anchored"
    rationale: "report Counts F8 (the a11y leg 84 → 85, stale site visual-design-verification.md:14) and F3 (the lint at boot, stale site visual-design-verification.md:14); New text, by line → `stand_lint.rs`"
    basis: ".andromeda/a11y-plan-detail/visual-design-verification.md:14"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "New entity after the lint's contrast entity: - Since 2026-10-25-cold-agent-bar the driver's visual lint reads a select's options as overlapping, since the snapshot gives each `ListBoxOption` its select's client rect, so the stand's flight booker at boot reads one `overlap` (`flight-type-one-way` × `flight-type-return`) in both layout modes; the defect is unresolved, its fix a driver change for every document in the lint's overlap reading (packages/escher-driver/src/lint.rs, `overlaps`), its owner pinned at this chunk's wrap, and `stand_lint` is re-pinned to the measured `overlap` on the founder's word «Перезакрепить как измерено (Рекомендую)» (inputs#I5), `the_stand_at_boot_reads_its_measured_findings` reading `[0, 1, 0, 0]` (tests/blitz-tests/tests/stand_lint.rs:277-285) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "visual-design-verification · new entity: the lint reads a select's options as overlapping, an unresolved driver defect; `stand_lint` re-pinned on the founder's word"
    rationale: "report Insufficient fixes (the unresolved overlap defect, its fix in `lint.rs` `overlaps`), Spec claims disproved (`[0, 1, 0, 0]`), Decisions (inputs#I5) and Counts F3"
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: "## 6. Visual Design Verification"
    change: "Replace «its text/background pair is not measured — the stand holds no select, so `stand_contrast`'s tables are unmoved» with «the user-agent pair is not measured; since 2026-10-25-cold-agent-bar the stand holds one select, `select#flight-type`, whose author CSS (15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10) replaces it, and `stand_contrast` reads its pair at 17.40, 15px 400, against 4.5, as a control row and as a return-chosen `STATE_ROWS` row (examples/seven_guis/src/tasks/flight_booker.rs:160-171)»"
    sidecar: "visual-design-verification · contrast: the stand now holds a select, measured at 17.40; the user-agent pair still unmeasured"
    rationale: "report Changes → Symbols / APIs (the select's CSS at flight_booker.rs:160-171) and Counts F5 (`flight-type` 17.40 in the control and return-chosen rows); contradicts «the stand holds no select»"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 31
    section: "## 6. Visual Design Verification"
    change: "Append: — since 2026-10-25-cold-agent-bar the flight booker's `select#flight-type` too, its `:focus` dropping the outline for the `#3a5ce5` border as the date inputs do (examples/seven_guis/src/tasks/flight_booker.rs:160-171); and re-anchor the date inputs' citation `examples/seven_guis/src/tasks/flight_booker.rs:183-186` to their focus rule's current lines, since the file's CSS was rewritten above it (lines added at 160-162 · 165-166 · 169-171, the mode buttons' CSS removed)"
    sidecar: "visual-design-verification · focus ring: the flight type select's focus border joins the inputs; the date inputs' citation re-anchored"
    rationale: "report Changes → Symbols / APIs (`:focus` drops the outline for the `#3a5ce5` border, as the date inputs do) and Coverage of new surfaces (focus cue `#3a5ce5` border); Retired (`.flight-type-row`, `.type-btn`, `.type-btn-active`)"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 78
    section: "## 6. Visual Design Verification"
    change: "New entity after the `scrollbar-width: none` entity: - Right after a `scroll`, a main-button pointerdown inside the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`hit_with_scrollbar` with `scrollbar_opacity > 0`, packages/blitz-dom/src/events/pointer.rs); on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer resting on the thumb keeps it shown, so a driver `click` on a cell in that band needs an `advance` past the fade first (tests/blitz-tests/tests/stand_cells_served.rs:176-192) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "visual-design-verification · visibility: new entity, the shown overlay scrollbar takes a click right after a scroll as a thumb drag"
    rationale: "report Decisions & corrections (Measured: the overlay scrollbar takes the pointerdown as a thumb drag until `advance` passes its fade), Deviations (`stand_cells_served` takes `advance 5000`) and Counts F21"
    fact: F4
note: "D-a11y-surface — the load page's `load-fetch` Button is a new interactive element with no `stand_keyboard` Tab or activation row and no `stand_contrast` row (the page is no `LeanTask`; the report gives it a11y ✓ by key and lint 0 alone, white on `#3a5ce5` the stand accent's measured 5.47 pair), recorded in F5's entity, not routed as a coverage proposal; the select's options-overlap fix is owed an owner at P5."
