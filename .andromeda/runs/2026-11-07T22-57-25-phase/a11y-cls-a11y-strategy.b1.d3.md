@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the platform tree build through Document::accessibility_tree with DioxusDocument's stable-id override is the tree the graph reads (task item 11)
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the engine changed-set semantics; the graph reads the tree after resolve and the chunk adds no has_changes check or changed-set reader
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the unhandled AccessibilityDeactivated and ActionRequested TODOs in blitz-shell; the chunk changes no event handling
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and outer/inner window bounds forwarded to the adapter; the chunk's bounds are node client rects, not window bounds
@answer n=5 entity=a11y-strategy#2.1 v=no
why: the HTML-AAM tag-to-role mapping of links, lists, tables, labels and landmarks; the role must stay untouched and the chunk adds no tag mapping
@answer n=6 entity=a11y-strategy#2.2 v=no
why: only html and body reading Unknown on a semantic page; the new click-reacting GenericContainer rule does not yield Unknown and the mapping is untouched
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state attribute mapping is named not_needed by task item 4, and the graph reads states only through the snapshot
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio toggle reads are outside the chunk and named not_needed by task item 4
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's label, description, expanded, disabled and level reads are named not_needed by task item 4; the graph reads names only through the existing snapshot mapping
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module selection and the null adapter; the chunk touches no platform adapter (task item 5)
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat design warning; no adapter change is made (task item 5)
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the empty set_focus and set_window_bounds and the no-op platform methods; the platform adapter is not_needed (task item 5)
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and parameters; the platform adapter is untouched (task item 5)
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field type; the platform adapter is untouched (task item 5)
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gates DioxusDocument::snapshot, Snapshot::to_text, text_parts, diff and unkeyed_actionable, which the new snapshot field and the graph sit behind
