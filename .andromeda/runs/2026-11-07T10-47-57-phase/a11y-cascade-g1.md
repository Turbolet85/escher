@carried n=1 label=a11y-plan/a11y-strategy fact=fct-p7eqwwyx
why: every DioxusDocument render must go through the buffer and still leave the tree refreshed through the Document::accessibility_tree trait method, so DioxusDocument's stable-id override reaches the tree on InitialTreeRequested and on change.
quote: The accessibility tree is built from the document on InitialTreeRequested and refreshed on change: on a poll that reported work the shell's `View::poll` takes the document's changed set (`BaseDocument::take_changed_nodes` — outside the `accessibility` cfg, so a shell built without the feature still drains it) and, under `accessibility`, rebuilds the platform tree when the set it took was non-empty; both paths go through the `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches the platform tree
@carried n=1 run=2 label=a11y-plan fact=fct-zyrpeqlm
quote: The accessibility tree is built from the document on InitialTreeRequested and refreshed on change: a poll that takes a non-empty changed set rebuilds the platform tree, both paths through the `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches it; proven headlessly on the stand in both layout modes
@carried n=2 label=a11y-plan/a11y-strategy fact=fct-3hktsq24
why: the replayed buffer's in-document mutations and a set_focus issued from a task polled during render must still mark the changed set, so a poll after a buffered render takes a non-empty set and rebuilds the tree.
quote: `changed_nodes` is the engine's changed set, the nodes written since the last `take_changed_nodes`: a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing mark it
@carried n=2 run=2 label=a11y-plan fact=fct-xhpyuh5z
quote: `changed_nodes` is the engine's changed set, the nodes written since the last `take_changed_nodes`: an in-document mutation, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing mark it
@carried n=3 label=a11y-plan/a11y-scope-summary fact=fct-r6zswm2d
why: the chunk edits dioxus-native-dom and the windowed launch renders through the buffer, so both stand binaries must keep compiling its accessibility modules with no platform adapter joining either graph.
quote: both its binaries, the windowed `seven_guis_native` and the `escher-session` host, now compile dioxus-native-dom's accessibility modules (the `accessibility_tree` override that carries `author_id`, the snapshot model, its text and its diff); neither builds a platform adapter
@carried n=3 run=2 label=a11y-plan fact=fct-kbjv4usk
quote: no stand binary builds a platform adapter, so no assistive technology is given a tree by the stand binary
@carried n=4 label=a11y-plan/a11y-strategy fact=fct-d5bs73pt
why: lib.rs's module list gains the buffer module, which must sit outside the accessibility gate while the feature's five gated items and the crate doc naming them stay as they are.
quote: dioxus-native-dom's feature also gates its own `accessibility_tree` override, its snapshot model (`DioxusDocument::snapshot`), that model's text form (`Snapshot::to_text`) and its parts (`Snapshot::text_parts`), its diff (`Snapshot::diff`) and its actionable-key check (`DioxusDocument::unkeyed_actionable`), all five of which its crate doc names
@carried n=5 label=a11y-plan/a11y-scope-summary fact=fct-lxyhqmtz
why: the served pages' Tab walks and keyed reads are among the stand_* checks the chunk re-asserts, so the replay must leave every author key and the stated Tab sequences unchanged.
quote: each holds a stated Tab sequence (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) that `stand_keyboard` walks, with keyboard rows of its own (the load page's `load-fetch` taking Enter and Space, Cells' arrows and Enter on `cells-grid`) and contrast rows in `stand_contrast`, and their own checks read them keyed, `unkeyed_actionable` empty, the load page's lint 0
@carried n=6 label=a11y-plan fact=fct-6aje67jv
why: the replay must still flush so the inert focus fixup runs, and a set_focus issued during render, before the buffer is applied, must never leave the tree's focus on an unbuilt node.
quote: a node made inert losing focus and taking no key, and the tree's focus falling back to the window where the focused node was not built
@carried n=7 label=a11y-plan fact=fct-zvvhy2hs
why: the inverted pins' precondition, the key target focused by Tab, must be driven through the harness's press and read back through its focused-node query before the spawned task's set_focus.
quote: a Tab and Shift+Tab key-press focus check exists on the stand, driven through the test harness's `press` / `press_with` (tests/blitz-tests/tests/stand_snapshot_state.rs:331-367)
@carried n=7 run=2 label=a11y-plan fact=fct-mmbkt4gl
quote: The harness exposes focused and hovered node queries (packages/blitz-test-harness/src/inspect.rs:107-115)
@binding n=1
**a11y-strategy tree refresh ↔ a11y-strategy changed set**: the buffered render must mark the changed set (replayed mutations, render-time focus) for the poll-time refresh through the trait method to rebuild the tree with DioxusDocument's stable ids (a11y-plan/a11y-strategy fct-3hktsq24) (a11y-plan/a11y-strategy fct-p7eqwwyx).
@binding n=2
**a11y-strategy feature gates ↔ a11y-scope-summary Dioxus crates**: the new buffer module must be ungated by the accessibility feature, while the gated override, snapshot and diff keep reaching both stand binaries through escher-driver with no adapter (a11y-plan/a11y-strategy fct-d5bs73pt) (a11y-plan/a11y-scope-summary fct-r6zswm2d).
@binding n=3
**the harness's focus fallback ↔ a11y-strategy changed set**: a focus move from a task polled during render must both mark the set and land the tree's focus on a built node or the window, never on a node the unapplied buffer has not built (a11y-plan fct-6aje67jv) (a11y-plan/a11y-strategy fct-3hktsq24).
@binding n=4
**a11y-scope-summary served pages ↔ the keyboard test harness**: the re-asserted Tab walks and the inverted pins' Tab precondition both drive press and read the focused node back, so the replay must keep author keys and focus order as the direct write gave them (a11y-plan/a11y-scope-summary fct-lxyhqmtz) (a11y-plan fct-zvvhy2hs).
