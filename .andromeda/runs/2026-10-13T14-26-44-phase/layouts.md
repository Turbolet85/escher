# layouts extract

## Relevance
partial: the chunk creates and changes no surface, but upstream's layout-moving delta (Taffy git `rev` to `0.15`, #1175 UA button centering, #1174 anonymous-box restyle, #1185/#1186/#1189 in `dioxus_document.rs`) reaches the headless stand's mount and its recorded layout readings. The README edit is front-page text, not a layout surface.

## Constraints
- The headless stand mounts one lean task in TaskShell through `task_in_shell`: `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. That mount must hold unchanged after the merge in both layout modes. Per layout-templates §Surface: desktop-native → Primary screens.
- The stand's author `id`s, `for`/`aria-label` names and `aria-*` state attributes add no element, class, style, wrapper, key or order, and no task CSS selects by them. A merge resolution or gate fix must not touch stand markup to absorb an upstream change. Per layout-templates §Surface: desktop-native → Primary screens.
- The `escher-session` host role and its `mcp` role each boot through the same `seven_guis::stand` mount, adding nothing to it. A held instance must read as a fresh boot, with the TaskShell mount and pinned viewport holding after commands, in both layout modes. Per layout-templates §Surface: desktop-native → Primary screens.
- Every document starts from the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, with the app mounted into `main` and head elements appended to `<head>`, as read in `dioxus_document.rs`. Upstream edits that file (+318 −6, #1185/#1186/#1189), so whether the merged file still builds the same skeleton and mount point is research's question. Per layout-templates §Surface: desktop-native → IA notes.
- The scrolling model requires two things:
  - `scroll_into_view` scrolls every scrolling box that holds its target, innermost first, then the viewport.
  - `visible_region` reads the viewport narrowed to the padding box of every clipping box.

  On the stand the nested scroller is `crud-list`, not `task-body`. Whether upstream's `document.rs`/`node.rs` edits change the geometry reader under these is research's question. Per layout-templates §Surface: desktop-native → IA notes.
- The test harness defaults to an 800 × 600 viewport at scale 1 in light mode, and the stand's pinned viewport rests on it. Per layout-templates §Surface: desktop-native → IA notes.
- The `escher-session` command line serves eleven verbs in three levels (`screenshot` the seventh instance verb, `help` last). The front page the chunk brings current must state the same count and the built screenshot. Per layout-templates §Surface: cli → Primary screens.

## Patterns to follow
- Stand changes, where any are forced, follow the attribute-only pattern: they state names, roles and states by attributes alone and add no element, class, style or order. Per layout-templates §Surface: desktop-native → Primary screens.
- Layout figures are recorded as measured readings with an evidence path ("as measured at escher-0.1.0/chunks/…/evidence/…"). These include the `crud-list` bounds, the `task-body` bounds and Home's Cells card at 615 to 684. A figure that Taffy 0.15 or #1175 moves is re-measured and recorded the same way. Per layout-templates §Surface: desktop-native → IA notes.
- One boot path serves three consumers: the stand checks, the host role and the `mcp` role. A post-merge stand proof covers all three through that one mount. Per layout-templates §Surface: desktop-native → Primary screens.

## Anti-patterns to avoid
- Editing stand markup (an id, class, style, wrapper or order) to make a moved layout number or a changed button alignment pass. The stand's attributes are specified to add none of these. Per layout-templates §Surface: desktop-native → Primary screens.
- Treating a recorded layout reading as a tolerance to widen. The `crud-list` figures, the scroll behaviour at 12 and 14 Creates and `task-body`'s figures are measured values, to be re-read and either confirmed or re-recorded. Per layout-templates §Surface: desktop-native → IA notes.

## Contract bindings
- layouts ↔ a11y: TaskShell's header order (`#back-btn` then `#task-title`, above `#task-body`) is the focus-order surface (WCAG SC 2.4.3) that the scope says the standing checks re-prove. Per layout-templates §Surface: desktop-native → Primary screens.
- layouts ↔ tests: the mount is pinned by `tests/blitz-tests/tests/stand_boot.rs:48-58` and the fresh held instance by `stand_session_fresh.rs:39-117`. The nested-scroller readings are pinned by `stand_act_scroll.rs`. All three run under `for incremental in [false, true]`. Per layout-templates §Surface: desktop-native → Primary screens / IA notes.
- layouts ↔ design: the pinned Light mode and the bundled DejaVu Sans font are design's tokens. Layout requires only that the stand boots with them. Per layout-templates §Surface: desktop-native → Primary screens.

## Acceptance criteria contributions
- (layouts) After the merge, a stand boot of each lean task (Counter, FlightBooker, Timer, Crud) reads `main#main > #task-shell` with `#task-header > #back-btn + #task-title` above `#task-body` at 800 × 600, scale 1.0, Light, in both layout modes (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The merge commit changes no file under `examples/seven_guis/src/`. Any forced stand edit is named, minimal and recorded non-additive (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Both of these read the recorded figures, or the new figure is recorded with its evidence path and never loosened (per layout-templates §Surface: desktop-native → IA notes):
  - `crud-list` reads (24, 112.796875, 369 × 419.203125) and `task-body` reads (0, 46, 800 × 554) at boot.
  - After twelve Creates, a `click` on the last row is refused `off-screen`, and a `scroll` brings that row inside the list.
- (layouts) A held instance reads as a fresh boot after commands, in both layout modes, through the host role's boot (per layout-templates §Surface: desktop-native → Primary screens).
