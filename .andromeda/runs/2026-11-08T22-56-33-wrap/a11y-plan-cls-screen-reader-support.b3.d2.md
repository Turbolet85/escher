@answer n=1 entity=screen-reader-support#11.6 v=no
why: the fixture examples/assets/servo_header_reduced.html is not among the files the session reached
@answer n=2 entity=screen-reader-support#12.1 v=no
why: its rule lives in packages/blitz-dom node.rs, which the session did not reach
@answer n=3 entity=screen-reader-support#12.2 v=yes mode=stale
why: the session adds record_reached and its handle_event write to dioxus_document.rs, where this entity's cited test sits at lines 786-820, so those citations can have moved
@answer n=4 entity=screen-reader-support#12.3 v=no
why: its rule lives in packages/blitz-dom text.rs, which the session did not reach
@answer n=5 entity=screen-reader-support#13.1 v=yes mode=stale
why: its absence search covers 17 s06 files, the session reaches dioxus-native-dom and escher-driver sources, and the report does not show the no-match still holds on them
