@answer n=1 entity=a11y-strategy#1.1 v=no
why: the refresh path lives in blitz-shell and blitz-dom, which the session does not reach, and the report keeps the accessibility tree lifecycle and every existing stable id as before
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed_nodes semantics live in blitz-dom, which the session does not reach, and the report leaves them untouched
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs sit in blitz-shell, which the session does not reach
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding sit in blitz-shell, which the session does not reach
@answer n=5 entity=a11y-strategy#2.1 v=no
why: the role mapping lives in blitz-dom and its tests in accessibility_roles.rs, neither reached, and the report keeps role derivation unchanged
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page Unknown-role assertion lives in accessibility_roles.rs, which the session does not reach, and roles read as before
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the builder's ARIA state writes live in blitz-dom accessibility.rs, which the session does not reach, and the report keeps the state mapping unchanged
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio toggled write lives in blitz-dom accessibility.rs, which the session does not reach, and the state mapping is unchanged
@answer n=9 entity=a11y-strategy#2.5 v=no
why: the six upstream reads live in blitz-dom accessibility.rs, which the session does not reach, and the snapshot's description and enabled readings hold on the changed snapshot.rs since the session adds only clickable and the disabled rule is unchanged
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module lives in accesskit_xplat, which the session does not reach, and the report leaves the platform adapter untouched
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat lib.rs note sits in a crate the session does not reach, and the adapter is untouched
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty set_focus and set_window_bounds stubs sit in accesskit_xplat, which the session does not reach, and the adapter is untouched
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android Rect import and parameters sit in accesskit_xplat, which the session does not reach, and the adapter is untouched
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android mSurfaceView field read sits in accesskit_xplat, which the session does not reach, and the adapter is untouched
@answer n=15 entity=a11y-strategy#4.1 v=yes mode=stale
why: DioxusDocument::graph is a new item behind the accessibility feature, so the feature gates six items where this entity says all five, and dioxus_document.rs and lib.rs are reached by the session
