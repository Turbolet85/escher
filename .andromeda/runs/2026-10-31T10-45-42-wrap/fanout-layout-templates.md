proposals:
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: desktop-native → Primary screens → seven_guis Home and TaskShell'
    change: 'After the 2026-10-14-stand-requirement-sweep id clause, add: since 2026-10-31-engine-debts the Cells grid is a keyboard region and one Tab stop — `cells-grid` takes `id`, `tabindex="0"`, `aria-label` "Cells" and an `onkeydown`, with no role, so it joins the 676 data cells as a 677th reacting element; arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`; Enter opens the editor on `current`, focused, and the editor''s Enter commits, stops propagation and returns focus to the grid; a click inside a cell''s own open editor returns early, so typed text stays; `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5; outline-offset: -2px`, the stand accent; the Cells Tab sequence is the one `CELLS_TAB_ORDER` states (tests/blitz-tests/tests/common/mod.rs:93-98) and the windowed path was read, not run (examples/seven_guis/src/tasks/cells.rs:425-433; examples/seven_guis/src/tasks/cells.rs:551-554).'
    sidecar: 'Primary screens: the Cells grid recorded as a keyboard region and one Tab stop, with its keys and current-cell rule (F13, 2026-10-31-engine-debts).'
    rationale: 'The report adds a user-facing keyboard region at Symbols / APIs (driver and stand) F13 and Coverage of new surfaces (`cells-grid` Tab stop, keys and `.cell.current` outline), and Deviations 2 and 6a state the synchronous handles and the editor click; the wireframe names only `cell-{A..Z}{0..25}` and `cell-input` for Cells, and the report lists `cells-grid` at `layout-templates.md:10` as changed and as an Expected amendment.'
    basis: examples/seven_guis/src/tasks/cells.rs:551-554
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: desktop-native → Primary screens → seven_guis served screens beside the lean four'
    change: 'In the load page clause add: the load page''s Tab sequence is the one `LOAD_TAB_ORDER` states (tests/blitz-tests/tests/common/mod.rs:93-98), walked in both layout modes, its button `load-fetch` taking Enter and Space (tests/blitz-tests/tests/stand_keyboard.rs:157-177).'
    sidecar: 'Served screens: the load page''s stated Tab sequence and its button''s Enter and Space recorded (2026-10-31-engine-debts).'
    rationale: 'Expected amendments names the load page''s Tab sequence for layout-templates at `load-rows` (`:14`); Symbols / APIs F13 states `LOAD_TAB_ORDER`, and Coverage of new surfaces lists the load page rows at SC 2.1.1 through `stand_keyboard`.'
    basis: tests/blitz-tests/tests/stand_keyboard.rs:157-177
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: desktop-native → Primary screens → seven_guis stand colours'
    change: 'Replace `and no contrast row reads the load page, no lean task` with: and since 2026-10-31-engine-debts `stand_contrast` reads the load page and Cells too, neither a lean task, each in both layout modes (tests/blitz-tests/tests/stand_contrast.rs:647-677; tests/blitz-tests/tests/stand_contrast.rs:679-704).'
    sidecar: 'Stand colours: the retired statement that no contrast row reads the load page replaced by the load page and Cells rows of `stand_contrast` (2026-10-31-engine-debts).'
    rationale: 'Coverage of new surfaces lists `stand_contrast` for the load page rows and the `cells-grid` outline at SC 1.4.3, and New text, by line adds `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`; the passage still states the opposite.'
    basis: tests/blitz-tests/tests/stand_contrast.rs:647-677
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: desktop-native → IA notes'
    change: 'In the visual lint note replace the boot reading with: at boot the counter, the flight booker, the timer and CRUD read 0 findings (`stand_lint`''s `at_boot` `[0, 0, 0, 0]`) — the flight booker''s `overlap`, `flight-type-one-way` × `flight-type-return`, ended at 2026-10-31-engine-debts, `overlaps` skipping on either side an option of a closed select (`Entry.in_closed_select`, through `closed_select_of`, reading the element and never the role, so CRUD''s `role="option"` rows are not read as a select''s), while the snapshot still gives each such option its select''s client rect and a `multiple` select''s rows are linted as usual; drop the clause naming a `.andromeda/residuals.md` line as this overlap''s owner.'
    sidecar: 'IA notes: the flight booker reads 0 lint findings at boot, a closed select''s options being skipped by `overlaps` (F9, 2026-10-31-engine-debts).'
    rationale: 'Symbols / APIs (driver and stand) F9 and Counts / qualifiers moved (`stand_lint`''s `at_boot` `[0, 1, 0, 0]` → `[0, 0, 0, 0]`, `lint-overlap` changed at `layout-templates.md:60`) move this reading, and Expected amendments names the flight booker''s lint reading for this document.'
    basis: packages/escher-driver/src/lint.rs:364
note: 'Only the first proposal is a new surface in the strict sense of D-layout-surface; the other three carry that detector because no other detector fits: `:14` and `:60` are the report''s Expected amendments and the stand colours passage is a stale claim this sweep found, which the report''s `load-rows` line does not list. No passage names a Retired item, so no D-retired proposal is written. The report does not spell out the ids in `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, so the first two proposals name the constants at tests/blitz-tests/tests/common/mod.rs:93-98; the orchestrator can read them there if the sequences should be written out as the CRUD and flight booker ones are.'
