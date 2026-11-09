@answer n=1 entity=screen-reader-support#11.6 v=no
why: the servo fixture's missing alt attribute lies in a file the session does not reach
@answer n=2 entity=screen-reader-support#12.1 v=no
why: IME enable and disable on focus and blur in blitz-dom node.rs is in a file the session does not reach
@answer n=3 entity=screen-reader-support#12.2 v=yes mode=stale
why: the session edits dioxus_document.rs, which holds the cited input_focused_on_mount_enables_ime test at lines 786-820, so that line citation may have moved
@answer n=4 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and delete-surrounding handling in blitz-dom text.rs is in a file the session does not reach
@answer n=5 entity=screen-reader-support#13.1 v=no
why: the session adds no aria, role, accesskit or screen-reader mapping, its new UI graph reader stating no second role or name mapping
