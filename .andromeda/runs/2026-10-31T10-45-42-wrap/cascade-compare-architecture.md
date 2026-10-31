# Cascade compare — architecture · wrap · 2026-10-31-engine-debts
E items 96 · held 30 · lost at classify 5 · lost at strong 3 · beside a placed fact 1 · cascade only 5 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### cross-cutting-patterns#5.1 · lost at strong · .andromeda/architecture-detail/cross-cutting-patterns.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 33
    section: "## Cross-cutting Patterns"
    change: "New item in the Event processing group: - A task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed`; any app meets it and no engine line moved, and Cells instead calls its handles synchronously in the handler, the native handle acting at the call; measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines, read and not run, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md"
    sidecar: "Cross-cutting Patterns, Event processing: a `MountedData` call in a task spawned from a key handler panics on the document's `RefCell`, a measured hazard (2026-10-31-engine-debts)."
    rationale: "Report Reverted / negative API facts and Decisions & corrections, A hazard outside this chunk's fixes: the panic is measured and applies to any app; the `mounted-hazard` row reads the hazard as a new fact no master states, and this group's `mounted` clause at line 33 is where event processing is described."

### design-philosophy#3.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 38
    section: "## Design Philosophy"
    change: "Add after line 38, under Behave like a browser, one item: `  - The default sheet's chosen-row pair on a `multiple` select, `#c8c8c8` behind `#000000`, is Chromium's inactive selected pair, 12.56:1 as `select_interaction` measures it (packages/blitz-dom/assets/default.css:161-188; tests/blitz-tests/tests/select_interaction.rs:691-712)`"
    sidecar: "Design Philosophy, Behave like a browser, gains the `multiple` select's chosen-row pair as a named engine's values (2026-10-31-engine-debts)."
    rationale: "The report's Changes, Schema / config, names this new engine-default pair as Chromium's inactive selected pair, measured by `select_interaction`; line 38 records Chromium's overlay scrollbar values the same way, and the group's broad line says web semantics are written against named browser engines."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:44

### established-decisions#11.4 · lost at classify · beside a placed fact F25 (near 75, established-decisions#11.1) · .andromeda/architecture-detail/established-decisions.md:79
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: "New row: - Eight direct engine changes for every document stand beside those, each ratified by the founder's own word (2026-10-31, by dialog in the overseer session, relayed verbatim by the operator): `visible_region` read through transforms, the non-invertible hit, the focus fixup, the tree's focus, native `toggled`, the `multiple` select, `optgroup` and `change` on a checkbox and a radio. Each is flagged upstreamable at upstream `main` `ebacd68bd` with nothing sent; their merge surface is a line owned by Epoch 3's sync, where `0980cce5f` has no hunk on `visible_region` and its `hit_inner` hunk is a textual conflict with the non-invertible hit (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md)."
    sidecar: "Established Decisions, the eight engine changes of 2026-10-31 recorded with their ratification, upstreamable flag and merge-surface owner (2026-10-31-engine-debts)."
    rationale: "The report's Expected amendments name §Established Decisions, the eight changes ratified, flagged upstreamable and their merge surface a line owned by Epoch 3's sync; the Cross-project claims hold the upstream readings."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:128

### established-decisions#12.1 · lost at strong · beside a placed fact F21 (near 159, established-decisions#23.1) · .andromeda/architecture-detail/established-decisions.md:85
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - `BaseDocument::visible_region` reads each clipping box through its transforms, its signature kept (ratified by the founder, 2026-10-31, «Починить (Рекомендую)»): the private `padding_box_in_view` reads over `transformed_bounds`, now `pub(crate)`; with no transform on the chain it reads as before, and a mapped box with a non-finite edge answers `None`. It is exact under a translate and a scale; under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show. Its callers are unchanged, and `physical_unrounded_geometry`, `Node::absolute_position` and `scroll_into_view` are not edited (packages/blitz-dom/src/scrolling.rs:790-812; packages/blitz-dom/src/document.rs:2574-2578; §Standard Contracts → Scrolling, selection, tree)."
    sidecar: "Established Decisions, visible_region through transforms, its rotation reading stated (F1, 2026-10-31-engine-debts)."
    rationale: "F1 is a founder-ratified engine change for every document, and this file records each such change under [Scrolling and input] (report, Symbols / APIs, F1)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:19

