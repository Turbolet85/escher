# Classified — a11y-plan · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · stale · the poll-time refresh consumes the changed set, which the session now marks on list open, close and reconcile, and document.rs is reached whole so its cited lines may have moved
a11y-strategy#1.2 · yes · contradict · the unqualified "scroll ... do not" mark is no longer true, since a page scroll or a wheel outside now closes an open list and marks the select, and list open and close are a new marked site
a11y-strategy#1.3 · no · the AccessibilityDeactivated and ActionRequested TODOs sit in blitz-shell application.rs, which the session does not reach
a11y-strategy#1.4 · no · window focus and bounds forwarding sit in blitz-shell, which the session does not reach and no window event path changes
a11y-strategy#2.1 · no · role mapping logic is unchanged, the only accessibility.rs edit being the insert at 238-242 that writes a select's expanded, and this entity cites only accessibility_roles.rs
a11y-strategy#2.2 · no · an open list's rows are painted and add no tree node, so a semantic page still maps only html and body to Unknown and accessibility_roles.rs is unedited
a11y-strategy#2.3 · no · the ARIA state reads at accessibility.rs:164-194 and 182-236 sit above the only insert at 238-242, and the session adds no ARIA read, a select's expanded coming from engine state
a11y-strategy#2.4 · no · the native checkbox and radio reads at accessibility.rs:199-224 sit above the insert at 238-242 and are unchanged, and no Toggled is written on a select
a11y-strategy#2.5 · yes · contradict · the builder's aria-expanded read, listed among upstream's six, is no longer made on a single select, and its cited lines past 238 have moved with the insert
a11y-strategy#3.1 · no · the platform module selection lives in accesskit_xplat, which the session does not reach
a11y-strategy#3.2 · no · the accesskit_xplat developer warning lives in accesskit_xplat, which the session does not reach
a11y-strategy#3.3 · no · the Android and Windows and macOS no-op methods live in accesskit_xplat, which the session does not reach
a11y-strategy#3.4 · no · the Android Rect import and parameters live in accesskit_xplat, which the session does not reach
a11y-strategy#3.5 · no · the Android mSurfaceView field lives in accesskit_xplat, which the session does not reach
a11y-strategy#4.1 · no · the feature gates and the five gated names in dioxus-native-dom's lib.rs are unchanged, the session adds no gated item, and the crate doc is not in the session's touches
