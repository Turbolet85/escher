@amendment marker=2026-11-07-mounteddata-buffered-render
title: render-time MountedData calls on every Dioxus document, and their previous-render limit
section: Surface: desktop-native, Platform-Specific Notes (design-system/surface)
change:
  - A new platform note, after the wasm32 item: on every Dioxus document, the windowed `dioxus-native` launch and the headless stand alike, code the vdom runs during a render (a component body or a task it polls) can call `MountedData`'s `set_focus`, `get_client_rect`, `scroll_to`, `scroll`, `get_scroll_offset` and `get_scroll_size` with no `RefCell already borrowed` panic.
  - The note states the mechanism: the render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow.
  - The note states the limit, by construction: such a call acts on the document as the previous render left it, so a node created in that same render is not yet in it and `set_focus_to` returns `false` for it.
why: the report's engine change for every Dioxus document (the buffered render ported onto the Dioxus 0.7 bridge removes the `RefCell already borrowed` panic of a `MountedData` call in a task polled during render) and its limit, by construction (a task polled during render sees the document as the previous render left it).
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
