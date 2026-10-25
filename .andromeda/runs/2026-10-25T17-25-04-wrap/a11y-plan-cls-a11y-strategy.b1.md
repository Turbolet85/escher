# Classified — a11y-plan · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · stale · the session reached stand_accessibility_ids.rs, which this entity cites at lines 363-410 for its Counter proof, and the report does not show those lines hold on the changed file
a11y-strategy#1.2 · no · the changed set, its drain and the harness settle are untouched and none of the files it cites is reached
a11y-strategy#1.3 · no · the shell's unhandled AccessibilityDeactivated and ActionRequested events are untouched and no shell file is reached
a11y-strategy#1.4 · no · the shell's window focus and bounds forwarding to the adapter is untouched and no shell file is reached
a11y-strategy#2.1 · no · the engine's role mapping is untouched, no engine crate moved, and accessibility_roles.rs is not reached
a11y-strategy#2.2 · no · the semantic-page assertion in accessibility_roles.rs is untouched and the session's served pages do not touch it
a11y-strategy#2.3 · yes · stale · the flight booker's aria-pressed mode buttons are retired, so the stand no longer proves the toggled reading (its witness moved to the select's selected reading and to the accessibility_states fixture)
a11y-strategy#2.4 · yes · stale · the reached stand files (flight_booker.rs, stand.rs with its two new served pages) fall under its claim that no stand file states one of the six, and the report does not show that claim still holds
a11y-strategy#3.1 · no · the platform adapter is untouched (no stand binary builds one) and no accesskit_xplat file is reached
a11y-strategy#3.2 · no · the accesskit_xplat lib docs are untouched and no accesskit_xplat file is reached
a11y-strategy#3.3 · no · the adapters' set_focus and set_window_bounds bodies are untouched and no accesskit_xplat file is reached
a11y-strategy#3.4 · no · the Android adapter's imports and parameters are untouched and no accesskit_xplat file is reached
a11y-strategy#3.5 · no · the Android adapter's mSurfaceView field is untouched and no accesskit_xplat file is reached
a11y-strategy#4.1 · no · the accessibility feature gates of dioxus-native-dom and dioxus-native are untouched and no file of either crate is reached
