@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the Document::accessibility_tree override with stable ids is the build path item 11 has the graph read the same tree nodes through
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed-set refresh and has_changes read are not moved by the chunk, which reads the snapshot and tree and checks no changed-set reading
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs are named by no chunk item
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and window bounds forwarded to the adapter are not node bounds, the only bounds item 10 carries
@answer n=5 entity=a11y-strategy#2.1 v=yes
why: item 5 changes role derivation for a clickable element with no role, so the HTML-AAM role mapping of links, lists, tables and labels is one the new fallback must not override
@answer n=6 entity=a11y-strategy#2.2 v=yes
why: the semantic page pin that only html and body read Role::Unknown is what the new role fallback must keep holding
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state attribute mapping is excluded by the task and no graph field reads these states, so the chunk must act on none of it
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio toggled write is an ARIA state read the task excludes and the chunk does not change
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: the aria-labelledby name precedence is the name mapping the graph must reuse and not duplicate under item 1, and the seven_guis names item 12 re-asserts are read through it
@answer n=10 entity=a11y-strategy#3.1 v=no
why: platform module selection of windows, macos, unix, android or null adapter is excluded by the task and the chunk touches no adapter code
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat developer warning is platform adapter code the task excludes
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the Android, Windows and macOS adapter no-ops are platform adapter code the task excludes
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter Rect import and set_window_bounds parameters are platform adapter code the task excludes
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android mSurfaceView field read is platform adapter code the task excludes
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gates the snapshot model, its text form, parts, diff and unkeyed_actionable, all of which the chunk changes
