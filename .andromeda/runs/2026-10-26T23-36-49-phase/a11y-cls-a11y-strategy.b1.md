# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · names stand_accessibility_ids.rs:363-410 as the witness of the refresh, an a11y-leg file the chunk re-runs green; the shell poll path itself is not reached
a11y-strategy#1.2 · no · the changed set and its drain are not reached (task Not needed); its only named witness, stand_settle.rs, is covered only by the fast leg, a re-check every change owes
a11y-strategy#1.3 · no · blitz-shell's AccessibilityDeactivated and ActionRequested handling is not reached and no check the chunk re-runs witnesses it
a11y-strategy#1.4 · no · blitz-shell's window focus and bounds forwarding to the adapter is not reached and no check the chunk re-runs witnesses it
a11y-strategy#2.1 · yes · names accessibility_roles.rs:1-7, an a11y-leg file the chunk re-runs green; the lint reads role from the Snapshot and maps none
a11y-strategy#2.2 · yes · names accessibility_roles.rs:182-198, the html and body Unknown pin in an a11y-leg file the chunk re-runs green
a11y-strategy#2.3 · yes · names accessibility_states.rs, one of the eight a11y files stand-repeat re-runs three rounds, witnessing the builder's selected, toggled and invalid writes the lint does not read
a11y-strategy#2.4 · yes · names accessibility_label.rs and accessibility_description.rs, a11y-leg files the chunk re-runs green; its enabled reads the disabled attribute, so aria-disabled moves no snapshot reading, the input of the exemption the lint keys on
a11y-strategy#3.1 · no · accesskit_xplat's platform selection is the adapter, not reached (task Not needed) and no check the chunk re-runs names it
a11y-strategy#3.2 · no · accesskit_xplat's upstream caveat on its approach is the adapter, not reached and no check the chunk re-runs names it
a11y-strategy#3.3 · no · the Android, Windows and macOS adapter no-op setters are the adapter, not reached (task Not needed) and no check the chunk re-runs names them
a11y-strategy#3.4 · no · the Android adapter's Rect import and set_window_bounds parameters are the adapter, not reached and no check the chunk re-runs names them
a11y-strategy#3.5 · no · the Android adapter's mSurfaceView field type is the adapter, not reached and no check the chunk re-runs names it
a11y-strategy#4.1 · yes · the snapshot model the lint walks (DioxusDocument::snapshot) is gated by dioxus-native-dom's accessibility feature, so the refactored helpers stay on Snapshot types inside that gate; the accessibility_tree override and actionable-key check it also names are not reached
