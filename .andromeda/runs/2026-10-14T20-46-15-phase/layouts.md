# layouts extract

## Relevance
partial. The plan's desktop-native seven_guis Home/TaskShell row and its IA notes bind the stand markup the sweep may touch (C2 keys, C8 Cells' selected cell, C3 Home's cards, C15 focus) and the in-file fixtures it adds. Its cli row binds the `escher-session` surface the sweep drives. Decisions Log reads NO RECORDED INTENT and is not cited.

## Constraints
- Any stand markup the sweep adds must be by attributes alone, adding no element, class, style, wrapper, key, id or order: an author `id` or key for Temperature Converter, Circle Drawer and Cells if C2 is taken, and `aria-selected` and a role on Cells' selected cell if C8 is taken. Per layout-templates §Surface: desktop-native → Primary screens (the lean tasks' ids, the 2026-10-10-snapshot-states attribute-only roles and the 2026-10-14 `aria-label` set this pattern).
- Author `id`s are the controls' stable element ids, and no task CSS or Home CSS may select by them. Per layout-templates §Surface: desktop-native → Primary screens.
- The plan's headless stand mounts only one lean task (Counter, FlightBooker, Timer, Crud). It mounts it in TaskShell through `task_in_shell` (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. A held instance must read as a fresh boot, with mount and viewport holding after commands in both layout modes. Per layout-templates §Surface: desktop-native → Primary screens. The plan describes no layout for the three non-lean tasks beyond their Home card ids. Whether the headless stand can mount them, which C2, C3 and C8 measurement needs, is research's question.
- CRUD's Tab sequence is six stops: `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. The tree's focus is the list, never a row, and no `aria-activedescendant` is read. A C15 change to `disabled` focusability must leave this order as stated unless the order is re-recorded. Per layout-templates §Surface: desktop-native → Primary screens.
- On Home unscrolled, `#home` is a 600-high scrolling box. The Cells card (615 to 684) shows and takes a hit only once scrolled into view. A C3 measurement of Home's cards must account for this, and the windowed stand is not measured. Per layout-templates §Surface: desktop-native → Primary screens.
- The stand's one nested scroller is `crud-list`, reading (24, 112.796875, 369 × 419.203125) at boot and after its content scrolls. `task-body` reads (0, 46, 800 × 554). Fixtures C4, C5, C6, C13 and C14 must be in-file, with no stand markup, id, class or style moved. The pinned viewport is scrolled only on an in-file fixture taller than it. Per layout-templates §Surface: desktop-native → IA notes.
- `escher-session start` takes only `counter|flight-booker|timer|crud`, and the host and `mcp` roles boot only those four lean tasks through `seven_guis::stand`, adding no markup. A sweep "by an agent" over the non-lean tasks or fixtures has no plan-recorded boot path on this surface. Per layout-templates §Surface: cli → Primary screens.

## Patterns to follow
- Repeated actionable elements: an author `id` beside a Dioxus key derived from the model's id (`crud-person-{person.id}` beside key `{person.id}`), so the stable id follows the item under filter, create and delete. This is the model for keying Cells' cells or Circle Drawer's circles (C2). Per layout-templates §Surface: desktop-native → Primary screens.
- Role and state by attribute only, with the existing styling classes kept: the flight booker's `aria-pressed` and `aria-invalid`, and CRUD's `role="listbox"` / `role="option"` + `aria-selected`. This is the model for C8 and for any C3 role. Per layout-templates §Surface: desktop-native → Primary screens.
- Naming by attribute only (`for` on existing labels, `aria-label`), adding no element. Per layout-templates §Surface: desktop-native → Primary screens.
- Viewport-scroll cases go on an in-file fixture taller than the pinned viewport, never on the stand. Per layout-templates §Surface: desktop-native → IA notes.

## Anti-patterns to avoid
- Adding a wrapper, class, style or reordering when keying or stating state on a stand task, or selecting by the new ids in task CSS. Per layout-templates §Surface: desktop-native → Primary screens.
- Moving stand markup, ids, classes or styles to host a C4, C5, C6, C13 or C14 case, or changing the headless boot's mount, viewport, scale, theme or font for a fixture. Per layout-templates §Surface: desktop-native → IA notes.
- Recording a windowed-stand layout reading as measured: the plan holds the windowed stand as not booted or not measured. Per layout-templates §Surface: desktop-native → Primary screens.

## Contract bindings
- Focus order ↔ a11y SC 2.4.3: CRUD's six-stop Tab sequence is what `stand_keyboard`'s `common::tab_order` / `ENABLED_TAB_ORDER` assert. A C15 change, or a newly focusable element from C2 or C8, moves both records.
- Stable element ids ↔ dioxus-native-dom `element_id` / `unkeyed_actionable` (`stand_actionable_keys.rs` pins 2 · 3 · 676). Keying under C2 changes those pinned counts, and the layout record must gain the new ids.
- C12 (Circle Drawer's `.cd-dialog input[type="range"]` rule newly applying) ↔ design: any layout or colour change in the dialog is a design-system and stand-contrast question. The plan records no Circle Drawer layout.
- `escher-session` task set ↔ test-plan §3 Session lifecycle, the contract the plan's cli row points to.

## Acceptance criteria contributions
- (layouts) Every stand-task edit the sweep makes (C2 keys, C8 state, C3 role) adds only attributes. Each lean task's element bounds read identically before and after, in both layout modes. (per layout-templates §Surface: desktop-native → Primary screens)
- (layouts) Focus order: CRUD's Tab sequence still reads the six stops `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`, unless a ratified C15 change re-records it. (per layout-templates §Surface: desktop-native → Primary screens)
- (layouts) After the sweep's commands, the headless stand holds the TaskShell mount and the pinned 800 × 600 viewport. `crud-list` reads (24, 112.796875, 369 × 419.203125) at boot and `task-body` reads (0, 46, 800 × 554). (per layout-templates §Surface: desktop-native → IA notes)
- (layouts) Every new fixture check (C4, C5, C6, C11, C13, C14) is an in-file fixture, and `git diff examples/seven_guis/src/` shows no markup, id, class or style change made for it. (per layout-templates §Surface: desktop-native → IA notes)
