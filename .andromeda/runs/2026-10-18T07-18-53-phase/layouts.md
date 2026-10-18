# layouts extract

## Relevance
partial — the chunk adds no surface, region or focusable element. Its proof does scroll and then shoot inside the stand's headless mount and its scrolling boxes. Its schema-text edit touches the `escher-session` CLI surface's `screenshot` and `advance` verbs. Both bind layout facts the change must leave as they are.

## Constraints
- Every proof of the fade (a scroll, then two shots, then an advance) runs in the headless stand's mount. layout-templates §Surface: desktop-native → Primary screens requires that mount: one lean task in TaskShell through `task_in_shell` (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`), at the pinned 800 × 600 viewport, scale 1.0, Light, with the bundled DejaVu Sans. Per §Surface: desktop-native → Primary screens.
- layout-templates §Surface: desktop-native → Primary screens also requires that a held instance read as a fresh boot, with the TaskShell mount and the pinned viewport holding after commands in both layout modes. A clock that only `advance` moves must not change that. Per §Surface: desktop-native → Primary screens.
- Ids, labels and ARIA states were each added to the stand by an attribute alone: "adding no element, class, style, wrapper or order". The same rule applies to any change made to give the stand an overlay scrollbar to shoot. Whether the stand's headless build paints overlay scrollbars at all (`cfg!(feature = "scrollbars")`) is research's question. Per §Surface: desktop-native → Primary screens.
- layout-templates §Surface: desktop-native → IA notes names the stand's scrolling boxes. On the headless stand the nested scroller is `crud-list`, the box that clips its rows. It reads (24, 112.796875, 369 × 419.203125) at boot and the same four figures after its content scrolls. `task-body` reads (0, 46, 800 × 554) and is not the clipping box. The pinned viewport itself is scrolled only on an in-file fixture taller than it. These are where a scroll-then-shoot proof can stamp scrollbar activity. Per §Surface: desktop-native → IA notes.
- `scroll_into_view` moves every scrolling box that holds its target, innermost first, then the viewport, for every document. A driver `scroll` can therefore show scrollbars on more than one box in one step, and each box's fade must read the one clock. Per §Surface: desktop-native → IA notes.
- The harness defaults to an 800x600 viewport at scale 1 in light mode, and the root element's `clientWidth`/`clientHeight` are the viewport size minus the scrollbar size. The fade is an opacity over time and must move no geometry reading. Whether overlay scrollbars take layout space today is research's question. Per §Surface: desktop-native → IA notes.
- layout-templates §Surface: cli → Primary screens fixes the `escher-session` surface: eleven verbs in three levels, `--ms` the flag of `advance`, and `screenshot [--id]` answering one uncoloured line `{"width":…,"height":…,"png":"<base64>"}` with exit 0, refused `fonts-unavailable` / `off-screen` with exit 1. Over MCP an accepted shot is an `image` block, then a text block. Rewriting the stated limit in the help text must move none of these shapes. Per §Surface: cli → Primary screens.

## Patterns to follow
- A minimal fixture beside the stand where the stand lacks the case. The viewport scroll is already proven only on an in-file fixture taller than the 800 × 600 viewport. A fade proof that needs a painted overlay scrollbar or a viewport scroll follows that form rather than reshaping a stand task (layout-templates §Surface: desktop-native → IA notes).
- The CRUD list after twelve Creates is the stand's measured nested-scroll case. A driver `scroll` brings the last row inside the list, and the list's own bounds hold. This is the existing stand scene for "right after a scroll" (layout-templates §Surface: desktop-native → IA notes).
- A held instance proven to read as a fresh boot after commands, in both layout modes (`stand_session_fresh`). That proof's shape fits "the same state shot twice" (layout-templates §Surface: desktop-native → Primary screens).

## Anti-patterns to avoid
- Adding an element, class, style, wrapper, id or order to a stand task to make it scroll or show a scrollbar (layout-templates §Surface: desktop-native → Primary screens).
- Changing the CLI surface while rewriting the `screenshot` limit sentence: no new verb, flag, ending or answer key, and nothing the caller typed written back (layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts ↔ tests: the `crud-list` four-figure readings and the scroll-into-view scene are pinned by `tests/blitz-tests/tests/stand_act_scroll.rs`, and the fresh-boot reading by `stand_session_fresh.rs`. A clock change that moved either reading would show there (layout-templates §Surface: desktop-native → IA notes; §Surface: desktop-native → Primary screens).
- layouts ↔ a11y §Focus Order (SC 2.4.3): CRUD's Tab sequence `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` (six stops) is layout-recorded. A clock must not move it, and `stand_keyboard` re-proves it (layout-templates §Surface: desktop-native → Primary screens).
- layouts ↔ test-plan §3 Session lifecycle: the `escher-session` endings and statuses that the CLI surface describes are owned there (layout-templates §Surface: cli → Primary screens).

## Acceptance criteria contributions
- (layouts) After a driver `scroll`, two `screenshot`s and an `advance`, the held instance still reads the TaskShell mount at the pinned 800 × 600, scale 1.0, Light viewport, in both layout modes (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) `crud-list` reads (24, 112.796875, 369 × 419.203125) before and after its content is scrolled and before and after an `advance` moves the fade. The clock moves no bounds reading (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) Focus order: CRUD's Tab sequence still reads its six stops, `crud-list` third (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) `escher-session` still lists eleven verbs. `advance` still takes `--ms`, and an accepted `screenshot` still answers one line with exactly `width` · `height` · `png`, exit 0, after the help text's limit sentence is rewritten (per layout-templates §Surface: cli → Primary screens).
