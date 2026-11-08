@carried n=1 label=a11y-plan/a11y-scope-summary fact=fct-lszvr7oi
why: The graph must key each node on the agent-visible accesskit node of this tree, as the snapshot builds it, never on a DOM element.
quote: under the `accessibility` feature, `BaseDocument::build_accessibility_tree` builds an AccessKit tree from the DOM (packages/blitz-dom/src/lib.rs:86-87; packages/blitz-dom/src/accessibility.rs:10-124)
@carried n=2 label=a11y-plan/a11y-scope-summary fact=fct-gwtg4dlh
why: The graph's key must carry the accesskit node with an optional owner element `NodeId`, mapped through this integer interop and no second id scheme.
quote: `NodeId::as_u64` is described as useful for interop with integer-id APIs such as AccessKit (packages/blitz-traits/src/node_id.rs:15-22)
@carried n=3 label=a11y-plan/a11y-scope-summary fact=fct-r6zswm2d
why: The graph reader, the signal registry and the snapshot field must reach seven_guis through the feature's existing edge, and no platform adapter may join the stand's graph.
quote: `accessibility` is a default feature of both Dioxus crates; dioxus-native-dom's forwards to blitz-dom and enables its own optional `accesskit`, and dioxus-native's forwards to blitz-dom, blitz-shell and dioxus-native-dom (packages/dioxus-native-dom/Cargo.toml:13; packages/dioxus-native-dom/Cargo.toml:18; packages/dioxus-native-dom/Cargo.toml:41; packages/dioxus-native/Cargo.toml:13; packages/dioxus-native/Cargo.toml:32); the workspace takes both with `default-features = false`, so a crate gets the feature only by naming it or by depending on a crate that names it — the seven_guis stand binary names none in its own manifest and builds no platform adapter
@carried n=3 run=2 label=a11y-plan fact=fct-kbjv4usk
quote: `accessibility` is a default feature of both Dioxus crates, which the workspace takes with `default-features = false`; it reaches seven_guis through `escher-driver`, which names it unconditionally, while seven_guis' own manifest names none, and no stand binary builds a platform adapter, so no assistive technology is given a tree by the stand binary
@carried n=4 label=a11y-plan/a11y-scope-summary fact=fct-lxyhqmtz
why: Moving the stand's signals onto the registry must leave the served pages keyed, `unkeyed_actionable` empty and their stated Tab sequences walking as stated.
quote: two pages the stand serves stand beside the four lean tasks, Cells (`seven_guis::stand::boot_cells`, mounting `Task::Cells` in `TaskShell`) and the load page (`seven_guis::stand::boot_load`, its `load-fetch` Button and `load-done` Paragraph), both served by `escher-session`; neither is a `LeanTask`; each holds a stated Tab sequence (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) that `stand_keyboard` walks, with keyboard rows of its own (the load page's `load-fetch` taking Enter and Space, Cells' arrows and Enter on `cells-grid`) and contrast rows in `stand_contrast`, and their own checks read them keyed, `unkeyed_actionable` empty, the load page's lint 0
@carried n=5 label=a11y-plan/a11y-scope-summary fact=fct-fichvpza
why: A test file the plan adds to the `a11y` leg, or a change to the published reach, must be written into the capability statement by hand, its count of files moved with it.
quote: `docs/capabilities.md` («What Escher can do») publishes the accessibility reach under `## Accessibility reach`: tier 0, SC 2.1.1 · 1.4.3 · 2.4.3 and no other criterion, the `a11y` leg's fourteen files with no required status check, what an assistive technology reads unmeasured, and the named limits (the focus fixup firing no `blur`, a single select's `size` unread, `aria-activedescendant` unread); its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the reach is made in that document by hand
@carried n=5 run=2 label=a11y-plan fact=fct-r5pehooz
quote: **A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`
@carried n=6 label=a11y-plan/a11y-strategy fact=fct-p7eqwwyx
why: The graph must read the tree through the `Document::accessibility_tree` override carrying stable ids, the same nodes the snapshot and the platform tree consume.
quote: both paths go through the `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches the platform tree
@carried n=6 run=2 label=a11y-plan fact=fct-zyrpeqlm
quote: both paths through the `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches it; proven headlessly on the stand in both layout modes
@carried n=7 label=a11y-plan/a11y-strategy fact=fct-d5bs73pt
why: The new snapshot field and the graph reader must sit behind dioxus-native-dom's `accessibility` feature with the snapshot model, and the crate doc must name what that feature now gates.
quote: dioxus-native-dom's feature also gates its own `accessibility_tree` override, its snapshot model (`DioxusDocument::snapshot`), that model's text form (`Snapshot::to_text`) and its parts (`Snapshot::text_parts`), its diff (`Snapshot::diff`) and its actionable-key check (`DioxusDocument::unkeyed_actionable`), all five of which its crate doc names
@carried n=8 label=a11y-plan fact=fct-4abpljjc
why: No log site, span field or print of the graph may carry a node, a component name, a handler, a state or a value, the allowlist's redaction of `text` and `value` naming none of the graph's new content.
quote: both behind a target allowlist that, among others, prints only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` fields for `blitz*` and `accesskit_xplat` targets, redacts `text`, `value`, `html` and `attrs` for escher's own targets (`escher_*`) and drops a record from any other target whole, at every level — a closed span's every pair judged by the same rule, an outside target's span writing nothing
@binding n=1
**a11y-scope-summary ↔ a11y-strategy**: the graph reader and the «reacts to a click» field must be gated by dioxus-native-dom's `accessibility` feature with the snapshot model (a11y-plan/a11y-strategy fct-d5bs73pt), and that feature must reach seven_guis only through escher-driver's existing edge, no platform adapter joining the stand graph (a11y-plan/a11y-scope-summary fct-r6zswm2d).
@binding n=2
**a11y-strategy ↔ a11y-scope-summary**: the graph's agent-visible node must be the node the `Document::accessibility_tree` override yields (a11y-plan/a11y-strategy fct-p7eqwwyx) from the tree blitz-dom builds (a11y-plan/a11y-scope-summary fct-lszvr7oi), its owner element mapped through `NodeId::as_u64` (a11y-plan/a11y-scope-summary fct-gwtg4dlh).
@binding n=3
**a11y-scope-summary ↔ ci-integration**: a test file the plan adds to the `a11y` leg's list (a11y-plan fct-6kcxmj7u) moves the fourteen the capability statement publishes, changed there by hand (a11y-plan/a11y-scope-summary fct-fichvpza).
