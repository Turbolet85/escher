# layouts extract

## Relevance
partial — the chunk builds no surface and moves no region, but it adds author `id`s to stand task elements and boots an edited-markup fixture in the stand's shell, both of which layout-templates §Surface: desktop-native → Primary screens constrains; the id grammar itself is not layout content.

## Constraints
- The surface is the headless seven_guis stand: one lean task mounted in TaskShell, Home skipped, at the pinned viewport, scale, colour scheme and bundled font — any boot this chunk adds (the edited-variant fixture included) is required to use the same shell and the same pins (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell bullet).
- Author ids on the stand are required to be layout-neutral: an id adds no class, no style, no wrapper and no order change, and no task CSS selects by an id. Every author key rule 2 adds (CRUD person rows are the scope's likely case) falls under this requirement (per layout-templates §Surface: desktop-native → Primary screens).
- The plan names the author ids each lean task's controls and value displays carry as their stable element ids (counter, flight booker, timer, CRUD lists in that bullet). Rule 1 changes how unkeyed paths are spelled, not what a keyed element reads, so those named ids are required to stay exactly as listed after the grammar change (per layout-templates §Surface: desktop-native → Primary screens). Whether the code already carries every listed id as an HTML `id` is research's question.
- The plan requires a CRUD row's id to follow its person under filter, Create and Delete through the Dioxus key `{person.id}`. Whatever rule 2 gives a row (an author key) or rule 1 gives it (a path anchored at a keyed ancestor), that follow-the-person property is required to hold (per layout-templates §Surface: desktop-native → Primary screens).
- The plan places id-bearing shell containers above every lean task (the shell, its header, its body, and the document skeleton's mount element). Under rule 1 these are candidate nearest keyed ancestors shared by all four tasks, which is the collision the scope names for component boundaries below a shared keyed container. Whether each of those containers reads an author key in the code, and which one is nearest for a given task element, is research's question (per layout-templates §Surface: desktop-native → Primary screens; §Surface: desktop-native → IA notes, fixed-skeleton bullet).
- The six task inputs are required to be named by attributes alone, adding no element, class, style or order; a new author key on or near those inputs must not disturb that naming (per layout-templates §Surface: desktop-native → Primary screens).

## Patterns to follow
- Key by attribute only: the plan's stand pattern is an `id` attribute on the existing element, with the component hierarchy, sibling order and classes left as they are (per layout-templates §Surface: desktop-native → Primary screens).
- List identity through the model: a repeated row takes its identity from its model-assigned id, not from its position in the list, so the id survives filtering and insertion (per layout-templates §Surface: desktop-native → Primary screens, CRUD row clause).
- Boot through the stand's one mount path into the shell's body region rather than a bespoke document, so a fixture's elements sit in the same hierarchy as a task's (per layout-templates §Surface: desktop-native → Primary screens; §Surface: desktop-native → IA notes, fixed-skeleton and harness-default bullets).
- The stand checks run under the agent-run contract's `stand` selection, whose usage and output shape the plan records for the cli surface (per layout-templates §Surface: cli → Primary screens, `scripts/agent-run.sh` bullet). Whether a new `stand_*.rs` file is picked up by that selection without a script change is research's question.

## Anti-patterns to avoid
- Adding a wrapper element, class, style or reorder to a stand task in order to give an element a key or a keyed ancestor — the plan requires ids to add none of these (per layout-templates §Surface: desktop-native → Primary screens). The wrap and sibling edits the proof needs belong in the fixture's edited variant, not in the four lean tasks.
- Selecting by a new id in task CSS (per layout-templates §Surface: desktop-native → Primary screens).
- Booting the fixture at a viewport, scale, scheme or font other than the stand's pins, which would make its bounds and ids incomparable with the tasks' (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- layouts ↔ architecture §Standard Contracts → Dioxus DOM bridge: the plan's Primary screens bullet spells a CRUD row's id in today's grammar; the re-spelled grammar and any new row key make that clause a wrap-cascade amendment of layout-templates, alongside the architecture amendment the scope already names.
- layouts ↔ a11y: the plan carries no focus-order section, and this chunk is required to add no focusable element and change no order; a11y-plan owns SC 2.4.3. Rule 2's "focusable or role-bearing" reader is a11y's and the engine's, not layouts'.
- layouts ↔ tests: the pinned viewport, scale, scheme and font the plan records are the harness's boot contract (test-plan §3); the fixture's boot binds there.
- layouts ↔ design: none — no token is touched.

## Acceptance criteria contributions
- Every author key added to a stand task lands as an attribute on an existing element: the task's element count, sibling order, classes and styles are unchanged by the keying (per layout-templates §Surface: desktop-native → Primary screens).
- After the grammar change, each author id the plan lists for the four lean tasks still reads as its element's stable id, unchanged in spelling (per layout-templates §Surface: desktop-native → Primary screens).
- A CRUD row's id still follows its person under filter, Create and Delete, in both layout modes (per layout-templates §Surface: desktop-native → Primary screens).
- The edited-variant fixture boots in the stand's shell at the stand's pinned viewport, scale, colour scheme and bundled font, with no network (per layout-templates §Surface: desktop-native → Primary screens).
