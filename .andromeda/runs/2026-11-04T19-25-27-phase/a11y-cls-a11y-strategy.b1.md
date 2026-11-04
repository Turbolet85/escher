@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the refresh on a poll that takes a non-empty changed set, through Document::accessibility_tree and DioxusDocument's stable-id override, is the path the probe re-verifies (task items 1 and 5)
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: the changed set's marking rules and the harness settle neither reading nor draining it are the witness for link (4), and the fresh-document non-empty set forces a drain-first step (task item 4)
@answer n=3 entity=a11y-strategy#1.3 v=no
why: AccessibilityDeactivated and ActionRequested handling at application.rs:84-89 is not reached; the probe drives no action request or deactivation and reads only the refresh at lines 77-83
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding to the adapter is an adapter path, and the probe is headless and builds no adapter
@answer n=5 entity=a11y-strategy#2.1 v=no
why: HTML-AAM role mapping for links, lists, tables, labels and landmarks is role derivation the task names as not needed, since the probe names no role and reads none of these element kinds
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the page-level assertion that only html and body map to Role::Unknown is a test pin of role derivation the probe does not rely on
@answer n=7 entity=a11y-strategy#2.3 v=yes
why: the CRUD listbox's option rows read aria-selected into `selected` by exact lower-case match, so the probe's selection reading for links (3) and (4) on CRUD depends on this value rule (task item 8)
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio toggled-state write concerns controls the Counter and CRUD stand does not have, and the probe reads no checked state of one
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's aria-labelledby, aria-describedby, aria-description, aria-expanded, aria-disabled and aria-level reads govern name and description computation, which the task names as not needed and the scope does not name
@answer n=10 entity=a11y-strategy#3.1 v=no
why: accesskit_xplat's per-target adapter selection sits below the tree the probe reads, and no stand binary builds a platform adapter (task item 10)
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's upstream note on its approach is a remark on the platform crate that the headless probe's code and tests do not act on
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty or no-op set_focus and set_window_bounds on the Android, Windows and macOS adapters are platform behaviour, and the probe builds no adapter and forwards no focus or bounds
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's unimported Rect and its set_window_bounds parameter types concern an Android adapter the probe neither builds nor reads
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field type concerns an Android surface the probe does not read
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: it names the accessibility feature that gates accessibility_tree, the snapshot model, its text form and parts, its diff and unkeyed_actionable, so the probe's test must enable that feature to reach the surfaces it reads (task items 1 to 3 and 10)
