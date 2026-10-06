# layouts extract

## Relevance
partial — the chunk creates no new surface and changes no markup, but its headless mount must reproduce the seven_guis TaskShell layout and pin the viewport that layout resolves against.

## Constraints
- The headless mount must yield the same TaskShell structure the windowed stand shows (header region holding back button, title and spacer, over a scrolling body region), per layout-templates §Surface: desktop-native / Primary screens ("seven_guis Home and TaskShell"); whether a named-task entry can reach that tree without altering it is research's question.
- The windowed Home → TaskShell flow (Home's centered task-card column, then TaskShell) must stay as the plan describes it, per layout-templates §Surface: desktop-native / Primary screens; the scope's "windowed app renders the same tree" boundary binds here.
- The headless document mounts into the fixed Dioxus skeleton's `main` element and cannot take an arbitrary `index.html` template, per layout-templates §Surface: desktop-native / IA notes; any stand stylesheet the windowed launch supplies must reach the headless boot by a path that skeleton supports (head-element append or launch-config stylesheets, same §IA notes) — which path the stand uses today is research's question.
- The pinned viewport must be named once; the plan records the harness default viewport size, scale and colour scheme, per layout-templates §Surface: desktop-native / IA notes — whether the stand adopts that default or its own value is the P4 decision the scope already names.
- Viewport semantics differ between the windowed shell (viewport is window surface minus safe-area insets) and the harness, per layout-templates §Surface: desktop-native / IA notes; the headless pinned viewport is the layout box, with no inset subtraction, and checks must not assume windowed geometry.
- The wasm entry's layout (canvas fills the full body) must keep working, per layout-templates §Surface: web-spa / Primary screens.

## Patterns to follow
- Mount the app under the fixed `<html><head></head><body><main id="main">` skeleton the headless Dioxus document already builds, per layout-templates §Surface: desktop-native / IA notes.
- Reuse the harness's pinned-viewport construction (one size/scale/scheme tuple) rather than per-check sizes, per layout-templates §Surface: desktop-native / IA notes.
- Treat the document element as the scrolling element when a check reads TaskShell body scroll or client size (root clientWidth/Height are viewport minus scrollbar), per layout-templates §Surface: desktop-native / IA notes.

## Anti-patterns to avoid
- Mounting a bare task component outside TaskShell's header/body regions for a check — the plan's stand surface is TaskShell-wrapped, per layout-templates §Surface: desktop-native / Primary screens.
- Restructuring TaskShell's or Home's layout to make a headless entry reachable — the windowed surface the plan records must not move, per layout-templates §Surface: desktop-native / Primary screens.

## Contract bindings
- layouts ↔ a11y: TaskShell's back button sits first in its header region before the task body's controls; that visible order binds to a11y SC 2.4.3 focus order — layout-templates records no focus-order section, so the order is a11y-plan's to state (stand keyboard harness is out of scope for this chunk).
- layouts ↔ tests: the pinned viewport (layout-templates §Surface: desktop-native / IA notes) is the input every stand check's layout resolves against; it binds to the test-plan harness and the incremental/non-incremental identity invariant where checks assert layout.
- layouts ↔ design: bundled-font text measurement changes TaskShell header/title box sizes; font choice is design's, layout consequence only flagged here.

## Acceptance criteria contributions
- (layouts) Each of the four headless-booted tasks renders inside TaskShell: a header region containing the back button and the task title precedes the task body region, under `main#main` (per layout-templates §Surface: desktop-native / Primary screens; §IA notes).
- (layouts) Every stand check boots at the one named viewport size/scale/colour scheme; no check overrides it (per layout-templates §Surface: desktop-native / IA notes).
- (layouts) The windowed stand still opens on Home and enters the same TaskShell header-over-body layout for each task; the wasm canvas still fills the body (per layout-templates §Surface: desktop-native / Primary screens; §Surface: web-spa / Primary screens).
