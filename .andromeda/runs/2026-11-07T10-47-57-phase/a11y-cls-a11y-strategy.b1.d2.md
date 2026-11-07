@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the tree refresh through the Document::accessibility_tree trait method reaches DioxusDocument's stable-id override, which task needs 3 and 4 keep.
@answer n=2 entity=a11y-strategy#1.2 v=no standing=S3
why: states the has_changes() drain-first rule and the fresh-set non-empty clause, the convention S3 lists.
@answer n=3 entity=a11y-strategy#1.3 v=no
why: ActionRequested and AccessibilityDeactivated are shell event TODOs the buffered render does not touch.
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding to the adapter is window-event handling the buffered render does not reach.
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role derivation from tag names is the builder's own code, not touched by the chunk (not_needed 2); the buffer passes tag names verbatim.
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page role pin is the builder's own check; the chunk does not change role mapping.
@answer n=7 entity=a11y-strategy#2.3 v=no
why: ARIA state attribute reads are the builder's own code, not touched by the chunk (not_needed 2).
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox Toggled read is the builder's own code, a re-check every change owes, not a behaviour the chunk's scope names.
@answer n=9 entity=a11y-strategy#2.5 v=no
why: the six upstream builder reads (labelledby, describedby, description, expanded, disabled, level) are the builder's own code, untouched by the chunk.
@answer n=10 entity=a11y-strategy#3.1 v=no
why: platform adapter selection in accesskit_xplat is not built or moved by the chunk (not_needed 7).
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's upstream approach warning is platform adapter code the chunk does not touch.
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the Android and Windows set_focus and macOS set_window_bounds no-ops are platform adapter code the chunk does not touch.
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and parameters are platform adapter code the chunk does not touch.
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field read is platform adapter code the chunk does not touch.
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility gate on the accessibility_tree override, snapshot, diff and unkeyed_actionable sits in dioxus_document.rs, the file the render sites and tests edit.
