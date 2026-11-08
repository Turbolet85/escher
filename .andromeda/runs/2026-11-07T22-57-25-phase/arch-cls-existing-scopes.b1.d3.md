@answer n=1 entity=existing-scopes#1.1 v=no
why: the graph keys on the accesskit TreeUpdate node; the platform adapter that consumes it is neither used nor changed by the chunk
@answer n=2 entity=existing-scopes#1.2 v=no
why: the debug timer is not read by the graph, the stand or the driver
@answer n=3 entity=existing-scopes#1.3 v=no
why: a module index; the never-a-DOM-NodeId key rule lives in task item 4, not in this listing
@answer n=4 entity=existing-scopes#1.4 v=no
why: the blitz-dom module index states no rule the chunk's code or tests must honour; its accessibility module is judged at #1.8
@answer n=5 entity=existing-scopes#1.5 v=no
why: CSSOM and getComputedStyle are not on the graph's read path, which walks the vdom
@answer n=6 entity=existing-scopes#1.6 v=no
why: query selectors, form owner reset and iframe loading are not on the graph's read path
@answer n=7 entity=existing-scopes#1.7 v=no
why: hit testing, hover and focus are not named by the scope; the click reading is handler presence, not hit dispatch
@answer n=8 entity=existing-scopes#1.8 v=yes
why: the graph keys on the agent-visible nodes this accessibility tree building emits, and task item 4 names build_accessibility_tree
@answer n=9 entity=existing-scopes#1.9 v=no
why: the event driver's dispatch is neither called nor changed; the graph reads handlers from dynamic attributes (task item 6), not from dispatch
@answer n=10 entity=existing-scopes#1.10 v=no
why: scrolling and text selection are not read by the graph and the scope names neither
@answer n=11 entity=existing-scopes#1.11 v=no
why: Stylo styling is not read by the graph, which changes no style
@answer n=12 entity=existing-scopes#1.12 v=no
why: the graph walks the vdom, not blitz-dom's versioned node storage, and the scope names no read of it
@answer n=13 entity=existing-scopes#1.13 v=no
why: layout and box construction are unchanged, and the graph reads already-built nodes (task not_needed item 6)
@answer n=14 entity=existing-scopes#1.14 v=no
why: the node attributes, element data and select model are not named by the scope; link 2 reads handlers from dynamic attributes instead
@answer n=15 entity=existing-scopes#1.15 v=no
why: HTML parsing is not on the stand's path, since the Dioxus app builds its tree through mutations
@answer n=16 entity=existing-scopes#1.16 v=no
why: the graph makes no fetch (task not_needed item 3)
@answer n=17 entity=existing-scopes#1.17 v=no
why: the graph is a read and paints nothing (task not_needed item 4)
@answer n=18 entity=existing-scopes#1.18 v=no
why: the stand is a Dioxus app and no script is involved (task not_needed item 5)
@answer n=19 entity=existing-scopes#1.19 v=no
why: blitz-shell's window accessibility and event loop are not used by the headless stand
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate only re-exports the other crates and holds no rule over the chunk
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy's style-to-layout conversion is unchanged and not read by the graph
@answer n=22 entity=existing-scopes#1.22 v=no
why: dioxus-native's renderer and hooks are not the surface the registry or graph reader builds on
@answer n=23 entity=existing-scopes#1.23 v=yes
why: the chunk edits element_id, the actionable check, and the snapshot, snapshot_diff and snapshot_text modules, and adds the registry in this crate (task items 1, 2, 3, 5)
@answer n=24 entity=existing-scopes#1.24 v=no
why: the stand's checks drive the harness, but the listing states no rule the graph or its checks must honour
@answer n=25 entity=existing-scopes#1.25 v=yes
why: the command-span diff counts and any new span or log pass the target allowlist and field scrub, which keep node, name and value fields out of print (task item 12)
