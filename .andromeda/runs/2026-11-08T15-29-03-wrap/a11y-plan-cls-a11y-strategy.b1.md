@answer n=1 entity=a11y-strategy#1.1 v=no
why: the session reaches no blitz-shell or blitz-dom file, so the changed-set refresh path and the wrapper's accessibility_tree override it cites stand as written
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed-set semantics and the harness settle live in blitz-dom and blitz-test-harness, which the session did not reach
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs in blitz-shell are untouched by the session
@answer n=4 entity=a11y-strategy#1.4 v=no
why: the window focus and bounds forwarding in blitz-shell is untouched by the session
@answer n=5 entity=a11y-strategy#2.1 v=no
why: the HTML-AAM role mapping in blitz-dom is unchanged, and the session's Readings reads the existing 27 roles without a second mapping
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the html and body Role::Unknown rule lives in blitz-dom's role mapping, which the session did not reach
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state reads in blitz-dom's builder are unchanged, and the session reads no state attribute
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio state write in blitz-dom is unchanged by the session
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's six reads in blitz-dom's builder are unchanged, and the snapshot's enabled reading is not moved by the session
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module selection in accesskit_xplat is not among the files the session reached
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat developer note is untouched, the session reaching no accesskit_xplat file
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty set_focus and set_window_bounds adapter methods in accesskit_xplat are untouched by the session
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and parameters in accesskit_xplat are untouched by the session
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field read in accesskit_xplat is untouched by the session
@answer n=15 entity=a11y-strategy#4.1 v=yes mode=add-to
why: the session adds DioxusDocument::actionable_nodes in the actionable module beside unkeyed_actionable, in the reached dioxus-native-dom lib.rs and actionable.rs, so the crate doc's gated-item list must carry it if the feature gates it
