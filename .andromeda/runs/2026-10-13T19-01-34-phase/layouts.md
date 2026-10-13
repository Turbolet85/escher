# layouts extract

## Relevance
partial — the chunk changes engine interaction, not a layout. Its proving ground is the Timer's `timer-duration` slider inside the headless TaskShell mount, so the measured desktop-native stand facts bind what the change and its checks may touch. The surface's own `> NOT YET MEASURED` blockquote, about tooling context, expression level and signature placement, is not cited.

## Constraints
- The stand's controls carry author `id`s that are their stable element ids, `timer-duration` and `timer-duration-value` among the Timer's. layout-templates §Surface: desktop-native → Primary screens requires that ids add no class, style, wrapper or order, and that no task CSS selects by them. The range model therefore lands in the engine with no change to the Timer's markup.
- `timer-duration` is named by the `for` on its existing label, an attribute that adds no element, class, style or order (per layout-templates §Surface: desktop-native → Primary screens). The rewritten `stand_act_range` keeps reading the slider through that name and id.
- The headless stand mounts one lean task through `task_in_shell`: `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`. It runs at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans (per layout-templates §Surface: desktop-native → Primary screens). A held instance reads as a fresh boot, and the mount and pinned viewport hold after commands in both layout modes. Range steps and clicks driven through `click` and `press` must leave that unchanged.
- TaskShell is a header over a scrolling body (per layout-templates §Surface: desktop-native → Primary screens). The slider sits inside `#task-body`, a scrolling box, so a click's position along the track has to be resolved against the slider's laid-out box where it actually stands, including any scroll of the boxes that hold it.
- Pointer client coordinates subtract the safe-area left/top insets, and page coordinates add the viewport scroll (per layout-templates §Surface: desktop-native → IA notes). Set-by-position must read the click in one consistent frame with the element's bounds.
- The test harness defaults to an 800×600 viewport at scale 1 in light mode (per layout-templates §Surface: desktop-native → IA notes). Track positions and expected snapped values in the rewritten check are computed at that pinned geometry.

## Patterns to follow
- Where a stand task lacks a case, a minimal fixture sits beside the stand and the stand stays as it is (per layout-templates §Surface: desktop-native → Primary screens). This is how the scrolled CRUD-list and pinned-viewport readings were taken, with "no stand markup, id, class or style moved". A fixture is the layout-safe route for an off-centre boot value or a different `min`/`max`, rather than editing the Timer's markup.
- Scroll and bounds readings are measured on the headless stand in both layout modes and cited to a chunk's evidence (per layout-templates §Surface: desktop-native → IA notes). Whether the slider's bounds and a centre `click` land where the track maths expects is a measured reading at P3/P4, not an assumption.

## Anti-patterns to avoid
- Adding a wrapper, class, style, id or reorder to the Timer, or a CSS selector on `timer-duration`, to make the range testable or paintable. layout-templates §Surface: desktop-native → Primary screens holds each lean task's id and naming additions to "adding no element, class, style or order".
- Building vertical or right-to-left track geometry, or a styled track or thumb. The plan records no range-track layout anywhere in §Surface: desktop-native, and the scope rules these out.

## Contract bindings
- layouts ↔ tests: the TaskShell mount and pinned viewport "hold after commands, in both layout modes" (layout-templates §Surface: desktop-native → Primary screens, measured by `stand_session_fresh`). Range calls through the driver fall under that invariant, together with the incremental/non-incremental identity tests.
- layouts ↔ a11y: whether a pointer click focuses the slider, and the slider's place after two Tab presses, bind to a11y focus order (SC 2.4.3) and keyboard reach (SC 2.1.1). layout-templates holds no focus-order section, so focus placement is a11y's call and P4's decision, not this domain's.

## Acceptance criteria contributions
- The Timer's markup diff is empty: `timer-duration`, `timer-duration-value` and the `for` label keep their ids and naming, with no added element, class, style, wrapper or order (per layout-templates §Surface: desktop-native → Primary screens).
- After range `press` and `click` calls on the held Timer, the TaskShell mount (`main#main > #task-shell`, header above `#task-body`) and the pinned 800 × 600 viewport read as on a fresh boot, in both layout modes (per layout-templates §Surface: desktop-native → Primary screens).
- A click at a chosen off-centre point on the track sets the value its position maps to, computed from the slider's bounds as laid out at the harness's pinned 800×600 scale-1 viewport. A centre click on a fixture whose boot value is not the midpoint reads the midpoint (per layout-templates §Surface: desktop-native → IA notes).
