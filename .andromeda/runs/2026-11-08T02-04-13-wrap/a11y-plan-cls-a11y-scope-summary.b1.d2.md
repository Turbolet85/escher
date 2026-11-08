@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the workspace manifest and accesskit dependency are not in the session's reach, and no dependency moved.
@answer n=2 entity=a11y-scope-summary#1.2 v=no
why: the accesskit_xplat package manifest is not in the session's reach and it joins no graph.
@answer n=3 entity=a11y-scope-summary#1.3 v=no
why: blitz-dom's accessibility tree is not among the session's reached files.
@answer n=4 entity=a11y-scope-summary#1.4 v=no
why: the select list lives in blitz-dom, which the session does not reach.
@answer n=5 entity=a11y-scope-summary#1.5 v=no
why: the custom-widget hook lives in blitz-dom, which the session does not reach.
@answer n=6 entity=a11y-scope-summary#1.6 v=no
why: the blitz-html manifest feature is not in the session's reach.
@answer n=7 entity=a11y-scope-summary#1.7 v=no
why: blitz-shell and its adapter state are not in the session's reach.
@answer n=8 entity=a11y-scope-summary#1.8 v=no
why: the blitz manifest feature is not in the session's reach.
@answer n=9 entity=a11y-scope-summary#1.9 v=no
why: blitz-traits node_id.rs is not in the session's reach; the reserved ///{key} form lives in dioxus-native-dom and escher-driver.
@answer n=10 entity=a11y-scope-summary#1.10 v=yes mode=stale
why: escher-driver's executor now reads the graph through the same accessibility edge, and the stand binaries now compile graph.rs, which this entity's compiled-module list omits.
@answer n=11 entity=a11y-scope-summary#1.11 v=yes mode=stale
why: it cites stand_cells_served.rs:94-136 and common/mod.rs:93-98, both reached whole, and the Cells served screen's snapshot moved from 5 to 6 parts.
@answer n=12 entity=a11y-scope-summary#1.12 v=no
why: the browser app's manifest feature is not in the session's reach.
@answer n=13 entity=a11y-scope-summary#1.13 v=no
why: the blitz-tests manifest is not in the session's reach, and the new stand_graph file uses features already enabled.
@answer n=14 entity=a11y-scope-summary#1.14 v=no
why: the input fixture is not in the session's reach.
@answer n=15 entity=a11y-scope-summary#1.15 v=yes mode=stale
why: docs/capabilities.md is a reached file, and the session removed its not-built line for a snapshot field saying an element reacts to a click, which this entity's accessibility reach section describes.
@answer n=16 entity=a11y-scope-summary#2.1 v=no
why: the 18 searched files are top-level examples/*.rs files, and none of them is in the session's reach.
@answer n=17 entity=a11y-scope-summary#2.2 v=no
why: the session adds no aria-, accesskit, role or screen-reader text, since the graph answer carries ids, component names, state names, event names, flag words and booleans only.
@answer n=18 entity=a11y-scope-summary#2.3 v=yes mode=stale
why: the 8 searched files are the seven_guis task files, all reached, and the session wraps each task's state as use_graph_signal, accessibility-graph wiring the absence claim did not contain.
@answer n=19 entity=a11y-scope-summary#2.4 v=no
why: the 12 searched files are wpt runner files, none of which the session reaches.
