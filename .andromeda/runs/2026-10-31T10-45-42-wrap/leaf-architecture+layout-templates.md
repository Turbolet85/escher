group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:31   class: omission
    leaf-text: «commits on Enter or blur.»
    source: .andromeda/architecture-detail/standard-contracts.md:272 «Cells' keyboard on the stand: `cells-grid` takes `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role, so it is one Tab stop»
    replace-with: «commits on Enter or blur. Since 2026-10-31-engine-debts `cells-grid` is a keyboard region and one Tab stop — `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a click inside an open editor returns early and keeps its typed text; `.cells-grid:focus .cell.current` draws the accent outline, `2px solid #3a5ce5`; the Tab sequence is the one `CELLS_TAB_ORDER` states.»

- leaf: .claude/docs/services/seven_guis.md:31   class: omission
    leaf-text: «Cells: tokenizer rejects unknown characters,»
    source: .andromeda/architecture-detail/established-decisions.md:171 «and a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`), so Cells calls its handles synchronously.»
    replace-with: «Cells: it calls its `MountedData` handles synchronously, such a call in a signal-writing task spawned from a key handler panicking at `events.rs:162` (`RefCell already borrowed`); tokenizer rejects unknown characters,»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
    leaf-text: «ten named blitz-tests targets outside that prefix as literal `--test` words»
    source: .andromeda/architecture-detail/standard-contracts.md:312 «fifteen named blitz-tests targets outside that prefix as literal `--test` words, each file keeping its name»
    replace-with: «fifteen named blitz-tests targets outside that prefix as literal `--test` words (`stand_requirement_sweep`'s `STAND_REPEAT_NAMED` still holding the first ten)»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
    leaf-text: «`cell-input` on the one edit input (`stand_non_lean_tasks` reads it)»
    source: .andromeda/architecture-detail/standard-contracts.md:271 «`cell-input`, and, beside those nine, `cells-grid` on the grid, focusable and carrying a listener, so it reads its key»
    replace-with: «`cell-input` on the one edit input and, beside those, `cells-grid` on the grid, focusable and carrying a listener, so it reads its key (`stand_non_lean_tasks` reads it)»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
    leaf-text: «`stand_contrast` (5 tests, both layout modes, 2 until 2026-10-14-stand-a11y-assertions;»
    source: .andromeda/architecture-detail/existing-scopes.md:113 «`stand_contrast`, 7 tests in both layout modes (2 until 2026-10-14-stand-a11y-assertions; two of them the load page's ratios and Cells' ratios»
    replace-with: «`stand_contrast` (7 tests, both layout modes, 2 until 2026-10-14-stand-a11y-assertions; two of them the load page's ratios and Cells' ratios;»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
    leaf-text: «`stand_keyboard` (15 tests, both layout modes, 11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg, the fifteenth the flight type select moving its choice by its arrows;»
    source: .andromeda/architecture-detail/existing-scopes.md:106 «`stand_keyboard`, 21 tests in both layout modes»
    replace-with: «`stand_keyboard` (21 tests, both layout modes, 11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg, the fifteenth the flight type select moving its choice by its arrows, the sixteenth to the twenty-first the load page's stated walk with Enter and Space on its button, Cells' stated walk, Cells' arrows and Enter opening the moved-to cell and its value committed by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and the arrows clamping at the grid's ends and bringing a cell into view, those two walks stated by `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`;»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
    leaf-text: «pins the per-task count of elements that react to a click and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676)»
    source: .andromeda/architecture-detail/existing-scopes.md:116 «the per-task count of elements that react to input and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, its 676 data cells and `cells-grid`»
    replace-with: «pins the per-task count of elements that react to input and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, its 676 data cells and `cells-grid` with its `onkeydown`)»

- leaf: .claude/docs/services/seven_guis.md:38   class: retired
    leaf-text: «at boot the counter, the timer and CRUD read 0 findings in both layout modes, through the verb as through the reader, and the flight booker one `overlap`, `flight-type-one-way` × `flight-type-return` (the snapshot giving each option its select's client rect — the driver lint's reading of a select's options, not a layout defect, owned by a `.andromeda/residuals.md` line), and after twelve Creates»
    source: .andromeda/layout-templates.md:61 «at boot the counter, the flight booker, the timer and CRUD read 0 findings (`stand_lint`'s `at_boot` `[0, 0, 0, 0]`) in both layout modes, through the verb as through the reader»
    replace-with: «at boot the counter, the flight booker, the timer and CRUD read 0 findings (`stand_lint`'s `at_boot` `[0, 0, 0, 0]`) in both layout modes, through the verb as through the reader — `overlaps` skipping on either side an option of a closed select, the snapshot giving each such option its select's client rect, a `multiple` select's rows linted as usual and the pair of overlapping rows `stand_lint` plants reading exactly `fx-row-a` × `fx-row-b` — and after twelve Creates»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
    leaf-text: «`stand_cells_served` (4 tests) holds Cells booted through `stand::boot_cells` as a session — its screen 47,949 bytes in 740 lines walked in 5 parts of `SNAPSHOT_TEXT_BUDGET`, committed cells reading their values, a cell past the viewport in the last part, a click right after a `scroll` landing on the shown overlay scrollbar, and the lint at boot reading its measured findings —»
    source: .andromeda/architecture-detail/existing-scopes.md:135 «`stand_cells_served`, 5 tests in both layout modes»
    replace-with: «`stand_cells_served` (5 tests) holds Cells booted through `stand::boot_cells` as a session — its screen 47,949 bytes in 740 lines walked in 5 parts of `SNAPSHOT_TEXT_BUDGET`, committed cells reading their values, a cell past the viewport in the last part, a click right after a `scroll` landing on the shown overlay scrollbar, the lint at boot reading its measured findings, and the keyboard walking to a cell and committing a value read in the first part —»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
    leaf-text: «`stand_load_page` (2 tests)»
    source: .andromeda/architecture-detail/existing-scopes.md:135 «`stand_load_page`, 3 tests in both layout modes»
    replace-with: «`stand_load_page` (3 tests)»

- leaf: .claude/docs/services/seven_guis.md:41   class: omission
    leaf-text: «with `focus_redraw` it holds»
    source: .andromeda/architecture-detail/established-decisions.md:147 «held by the windowed `seven_guis_native` smoke boot, `focus_redraw` and blitz-shell's unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read»
    replace-with: «with `focus_redraw` and blitz-shell's unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read, it holds»
