# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the poll-time drain of the changed set and the platform-tree rebuild through the Document::accessibility_tree override, which the stand_accessibility_ids reading and DioxusDocument's author_id depend on
a11y-strategy#1.2 · yes · the engine's changed-set contract of marks, has_changes and drain, which the mutator.rs insert and the stand_accessibility_ids reading depend on
a11y-strategy#1.3 · yes · AccessibilityDeactivated and ActionRequested stay unhandled in the blitz-shell adapter path that the tokio update must not change
a11y-strategy#1.4 · yes · window focus and bounds forwarded to the adapter on every window event is blitz-shell adapter behaviour the tokio update must not change
a11y-strategy#2.1 · yes · role derivation from the role attribute and the HTML element mapping, re-proven by accessibility_roles through Stylo's move
a11y-strategy#2.2 · yes · the check that only html and body map to Role::Unknown is the accessibility_roles re-proof the chunk must keep
a11y-strategy#2.3 · yes · the builder's selected, pressed or checked by role, invalid and role-switch reads are the four ARIA state reads accessibility_states re-proves
a11y-strategy#2.4 · yes · the merged name and description reads with expanded, disabled and level are the accessibility_label, accessibility_description and accessibility_merged_states re-proof
a11y-strategy#3.1 · yes · the unix arm selecting accesskit_unix is the adapter the tokio advisory fix reaches through accesskit_xplat
a11y-strategy#3.2 · no · a doc-comment remark on accesskit_xplat's approach that no chunk code, test or dependency update reads
a11y-strategy#3.3 · no · the no-op set_focus and set_window_bounds sit on Android, Windows and macOS adapter bodies that the Linux-reached tokio update never reaches
a11y-strategy#3.4 · no · the Android adapter's Rect import and parameter typing is a non-Linux detail that the tokio update and the chunk's tests never reach
a11y-strategy#3.5 · no · the Android adapter's surface-view field typing is platform code the chunk neither edits nor tests
a11y-strategy#4.1 · yes · the accessibility feature gating dioxus-native-dom's override, snapshot and actionable check, forwarded by dioxus-native, is the wiring a Dioxus 0.8 manifest move must keep reaching the stand
