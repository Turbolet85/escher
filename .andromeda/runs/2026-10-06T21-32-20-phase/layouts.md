# layouts extract

## Relevance
partial — the chunk builds no surface and moves no component; it reads state off the seven_guis TaskShell stand that layout-templates §Surface: desktop-native → Primary screens describes, and it may add one minimal fixture screen the plan has no row for.

## Constraints
- The proof surface is the headless stand, not Home: the plan requires the stand to skip Home and mount one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell`, with the header-over-body hierarchy it names, so every state reading the chunk proves "on the stand" is a reading of a node inside that task body (per layout-templates §Surface: desktop-native → Primary screens).
- The stand's viewport is pinned: the plan requires one fixed size, scale, colour scheme and bundled font for the stand, so the chunk's stand checks must not introduce a second viewport; the plan sets no viewport for a fixture beside the stand, so whether the fixture boots at the same pin is P4's to decide (per layout-templates §Surface: desktop-native → Primary screens).
- The harness default viewport is the one the stand's pin matches: a fixture booted through the bare harness lands on that default unless it is overridden, and whether the stand's boot or a fixture's boot overrides it in code is research's question (per layout-templates §Surface: desktop-native → IA notes).
- Controls are addressed by the author ids the plan lists: each lean task's controls and value displays must carry an author `id` that is its stable element id, and each CRUD row the `crud-person-{person.id}` form, so the chunk's per-control state checks address nodes by those ids. Whether every control the scope names has an id in that list — the flight booker's `select` in particular, since the plan lists `flight-one-way` and `flight-return` without saying what element each sits on — is research's question (per layout-templates §Surface: desktop-native → Primary screens).
- Ids and names are structure-neutral: the plan requires that ids add no class, style, wrapper or order, that no task CSS selects by them, and that the six task inputs are named by attributes alone; any attribute the chunk adds to a lean task to make a state readable falls under the same rule (per layout-templates §Surface: desktop-native → Primary screens).
- A fixture mounts into the fixed skeleton: each document must start from the fixed `html / head / body / main#main` skeleton with the app mounted into `main`, and arbitrary `index.html` templates are not supported, so the minimal fixture for checkbox, radio and password boots as a Dioxus component under `main#main`, not from an HTML template (per layout-templates §Surface: desktop-native → IA notes).
- The plan has no row for a fixture screen and no focus-order section: the screen list names the stand's four lean tasks and no "minimal fixture beside the stand", and neither it nor the Header, Primary navigation, Footer or IA notes subsections of the same surface set a Tab position for any stand control. A new screen is a layout-templates amendment at wrap, never this phase's, and the order the engine actually walks is research's to measure and a11y-plan's to own (per layout-templates §Surface: desktop-native → Primary screens).

## Patterns to follow
- Boot through the stand's existing entry and hierarchy — `task_in_shell`, TaskShell header above the task body — rather than a bespoke mount, so the snapshot's tree shape and bounds stay those the earlier stand checks pin (per layout-templates §Surface: desktop-native → Primary screens).
- Reach a control by its listed author id, and a CRUD row by its `crud-person-{id}` key; a row's id follows its person under filter, Create and Delete, so a state check after one of those actions re-reads by the same id (per layout-templates §Surface: desktop-native → Primary screens).
- Name or mark a control with attributes only — the `for` / `aria-label` precedent on the six task inputs — when a reading needs something the markup lacks (per layout-templates §Surface: desktop-native → Primary screens).
- Keep stand proofs selectable as stand checks: `agent-run.sh`'s run selections are `stand · all · <blitz-tests file name>`, with the grammar owned by test-plan §3, so a new check file is named so the `stand` selection picks it up (per layout-templates §Surface: cli → Primary screens).

## Anti-patterns to avoid
- Adding a checkbox, radio or password input to one of the four lean tasks to get a case to prove: it changes the control set the plan lists for that task, and the scope's ruling puts those cases on a fixture beside the stand (per layout-templates §Surface: desktop-native → Primary screens).
- Adding a wrapper, class, style or reordering to a lean task or to TaskShell for the sake of a state reading — the plan's "no class, style, wrapper or order" rule covers every id-bearing control (per layout-templates §Surface: desktop-native → Primary screens).
- Booting a state check through Home and navigating in, or at a viewport other than the pinned one — it changes the tree and bounds every stand check is pinned against (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- layouts ↔ a11y: keyboard focus on stand controls binds to a11y-plan's focus-order criterion (SC 2.4.3). layout-templates declares no focus order (§Surface: desktop-native → Primary screens sets none), so the binding is one-sided — a11y-plan is the only authority for which control Tab reaches next.
- layouts ↔ tests: the stand's boot shape and pinned viewport (layout-templates §Surface: desktop-native → Primary screens) are what the `stand_*` checks assert against, and the `stand` run selection (layout-templates §Surface: cli → Primary screens) binds to test-plan §3's agent-run contract.
- layouts ↔ architecture: the author-id list in layout-templates §Surface: desktop-native → Primary screens is the layout-side statement of the stable element ids the snapshot's `id` field carries (arch §Standard Contracts); a fixture's controls need author ids too if they are to pass the actionable-key rule.
- layouts ↔ design: none — no token, colour or spacing is touched.

## Acceptance criteria contributions
- Every stand state check boots one lean task in TaskShell at the pinned viewport, never through Home (per layout-templates §Surface: desktop-native → Primary screens)
- Each proven control is read by the author id the plan lists for it, and each CRUD row by `crud-person-{id}`; a control the scope names that has no listed id is surfaced, not addressed by path (per layout-templates §Surface: desktop-native → Primary screens)
- After the chunk, the four lean tasks and TaskShell carry no added element, wrapper, class, style or reordering — only attributes, if any (per layout-templates §Surface: desktop-native → Primary screens)
- The minimal fixture mounts as a component under `main#main` of the fixed document skeleton, with no HTML template (per layout-templates §Surface: desktop-native → IA notes)
