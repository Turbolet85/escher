# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the refresh's headless witness stand_accessibility_ids.rs:363-410 is an a11y-leg check the chunk keeps green; the shell's poll path itself is not reached
a11y-strategy#1.2 · no · the changed set and its drain are not reached (the lint is read-only and marks nothing); stand_settle.rs:78-119 is outside the chunk's re-runs
a11y-strategy#1.3 · no · the shell's AccessKit event handler (AccessibilityDeactivated, ActionRequested) is not reached and no re-run check names it
a11y-strategy#1.4 · no · window focus and bounds forwarding to the adapter is shell code the chunk does not reach and no re-run check names it
a11y-strategy#2.1 · yes · the lint takes each node's role from the Snapshot (Needs 3), and accessibility_roles.rs:1-7 is an a11y-leg check the chunk keeps green
a11y-strategy#2.2 · yes · only html and body map to Role::Unknown, the role value the lint takes from the Snapshot, and accessibility_roles.rs:182-198 is an a11y-leg check the chunk keeps green
a11y-strategy#2.3 · yes · names accessibility_states.rs:134-316, one of the eight a11y files the stand-repeat leg re-runs three rounds
a11y-strategy#2.4 · yes · the name and enabled readings the lint takes from the Snapshot (aria-labelledby and aria-label for name; enabled reads the disabled attribute, which aria-disabled leaves alone), and accessibility_label.rs and accessibility_description.rs are a11y-leg checks kept green
a11y-strategy#3.1 · no · the accesskit_xplat platform-module selection is the AccessKit platform adapter, which the chunk does not reach and no re-run check names
a11y-strategy#3.2 · no · an accesskit_xplat source warning in the platform adapter, which the chunk does not reach and no re-run check names
a11y-strategy#3.3 · no · the no-op set_focus and set_window_bounds in the Android, Windows and macOS adapters, which the chunk does not reach
a11y-strategy#3.4 · no · the Android adapter's Rect import in the platform adapter, which the chunk does not reach
a11y-strategy#3.5 · no · the Android adapter's mSurfaceView field type in the platform adapter, which the chunk does not reach
a11y-strategy#4.1 · yes · the Snapshot the lint walks (DioxusDocument::snapshot) is gated by dioxus-native-dom's accessibility feature, which the lint's code and tests must build under
