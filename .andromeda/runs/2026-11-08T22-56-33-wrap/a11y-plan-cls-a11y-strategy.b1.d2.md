@answer n=1 entity=a11y-strategy#1.1 v=no
why: the session adds an ungated record beside the accessibility override in dioxus_document.rs, which stays as it was, and the changed-set drain and poll path it cites are untouched
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the session's reached record is a DioxusDocument field that marks no node, so the changed set, the settle path and the has_changes reading it cites are unchanged
@answer n=3 entity=a11y-strategy#1.3 v=no
why: AccessibilityDeactivated and ActionRequested are blitz-shell event handlers that the session does not reach
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding in blitz-shell and accesskit are outside the files the session reaches
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role mapping in blitz-dom and accessibility_roles.rs is untouched, since the session adds no role or name mapping
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic page's Unknown-role assertion in accessibility_roles.rs is outside the files the session reaches
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state reads in blitz-dom's builder are untouched, and the session adds no ARIA mapping or snapshot state
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio toggle write in blitz-dom's builder is untouched by the session
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's six reads in blitz-dom's builder are untouched, and the new stand_handler_coverage file asserts click-handler coverage and states none of the six
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the accesskit_xplat platform module selection is outside the files the session reaches
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's upstream warning is untouched by the session
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the adapter set_focus and set_window_bounds no-ops in accesskit_xplat are untouched by the session
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and parameters in accesskit_xplat are untouched by the session
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field read in accesskit_xplat is untouched by the session
@answer n=15 entity=a11y-strategy#4.1 v=yes mode=stale
why: the session re-exports ReachedHandler from dioxus-native-dom's lib.rs and adds lines to dioxus_document.rs, so the lib.rs and dioxus_document.rs line citations in this entity move
