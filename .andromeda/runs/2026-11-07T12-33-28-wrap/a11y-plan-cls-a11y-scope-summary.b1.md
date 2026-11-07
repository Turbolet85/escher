@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the session edits dioxus-native-dom sources and one blitz-tests probe, no workspace manifest, so the accesskit dependency and member lines hold
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: accesskit_xplat is not among the reached files and the session does no adapter work
@answer n=3 entity=a11y-scope-summary#1.3 v=no
why: blitz-dom's accessibility tree build is not among the reached files and its feature wiring is unchanged
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list overlay and its checks are not reached by a change to the Dioxus bridge and the probe
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom widget hook in blitz-dom is not reached by the session
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's feature forwarding sits in a manifest the session does not touch
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: blitz-shell and its AccessibilityState adapter wiring are not reached by the session
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: blitz's accessibility feature forwarding sits in a manifest the session does not touch
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: NodeId in blitz-traits is not reached by the session
@answer n=10 entity=a11y-scope-summary#1.10 v=no
why: the session adds a crate-private module to dioxus-native-dom but changes no feature or manifest line, so the feature wiring still holds
@answer n=11 entity=a11y-scope-summary#1.11 v=no
why: the Cells and load-page stand files, their Tab orders and their keyboard and contrast rows are not reached
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app's manifest and default feature set are not touched
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: blitz-tests' manifest, its features and its accesskit dev-dependency are unchanged, the probe edit touching only a test file
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the input fixture is not among the reached files
@answer n=15 entity=a11y-scope-summary#1.15 v=no
why: the reach statement in docs/capabilities.md is not reached, and the session's limit is recorded as a residual row rather than as a change of reach
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the session's reached files are dioxus-native-dom sources and a blitz-tests probe, none of them an examples/*.rs file
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the session adds no accesskit, role, aria or screen-reader term to any reached file, only bridge code and focus pins
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: the session's reached files are not among the eight s07 slice files the accessibility search covered
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the session's reached files are not among the twelve s13 files, whose only focus hits are the quarantine reason and a wpt test path
