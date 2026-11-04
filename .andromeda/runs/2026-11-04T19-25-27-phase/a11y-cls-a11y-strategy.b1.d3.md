@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the poll refresh through Document::accessibility_tree and DioxusDocument's stable-id override is the path link (5) re-verifies headlessly
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: the changed set's semantics, its non-empty fresh-boot state and the settle that neither reads nor drains it are the link (4) witness the scope names
@answer n=3 entity=a11y-strategy#1.3 v=no
why: ActionRequested and AccessibilityDeactivated are adapter-event TODOs of the windowed shell, outside the headless poll path the probe reads
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding to the platform adapter sits outside the headless poll path, and no stand builds an adapter
@answer n=5 entity=a11y-strategy#2.1 v=no
why: the probe names no role, and role derivation and the HTML-AAM mapping are excluded by the scope
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page Role::Unknown assertion over html and body is a role-derivation check the probe does not reach
@answer n=7 entity=a11y-strategy#2.3 v=yes
why: CRUD's listbox option rows carry aria-selected, and this entity's closed-set read of that attribute is the selection state link (3) reads
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the stand's Counter and CRUD carry no native checkbox or radio, and the probe's Space path reaches only buttons and the listbox
@answer n=9 entity=a11y-strategy#2.5 v=no
why: the ARIA name and description reads are excluded by the scope, and aria-disabled, aria-expanded and aria-level are read by nothing the probe takes
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the per-platform adapter choice is the adapter creation the scope excludes, and no stand binary builds an adapter
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat design note concerns a platform adapter that no stand builds
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty or no-op set_focus and set_window_bounds bodies belong to platform adapters that no stand builds
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android Rect import concerns a platform adapter that no stand builds
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android mSurfaceView field read concerns a platform adapter that no stand builds
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gates the accessibility_tree override, the snapshot model and unkeyed_actionable, which the probe reaches through escher-driver with the feature on
