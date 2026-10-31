# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · .andromeda/layout-templates.md:15+#1
- fanout-layout-templates.md 2 · applied · .andromeda/layout-templates.md:14#1
- fanout-layout-templates.md 3 · applied · .andromeda/layout-templates.md:16#1
- fanout-layout-templates.md 4 · applied · .andromeda/layout-templates.md:60#1, .andromeda/layout-templates.md:60#2
- fanout-layout-templates.md 5 · folded into fanout-layout-templates.md 1

## Edits
- line: .andromeda/layout-templates.md:14
  old: «no CSS selecting by id; every other boot keeps»
  new: «no CSS selecting by id; the load page's Tab sequence is the one `LOAD_TAB_ORDER` states, walked in both layout modes, its button `load-fetch` taking Enter and Space (tests/blitz-tests/tests/common/mod.rs:93-98; tests/blitz-tests/tests/stand_keyboard.rs:157-177 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md); every other boot keeps»
- after: .andromeda/layout-templates.md:15
  anchor: «- seven_guis flight booker's mode region — one `select#flight-type` (class `type-select`, named by `aria-label` "Flight type" alone) holding `option#flight-type-one-way` (value `one-way`, "one-way flight") and `option#flight-type-return` (value `return`, "return flight"), each `selected` from the `flight_type` signal, its `oninput` a closed match (`"return"` → return, any other value → one-way), the two showing the chosen one alone by the engine's select rule; styled 15px, weight 400, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, its `:focus` dropping the outline for the `#3a5ce5` border as the date inputs do; the flight booker's Tab sequence `back-btn` → `flight-type` → `flight-start` → `flight-book` (enabled: + `flight-return-date`, five stops); the snapshot reads it at boot `ComboBox "Flight type"` with value `one-way` and two `ListBoxOption`s, each at the select's client rect (examples/seven_guis/src/tasks/flight_booker.rs:66-89; examples/seven_guis/src/tasks/flight_booker.rs:160-172 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  add: «- seven_guis Cells grid — a keyboard region and one Tab stop: `cells-grid` takes an author `id`, `tabindex="0"`, `aria-label` "Cells" and an `onkeydown`, with no role, so it joins the 676 data cells as the 677th reacting element `stand_non_lean_tasks` reads; the arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`; Enter opens the editor on `current`, focused, and the editor's Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early, so typed text stays; `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5; outline-offset: -2px`, the stand accent; the Tab sequence is the one `CELLS_TAB_ORDER` states; the synchronous handles measured headlessly, the windowed path read from the same lines and not run (examples/seven_guis/src/tasks/cells.rs:425-433; examples/seven_guis/src/tasks/cells.rs:551-554; tests/blitz-tests/tests/common/mod.rs:93-98 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/layout-templates.md:16
  old: «and no contrast row reads the load page, no lean task»
  new: «and `stand_contrast` reads the load page and Cells too, neither a lean task, each in both layout modes (tests/blitz-tests/tests/stand_contrast.rs:647-677; tests/blitz-tests/tests/stand_contrast.rs:679-704 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/layout-templates.md:60
  old: «at boot the counter, the timer and CRUD read 0 findings in both layout modes»
  new: «at boot the counter, the flight booker, the timer and CRUD read 0 findings (`stand_lint`'s `at_boot` `[0, 0, 0, 0]`) in both layout modes»
- line: .andromeda/layout-templates.md:60
  old: «and the flight booker reads one `overlap`, `flight-type-one-way` × `flight-type-return`, the snapshot giving each option its select's client rect: the driver lint's reading of a select's options, not a layout defect, owned by a `.andromeda/residuals.md` line;»
  new: «and `overlaps` skips on either side an option of a closed select (`Entry.in_closed_select`, through `closed_select_of`, reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's), the snapshot giving each such option its select's client rect and a `multiple` select's rows linted as usual (packages/escher-driver/src/lint.rs:364 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md);»

## Patterns
[[pattern]]
id = "lt-01"
fixed = 'no contrast row reads the load page'

[[pattern]]
id = "lt-02"
fixed = 'the counter, the timer and CRUD read 0 findings'

[[pattern]]
id = "lt-03"
fixed = 'the flight booker reads one `overlap`'
