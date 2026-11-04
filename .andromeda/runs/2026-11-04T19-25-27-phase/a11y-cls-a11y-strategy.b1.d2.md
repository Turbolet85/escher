@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the poll takes a non-empty changed set and refreshes the platform tree through Document::accessibility_tree, the DioxusDocument override the chunk's item 5 re-verifies on the same poll path
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: changed_nodes marking, that harness settle neither reads nor drains it, and a fresh harness set being non-empty bind the probe's link (4) drain-first reading
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs are adapter events in the shell; the probe builds no adapter and handles no assistive-technology action
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding to the adapter on window events is the windowed path, which the probe reads only through the poll refresh and does not run
@answer n=5 entity=a11y-strategy#2.1 v=no
why: the HTML-AAM mapping of links, lists, tables, labels and landmarks is role derivation the probe does not reach; its roles come from the tree and the scope's not-needed item 4 names this reading
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the pin that only html and body map to Unknown is a test assertion over a semantic page, which the probe neither states nor reads
@answer n=7 entity=a11y-strategy#2.3 v=yes
why: aria-selected true or false sets the selected state on an element, and the probe reads the CRUD option rows' selection through the tree for the handler-to-state links
@answer n=8 entity=a11y-strategy#2.4 v=no
why: a native checkbox or radio toggled state is read on a native control, and the probe reads no native checkbox or radio on Counter or CRUD
@answer n=9 entity=a11y-strategy#2.5 v=no
why: aria-labelledby, aria-describedby, aria-description, aria-expanded, aria-disabled and aria-level are name, description and state reads the probe neither adds nor relies on; the scope's not-needed item 5 names the name and description reads
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module's choice of OS adapter is outside the headless probe, since no stand binary builds a platform adapter
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's developer warning about its approach concerns an adapter the chunk does not build
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty or no-op set_focus and set_window_bounds on the Android, Windows and macOS adapters concern adapters the chunk does not build
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's unimported Rect parameter concerns an adapter the chunk does not build
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field type concerns an adapter the chunk does not build
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gates the tree override, the snapshot model, its diff and unkeyed_actionable, and dioxus-native forwards it, so the probe's headless run must enable it to reach seven_guis
