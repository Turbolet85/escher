@leaf_group group=design-system
@leaf_read leaf=.claude/docs/design-summary.md lines=44
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=33 class=omission
leaf-text: the app mounts into `main`.
source: fct-zgbhkh7f «can call `MountedData`'s `set_focus`, `get_client_rect`, `scroll_to`, `scroll`, `get_scroll_offset` and `get_scroll_size` with no `RefCell already borrowed` panic»
replace-with: the app mounts into `main`. On every Dioxus document, windowed and headless alike, code run during a render (a component body or a task it polls) can call `MountedData`'s `set_focus`, `get_client_rect`, `scroll_to`, `scroll`, `get_scroll_offset` and `get_scroll_size` with no `RefCell already borrowed` panic — the render is recorded into a mutation buffer and replayed under one borrow, so such a call acts on the document as the previous render left it and `set_focus_to` returns `false` for a node created in that same render.
