# layouts extract

## Relevance
partial. The chunk asserts keyboard reach, focus order and contrast on the four lean stand tasks mounted in TaskShell, and may touch stand markup: a name for `crud-list`, a re-colour of the accent buttons and, depending on how the current option reaches the tree, a roving `tabindex` in CRUD. This domain covers those tasks' layout structure, their Tab sequence and the rule that markup changes use attributes only. Colour values are design's and WCAG criteria are a11y's.

## Constraints
- layout-templates §Surface: desktop-native → Primary screens requires the headless stand to mount one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell. The mount is `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. Every stand assertion in this chunk reads this mount. Home and the three non-lean tasks are not on it.
- layout-templates §Surface: desktop-native → Primary screens records CRUD's Tab sequence as six stops: `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. A change that moves real focus onto a row (a roving `tabindex`) changes that sequence, and the section must then be restated.
- layout-templates §Surface: desktop-native → Primary screens records that the tree's focus is the list, never the row, because no `aria-activedescendant` is read. Whichever shape CARRY 11 takes, this sentence must be restated to match.
- layout-templates §Surface: desktop-native → Primary screens requires stand roles, states and names to be stated by attributes alone (`for`, `aria-label`, `aria-pressed`, `aria-invalid`, `role`, `aria-selected`), adding no element, class, style, wrapper or order. This applies to naming `crud-list` (CARRY 5).
- layout-templates §Surface: desktop-native → Primary screens requires each lean task's controls and value displays to keep their author `id`s as stable element ids. Each CRUD row keeps `crud-person-{person.id}`. No task CSS selects by these ids, and a re-colour (CARRY 7) or a state style (CARRY 8) must keep it that way.
- layout-templates §Surface: desktop-native → IA notes records `crud-list` as the CRUD task's nested scrolling box, reading (24, 112.796875, 369 × 419.203125) at boot and clipping its rows; `task-body` reads (0, 46, 800 × 554). A row that is measured (CARRY 9) or reached by key lies inside that clipping box and may need scrolling into view.
- layout-templates §Surface: desktop-native → IA notes records that the test harness defaults to an 800x600 viewport at scale 1 in light mode. Any added contrast or keyboard reading at a non-boot state is taken at that same viewport.

## Patterns to follow
- Name an input-like control by attribute only, with no element added. The flight dates take `aria-label` "Departure date" / "Return date", and the timer and CRUD inputs take `for` on their existing labels (per layout-templates §Surface: desktop-native → Primary screens). This is the template for naming `crud-list`.
- Build a keyboard region with no element: `crud-list` takes `tabindex="0"` plus an `onkeydown` (ArrowDown · ArrowUp · Home · End fill the fields as a row's click does), adding no element, class, style, wrapper or row (per layout-templates §Surface: desktop-native → Primary screens). Any roving-focus variant follows the same no-new-element rule.
- State attributes sit beside the classes that style those states, which stay (per layout-templates §Surface: desktop-native → Primary screens). A re-colour edits the existing class styles, not the markup's structure.

## Anti-patterns to avoid
- Do not add a wrapper, element, class or reorder anything to name, re-colour or focus a stand control (per layout-templates §Surface: desktop-native → Primary screens).
- Do not use a task CSS selector keyed on a stable element id, such as `#crud-create` or `#counter-increment`, to re-colour one control (per layout-templates §Surface: desktop-native → Primary screens).
- Do not change CRUD's focus model (list focus vs. row focus) without restating the recorded six-stop Tab sequence and the sentence about the tree's focus (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- Focus order: the recorded per-task Tab sequence (layout-templates §Surface: desktop-native → Primary screens) binds to a11y-plan SC 2.4.3 and to the table that `common::tab_order` / `stand_keyboard` assert. A roving-`tabindex` choice moves all three together.
- Stable element ids and named controls: the author `id`s and attribute-only names (layout-templates §Surface: desktop-native → Primary screens) bind to the named-controls table in `stand_accessibility_ids`. Naming `crud-list` restates that table (CARRY 5).
- Re-colour: the accent buttons' placement here binds to design-system §Color Palette for the colour itself, which this domain does not own. Whether `#4a6cf7` reaches non-lean tasks' layout is research's question.

## Acceptance criteria contributions
- (layouts) After this chunk, CRUD's Tab sequence matches `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. If the chunk chooses a roving `tabindex`, the asserted sequence and this section's record are restated identically (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) `crud-list` reads a non-empty accessible name supplied by an attribute alone. The CRUD markup diff adds no element, class, style, wrapper or order (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Any re-colour of the five accent buttons leaves every stable element id, the TaskShell mount and the per-task order unchanged, and adds no task CSS selector by id (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Every new stand assertion boots through the TaskShell mount at the pinned 800 × 600, scale 1.0, Light viewport (per layout-templates §Surface: desktop-native → IA notes).