### established-decisions#12.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:85
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - A transform whose determinant is 0 or not finite takes no hit: `Node::hit_inner` answers `None` for the whole subtree there (ratified by the founder, 2026-10-31, «Починить (Рекомендую)»; packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56)."
    sidecar: "Established Decisions, the non-invertible hit: no hit for a subtree under a zero or non-finite determinant (F2, 2026-10-31-engine-debts)."
    rationale: "F2 is a founder-ratified engine change for every document, and the report states that no master holds it yet: `non-invertible`'s control is unfired (report, Counts / qualifiers moved, new targets)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:20

### established-decisions#12.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:85
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - The engine fires `change` on a checkbox and a radio (ratified by the founder, 2026-10-31, «Шлёт движок (Рекомендую)»): `handle_click`'s checkbox and radio arms dispatch, after `input`, a `DomEventData::Change` with the same value, a checkbox always and a radio only where it was not checked before the click, so a page's script hears one `change`, from the engine (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836)."
    sidecar: "Established Decisions, change on a checkbox and a radio is the engine's (F10, 2026-10-31-engine-debts)."
    rationale: "F10 is a founder-ratified engine change for every document, and it replaces the synthesis [JavaScript] held (report, Symbols / APIs, F10)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:35

### established-decisions#12.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:85
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - A `multiple` select takes keys and clicks of its own (the founder, 2026-10-31, «Построить сейчас»): focused, ArrowDown · ArrowUp · Home · End alone choose that row alone, with Shift they span from the anchor, with no cursor yet from the last chosen row, and with Ctrl or Meta they move the current row and change nothing; a Space with no typed text standing flips the current row, and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen, and Ctrl+A is not modelled (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:326-332; packages/blitz-dom/src/events/keyboard.rs:334-349; packages/blitz-dom/src/events/keyboard.rs:351-380; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358). `click_select_row`, behind a new `option` arm of `handle_click`: a disabled select stops it; it focuses the select and clears the typeahead and text selection; plain chooses the row alone, Ctrl or Meta flips it, Shift spans; a disabled row changes nothing (packages/blitz-dom/src/events/pointer.rs:1013-1054). `apply_select_choice` takes a set and stays the one write path."
    sidecar: "Established Decisions, a multiple select's keys and clicks (F6, 2026-10-31-engine-debts)."
    rationale: "F6's keys and pointer are the model the old limit at this category's select row excluded; [DOM semantics] sends a select's keys and click here (report, Symbols / APIs, F6)."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:27

### established-decisions#12.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:85
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: "## Established Decisions"
    change: "New row: - A measured hazard of a key handler's spawned task, no engine change: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in that task panics `RefCell already borrowed` (`events.rs:162`), so Cells calls its handles synchronously from the handler, `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call. Measured headlessly through the harness's `poll(None)`; the windowed shell runs the same lines, read and not run (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md)."
    sidecar: "Established Decisions, a measured MountedData hazard in a task spawned from a key handler, beside the overlay scrollbar's (2026-10-31-engine-debts)."
    rationale: "The report states the hazard as a new fact any app meets, with no master holding it (report, Decisions & corrections; Reverted / negative API facts; `mounted-hazard`); this group's overlay-scrollbar row is the form's precedent, and its owner stays P5's to name."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:182

### existing-scopes#1.14 · lost at classify · .andromeda/architecture-detail/existing-scopes.md:49
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 49
    section: "## Existing Scopes"
    change: "In the blitz-dom node module list, after the range input data clause, add: select, the crate-private rules of the select model, since 2026-10-31-engine-debts holding `SelectCursor`, `RowGesture` and `Gestured` (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89)."
    sidecar: "2026-10-31-engine-debts: blitz-dom's node record names the crate-private select rules and their three new types."
    rationale: "Report Changes, Symbols / APIs, F6 names these rule types in `node/select.rs`; the body's `node::select` line at architecture.md:151 is the other side of the relation."
    basis: packages/blitz-dom/src/node/select.rs:56-89

### stack-and-technologies#2.1 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:86
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 86
    section: "## Stack and Technologies"
    change: "Add after this item, one item: - **CI/CD** beside the actions named above: since 2026-10-31-engine-debts the job `Stand checks repeated` runs `jlumbroso/free-disk-space` at its pinned 40-hex SHA `54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, with `android: true`, `large-packages: false`, `swap-storage: false` — an action site and no crate, read `completed · success` on CI#41192433140, its first measured run in this workflow, since the matrix job's older step of the same action is skipped on every row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` while the android row runs `ubuntu-latest`; the step is not shown to cure the out-of-disk red of CI#41190771843, that runner's free disk unread (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)."
    sidecar: "2026-10-31-engine-debts: Stack, new CI/CD item for the free-disk-space action on Stand checks repeated, its older matrix step skipped on every row and its cure unproven."
    rationale: "The report's Harness / gate surface adds this action site on the operator's word and states that no master holds this step or the older one (the free-disk control unfired); Insufficient fixes and the Addendum keep the cure unproven. The category's list of ci.yml actions does not name it, so it lands as its own item, in the form of the Code quality item beside it."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:85

