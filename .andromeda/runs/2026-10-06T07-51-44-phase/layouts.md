# layouts extract

## Relevance
partial. The chunk adds no surface, screen, region or focusable element. But the upstream delta changes layout (the Taffy bumps, `align-content` for block/inline/table, self-alignment of absolutely positioned boxes, the `normal` keyword in stylo_taffy) and touches files behind the desktop-native IA notes. So the measured layout of the seven_guis stand has to survive the merge.

## Constraints
- layout-templates §Surface: desktop-native / Primary screens requires the headless stand to mount one lean task through `task_in_shell` into the hierarchy `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`. The merge must leave this hierarchy intact for all four tasks (Counter, FlightBooker, Timer, Crud).
- layout-templates §Surface: desktop-native / Primary screens pins the stand at 800 × 600, scale 1.0, Light, with the bundled DejaVu Sans. Upstream's Taffy and Parley pin moves must not require changing that viewport or font to keep the stand checks green.
- layout-templates §Surface: desktop-native / IA notes requires every Dioxus document to start from the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, with the app mounted into `main`. Whether upstream's new `Document::children` accessor (#1069) or its `document.rs` changes touch that skeleton path is a question for research.
- layout-templates §Surface: desktop-native / IA notes lists three behaviours whose cited files are in the upstream delta:
  - the document element is the scrolling element, and `scrollTo`/`scrollBy` scroll the root (blitz-vibey-script `dom/document.rs`, `runtime.rs`)
  - root `clientWidth`/`clientHeight` equal the viewport size minus the scrollbar (`dom/element.rs`)
  - fixed-position children of the root do not scroll with the viewport (blitz-paint `render.rs`)

  Whether the merged code still behaves this way is a question for research.
- layout-templates §Surface: desktop-native / IA notes records the test harness default of 800x600, scale 1, light mode. The merge carries our `blitz-test-harness` changes unchanged, so that default must hold after the merge.

## Patterns to follow
- Boot the stand through `task_in_shell` in `seven_guis::stand`, skipping Home, as layout-templates §Surface: desktop-native / Primary screens records. That path is how post-merge layout is proven.
- Use the harness default viewport (layout-templates §Surface: desktop-native / IA notes) for every post-merge layout check. Do not pick a viewport per test.
- Record the new upstream layout APIs (`Node::inline_fragment_boxes`, `Document::children`) for later chunks only. layout-templates §Surface: desktop-native carries no region or component entry that this chunk would add them to.

## Anti-patterns to avoid
- Do not change TaskShell's structure, the stand's viewport pin, or the task CSS to absorb a box shift caused by upstream's alignment or Taffy changes. A layout regression shows as a failing stand check and is a halt to name. It must not be silently re-baselined against layout-templates §Surface: desktop-native / Primary screens.
- Do not add, rename or re-region a surface, screen or component in this chunk. layout-templates §Decisions Log has no recorded intent that would authorize it.

## Contract bindings
- layouts ↔ tests: the TaskShell hierarchy and pinned viewport in layout-templates §Surface: desktop-native / Primary screens are asserted by `tests/blitz-tests/tests/stand_boot.rs:48-58`, which the plan cites. The post-merge stand checks are the layout proof for this chunk.
- layouts ↔ a11y: the TaskShell order (`#back-btn` before `#task-title` and `#task-body`, per layout-templates §Surface: desktop-native / Primary screens) is the DOM order that SC 2.4.3 focus order follows. The merge adds no focusable element, so this order must not change.

## Acceptance criteria contributions
- (layouts) After the merge, `stand_boot` passes. Each lean task mounts at `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600 / scale 1.0 / Light viewport with DejaVu Sans, and stand.rs, app.rs and the task CSS are unchanged (per layout-templates §Surface: desktop-native / Primary screens).
- (layouts) After the merge, `stand_counter`, `stand_crud`, `stand_flight_booker` and `stand_timer` pass without edits to their layout expectations (per layout-templates §Surface: desktop-native / Primary screens).
- (layouts) After the merge, the root-scrolling, root `clientWidth`/`clientHeight` and fixed-position-not-scrolled behaviours still hold, wherever a workspace test covers them. Research names those tests, or records that none exist (per layout-templates §Surface: desktop-native / IA notes).
- (layouts) The chunk's diff leaves `.andromeda/layout-templates.md` unchanged and adds no surface, screen or component region (per layout-templates §Decisions Log).
