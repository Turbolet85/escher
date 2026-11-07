@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace accesskit dependency and member are unchanged by the chunk, and Cargo.toml is not among its reached files
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: the accesskit_xplat crate's existence is a fact the chunk does not act on, since no adapter is built or moved
@answer n=3 entity=a11y-scope-summary#1.3 v=yes
why: the tree behind the Document::accessibility_tree override and the focus fallback of item 10 run through blitz-dom's build_accessibility_tree from the DOM
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list overlay is not in the chunk's render path, and its keys and paint are out of scope
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the commented-out custom widget tree hook is not reached by the Dioxus buffered render
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: blitz-html's feature forwarding is unchanged and the chunk does not reach blitz-html
@answer n=7 entity=a11y-scope-summary#1.7 v=yes
why: the windowed dioxus-native launch's tree refresh to the platform adapter, which item 14 names, is carried by the View's AccessibilityState this entity states
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: blitz's default accessibility feature is a manifest fact the chunk does not change, and the windowed path reaches it through dioxus-native
@answer n=9 entity=a11y-scope-summary#1.9 v=yes
why: the AccessKit author_id is the integer form that NodeId::as_u64 gives, so the replay must leave each element's id as the direct write gave it (item 5)
@answer n=10 entity=a11y-scope-summary#1.10 v=yes
why: the accessibility_tree override, snapshot model and diff the chunk edits build only under dioxus-native-dom's accessibility feature, which the stand binaries and blitz-tests enable
@answer n=11 entity=a11y-scope-summary#1.11 v=yes
why: Cells and the load page are stand tasks under item 7's unkeyed_actionable and author-key rules, and the stand_keyboard and stand_cells_served checks the chunk re-asserts read them
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser's default feature set is a manifest fact the chunk does not change, and the scope does not name the browser
@answer n=13 entity=a11y-scope-summary#1.13 v=yes
why: the ported test and the inverted probe pins of items 1, 2 and 4 read the accessibility tree in blitz-tests, which builds with the accessibility feature and accesskit as a dev-dependency
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the Tab-order fixture is a static HTML asset outside the chunk's reached files, and the chunk's Tab precondition is on the probe's own key target
@answer n=15 entity=a11y-scope-summary#1.15 v=yes
why: the capability statement publishes the focus fixup's no-blur named limit, which the replayed flush of item 9 must keep, so the reach it states must stay true
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the absence of ARIA and tabindex in the Rust examples is an audit of files the chunk does not reach
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the absence of accessibility code across the s06 files is a finding over files the chunk does not reach
@answer n=18 entity=a11y-scope-summary#2.3 v=no
why: the absence of accessibility handling in the s07 slice is a finding over files the chunk does not reach
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the quarantined wpt focus-events path belongs to the wpt runner, which the chunk does not touch
