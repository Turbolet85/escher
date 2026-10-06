# layouts extract

## Relevance
partial — the chunk adds no surface, region, focusable element, modal or breakpoint; it reads the existing seven_guis TaskShell layout (desktop-native) into a tree with bounds, so the stand's screen structure, its author ids and the viewport/coordinate facts bind it.

## Constraints
- The snapshot is taken over the TaskShell screen as the stand mounts it: `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`, one lean task (Counter, FlightBooker, Timer, Crud) at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. The tree's hierarchy and the bounds the chunk asserts are read against that structure and viewport (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell).
- The named stand controls the proof looks up by id are the author `id`s that §Primary screens lists per task (`counter-*`, `flight-*`, `timer-*`, `crud-*`). The plan requires those ids to *be* the stable element ids. Whether the snapshot's id for each one equals that author id is the chunk's check to make, not a given (per layout-templates §Surface: desktop-native → Primary screens).
- CRUD rows are keyed by `{person.id}` (fixture people 0–2, Create from 3), so a row's id `…/div[{person.id}]` follows its person under filter, Create and Delete. A snapshot of the CRUD task carries row ids in that form, never positional ones (per layout-templates §Surface: desktop-native → Primary screens).
- The plan requires that ids add no class, style, wrapper or order, and that no task CSS selects by them. The snapshot builder reads the layout and does not change it: it adds no markup, wrapper, class or style to the stand tasks to make nodes easier to find (per layout-templates §Surface: desktop-native → Primary screens).
- The document root is the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, and the app mounts into `main`. The snapshot's tree has that skeleton above the task, and `<head>` children (title, meta, script, style, link) are not part of the visible screen (per layout-templates §Surface: desktop-native → IA notes).
- Bounds coordinate space: the document element is the scrolling element, fixed-position children of the root are not scrolled with the viewport, and the shell's viewport is the window minus safe-area insets. The chunk's single "document/viewport-absolute" space has to say how it treats scroll offset and fixed-position boxes. Whether headless stand boots ever scroll or carry a safe-area inset is research's question (per layout-templates §Surface: desktop-native → IA notes).
- The harness defaults to an 800x600 viewport at scale 1 in light mode, which matches the stand's pin. "Inside the viewport" checks use that 800 × 600 CSS-px box (per layout-templates §Surface: desktop-native → IA notes).

## Patterns to follow
- Address stand elements by the author `id`s listed in §Surface: desktop-native → Primary screens, and by the CRUD `{person.id}` key form. The stand checks already address controls this way.
- Boot through the headless stand path (`task_in_shell` at the pinned viewport). Skip Home, and take one lean task per boot (per layout-templates §Surface: desktop-native → Primary screens).
- Read the TaskShell header/body split (`#task-header` over `#task-body`) as parent/child structure in the tree, not as a flat list (per layout-templates §Surface: desktop-native → Primary screens).

## Anti-patterns to avoid
- Adding wrappers, classes, styles, author ids or reordering inside the stand tasks to serve the snapshot. The plan requires ids to add none of these (per layout-templates §Surface: desktop-native → Primary screens).
- Mixing coordinate spaces across nodes, for example one node's bounds scroll-relative and another's document-absolute, or fixed-position boxes treated like scrolled ones (per layout-templates §Surface: desktop-native → IA notes).

## Contract bindings
- layouts ↔ a11y: the six task inputs are named by attributes alone (`for` on the labels of `timer-duration`, `crud-filter`, `crud-name` and `crud-surname`, and `aria-label` "Departure date" / "Return date" on `flight-start` / `flight-return-date`). The snapshot's `name` for these must equal the accessibility tree's name from those sources (per layout-templates §Surface: desktop-native → Primary screens; a11y owns the name derivation).
- layouts ↔ a11y focus order (SC 2.4.3): the chunk changes no focus order. The snapshot's `focused` state reads the existing order, and the tree's child order follows document order (per layout-templates §Surface: desktop-native → Primary screens, "ids add no … order").
- layouts ↔ tests: the stand proof's pinned 800 × 600 / scale 1.0 / Light / bundled-font boot is the layout baseline that the bounds assertions are measured against (per layout-templates §Surface: desktop-native → Primary screens and IA notes; the harness is test-plan §3's).

## Acceptance criteria contributions
- (layouts) Booting each lean task headlessly and taking the snapshot finds every author id that §Primary screens lists for that task (counter: `counter-value`, `counter-increment`; flight booker: the 7 `flight-*` ids; timer: the 5 `timer-*` ids; CRUD: the 7 `crud-*` ids) plus `back-btn` and `task-title`. Each has non-zero bounds that lie inside the 800 × 600 CSS-px viewport (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The snapshot tree nests `#task-header` and `#task-body` under `#task-shell` under `main#main`, with `#back-btn` and `#task-title` under `#task-header`. In the bounds, `#task-header` sits above `#task-body` (header top < body top) (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The CRUD snapshot carries the three fixture rows with ids ending `div[0]`, `div[1]` and `div[2]`, keyed by `{person.id}` (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The chunk's diff adds no element, class, style, wrapper or reorder to `examples/seven_guis/src/tasks/*.rs` or `app.rs` TaskShell markup (per layout-templates §Surface: desktop-native → Primary screens).
