@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the graph reads the tree nodes built through Document::accessibility_tree, and DioxusDocument's stable-id override is the build path item 11 names
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed-set refresh trigger is untouched; the graph reads the built tree, not has_changes() or the set, and no check of the chunk reads it
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs in blitz-shell are outside the graph's read and outside the scope
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and outer or inner window bounds forwarded to the adapter are not node bounds, the only bounds item 10 names
@answer n=5 entity=a11y-strategy#2.1 v=yes
why: the HTML-AAM role mapping is the role derivation that item 5 names and item 1 forbids a second copy of, so the graph's role must match it
@answer n=6 entity=a11y-strategy#2.2 v=yes
why: the semantic-page Role::Unknown assertion over html and body is the role derivation item 5 changes for a clicked element with no role
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state attribute mapping is listed as not needed in the task's not_needed item 4, and the chunk changes no state write
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox or radio Toggled write is a state the scope does not name, and the graph neither masks nor reads it
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's label, description, expanded, disabled and level reads are listed as not needed in not_needed item 4, and the graph reads names off the built node
@answer n=10 entity=a11y-strategy#3.1 v=no
why: platform adapter module selection is the platform adapter the task's not_needed item 5 leaves out and no chunk file reaches it
@answer n=11 entity=a11y-strategy#3.2 v=no
why: an accesskit_xplat design warning in a platform module the chunk does not reach
@answer n=12 entity=a11y-strategy#3.3 v=no
why: Android set_focus and set_window_bounds and the Windows and macOS no-ops are platform adapter behaviour the chunk does not reach
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android Rect import is platform adapter code the chunk does not reach
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android mSurfaceView field read is platform adapter code the chunk does not reach
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gate covers the accessibility_tree override, the snapshot model, its text form, parts, diff and unkeyed_actionable, all of which the chunk changes
