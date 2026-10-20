# layouts extract

## Relevance
partial: the chunk adds no stand surface. Its select lives on a headless fixture page. The layout facts that bind it are the stand's unchanged structure and Tab order, the harness viewport the fixture boots in, and the `escher-session` command-line surface, in case the choice of an option adds a verb or an argument.

## Constraints
- The 7GUIs stand stays as built. The four lean tasks each have a fixed TaskShell mount (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`), and their controls keep their author `id`s as their stable element ids. Each past change added no element, class, style, wrapper or order, and no task CSS selects by an id. A select added to a stand task would break this, and the scope rules that out (per layout-templates §Surface: desktop-native → Primary screens).
- The Tab sequences recorded for the stand must not move. CRUD reads `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` (six stops). The tree's focus is the list, never a row. The matrix's "focus order unchanged" is measured against these sequences (per layout-templates §Surface: desktop-native → Primary screens).
- Controls are named by attributes alone: a `for` on an existing label, or an `aria-label`. Roles and states are also stated by attributes alone (`role="listbox"` / `role="option"` with `aria-selected` on `crud-list`). The fixture's select takes its accessible name the same way, with no wrapper element added for naming (per layout-templates §Surface: desktop-native → Primary screens).
- A fixture page boots at the harness default: an 800 × 600 viewport, scale 1, light mode. Each document mounts into the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`. The select's bounds and its `off-screen` / `covered` readings are taken in that viewport (per layout-templates §Surface: desktop-native → IA notes).
- `visible_region` reads what part of the viewport an element can be seen through: the viewport narrowed by every clipping box that holds the element. `scroll_into_view` moves each scrolling box that holds its target. A select addressed by id goes through the same visibility reading as `click`, `type` and `scroll`. Whether the code applies that reading to a select is a question for research (per layout-templates §Surface: desktop-native → IA notes).
- The `escher-session` command line serves eleven verbs from the driver's verb table. Argument flags are spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` each take one value; `--shift` is bare). The usage line is built from the verb table. No ending writes back anything the caller typed. `mcp` lists the same verbs as tools in order, `help` last. A new verb or argument for choosing an option moves all of these together (per layout-templates §Surface: cli → Primary screens).
- The in-process visual lint reads 0 findings on each lean task at boot. Keeping the stand unchanged keeps that reading (per layout-templates §Surface: desktop-native → IA notes).

## Patterns to follow
- The select carries an author `id` as its stable element id, as every stand control does: an `id:` line alone, with no CSS selecting by it (per layout-templates §Surface: desktop-native → Primary screens).
- The CRUD list is the precedent for a choice control: a keyboard region whose arrow, Home and End keys move a selection that the snapshot reads through `aria-selected`. Its focus stays on the container, never on an option row (per layout-templates §Surface: desktop-native → Primary screens).
- The range input (Timer `timer-duration`) is the precedent for a native control driven by `click` and keys. Its value change shows in the snapshot's `value` (per layout-templates §Surface: desktop-native → Primary screens).
- A held driver instance reads as a fresh boot, so the mount and the pinned viewport hold after commands in both layout modes. The fixture's session should read the same way (per layout-templates §Surface: desktop-native → Primary screens).

## Anti-patterns to avoid
- Adding an element, class, style, wrapper or Tab stop to any stand task, or CSS that selects by a stable id (per layout-templates §Surface: desktop-native → Primary screens).
- Adding a verb, flag or tool outside the one verb table, or writing a command-line or MCP ending that echoes caller-typed text such as an option's value (per layout-templates §Surface: cli → Primary screens).
- Painting or windowing a dropdown popup. The plan records popups only for the browser app's absolutely positioned menu and suggestion popovers, and the scope keeps the proof headless (per layout-templates §Surface: desktop-native → Component — Footer).

## Contract bindings
- Focus order connects to a11y SC 2.4.3. `stand_keyboard`'s `common::tab_order` and `common::ENABLED_TAB_ORDER` tables hold the stand sequences cited above. A select placed as a form control in a fixture's Tab order binds to the a11y plan's focus-order mandate (per layout-templates §Surface: desktop-native → Primary screens).
- The command-line and MCP surface connects to the test-plan §3 Session lifecycle contract, which the `escher-session` surface defers to for its session behaviour (per layout-templates §Surface: cli → Primary screens).
- Fixture placement connects to the tests domain. The layouts plan has no fixture-page row, and the shared fixtures module is tests-owned.

## Acceptance criteria contributions
- (layouts) The four lean tasks' Tab sequences read unchanged after the chunk, CRUD's six stops included, and no stand task gains a select or any element, class, style or wrapper (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The fixture's select carries an author `id` that is its stable element id, takes its accessible name from a label `for` or an `aria-label` with no wrapper added, and holds its place among the fixture's Tab stops, the stops around it keeping their order (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The fixture boots at the harness default of 800 × 600, scale 1, light mode, mounted in `main#main`, and the select reads in view there, so a driver call that addresses it by id is not refused `off-screen` or `covered` (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) If the chunk adds a verb or argument, `escher-session`'s usage line, `help` and the MCP `tools/list` all name it from the one verb table, keep `help` last, and echo nothing the caller typed (per layout-templates §Surface: cli → Primary screens).
