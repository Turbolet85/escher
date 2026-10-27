# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · View::poll's changed-set drain and its tree rebuild through `accessibility_tree` sit in window.rs, a merge surface the redraw routing (6587e7e79) must keep
a11y-strategy#1.2 · yes · what marks the changed set and what does not (focus, checked, text, select choice, range move) must survive the mutator.rs conflict, as the Needs' lifecycle line states
a11y-strategy#1.3 · no · the unhandled AccessibilityDeactivated and ActionRequested TODOs sit in application.rs, which no reached file names and the Needs never ask the take to handle
a11y-strategy#1.4 · yes · window focus and outer/inner bounds forwarded before a window event is handled sit in window.rs, a merge surface where winit beta.4 and the redraw routing land
a11y-strategy#2.1 · yes · the HTML-AAM role mapping, which the Needs re-assert through the accessibility_* checks (accessibility_roles re-read) after the take
a11y-strategy#2.2 · yes · only html and body map to Role::Unknown on a semantic page, a value accessibility_roles re-asserts after the take
a11y-strategy#2.3 · yes · the aria-selected, aria-pressed, aria-checked, aria-invalid and label reads plus the select option's chosen state, which the Needs re-assert through accessibility_states after the take
a11y-strategy#2.4 · yes · the merged upstream reads (aria-labelledby, aria-describedby, aria-description, aria-expanded, aria-disabled, aria-level) and the trimmed aria-label rule, which the Needs name as re-asserted by accessibility_label and accessibility_description
a11y-strategy#3.1 · no · the cfg-based platform pick in accesskit_xplat, a component the chunk uses and leaves unchanged, while the scope names only the adapter's creation timing in blitz-shell
a11y-strategy#3.2 · no · a doc note on accesskit_xplat's lib.rs, an unreached file the chunk does not edit and no clause of the scope names
a11y-strategy#3.3 · no · the no-op set_focus and set_window_bounds of the Android, Windows and macOS adapters, an effect the forwarding calls meet without needing action, which the scope never names
a11y-strategy#3.4 · no · the Android adapter's Rect import and parameters in android.rs, a file the take does not reach and the scope never names
a11y-strategy#3.5 · no · the Android adapter's mSurfaceView field typing in android.rs, a file the take does not reach and the scope never names
a11y-strategy#4.1 · yes · the accessibility feature's gate on dioxus-native-dom's accessibility_tree override, snapshot, text form, parts, diff and unkeyed_actionable, and its forwarding in dioxus-native's manifest, which the Needs keep on through the Dioxus default-features change
