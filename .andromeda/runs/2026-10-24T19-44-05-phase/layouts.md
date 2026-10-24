# layouts extract

## Relevance
partial. Stand focus order and the placement of controls in TaskShell are in this domain. Contrast is design's and a11y's. The `a11y` leg's file list is tests'.

## Constraints
- layout-templates §Surface: desktop-native → Primary screens records CRUD's Tab sequence as six stops: `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. It recorded five before 2026-10-13-stand-keyboard-harness. Whether the stand as 0.2.0 leaves it still reads these six stops is research's question.
- layout-templates §Surface: desktop-native → Primary screens requires `crud-list` to be a keyboard region with `tabindex="0"`. ArrowDown selects the next visible row, or the first when none is selected. ArrowUp selects the previous row, Home the first and End the last, and each fills `crud-name` and `crud-surname` as a row's click does. A key at the list's end, or ArrowUp with no row selected, changes nothing. The tree's focus is the list, never the row, and no `aria-activedescendant` is read.
- layout-templates §Surface: desktop-native → Primary screens requires the headless stand to mount one lean task in TaskShell at the pinned 800 × 600, scale 1.0, Light viewport. The mount is `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`. So the header's `back-btn` comes before the task body in each lean task's surface order.
- layout-templates §Surface: desktop-native → Primary screens records the flight booker's controls as `flight-one-way` · `flight-return` · `flight-start` · `flight-return-date` · `flight-book` · `flight-booked`. The mode buttons `flight-one-way` and `flight-return` are toggle buttons stating `aria-pressed`. The plan names no `select` on the flight booker. Whether the scope's candidate select exists on the stand, and where it sits in the flight booker's Tab sequence, is research's question.
- layout-templates §Surface: desktop-native → Primary screens requires every stand id, name and state to be added by attribute alone (`id`, `for`, `aria-label`, `aria-pressed`, `aria-invalid`, `role`, `aria-selected`). Such an addition moves no element, class, style, wrapper, key or order, and no task CSS selects by an id.
- layout-templates §Surface: desktop-native → IA notes records that each of the four lean tasks reads 0 visual-lint findings at boot in both layout modes. After twelve Creates, the last CRUD row reads `clipped` inside `crud-list`. A re-read that moves no markup should leave both readings as they stand.

## Patterns to follow
- A focus-order change is recorded with the chunk and the cause that moved it, and the earlier count is stated beside the new one (CRUD's "six stops, five until then"). This is the shape for a sequence 0.2.0 moved. Per layout-templates §Surface: desktop-native → Primary screens.
- A stand control's name or state is given by an attribute on the existing element. One example is `aria-label="People"` on `crud-list`, with its accessible name `People` in the tree and the snapshot. Per layout-templates §Surface: desktop-native → Primary screens.
- Every stand layout reading is taken on the headless TaskShell mount at the pinned viewport in both layout modes. The windowed stand is stated as not measured, never assumed. Per layout-templates §Surface: desktop-native → Primary screens.

## Anti-patterns to avoid
- Never add an element, class, style, wrapper or reorder to the stand's task markup so that a Tab or activation row passes. The plan's additions are attributes alone (per layout-templates §Surface: desktop-native → Primary screens).
- Never move focus onto a CRUD row or read `aria-activedescendant` to get a "current option". The plan keeps focus on the list and keeps that gap stated (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- Focus order (layout-templates §Surface: desktop-native → Primary screens) binds to a11y §Focus Order SC 2.4.3 and to `stand_keyboard`'s `common::tab_order` and `common::ENABLED_TAB_ORDER` tables. A sequence the layout plan records and a table row must read the same.
- The CRUD list's keyboard region (layout-templates §Surface: desktop-native → Primary screens) binds to the a11y SC 2.1.1 rows the scope re-asserts in `stand_keyboard`.
- The lint's boot reading (layout-templates §Surface: desktop-native → IA notes) binds to tests' `stand_lint`.

## Acceptance criteria contributions
- (layouts) At boot, in both layout modes, CRUD's Tab sequence reads `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. A different reading is recorded with the change named, never silently re-baselined (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) With focus on `crud-list`, ArrowDown, ArrowUp, Home and End move the selection and fill `crud-name` and `crud-surname`, while focus stays on the list and never on a row (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The chunk's diff under `examples/seven_guis/src/tasks/` adds no element, class, style, wrapper or order. Any change there is an attribute alone (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Each lean task still reads 0 visual-lint findings at boot in both layout modes (per layout-templates §Surface: desktop-native → IA notes).
