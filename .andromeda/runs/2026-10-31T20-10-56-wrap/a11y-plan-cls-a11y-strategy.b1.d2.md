# Classified — a11y-plan · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · stale · the list's open and close marks and the close-on-scroll mark reach this poll path, and document.rs is reached whole so the cited lines 191-199 may have moved
a11y-strategy#1.2 · yes · contradict · a scroll that closes an open list now marks the select, which the "scroll does not mark" reading denies, and opening or closing a list is a new marking site
a11y-strategy#1.3 · no · the AccessibilityDeactivated and ActionRequested TODOs live in blitz-shell, which the session leaves untouched
a11y-strategy#1.4 · no · window focus and bounds forwarding lives in blitz-shell, outside the change
a11y-strategy#2.1 · no · the role-mapping lines 146-156 and 149-150 sit above the accessibility.rs insert at 238-242 and hold, and the session maps no link, list, table, label or landmark differently
a11y-strategy#2.2 · no · accessibility_roles.rs is unedited and its citations stand per the inventory, and the list adds no Unknown-mapped element to a semantic page
a11y-strategy#2.3 · no · the three state reads at accessibility.rs 164-236 sit above the 238-242 insert and hold, and the single-select expanded write is not one of the three
a11y-strategy#2.4 · no · the native checkbox and radio reads at accessibility.rs 199-224 sit above the 238-242 insert and the session changes no checkable role
a11y-strategy#2.5 · yes · stale · aria-expanded is no longer read on a single select, which writes expanded from the list's open state, and the cited 237-268 spans the accessibility.rs insert at 238-242
a11y-strategy#3.1 · no · the accesskit_xplat platform module is unedited and outside the change
a11y-strategy#3.2 · no · accesskit_xplat is unedited and outside the change
a11y-strategy#3.3 · no · accesskit_xplat is unedited and outside the change
a11y-strategy#3.4 · no · accesskit_xplat is unedited and outside the change
a11y-strategy#3.5 · no · accesskit_xplat is unedited and outside the change
a11y-strategy#4.1 · no · no manifest moved so dioxus-native/Cargo.toml:32 holds, the dioxus-native-dom lib.rs citations are not in the moved list, and the snapshot model gains no field so its gated items are unchanged
