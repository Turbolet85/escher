@answer n=1 entity=a11y-strategy#1.1 v=yes
why: the poll-time tree refresh through the Document::accessibility_tree trait method is what DioxusDocument's stable-id override reaches on InitialTreeRequested and on change (task item 4).
@answer n=2 entity=a11y-strategy#1.2 v=yes
why: the changed-set marking of in-document mutations and focus changes is what the replayed buffer and a render-time set_focus must keep, so a non-empty poll rebuilds the tree (task item 3); its drain-first sentence is the standing S3 convention, not the reason it binds.
@answer n=3 entity=a11y-strategy#1.3 v=no
why: AccessibilityDeactivated and ActionRequested TODOs in blitz-shell application.rs are not touched and not named by the scope.
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding to the adapter per window event is not named; the windowed launch (task item 14) reads only the tree refresh.
@answer n=5 entity=a11y-strategy#2.1 v=yes
why: the role mapping reads the tag names and role attributes the buffer carries and replays, and the snapshot's re-asserted role reads take the role it yields (task items 5, 6, 15).
@answer n=6 entity=a11y-strategy#2.2 v=yes
why: only html and body mapping to Unknown is a role mapping over tag names the buffer must pass unchanged, read through the snapshot's re-asserted role reads (task items 5, 6, 15).
@answer n=7 entity=a11y-strategy#2.3 v=yes
why: the aria-selected, aria-pressed, aria-checked and aria-invalid reads are attribute values the buffer must carry unchanged, and the snapshot's re-asserted states read what they yield (task items 6, 7, 15).
@answer n=8 entity=a11y-strategy#2.4 v=yes
why: the native checkbox or radio toggled read depends on the type attribute and checked state the buffer replays, and the snapshot's re-asserted states read the toggled value it writes (task items 6, 8, 15).
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: the aria-labelledby name and the aria-disabled, aria-expanded and aria-level attribute reads are values the buffer carries, and the snapshot's re-asserted name and enabled reads take the disabled attribute replayed under task item 8.
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the per-OS adapter module selection in accesskit_xplat is not reached; the chunk builds no adapter and reads only the windowed refresh (not_needed item 7).
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's warning about AccessKit's approach has no link to the buffer, focus or the tree the chunk checks.
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the Android set_focus and Windows and macOS no-ops are adapter behaviour the chunk does not run; its focus checks are headless and the windowed path is read, not run.
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and set_window_bounds parameters are platform code the chunk does not touch.
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's mSurfaceView field type is platform code the chunk does not touch.
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gates the accessibility_tree override, snapshot, to_text, text_parts, diff and unkeyed_actionable that task items 4, 6 and 7 name, and dioxus-native forwards it to the windowed launch (task item 14).
