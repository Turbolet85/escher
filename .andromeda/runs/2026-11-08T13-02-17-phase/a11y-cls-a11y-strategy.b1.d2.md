@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the poll's take of the changed set and the refresh it drives are the change-set rule the read must not drain or depend on (task item 14), and the stable-id override reaches the platform tree (item 4)
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: states what marks changed_nodes and that a check reading has_changes() must drain first, the exact rule of task item 14
@answer n=3 entity=a11y-strategy#1.3 v=no
why: unhandled AccessibilityDeactivated and ActionRequested events in the shell; the read dispatches nothing and no adapter is read (not-needed item 4)
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarded to the platform adapter; no adapter is built or read by the chunk (not-needed item 4)
@answer n=5 entity=a11y-strategy#2.1 v=no
why: HTML-to-AccessKit role mapping for links, lists, tables and labels; the read adds no role mapping (not-needed item 7)
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page role inventory asserting only html and body are Role::Unknown; landmark and role mapping is not needed (not-needed item 7)
@answer n=7 entity=a11y-strategy#2.3 v=no
why: ARIA state attribute mapping to selected, toggled and invalid; the read lists verbs and refusal causes, not states (not-needed item 2)
@answer n=8 entity=a11y-strategy#2.4 v=no
why: native checkbox and radio toggled state from checked; the read lists no toggled state and reads no author attribute on it (not-needed item 2)
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: states that aria-disabled moves no snapshot reading because enabled reads the disabled attribute alone, which the read's disabled cause must follow (task items 5 and 6); its other reads are names and states not needed
@answer n=10 entity=a11y-strategy#3.1 v=no
why: platform module selection of the windows, macos, unix, android or null adapter; no adapter is built or read (not-needed item 4)
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat developer warning about its approach; no adapter is built or read (not-needed item 4)
@answer n=12 entity=a11y-strategy#3.3 v=no
why: Android and Windows set_focus and macOS set_window_bounds no-ops in the platform adapter; no adapter is read (not-needed item 4)
@answer n=13 entity=a11y-strategy#3.4 v=no
why: Android adapter Rect import and set_window_bounds parameters; the platform adapter is not reached (not-needed item 4)
@answer n=14 entity=a11y-strategy#3.5 v=no
why: Android adapter reading the mSurfaceView field; the platform adapter is not reached (not-needed item 4)
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: names the accessibility feature gating the snapshot, its text form, parts, diff, the actionable-key check and the graph reader, which the new read in graph.rs, snapshot.rs and actionable.rs must sit under
