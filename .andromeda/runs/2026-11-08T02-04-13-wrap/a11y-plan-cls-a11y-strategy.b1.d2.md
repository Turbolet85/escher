@answer n=1 entity=a11y-strategy#1.1 v=no
why: the session leaves the tree lifecycle untouched, and its dioxus_document.rs change adds a graph read and an event dispatch, not the accessibility_tree override this entity cites
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the session leaves changed_nodes semantics untouched, and a graph read changes nothing, so the changed set and its settle behaviour hold
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs sit in blitz-shell, which the session does not reach
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding sit in blitz-shell, which the session does not reach
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role derivation and the HTML-AAM mapping are untouched, and the session's roles read as before
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page Role::Unknown assertion is untouched, and the session moves no role
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the builder's ARIA state writes are untouched, and the session's clickable is a snapshot field, not a builder state
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio toggled write is unchanged, and the session touches no native control or key path
@answer n=9 entity=a11y-strategy#2.5 v=no
why: the six upstream reads are unchanged, and stand_graph asserts only graph nodes, the reserved separator and read-only graph reads, none of the six
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module selection in accesskit_xplat is not reached by the session
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat warning comment is not reached by the session
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the Android, Windows and macOS adapter methods in accesskit_xplat are not reached by the session
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect imports in accesskit_xplat are not reached by the session
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android mSurfaceView field read in accesskit_xplat is not reached by the session
@answer n=15 entity=a11y-strategy#4.1 v=yes mode=stale
why: the accessibility feature now also gates DioxusDocument::graph, so the five gated surfaces this entity counts and its lib.rs and dioxus_document.rs citations moved with the session's changes to both files
