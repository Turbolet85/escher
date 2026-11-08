@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the tree's change set and refresh, and the DioxusDocument stable ids reaching the platform tree through accessibility_tree, are the tree refresh item the read must not drain or disturb
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: the has_changes() drain-first rule and what marks the changed set bind the read, which runs no step and must neither drain nor depend on the set
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the unhandled AccessibilityDeactivated and ActionRequested shell events are platform events the read neither receives nor dispatches
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarded to the platform adapter on window events, and the read builds no adapter and reads no window event
@answer n=5 entity=a11y-strategy#2.1 v=no
why: the HTML-AAM role mapping of links, lists, tables, labels and landmarks is the builder's, and the read adds no role mapping and consumes the tree's roles unchanged
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the check that only html and body map to Role::Unknown pins the builder's role mapping, which the read adds no role mapping to and does not change
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state mapping to selected, toggled and invalid, and the option's chosen state, are not read by the list, which lists verbs and refusal causes only
@answer n=8 entity=a11y-strategy#2.4 v=no
why: a native checkbox's or radio's toggled state from checkbox_input_checked is not read by the list, which lists verbs and refusal causes only
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: the snapshot's enabled reading the disabled attribute, with aria-disabled moving no snapshot reading, is the keying the read's disabled cause must follow
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module's choice of windows, macos, unix, android or null adapter, and the read builds and reads no platform adapter
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat upstream note on its approach, and the chunk builds and reads no platform adapter
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty or no-op set_focus and set_window_bounds methods of the platform adapters, and the chunk builds and reads no platform adapter
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and set_window_bounds parameter, and the chunk builds and reads no platform adapter
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field typing, and the chunk builds and reads no platform adapter
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gates the snapshot, the unkeyed_actionable check and the graph reader that the chunk adds to and tests, so its code and stand proof sit under that feature
