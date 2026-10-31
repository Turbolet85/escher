# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the poll-time drain of the changed set, which runs without the accessibility feature too, and the tree refresh a focus fixup or checkedness fix reaches through it, the route the windowed redraw check reads
a11y-strategy#1.2 · yes · what marks changed_nodes (focus, a checked change, a text input, a select's changed choice in events/keyboard.rs) and that a check reading has_changes() drains first, since a fresh boot leaves the set non-empty
a11y-strategy#1.3 · no · AccessibilityDeactivated and ActionRequested are unhandled arms of the AccessKit user-event match, a per-platform adapter gap the task leaves out; the redraw check reads focus_redraw and neither event
a11y-strategy#1.4 · no · window focus and bounds forwarded to the platform adapter, a gap the task leaves out and one the stand's windowed binary does not build; the redraw check reads element focus, not window focus
a11y-strategy#2.1 · no · role mapping is another unit of accessibility.rs that the change only keeps holding, its edit being the toggled write; the links, lists, tables, labels and landmarks it names are not reached
a11y-strategy#2.2 · no · the assertion that only html and body map to Role::Unknown lives in accessibility_roles.rs, which the chunk does not reach; the served-page rows read focus order and contrast, not roles
a11y-strategy#2.3 · yes · the builder writes no toggled from a native checkbox's or radio's own checked state, the gap the toggled write fix closes; the aria-pressed and aria-checked rules over the five checkable roles, each read once, and an option's selected as its chosen state, bind
a11y-strategy#2.4 · yes · the snapshot's enabled reads the disabled attribute, not aria-disabled, the reading the stand_contrast exempt rule (enabled Some(false)) and the ENABLED_TAB_ORDER rows key on; the labelledby and description reads are text alternatives the task leaves out, and the expanded, level and tag reads are other units of accessibility.rs
a11y-strategy#3.1 · no · the platform module's adapter selection in accesskit_xplat, a per-platform adapter gap the task leaves out
a11y-strategy#3.2 · no · accesskit_xplat's own warning about its approach, a per-platform adapter gap the task leaves out
a11y-strategy#3.3 · no · the Android, Windows and macOS adapter no-ops, a per-platform adapter gap the task leaves out
a11y-strategy#3.4 · no · the Android adapter's Rect import in set_window_bounds, a per-platform adapter gap the task leaves out
a11y-strategy#3.5 · no · the Android adapter's surface-view field type, a per-platform adapter gap the task leaves out
a11y-strategy#4.1 · no · the accessibility feature's gating and forwarding in the Dioxus crates, which the task leaves out with no manifest change; the snapshot the chunk reads is kept as it stands
