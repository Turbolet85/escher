@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the poll takes the changed set and rebuilds through Document::accessibility_tree, the path needs item 4 keeps on DioxusDocument's stable-id override, headless on the stand in both layout modes.
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: changed_nodes marking of in-document mutations and focus changes binds through needs item 3 (the replayed buffer and render-time set_focus must still mark it); its drain-first clause is standing S3 and is not carried on its own.
@answer n=3 entity=a11y-strategy#1.3 v=no
why: AccessibilityDeactivated and ActionRequested are blitz-shell event TODOs that the chunk neither touches nor needs.
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding is blitz-shell window-event plumbing, not the element focus or tree refresh the chunk reaches.
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role derivation from tag names is the builder's own code, left untouched by the chunk and named in the scope's not_needed item 2.
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the html and body Unknown role test is role derivation in the builder, untouched by the chunk and named in the scope's not_needed item 2.
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state attribute reads are the builder's own code, untouched by the chunk and named in the scope's not_needed item 2.
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox toggled state comes from the engine's checked state, which the buffered writes do not carry, and no scope item names it.
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: its clause that the snapshot's enabled reads the disabled attribute binds through needs item 8 (falsy boolean attribute routing) and needs item 6 (snapshot reads unchanged); the aria-label, description and labelledby reads are the scope's not_needed item 3.
@answer n=10 entity=a11y-strategy#3.1 v=no
why: accesskit_xplat platform-module selection, dropped by the scope's not_needed item 7 since no adapter is built or moved.
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's upstream-approach warning, dropped by the scope's not_needed item 7 since no adapter is built or moved.
@answer n=12 entity=a11y-strategy#3.3 v=no
why: adapter-level set_focus and set_window_bounds no-ops lie outside the chunk; needs item 14 reads only the tree refresh, and not_needed item 7 drops the adapter.
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import is accesskit_xplat internals, dropped by the scope's not_needed item 7.
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field type is accesskit_xplat internals, dropped by the scope's not_needed item 7.
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: lib.rs's module list is edited, and the feature gates the accessibility_tree override, snapshot, diff and unkeyed_actionable that needs items 4, 6 and 7 keep, so the new buffer module's gating must respect it.