### standard-contracts#18.1 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:140
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 140
    section: '## Standard Contracts'
    change: 'New item after this line: - The grid of Cells, since 2026-10-31-engine-debts (F13 «Одна Tab-остановка (Рекомендую)», the founder): `cells-grid` takes `id`, `tabindex: "0"`, `aria_label: "Cells"` and an `onkeydown`, with no role; arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a click inside the open editor of a cell returns early, so typed text stays; one `onmounted` per data cell (676) holds handles in a map; `.cells-grid:focus .cell.current` outlines the current cell `2px solid #3a5ce5` at `outline-offset: -2px` (examples/seven_guis/src/tasks/cells.rs:551-554; examples/seven_guis/src/tasks/cells.rs:425-433); `tests/common/mod.rs` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` (tests/blitz-tests/tests/common/mod.rs:93-98).'
    sidecar: 'New detail, the Tab stop, keys and current-cell outline of the Cells grid.'
    rationale: 'Report Symbols / APIs F13, and Deviations 6a.'
    basis: examples/seven_guis/src/tasks/cells.rs:551-554

### standard-contracts#22.2 · lost at classify · beside a placed fact F18 (near 202, standard-contracts#21.3) · .andromeda/architecture-detail/standard-contracts.md:210
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 210
    section: '## Standard Contracts'
    change: 'New item after this line: - The `multiple` select through the driver, since 2026-10-31-engine-debts (F6): `Target` gains `multiple`; on `type` the private `focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead; a text opening on a space types nothing, since no label can match and the Space would flip a row; no verb, argument, key name, result field, cause, session error or exit is added (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239).'
    sidecar: 'New detail, type on a multiple select.'
    rationale: 'Report Symbols / APIs, driver and stand, F6; Deviations 6c.'
    basis: packages/escher-driver/src/execute.rs:557-563

### standard-contracts#22.2 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:210
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 210
    section: '## Standard Contracts'
    change: 'New item after this line: - The help of `press`, since 2026-10-31-engine-debts, states its limit on a `multiple` select: "a set of rows that are not adjacent needs Ctrl, which this verb does not carry" (packages/escher-driver/src/schema.rs:261-263).'
    sidecar: 'New detail, the help of press states the Ctrl limit.'
    rationale: 'Report Fixed text moved quotes it; pattern `press-ctrl` records that no site states the help of `press`.'
    basis: packages/escher-driver/src/schema.rs:261-263

### standard-contracts#33.1 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:311
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 311
    section: '## Standard Contracts'
    change: 'New item after this line: - The free-disk step, since 2026-10-31-engine-debts on the operator word: the `Stand checks repeated` job runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false` (.github/workflows/ci.yml:311-316), `completed · success` on CI#41192433140; it is not shown to be the cure of the out-of-disk red on CI#41190771843. The older step of that action, under the matrix job, is skipped on every row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest` (.github/workflows/ci.yml:392-394).'
    sidecar: 'New detail, the free-disk action site in stand-repeat and the skipped older step.'
    rationale: 'Report Harness / gate surface and Insufficient fixes; `free-disk` is unfired, so no master states either step; the owners of the skipped step and the unproven cure are asked of P5.'
    basis: .github/workflows/ci.yml:311-316

## Held

- conventions#8.1
- cross-cutting-patterns#6.1
- established-decisions#10.1
- established-decisions#11.3
- established-decisions#14.2
- established-decisions#16.1
- established-decisions#16.3
- established-decisions#17.2
- established-decisions#20.1
- established-decisions#22.1
- existing-scopes#1.19
- existing-scopes#1.23
- existing-scopes#1.31
- existing-scopes#2.1
- existing-scopes#3.2
- existing-scopes#3.5
- stack-and-technologies#1.19
- standard-contracts#2.2
- standard-contracts#20.1
- standard-contracts#22.1
- standard-contracts#27.1
- standard-contracts#28.2
- standard-contracts#3.1
- standard-contracts#32.1
- standard-contracts#4.1
- standard-contracts#5.1
- standard-contracts#6.1
- standard-contracts#6.3
- standard-contracts#9.2
- standard-contracts#9.4

## Cascade only

- established-decisions#11.1
- established-decisions#23.1
- occupied-resources#12.1
- stack-and-technologies#1.22
- standard-contracts#21.3
