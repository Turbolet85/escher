# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · .andromeda/layout-templates.md:15+#1
- fanout-layout-templates.md 2 · applied · .andromeda/layout-templates.md:60#1
- fanout-layout-templates.md 3 · applied · .andromeda/layout-templates.md:61#1

## Edits
- after: .andromeda/layout-templates.md:15
  anchor: «- seven_guis flight booker's mode region — one `select#flight-type` (class `type-select`, named by `aria-label` "Flight type" alone) holding `option#flight-type-one-way` (value `one-way`, "one-way flight") and `option#flight-type-return` (value `return`, "return flight"), each `selected` from the `flight_type` signal, its `oninput` a closed match (`"return"` → return, any other value → one-way), the two showing the chosen one alone by the engine's select rule; styled 15px, weight 400, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, its `:focus` dropping the outline for the `#3a5ce5` border as the date inputs do; the flight booker's Tab sequence `back-btn` → `flight-type` → `flight-start` → `flight-book` (enabled: + `flight-return-date`, five stops); the snapshot reads it at boot `ComboBox "Flight type"` with value `one-way` and two `ListBoxOption`s, each at the select's client rect (examples/seven_guis/src/tasks/flight_booker.rs:66-89; examples/seven_guis/src/tasks/flight_booker.rs:160-172 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  add: «- seven_guis flight booker's mode list, the engine's select list in every document, remote pages included — a click, Enter, or a Space with no typed text standing opens `select#flight-type`'s list, which `paint_scene` paints after the root's render and before the devtools overlays, whole past an `opacity` ancestor or a box that clips its select, in no second OS window and with no row a layout box; it opens where its rows fit and else on the side with more room, at least the select's width, at most the viewport's and inside it, shows up to twenty rows and scrolls by key and wheel, its thumb painted, taking no hit and not draggable, a group's `label` a painted header that chooses nothing; Enter, Space, Tab or a click on a row chooses, and Escape, a second click, a click outside, the page scrolling or a wheel outside the list closes it with the value unchanged; focus stays on the select and the Tab sequence holds with the list open and after each close, Tab choosing the current row and closing before focus moves; while it is open each row reads its own `bounds` in the snapshot and the diff, the highlight in neither the snapshot nor the tree, and after a close the rows read back at the select's client rect; no stand markup, id, class or style moved; the windowed list read and not run (packages/blitz-paint/src/render.rs:246-247; packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-267; packages/blitz-dom/src/events/pointer.rs:415-420; tests/blitz-tests/tests/stand_act_popup.rs:128-155; tests/blitz-tests/tests/stand_keyboard.rs:1082-1111 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- line: .andromeda/layout-templates.md:60
  old: «for every document); on the headless stand»
  new: «for every document); a row or a group of an open select list is the exception, seen through the list's scrollport, `visible_region` answering that and no clipping box, and `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no scrolling box or viewport moving (packages/blitz-dom/src/scrolling.rs:669-672; packages/blitz-dom/src/scrolling.rs:754-756 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); on the headless stand»
- line: .andromeda/layout-templates.md:61
  old: «`overlaps` skips on either side an option of a closed select (`Entry.in_closed_select`, through `closed_select_of`, reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's), the snapshot giving each such option its select's client rect and a `multiple` select's rows linted as usual (packages/escher-driver/src/lint.rs:364 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «`overlaps` skips on either side an entry with `Entry.in_select_box`, true for an option of a closed select (through `closed_select_of`, which answers `None` for a select whose list is open, reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's) and for an open list's row or group; the snapshot gives a closed select's option its select's client rect and an open list's row its own `bounds`, a `multiple` select's rows linted as usual, and `clip_region` takes an open row's region from `visible_region` alone, no `contain: paint` cut (packages/escher-driver/src/lint.rs:339-342; packages/escher-driver/src/lint.rs:405-408 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»

## Patterns
[[pattern]]
id = "lt-01"
regex = 'every (scrolling|clipping) box that holds'

[[pattern]]
id = "lt-02"
fixed = 'skips on either side an option of a closed select'
