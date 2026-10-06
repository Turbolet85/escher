# layouts extract

## Relevance
partial — no surface is created or redesigned, but the remount path, any CRUD row re-key and any new author keys touch the seven_guis TaskShell tree and the lean tasks' author-id set that layout-templates records (desktop-native surface; the stand mounts headlessly into the same tree).

## Constraints
- The headless stand must mount one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell`, skipping Home, with the tree `main#main > #task-shell` and `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. Any remount affordance must leave this tree and viewport as they are (per layout-templates §Surface: desktop-native → Primary screens, seven_guis Home and TaskShell bullet).
- The lean tasks' controls and value displays carry the recorded author `id`s, which are their stable element ids: the `counter-*`, `flight-*`, `timer-*` and `crud-*` set. Persistence checks read against this set. A new or renamed author key is a change to the recorded set and needs an amendment at wrap (per layout-templates §Surface: desktop-native → Primary screens).
- Each CRUD row is recorded as carrying the Dioxus key `{i}`, its index in the people list. If P4 re-keys rows by person identity, the plan's record of the row key changes and must be amended at wrap. If P4 keeps the position key, the record stands (per layout-templates §Surface: desktop-native → Primary screens).
- Author ids must add no class, style, wrapper or order, and no task CSS may select by them. Any author key or row key this chunk adds or changes must keep that rule. Whether the current lean-task CSS already avoids id selectors is research's question (per layout-templates §Surface: desktop-native → Primary screens).
- Home is a centered 640px column of task cards, and TaskShell is a header (back button, title, spacer) over a scrolling body. A remount through the app's own Back/Home navigation would traverse Home ⇄ TaskShell, which the stand does not host today. Choosing that path means the stand has to host Home without changing the TaskShell layout it boots into (per layout-templates §Surface: desktop-native → Primary screens).
- Each document starts from the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, and the app mounts into `main`. A remounted task must mount back under `main#main`, inside the same skeleton (per layout-templates §Surface: desktop-native → IA notes).
- The test harness's default viewport is 800x600 at scale 1 in light mode. A fresh-process boot must run at the same pinned viewport as the first process, so the two id sets come from identical layouts (per layout-templates §Surface: desktop-native → IA notes).

## Patterns to follow
- Mount through `task_in_shell` into the TaskShell header and body regions. This is the stand's one way of placing a task, and both the remount path and the second-process boot should reuse it (per layout-templates §Surface: desktop-native → Primary screens).
- To make an element's id persist, put an author HTML `id` on that control or value display, as the lean tasks already do for their controls. Do not restructure the tree (per layout-templates §Surface: desktop-native → Primary screens).
- `#back-btn` in `#task-header` is the existing navigation exit from a task. An app-path remount goes through this control instead of adding a new one (per layout-templates §Surface: desktop-native → Primary screens).

## Anti-patterns to avoid
- Do not add a wrapper element, class, style or reordering to key elements or to host a remount toggle. The plan bans ids that change structure, and an added sibling or wrapper also shifts the positional `{tag}:{n}` parts this chunk is trying to hold steady (per layout-templates §Surface: desktop-native → Primary screens).
- Do not let task CSS select by the new author ids or row keys (per layout-templates §Surface: desktop-native → Primary screens).
- Do not give the stand a TaskShell tree that differs from the windowed and wasm builds. The stand boots the same `task_in_shell` layout the app renders (per layout-templates §Surface: desktop-native → Primary screens; §Surface: web-spa → Primary screens).

## Contract bindings
- layouts ↔ arch §Standard Contracts → Dioxus DOM bridge: in the id grammar, `{tag}:{n}` sibling indices and `{name}:{k}` component ordinals are positional. Any element the layout adds, removes or reorders inside `#task-shell` therefore changes the ids of the elements after it.
- layouts ↔ a11y §Focus Order (SC 2.4.3): a stand-side remount control that is focusable would add a tab stop to TaskShell. The scope intends no focus-order change, so such an affordance should not be a focusable element in the rendered tree.
- layouts ↔ tests (`tests/blitz-tests/tests/stand_boot.rs`): the pinned viewport and the TaskShell tree recorded here are what the stand boot checks assert. The persistence checks inherit them.

## Acceptance criteria contributions
- (layouts) After a remount, and in a fresh process, the stand task still reads `main#main > #task-shell` and `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Every author key or row key the chunk adds or changes adds no class, style, wrapper or order, and no lean-task CSS selects by it (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) If the chunk changes the lean tasks' author-id set or the CRUD row key, from `{i}` to a person identity, layout-templates' Primary screens record is amended at wrap to the as-built set and key (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) A remount affordance adds no element, and no focusable element, to the TaskShell header or body that the windowed and wasm builds do not also render (per layout-templates §Surface: desktop-native → Primary screens).
