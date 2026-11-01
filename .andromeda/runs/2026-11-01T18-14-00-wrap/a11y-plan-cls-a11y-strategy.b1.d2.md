# Classified — a11y-plan · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · stale · the document.rs hunks (290 lines from line 45 on, a merge surface) move the citation at packages/blitz-dom/src/document.rs:213-221
a11y-strategy#1.2 · yes · stale · the document.rs and settle.rs hunks move the citations at document.rs:393-396, 892-918, 1507-1520 and settle.rs:135-197; the changed-set rule itself reads as before
a11y-strategy#1.3 · no · the cited TODOs sit in blitz-shell application.rs, which the session does not reach
a11y-strategy#1.4 · no · the cited forwarding sits in blitz-shell accessibility.rs and window.rs, neither reached
a11y-strategy#2.1 · no · the role mapping and accessibility_roles.rs are untouched and unreached
a11y-strategy#2.2 · no · the Role::Unknown assertion in accessibility_roles.rs is untouched and unreached
a11y-strategy#2.3 · no · the state reads in blitz-dom accessibility.rs and the accessibility_states tests are unedited and unreached
a11y-strategy#2.4 · no · the native checkbox and radio write in accessibility.rs and its pin in accessibility_states.rs are unedited, and the a11y leg holds green on the changed tree
a11y-strategy#2.5 · no · the six ARIA reads in accessibility.rs and their label and description tests are unedited and unreached
a11y-strategy#3.1 · no · the platform module picks live in accesskit_xplat, which the session does not reach
a11y-strategy#3.2 · no · the accesskit_xplat lib.rs warning is unreached
a11y-strategy#3.3 · no · the adapter set_focus and set_window_bounds stubs live in accesskit_xplat, unreached
a11y-strategy#3.4 · no · the Android Rect import lives in accesskit_xplat, unreached
a11y-strategy#3.5 · no · the Android mSurfaceView field lives in accesskit_xplat, unreached
a11y-strategy#4.1 · yes · stale · the +5 hunk at dioxus_document.rs 235-239 (a merge surface) moves the citation at packages/dioxus-native-dom/src/dioxus_document.rs:314-330
