proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "Line 14's closing clause now reads: the `a11y` leg reads 112 tests over 14 targets, `stand_contrast`'s 7 among them (§9; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-contrast: the `a11y` leg's tally 85 → 112 over 14 targets and `stand_contrast` 5 → 7, at 2026-10-31-engine-debts."
    rationale: "Report, Changes, Counts / qualifiers moved: `a11y-tally` and `n-contrast` each name visual-design-verification.md:14 as changed (a11y leg 14 · 85 → 14 · 112; `stand_contrast` 5 → 7)."
    basis: .andromeda/a11y-plan-detail/visual-design-verification.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: "## 6. Visual Design Verification"
    change: "Line 15 now reads: - The driver's visual lint leaves a closed select's options out of its overlap reading, the snapshot still giving each such `ListBoxOption` its select's client rect: `overlaps` skips an entry reading `Entry.in_closed_select` on either side, set through `closed_select_of`, which reads the element and never the role, so CRUD's `option`-role rows are not read as a select's; a `multiple` select's rows are linted as usual; so the stand's flight booker at boot reads no `overlap` in both layout modes, `stand_lint`'s `the_stand_at_boot_reads_its_measured_findings` reading `[0, 0, 0, 0]`; a driver change for every document, the founder's F9 (2026-10-31), restated when options read real boxes open (packages/escher-driver/src/lint.rs:246-247; packages/escher-driver/src/lint.rs:337-338; packages/escher-driver/src/lint.rs:347; packages/escher-driver/src/lint.rs:364; packages/escher-driver/src/lint.rs:368; tests/blitz-tests/tests/stand_lint.rs:277-363 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-contrast: the flight booker's boot `overlap` is gone, F9 excluding a closed select's options; `stand_lint`'s `at_boot` reads `[0, 0, 0, 0]`, the open defect and its residual owner dropped."
    rationale: "Report, Changes, Counts / qualifiers moved: `lint-overlap` names visual-design-verification.md:15 as changed (`at_boot` `[0, 1, 0, 0]` → `[0, 0, 0, 0]`); Symbols / APIs (driver and stand), F9, states the skip; the P5 card's row 2 carries the restatement."
    basis: packages/escher-driver/src/lint.rs:364
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "In line 14, replace `— its one finding at boot the flight booker's `overlap` of its select's two options (below)` with `— and since 2026-10-31-engine-debts no finding of any defect at boot, a closed select's options left out of `overlaps` (below)`"
    sidecar: "§6 visual-contrast: line 14's aside on the one boot `overlap` restated, no finding at boot since F9."
    rationale: "A second occurrence of the claim `lint-overlap` retires, which the report's hit list does not name at this line: with `at_boot` `[0, 0, 0, 0]` the stand holds no finding at boot, the overlap included."
    basis: .andromeda/a11y-plan-detail/visual-design-verification.md:14
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 56
    section: "## 6. Visual Design Verification"
    change: "Line 56 now reads: - Author styles keyed on `:open` apply to an open `details` or `dialog` in every document, `:open` reading the element's `OPEN` state (§5 Focus restoration); since 2026-10-31-engine-debts `open_state` asserts it, 4 tests in both layout modes: at construction from the attribute, a write and a removal through the mutator, a `summary`'s click and its Enter each flipping its `details` (packages/blitz-dom/src/stylo.rs:510; tests/blitz-tests/tests/open_state.rs:54-143 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-state-color: `:open`'s `no test asserts` retired, `open_state` asserting it in 4 tests."
    rationale: "Report, Changes, Counts / qualifiers moved, Gaps CARRY 2 closes: `no-assert-gap` names visual-design-verification.md:56 as changed; New targets: `open_state` 4; Outcome (CARRY 2) met."
    basis: tests/blitz-tests/tests/open_state.rs:54-143
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 13
    section: "## 6. Visual Design Verification"
    change: "New item after line 13: - Since 2026-10-31-engine-debts `stand_contrast` also reads two served pages beside the lean four, 7 tests in all: `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`, five rows, each in both layout modes through one `assert_meets_its_ratio`, so SC 1.4.3 is asserted on the load page's rows and Cells', the load page's colours read through the shared `read` with no hex copied (tests/blitz-tests/tests/stand_contrast.rs:638-645; tests/blitz-tests/tests/stand_contrast.rs:647-677; tests/blitz-tests/tests/stand_contrast.rs:679-704 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-contrast: `stand_contrast`'s two served-page tests recorded, the load page and Cells."
    rationale: "Report, Changes: `stand_contrast` 5 → 7; Coverage of new surfaces, Load page rows and `cells-grid`, state SC 1.4.3; New text, by line, lists the two tests. The file's contrast record stops at the lean four's three tables."
    basis: tests/blitz-tests/tests/stand_contrast.rs:647-704
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "New item after line 16: - Since 2026-10-31-engine-debts a `multiple` select lists its rows in the default sheet, each `option` under it or its `optgroup` `display: block; padding: 0 2px; white-space: nowrap`, the `optgroup` `display: block`, every row shown (`size` unread), its rows and groups reading client rects of their own in the snapshot; a chosen row takes the new engine-default pair `#000000` on `#c8c8c8`, 12.56:1, Chromium's inactive selected pair, hardcoded by design, the engine holding no tokens, measured with a plain row by `select_interaction` in both layout modes; under a single select an `optgroup` reads `display: contents` and its chosen option `display: inline`; a group's label is named in the tree and unpainted, and no popup is painted (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:447-476; tests/blitz-tests/tests/select_interaction.rs:691-712 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-contrast: the `multiple` select's rows, groups and engine-default chosen-row pair recorded."
    rationale: "Report, Changes, Schema / config: the default sheet's rows and the pair, 12.56:1; Coverage of new surfaces, `multiple` select's keys and clicks and `optgroup`; Symbols / APIs, F6, F7, F8. A new interactive surface whose contrast §6 does not yet record."
    basis: packages/blitz-dom/assets/default.css:161-188
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "In line 16, replace `holding its chosen option` with `holding, on a single select, its chosen option (a `multiple` select's rows: the next item)`, and `that user-agent pair is not measured` with `that user-agent pair is not measured on a single select`"
    sidecar: "§6 visual-contrast: the user-agent `select` line narrowed to a single select, a `multiple` select's rows and pair standing apart."
    rationale: "A second occurrence of the select's reading the new rows move: a `multiple` select now shows every row and `select_interaction`'s `a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair` measures its pair, so neither phrase holds for every select."
    basis: tests/blitz-tests/tests/select_interaction.rs:691-712
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 32
    section: "## 6. Visual Design Verification"
    change: "New item after line 32: - seven_guis Cells shows focus on its one Tab stop, `cells-grid`, by an outline on the current cell, `.cells-grid:focus .cell.current` `2px solid #3a5ce5` at `outline-offset: -2px`, the stand accent's held value; `stand_keyboard` reads which cell is current (examples/seven_guis/src/tasks/cells.rs:551-554; tests/blitz-tests/tests/stand_keyboard.rs:186-191 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-focus-ring: Cells' `cells-grid` Tab stop and its current-cell outline recorded."
    rationale: "Report, Changes, Symbols / APIs, F13, adds `cells-grid` as a Tab stop with this CSS; Coverage of new surfaces, `cells-grid`. The file's focus-ring group lists seven_guis' focus styles and not this one, the new element's §6 coverage."
    basis: examples/seven_guis/src/tasks/cells.rs:551-554
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 77
    section: "## 6. Visual Design Verification"
    change: "In line 77, replace `a select's option excepted from `display: none` (§4 Hidden content), so a select's options are its children in the tree and the snapshot` with `a select's option excepted from `display: none` (§4 Hidden content), an option under its `optgroup` too, so a select's options are in its subtree in the tree and the snapshot, an `optgroup` reading `Role::Group`, its non-empty `label` its name`"
    sidecar: "§6 visual-visibility: a select's options may now stand under an `optgroup`, `Role::Group` in the tree."
    rationale: "Report, Changes, Symbols / APIs, F7: `is_select_option` holds under a group and `optgroup` → `Role::Group`, so `its children` no longer holds of every option. Relation: §4 Hidden content, whose own detail owns the exception."
    basis: .andromeda/a11y-plan-detail/visual-design-verification.md:77
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 79
    section: "## 6. Visual Design Verification"
    change: "New item after line 79: - An element under a transform whose determinant is 0 or not finite is no hit target, nor is anything in its subtree: `Node::hit_inner` answers `None` for the whole subtree, an invertible twin hit where it shows, in both layout modes; an engine change the founder ratified (F2, 2026-10-31) (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-visibility: F2's non-invertible transform takes no hit, recorded beside the other hit exclusions."
    rationale: "Report, Changes, Symbols / APIs, F2; New targets: `hit_non_invertible_transform` 2; `non-invertible`'s control unfired, so no master states it. This group lists the hit exclusions; relation: design-system §Depth Strategy takes the same fact (Expected amendments)."
    basis: packages/blitz-dom/src/node/node.rs:1472-1477
note: "D-a11y-obs-schema holds: the report moves no log site or violation schema, and the lint's finding shape is unmoved. The report states no visible indicator for a `multiple` select's current row, which Ctrl or Meta with an arrow moves alone; no proposal, as that is outside tier 0's three criteria."
