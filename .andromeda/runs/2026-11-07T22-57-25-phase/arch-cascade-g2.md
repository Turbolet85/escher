@carried n=1 label=architecture/established-decisions fact=fct-oraccejk
why: the graph reader and the «reacts to a click» snapshot field must sit behind dioxus-native-dom's `accessibility` feature, on by default, and any dependency the registry or reader takes must leave that crate's default feature list as it stands.
quote: `dioxus-native-dom`: accessibility, svg, system-fonts, text-transform-icu (packages/dioxus-native-dom/Cargo.toml:13);
@carried n=1 run=2 label=architecture fact=fct-33m5glp2
quote: each crate fixes its default feature set in its manifest
@carried n=2 label=architecture/established-decisions fact=fct-7rnkaga4
why: the graph is keyed on the agent-visible node this builder writes and the new field joins the snapshot built from it, so the reading must be taken in dioxus-native-dom from the vdom, never by a new write in the builder's file, which would be one more direct engine change for every document and a founder's fork.
quote: a third direct engine change for every document stands beside those two since 2026-10-10-snapshot-states, in the accessibility builder's own file: the builder writes AccessKit's `selected`, `toggled` and `invalid` on an element's node from `aria-selected`, `aria-pressed` and `aria-invalid`, read by value
@carried n=2 run=2 label=architecture fact=fct-gejxlyk5
quote: Those two fixes, the accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable with no upstream pull request open.
@carried n=3 label=architecture/established-decisions fact=fct-nl4nv5ny
why: link 1 maps vdom nodes to DOM nodes through the bridge's mappings, so the graph of the current screen must name only nodes that still resolve after a re-render removal, and the reserved virtual-node id form must leave `unkeyed_actionable`'s report as focusability moves it.
quote: the Tab walk (`focus_next_node` / `focus_prev_node`), autofocus and `DioxusDocument::unkeyed_actionable` move with focusability, and the recompute on a `disabled` write is unchanged;
@carried n=3 run=2 label=architecture/established-decisions fact=fct-nl4nv5ny
quote: a node the Dioxus bridge removes is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so a re-render-removed node no longer resolves
@carried n=3 run=3 label=architecture fact=fct-fe5adstg
quote: a node the Dioxus bridge removes is dropped with its descendants and no longer resolves
@carried n=4 label=architecture/established-decisions fact=fct-qnjfjj37
why: the graph is keyed on the agent-visible node and the «reacts to a click» field rides the snapshot, so an inert subtree, out of the tree and the snapshot, must carry no graph node and no field reading.
quote: and `DioxusDocument::unkeyed_actionable` skip it; `hit_inner` answers no hit at an element carrying `inert`, before any other test, so a hit passes through it; the accessibility builder's `is_hidden_from_accessibility_tree` reads true for it, so the subtree leaves the tree and the snapshot
@carried n=4 run=2 label=architecture fact=fct-ay4qkh7m
quote: `inert` is honoured by its presence on the element or an ancestor — not focusable, not hit, out of the accessibility tree and the snapshot
@binding n=1
**Established decisions (default features) ↔ cross-cutting config management**: the graph reader and the snapshot field must follow the `accessibility` gate in dioxus-native-dom's default list (architecture/established-decisions fct-oraccejk), the gate that also covers the snapshot modules, with features forwarded down the crates (architecture fct-nmnb5xux).
@binding n=2
**Established decisions (the builder's engine change) ↔ standard contracts (mutation, query and CSSOM; Dioxus DOM bridge)**: `build_accessibility_tree` builds the agent-visible node the graph keys on (architecture fct-4eadslhd), and the «reacts to a click» closed boolean must come from the bridge's vdom walk under `accessibility` (architecture fct-srb7slak), adding no direct engine change in the builder's file (architecture/established-decisions fct-7rnkaga4).
@binding n=3
**Established decisions (DOM semantics, inert) ↔ standard contracts (Dioxus DOM bridge)**: link 1 must reuse the stable-id vdom walk over mappings that a removal clears (architecture/established-decisions fct-nl4nv5ny; architecture fct-srb7slak), and the reserved virtual-node separator must leave `unkeyed_actionable`'s `/` test and its skip of inert or non-focusable elements reporting as today (architecture/established-decisions fct-qnjfjj37; architecture fct-d5zuqqfb).
