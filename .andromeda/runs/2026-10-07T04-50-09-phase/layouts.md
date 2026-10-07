# layouts extract

## Relevance
partial — the chunk renders no new markup and adds no surface region; layouts binds it only through the surface its held instance must be (the headless TaskShell mount at the pinned viewport) and, if the session gets a binary, the cli surface's command conventions.

## Constraints
- layout-templates §Surface: desktop-native / Primary screens (seven_guis Home and TaskShell) requires the headless stand to skip Home and mount exactly one lean task (Counter, FlightBooker, Timer or Crud) in TaskShell through the `task_in_shell` structure it names; a session's held instance must be that mount for its whole life, not a second layout. Whether a session booted through `stand::boot` yields that structure unchanged is research's question.
- layout-templates §Surface: desktop-native / Primary screens (seven_guis Home and TaskShell) requires the stand's viewport to be pinned (size, scale, colour scheme) with the bundled font; a held instance must keep that pin across every command, so no lifecycle step or command may resize or re-theme it. Whether anything in the session path can alter the viewport after boot is research's question.
- layout-templates §Surface: desktop-native / Primary screens (seven_guis Home and TaskShell) requires each lean task's controls and value displays to carry the author `id`s it lists as their stable element ids, adding no class, style, wrapper or order; the session must reach elements by those ids and must add nothing to the mounted tree to do so.
- layout-templates §Surface: desktop-native / Primary screens (seven_guis Home and TaskShell) requires each CRUD row's author id to follow its person under filter, Create and Delete; on a held instance that rule now spans commands, so a row created or filtered in one command must read the same id in the next.
- layout-templates §Surface: desktop-native / IA notes requires every document to start from the one fixed skeleton with the app mounted into `main`, and records that arbitrary `index.html` templates are not supported; `start` therefore names a task, never a template or an alternate root.
- layout-templates §Surface: desktop-native / IA notes records the test harness's default viewport, scale and colour scheme; a session built on `Harness<DioxusDocument>` must not rely on a default that differs from the stand's pin. Whether the stand's options and the harness default coincide in the session's construction path is research's question.
- layout-templates §Surface: cli / Primary screens describes the agent-invocable command shape of the two existing scripts (a verb with an optional selection, usage on stderr with a usage-error exit, JSON-lines-only stdout, raw output kept under `target/`); if P4 gives the session a binary with `start` / `attach` / `stop` verbs, that shape is the only recorded cli layout to align with. The surface's tooling context, expression level and signature placement are marked not measured and bind nothing.

## Patterns to follow
- Reuse the stand's existing headless mount as the session's screen rather than composing one: one lean task inside TaskShell, header above a scrolling body (per layout-templates §Surface: desktop-native / Primary screens).
- Address elements by the author ids the plan lists per task, and CRUD rows by their person-derived id, so a later command's layout reading lines up with an earlier one's (per layout-templates §Surface: desktop-native / Primary screens).
- Treat the pinned viewport as part of the session's identity: fixed at `start`, the same at `stop` (per layout-templates §Surface: desktop-native / Primary screens and §Surface: desktop-native / IA notes).
- For any lifecycle verb surface, follow the verb-plus-selection usage, stderr usage error and JSON-lines stdout shape of the existing agent-run and cold-agent contracts; the exit grammar and event schema belong to test-plan §3, not to layouts (per layout-templates §Surface: cli / Primary screens).

## Anti-patterns to avoid
- Adding a wrapper, class, style, element or reordering to the task or TaskShell markup for the session's sake, or a CSS selector keyed on the stable ids (banned by layout-templates §Surface: desktop-native / Primary screens).
- Holding Home, a non-lean task or a custom `index.html` template as the session's instance (outside layout-templates §Surface: desktop-native / Primary screens and §Surface: desktop-native / IA notes).
- Reading layout intent from the parts marked `NOT YET MEASURED` (each surface's tooling context, expression level and signature placement) or from §Decisions Log, which reads `NO RECORDED INTENT`.

## Contract bindings
- layouts ↔ tests: the TaskShell structure and viewport pin are witnessed by the stand boot check the plan cites; the session's proof checks under `tests/blitz-tests/tests/` extend that witness to a held instance (layout-templates §Surface: desktop-native / Primary screens ↔ test-plan §2/§5).
- layouts ↔ tests: the cli command shape defers its exit grammar and event schema to test-plan §3 (layout-templates §Surface: cli / Primary screens ↔ test-plan §3).
- layouts ↔ a11y: the six task inputs are named by attributes alone with no added element or order; the session reads the tree those names live in and changes none of it (layout-templates §Surface: desktop-native / Primary screens ↔ a11y-plan §1/§3).
- layouts ↔ architecture: a session binary's verb surface, if P4 opens one, is a cli-surface addition to layout-templates at wrap, alongside the arch §Occupied Resources registration the scope names (layout-templates §Surface: cli / Primary screens ↔ architecture §Occupied Resources).

## Acceptance criteria contributions
- A session started on each of the four lean tasks holds a document whose mount reads the TaskShell structure the plan names, with Home absent (per layout-templates §Surface: desktop-native / Primary screens).
- The held instance's viewport size, scale and colour scheme read the stand's pin after `start` and read the same after a sequence of commands (per layout-templates §Surface: desktop-native / Primary screens).
- After commands that change state on the held instance, every author id the plan lists for that task still resolves to exactly one element, and the mounted tree carries no element, wrapper or order the fresh-per-boot stand does not (per layout-templates §Surface: desktop-native / Primary screens).
- The session's document starts from the fixed skeleton with the app mounted into `main`, and `start` accepts no template or alternate root (per layout-templates §Surface: desktop-native / IA notes).
