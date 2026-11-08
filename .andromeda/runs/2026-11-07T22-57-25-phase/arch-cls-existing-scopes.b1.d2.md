@answer n=1 entity=existing-scopes#1.1 v=no
why: the platform adapter consumes the TreeUpdate the graph reads, and the chunk changes no platform implementation
@answer n=2 entity=existing-scopes#1.2 v=no
why: the debug timer sits off the graph read path and the chunk adds no timing
@answer n=3 entity=existing-scopes#1.3 v=no
why: the graph is keyed on the agent-visible node, not on a blitz-traits DOM NodeId, and this entity states no rule over that crate
@answer n=4 entity=existing-scopes#1.4 v=no
why: blitz-dom gets no engine change, and its accessibility module is the one that binds (see existing-scopes#1.8)
@answer n=5 entity=existing-scopes#1.5 v=no
why: CSSOM and computed-style reading is off the graph read path
@answer n=6 entity=existing-scopes#1.6 v=no
why: form, iframe and query-selector code is off the graph read path, which walks the vdom
@answer n=7 entity=existing-scopes#1.7 v=no
why: hit testing, focus and hover stay as they are, and the graph reads handlers from the vdom, not from hit results
@answer n=8 entity=existing-scopes#1.8 v=yes
why: build_accessibility_tree and its agent-visible AccessKit nodes and bounds live here, and the graph is keyed on that node with an optional owner element
@answer n=9 entity=existing-scopes#1.9 v=no
why: event dispatch stays as it is, and the "reacts to a click" reading comes from vdom handlers, not from blitz-dom's pointer and keyboard modules
@answer n=10 entity=existing-scopes#1.10 v=no
why: scrolling and selection are off the graph read path
@answer n=11 entity=existing-scopes#1.11 v=no
why: Stylo styling integration is off the graph read path, and the chunk changes no style
@answer n=12 entity=existing-scopes#1.12 v=no
why: the graph walks the vdom, not blitz-dom's versioned node storage or traversal
@answer n=13 entity=existing-scopes#1.13 v=no
why: the graph reads already-built nodes and changes no layout, damage or paint tree
@answer n=14 entity=existing-scopes#1.14 v=no
why: the graph reads vdom attributes and handlers, not blitz-dom element data, select or range input state
@answer n=15 entity=existing-scopes#1.15 v=no
why: the stand is a Dioxus app, so HTML parsing into a blitz-dom document is not on the chunk's path
@answer n=16 entity=existing-scopes#1.16 v=no
why: the graph makes no fetch, as the task's not-needed item for networking states
@answer n=17 entity=existing-scopes#1.17 v=no
why: the graph paints nothing and the canvas renderer is out of scope
@answer n=18 entity=existing-scopes#1.18 v=no
why: the stand is a Dioxus app, so no script is involved
@answer n=19 entity=existing-scopes#1.19 v=no
why: the window, event loop and accessibility adapter hand-off are off the chunk's read path and unchanged
@answer n=20 entity=existing-scopes#1.20 v=no
why: a re-export of the engine crates, which the chunk does not change
@answer n=21 entity=existing-scopes#1.21 v=no
why: style-to-layout conversion is off the graph read path and the chunk changes no layout
@answer n=22 entity=existing-scopes#1.22 v=yes
why: the stand's Dioxus app reaches dioxus-native-dom through this crate, so the accessibility feature forwarding (task item 16) and the Dioxus integration pass through its manifest and hooks
@answer n=23 entity=existing-scopes#1.23 v=yes
why: element_id, the actionable check, the snapshot, snapshot_diff, snapshot_text, the mutation buffer and dioxus_document are the crate the registry, the graph reader and the snapshot field are built in
@answer n=24 entity=existing-scopes#1.24 v=no
why: the chunk only uses the harness to run the stand and changes none of its modules, and no rule the entity states binds the chunk's code
@answer n=25 entity=existing-scopes#1.25 v=yes
why: the span formatter's field scrub and its drop of outside-target records stand behind the never-logged rule for the command span's diff counts
