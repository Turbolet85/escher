# layouts extract

## Relevance
partial. The chunk renders on the desktop-native surface's seven_guis TaskShell (the headless stand). It adds no screen, region or component, and the change it may make to stand markup (accessible names) has to leave that structure alone. The surface banner marks tooling context, expression level, signature placement and hero as NOT YET MEASURED, so none of those is cited. The plan has no focus-order, modal, responsive or empty-state section for this surface.

## Constraints
- The stand's fixed shape is the TaskShell hierarchy `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. Per layout-templates §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell), adding accessible names must leave this hierarchy as it is.
- Per layout-templates §Surface: desktop-native → Primary screens, ids "add no class, style, wrapper or order". A naming fix in markup must keep that property. That favours an attribute-only fix (`aria-label`) over a new `<label>` wrapper or sibling, which would change the element tree, the component and document paths, and the painted layout.
- Per layout-templates §Surface: desktop-native → Primary screens, the lean four's controls carry author `id`s that are their stable ids: `counter-*`, `flight-*`, `timer-*`, `crud-*` and `back-btn`. These are the ids the proof check must find present on accessibility nodes. Each CRUD row is keyed `{person.id}` (fixture people 0–2, Create from 3), so a row's id is `…/div[{person.id}]` and follows its person under filter, Create and Delete.
- The stand skips Home and mounts one lean task through `task_in_shell`, per layout-templates §Surface: desktop-native → Primary screens. The scope lists "task cards" under shell chrome. Whether task cards can be reached in a stand boot at all, or only in the windowed app, is research's question.
- Every document starts with the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, and the app mounts into `main` (per layout-templates §Surface: desktop-native → IA notes). The document root and the skeleton's non-app elements are the nodes the scope says carry no id, or carry their skeleton id. Which of the two applies is research's question.
- The test harness defaults to an 800x600 viewport at scale 1 in light mode (per layout-templates §Surface: desktop-native → IA notes). The accessibility-tree check runs at this viewport, through the stand boot.

## Patterns to follow
- Stable id = the author `id` on a control or value display. The chunk reads those ids and does not add new ones, per layout-templates §Surface: desktop-native → Primary screens.
- The stand boots one lean task in TaskShell (Counter, FlightBooker, Timer, Crud) through `task_in_shell`. The new `stand_*` check should boot through the same path the existing stand checks use, per layout-templates §Surface: desktop-native → Primary screens.
- A CRUD row's identity comes from its model key and does not depend on its position. A post-change coherence check (Create, Delete, filter) should expect the row ids to follow their persons, per layout-templates §Surface: desktop-native → Primary screens.

## Anti-patterns to avoid
- Adding a wrapper element, class, style or reordering to a stand task to hang a name on a control. That breaks the "no class, style, wrapper or order" property in layout-templates §Surface: desktop-native → Primary screens, and it can shift component and document paths.
- Task CSS selecting by the stable ids, which the same section forbids. Any markup change for names must not introduce such a selector.

## Contract bindings
- layouts ↔ a11y: the TaskShell visual order (header with `#back-btn`, then `#task-body` controls) binds to a11y-plan's focus-order criterion SC 2.4.3. The scope says focus order is not meant to change, so the shell's visual order and focus order must stay matched (per layout-templates §Surface: desktop-native → Primary screens).
- layouts ↔ tests: the pinned stand viewport, font and boot path in layout-templates §Surface: desktop-native → Primary screens / IA notes are the harness preconditions the new `stand_*` accessibility check inherits from the test-plan stand contract.

## Acceptance criteria contributions
- (layouts) Every named lean-task control id (`counter-value`, `counter-increment`, `flight-one-way` … `flight-booked`, `timer-progress` … `timer-reset`, `crud-filter` … `crud-delete`) and `back-btn` appears as a carried id on an accessibility node when its task is booted in TaskShell (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The TaskShell hierarchy `main#main > #task-shell > #task-header(#back-btn, #task-title) + #task-body` and each task's control order are the same before and after the chunk. No naming fix adds a wrapper, class, style or reorder (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) After CRUD Create and Delete, each surviving row's carried accessibility id stays `…/div[{person.id}]` for its own person (per layout-templates §Surface: desktop-native → Primary screens).
