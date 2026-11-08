@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the poll's changed-set drain and the accessibility_tree override carrying DioxusDocument's stable ids reach the tree the read lists ids from, and the read must not drain that set
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: the changed-set marking rules and the has_changes drain-first rule bind the read, which runs no step and must neither drain nor depend on the set
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs sit in the shell's event loop, and the read dispatches no platform action
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding to the platform adapter, which no stand binary builds and the read does not touch
@answer n=5 entity=a11y-strategy#2.1 v=no
why: the HTML-AAM role mapping of links, lists and landmarks is the builder's, and the read takes roles from the model with no mapping of its own (not_needed 7)
@answer n=6 entity=a11y-strategy#2.2 v=no
why: a test pinning only html and body to Role::Unknown over the builder's mapping, which the chunk does not change
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state writes (selected, toggled, invalid, option label) are not listed by the read, which lists verbs and refusal causes only (not_needed 2)
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio toggled state moves only on a click, and the read performs no click
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: the snapshot's enabled reads the disabled attribute and aria-disabled moves no snapshot reading, so the read's disabled cause must key on that attribute alone
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module selection in accesskit_xplat, for which no adapter is built or read (not_needed 4)
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's developer warning on its own approach, outside the read (not_needed 4)
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty set_focus and set_window_bounds in the Android, Windows and macOS adapters, which the read does not call
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and parameters, a platform adapter the read does not reach
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field read, a platform adapter the read does not reach
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility gate over the graph module and unkeyed_actionable is what the new read joins, and the crate doc must name it
