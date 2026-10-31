proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 13
    section: "## 6. Visual Design Verification"
    change: "Add a new row after this entity: - Since 2026-10-31-engine-debts `stand_contrast` holds two tests beside its three tables, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` (five rows), each asserting its rows' required ratios in both layout modes, so SC 1.4.3 is asserted on the load page and on Cells as well as the lean four; the load page's colours are read through the shared `read`, no hex copied (tests/blitz-tests/tests/stand_contrast.rs:647-677; tests/blitz-tests/tests/stand_contrast.rs:679-704)"
    sidecar: "visual-contrast: `stand_contrast` gains the load page's and Cells' ratio tests beside its three tables."
    rationale: "The report moves `stand_contrast` 5 to 7 and its Coverage rows assert SC 1.4.3 on the load page's rows and `cells-grid`; New text by line lists both tests in `for incremental in [false, true]` loops and adds nothing to the tables' own lines but imports."
    basis: "tests/blitz-tests/tests/stand_contrast.rs:647-677"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 156
    section: "## 6. Visual Design Verification"
    change: "After `stand_contrast` asserts three tables in both layout modes, a row exempt exactly where the snapshot's `enabled` reads `Some(false)`, add: and, since 2026-10-31-engine-debts, the load page's and Cells' ratios in two further tests; the closing clause stands."
    sidecar: "Body §6 Color contrast: the broad line names the load page's and Cells' ratio tests."
    rationale: "The broad line details the same check as visual-contrast and now omits what the report's `stand_contrast` 5 to 7 added; the report's Expected amendments carry a11y-plan §6 and Cells' row."
    basis: "tests/blitz-tests/tests/stand_contrast.rs:679-704"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "Replace `its one finding at boot the flight booker's `overlap` of its select's two options (below)` with: and, since 2026-10-31-engine-debts, no finding of any kind at boot, `at_boot` reading `[0, 0, 0, 0]` (below)."
    sidecar: "visual-contrast: the stand at boot reads no lint finding; the flight booker's `overlap` is gone."
    rationale: "The report moves `stand_lint`'s `at_boot` `[0, 1, 0, 0]` to `[0, 0, 0, 0]` because `overlaps` skips a closed select's options (F9, Counts `lint-overlap`)."
    basis: "packages/escher-driver/src/lint.rs:364"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "Replace `the `a11y` leg reads 85 tests over 14 targets, `stand_contrast`'s 5 among them` with: the `a11y` leg reads 112 tests over 14 targets, `stand_contrast`'s 7 among them."
    sidecar: "visual-contrast: `a11y` leg 14 · 85 to 14 · 112, `stand_contrast` 5 to 7."
    rationale: "The report's Counts name this line under both `a11y-tally` (14 · 85 to 14 · 112) and `n-contrast` (5 to 7), each as changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: "## 6. Visual Design Verification"
    change: "Restate the claim: The driver's visual lint leaves a closed select's options out of its overlap reading, the snapshot still giving each `ListBoxOption` its select's client rect: `overlaps` skips such an entry on either side, `Entry.in_closed_select` reading the element through `closed_select_of` and never the role, so CRUD's rows, `option` by `role`, are not read as a select's and a `multiple` select's rows are linted as usual; the stand's flight booker at boot reads no `overlap` in both layout modes, `stand_lint`'s `the_stand_at_boot_reads_its_measured_findings` reading `[0, 0, 0, 0]`; the defect is closed by a driver change for every document, on the founder's word (2026-10-31), and no residual line owns it."
    sidecar: "visual-contrast: the lint's closed-select overlap defect is closed; `overlaps` skips a closed select's options."
    rationale: "The report's F9 fixes the defect this line holds open, its `lint-overlap` row changes this line and keeps a closed select's options on its client rect as no change; the P5 card's row 5 absorbs the residual lines of the fixed debts."
    basis: "packages/escher-driver/src/lint.rs:337-338"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "Replace `The user-agent `select` lays out as an `inline-block` box holding its chosen option` with: The user-agent single `select` lays out as an `inline-block` box holding its chosen option, a group not `hidden` reading `display: contents` and its chosen option `display: inline`; a `multiple` select lays out its groups and rows as blocks, each row (`padding: 0 2px`, `white-space: nowrap`) with a box of its own (packages/blitz-dom/assets/default.css:161-188). The rest of the line stands."
    sidecar: "visual-contrast: the user-agent select layout narrows to a single select; a `multiple` select's rows and groups lay out as blocks."
    rationale: "The report's Schema / config adds these default-sheet rows; the line's inline-block statement no longer holds for a `multiple` select, whose rows have boxes of their own (`a_multiple_select_lays_out_its_rows_with_boxes_of_their_own`)."
    basis: "packages/blitz-dom/assets/default.css:161-188"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "Replace `that user-agent pair is not measured` with: that user-agent pair is not measured on a single select, while `select_interaction` measures a `multiple` select's plain and chosen rows at the user agent's pair."
    sidecar: "visual-contrast: the unmeasured user-agent pair narrows to a single select."
    rationale: "New text by line lists `select_interaction`'s `a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair`, and the report's Schema / config states the pair measured by it, so an unqualified not-measured no longer holds."
    basis: "tests/blitz-tests/tests/select_interaction.rs:691-712"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "Add a new row after this entity: - Since 2026-10-31-engine-debts the default sheet holds an engine-default chosen-row pair: `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` read `#000000` on `#c8c8c8`, Chromium's inactive selected pair, hardcoded by design as the engine holds no tokens, 12.56:1 measured by `select_interaction` in both layout modes (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:691-712)"
    sidecar: "visual-contrast: new row for the engine-default chosen-row pair of a `multiple` select."
    rationale: "The report's Schema / config and Coverage rows state the pair, its source and its 12.56:1 measure; its CSS sets `background-color: #c8c8c8; color: #000000`."
    basis: "tests/blitz-tests/tests/select_interaction.rs:691-712"
    fact: F24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 32
    section: "## 6. Visual Design Verification"
    change: "Add a new row after this entity: - Since 2026-10-31-engine-debts Cells shows grid focus on its current cell: `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5` at `outline-offset: -2px`, the accent's held value, within the coverage of `stand_keyboard`, `stand_contrast`, `stand_cells_served` and `stand_non_lean_tasks` (examples/seven_guis/src/tasks/cells.rs:551-554)"
    sidecar: "visual-focus-ring: new row for Cells' current-cell outline on grid focus."
    rationale: "The report's F13 states the rule and its held accent value, and its Coverage row for `cells-grid` names these four checks for the Tab stop, keys and outline together."
    basis: "examples/seven_guis/src/tasks/cells.rs:551-554"
    fact: F25
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 56
    section: "## 6. Visual Design Verification"
    change: "Replace `unasserted — no test asserts `:open`` with: asserted since 2026-10-31-engine-debts — `open_state`'s 4 tests, in both layout modes, read the `open` attribute by presence at construction, move it through a mutator write and removal and a summary's click and Enter, and read `:open` matching."
    sidecar: "visual-state-color: the `:open` gap closes; `open_state` 4 asserts it."
    rationale: "The report's `no-assert-gap` row changes this line and its Outcome reads CARRY 2 met by `open_state` 4; New text by line lists the four tests, each in a `for incremental in [false, true]` loop."
    basis: "tests/blitz-tests/tests/open_state.rs:54-73"
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 77
    section: "## 6. Visual Design Verification"
    change: "Replace `so a select's options are its children in the tree and the snapshot` with: so a select's options are under it in the tree and the snapshot, directly or through an `optgroup`, the exception holding under a group."
    sidecar: "visual-visibility: a select's options may sit under an `optgroup`; the `display: none` exception holds there too."
    rationale: "The report's F7 has `options` walk a select's `optgroup` children's options, `is_select_option` hold under a group, `optgroup` map to `Role::Group`, and a closed select's option under a group stay refused `covered`, so its-children no longer states every option."
    basis: "packages/blitz-dom/src/node/select.rs:131-134"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 78
    section: "## 6. Visual Design Verification"
    change: "Add a new row after this entity: - Since 2026-10-31-engine-debts a transform whose determinant is 0 or not finite answers `None` from `Node::hit_inner` for the whole subtree, so that subtree takes no hit, an invertible twin hit where it shows, asserted by `hit_non_invertible_transform` in both layout modes; ratified by the founder (2026-10-31) (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71)"
    sidecar: "visual-visibility: new row for a non-invertible transform's subtree taking no hit."
    rationale: "The report's F2 states the rule and `hit_non_invertible_transform` 2 meets it; its `non-invertible` control is unfired, so no master holds the fact yet."
    basis: "packages/blitz-dom/src/node/node.rs:1472-1477"
    fact: F26
note: "F24 reads `#c8c8c8` on `#000000`, but the report's CSS sets `background-color: #c8c8c8; color: #000000`, so the proposal writes `#000000` on `#c8c8c8`, the file's text-on-background order; F25's asserted-through-`stand_contrast` is written as the report's Coverage row, which names that check among four for the `cells-grid` surface as a whole."
