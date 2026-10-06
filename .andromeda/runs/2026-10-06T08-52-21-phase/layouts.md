# layouts extract

## Relevance
partial — the chunk creates and moves no surface, region or component. But the element tree it ids is the seven_guis TaskShell screen. Its author-key rule may add markup to that screen, and the screen's layout structure must not change.

## Constraints
- The headless stand screen is the desktop-native TaskShell, not Home. Per layout-templates §Surface: desktop-native §Primary screens, one lean task (Counter, FlightBooker, Timer, Crud) mounts in TaskShell through `task_in_shell` with this hierarchy: `main#main > #task-shell`, then `#task-header > #back-btn + #task-title` above `#task-body`. That hierarchy is the tree the ids are computed over, and any author key added to the stand must leave it intact.
- §Primary screens defines TaskShell as a header (back button, title, spacer) over a scrolling body. Adding keys to chrome or task controls must not add, remove or reorder these regions or put wrapper elements into them.
- Ids are read on the pinned boot: 800 × 600, scale 1.0, Light, bundled DejaVu Sans (per layout-templates §Surface: desktop-native §Primary screens), which matches the harness default of 800x600, scale 1, light (per §Surface: desktop-native §IA notes). The proof checks boot at that viewport.
- Per §Surface: desktop-native §IA notes, every document starts from the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, and the app mounts into `main`. These skeleton elements are part of the screen but come from no stand component. So "every element" must state whether the skeleton gets an author key, a component path or something else, and that is a P4 decision.
- Per §Surface: desktop-native §IA notes, head elements (title, meta, script, style, link) are appended to `<head>`. If any are present they are elements as well, so the id walk's coverage of `<head>` content needs a stated rule.
- Per layout-templates §Surface: web-spa §Primary screens, the seven_guis web build fills the full body with its WASM canvas. Stand markup changes made for keys must render the same TaskShell tree on the windowed, wasm and headless paths.

## Patterns to follow
- Chrome anchors: the TaskShell chrome is already addressed by HTML `id` anchors (`main`, `task-shell`, `task-header`, `back-btn`, `task-title`, `task-body`), per layout-templates §Surface: desktop-native §Primary screens. These anchors are the existing author-named layout landmarks and the natural candidates for the chrome's author keys. Whether the code's chrome ids match this list exactly is research's question.
- Selector-path notation: the plan writes the stand structure as a selector path (`main#main > #task-shell`, `#task-header > #back-btn + #task-title`), per §Surface: desktop-native §Primary screens. That is a structural, `NodeId`-free description of position in the tree, which makes it a reference point when P4 picks the component-path form.
- Region-scoped keys: give keys region by region (header controls vs. body controls) so each id stays readable against the surface hierarchy in §Primary screens.

## Anti-patterns to avoid
- Restructuring the screen for ids: do not add wrapper elements, reorder siblings or move a control between `#task-header` and `#task-body` to make an id derivable. The id follows the layout and must not reshape it (per §Surface: desktop-native §Primary screens).
- Changing the boot target: do not mount Home or a non-lean task on the stand. The stand screen is one lean task in TaskShell (per §Surface: desktop-native §Primary screens).
- Changing focus: do not make a keyed element focusable, or change its tab position, as a side effect of adding markup for keys.

## Contract bindings
- layouts ↔ a11y: the TaskShell controls (`#back-btn` and the task controls) are the screen's focus order, which binds to a11y SC 2.4.3. The scope intends no focusability or focus-order change, so key markup must be inert for focus.
- layouts ↔ tests: the TaskShell hierarchy at the pinned 800 × 600 boot is what the stand checks in `tests/blitz-tests/tests/stand_*.rs` boot against (per §Surface: desktop-native §Primary screens). The new per-task id checks share that boot.
- layouts ↔ v010-03 (out of scope): the region hierarchy here is the frame that the later accessibility-tree identity work names controls within. This chunk only must not foreclose it.

## Acceptance criteria contributions
- (layouts) For each lean task, the booted stand still has `main#main > #task-shell` with `#task-header > #back-btn + #task-title` above `#task-body` after any author-key markup is added (per layout-templates §Surface: desktop-native §Primary screens).
- (layouts) No element's layout box in the booted TaskShell, at 800 × 600, scale 1.0, Light, changes because of the key markup: compare boxes before and after, or check against the existing stand layout assertions (per layout-templates §Surface: desktop-native §Primary screens).
- (layouts) The "every element reads an id" walk covers the skeleton elements outside the app mount (`html`, `head`, `body`, `main#main`) under the rule P4 chose, and asserts it explicitly rather than skipping them silently (per layout-templates §Surface: desktop-native §IA notes).
- (layouts) Focus order: the TaskShell's focusable elements and their order are identical before and after the change (per layout-templates §Surface: desktop-native §Primary screens; binds a11y SC 2.4.3).
