@answer n=1 entity=a11y-strategy#1.1 v=yes mode=stale
why: the wrapper's accessibility_tree override and the Document::poll that feeds its refresh sit in dioxus_document.rs, which the buffered render reaches whole
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the engine's changed-set contract and the boot marking sit in blitz-dom and the harness, neither reached, and the replay applies the same mutations through the same writer
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs sit in blitz-shell application.rs, which the session does not reach
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding sits in blitz-shell accessibility.rs and window.rs, which the session does not reach
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role mapping sits in blitz-dom's builder and its roles test, neither reached, and the session moves no role code
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page role assertion sits in accessibility_roles.rs, which the session does not reach
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state reads sit in blitz-dom accessibility.rs, and the session leaves the builder unchanged
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio reads sit in blitz-dom accessibility.rs, and the session leaves the builder unchanged
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's six reads sit in blitz-dom accessibility.rs, and the session leaves the builder unchanged
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module selection sits in accesskit_xplat, and the session does no adapter work
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat developer warning sits in its lib.rs, and the session does no adapter work
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the adapter set_focus and set_window_bounds stubs sit in accesskit_xplat, and the session does no adapter work
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and parameters sit in accesskit_xplat, and the session does no adapter work
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android mSurfaceView field type sits in accesskit_xplat, and the session does no adapter work
@answer n=15 entity=a11y-strategy#4.1 v=yes mode=stale
why: its feature-gate citations into dioxus-native-dom lib.rs and dioxus_document.rs are in reached files, and the new mutation_buffer module and replay code can move those lines
