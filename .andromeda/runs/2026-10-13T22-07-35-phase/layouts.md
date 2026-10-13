# layouts extract

## Relevance
partial — the chunk adds no surface, region or component; it walks focus over the existing headless stand surface, whose mount, control ids and CRUD list structure the plan records (per layout-templates §Surface: desktop-native → Primary screens); the plan has no focus-order section, so the Tab order itself is not covered here.

## Constraints
- The harness runs on the headless stand's own mount: one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell`, with the shell header (`#back-btn`, `#task-title`) above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled font. A boot that skips TaskShell or Home is a different surface (per layout-templates §Surface: desktop-native → Primary screens).
- Every focus stop a step reads back is named by the stable element id the plan lists for that lean task's controls, or by the shell's `back-btn`. A CRUD row is named `crud-person-{person.id}` and follows its person (per layout-templates §Surface: desktop-native → Primary screens). Whether each listed control is in fact focusable, and in what order, is research's question.
- Stand ids and the attribute-stated roles and states (`aria-pressed` on `flight-one-way` / `flight-return`, `role="listbox"` on `crud-list`, `role="option"` with `aria-selected` on each row) were added with no element, class, style, key, id or order. A fork answer for freight 8 that puts a `tabindex` on the rows changes that order, so that stand-markup answer amends this plan's Primary screens entry beside the Tab order other plans pin (per layout-templates §Surface: desktop-native → Primary screens).
- A held driver instance reads as a fresh boot: after commands the TaskShell mount and the pinned viewport still hold, in both layout modes. Driver-run Tab and Shift+Tab steps (freight 3, `stand_act_keys`) are read against that same mount (per layout-templates §Surface: desktop-native → Primary screens).
- `crud-list` is the stand's one nested scrolling box and clips its rows. `task-body` is not the box that clips a row. The pinned viewport is scrolled only on an in-file fixture taller than it (per layout-templates §Surface: desktop-native → IA notes). For freight 7, an activation key on a focused box that is itself scrolled therefore needs a fixture beside the stand, unless research finds a stand control that is both scrolled and takes a key.
- The harness defaults to an 800 × 600 viewport at scale 1 in light mode. A fixture added beside the stand (inert, disabled-attribute or scrolled-box cases) states its viewport if it departs from that default (per layout-templates §Surface: desktop-native → IA notes).

## Patterns to follow
- Address controls by the author `id` the plan lists per lean task. That id is the stable element id, and no task CSS selects by it (per layout-templates §Surface: desktop-native → Primary screens).
- Use a minimal in-file fixture where the stand lacks a case, as the scrolled-viewport reading did. Leave the stand markup unchanged for test needs (per layout-templates §Surface: desktop-native → IA notes).
- Each document starts as the fixed skeleton with the app mounted into `main#main`. A fixture's focus sequence starts inside that skeleton (per layout-templates §Surface: desktop-native → IA notes).

## Anti-patterns to avoid
- Adding a class, style, wrapper or order to stand markup to make a key reach a control. A stand change must keep the "adds no element, class, style or order" rule the plan records, or amend that entry (per layout-templates §Surface: desktop-native → Primary screens).
- Driving the harness against the windowed stand or Home and calling it the stand's reading. The windowed stand is recorded as not booted, and Home is skipped headlessly (per layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- Focus order ↔ a11y-plan §5 (Tab and Shift+Tab focus checks, focus restoration) and SC 2.4.3: the layout plan records the visible structure (the shell header above the task body, controls in markup order) that the focus sequence is read against. It records no Tab order of its own.
- CRUD row reachability ↔ test-plan (the CRUD forward focus sequence held by `stand_accessibility_ids`): an answer that puts a `tabindex` on the rows moves both that pin and this plan's Primary screens entry.
- Stable ids ↔ dioxus-native-dom element ids and snapshot `focused`: the ids this plan lists are the ids the read-back's three readings must agree on.

## Acceptance criteria contributions
- (layouts) Every lean-task step boots TaskShell with `#task-header > #back-btn + #task-title` above `#task-body` at 800 × 600, scale 1.0, Light, in both layout modes (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Every focused id the harness reads back on a lean task is one of that task's listed control ids, a `crud-person-{n}` row, or `back-btn`. No unlisted id appears (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) `git diff` over `examples/seven_guis/src/{app.rs,tasks/}` adds no class, style, wrapper or element order. Otherwise the Primary screens entry is amended in the same chunk (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Any scrolled-box activation-key case (freight 7) runs on a fixture beside the stand, or on `crud-list`, the stand's one nested scroller (per layout-templates §Surface: desktop-native → IA notes).
