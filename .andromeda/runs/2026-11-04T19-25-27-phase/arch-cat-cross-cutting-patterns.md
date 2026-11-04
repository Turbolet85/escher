@carried n=1 label=architecture/cross-cutting-patterns fact=fct-dsg4uyo4
why: The MountedData reading must name its mechanism against the shared-document borrow this pattern fixes, and the probe must read the bridge through that one shared document, never a second copy of it.
quote: the blitz document is shared as `Rc<RefCell<BaseDocument>>` (packages/dioxus-native-dom/src/dioxus_document.rs:86; packages/dioxus-native-dom/src/events.rs:140-144)
@carried n=2 label=architecture/cross-cutting-patterns fact=fct-klon5sda
why: Link (2)'s verdict and the "reacts to a click" reading must account for the bridge registering only event kinds that carry a handler, so a listener read names that skip as part of its API.
quote: dioxus skips event kinds with zero handlers (packages/dioxus-native-dom/src/dioxus_document.rs:346-352; packages/dioxus-native-dom/src/mutation_writer.rs:349-359)
@carried n=3 label=architecture/cross-cutting-patterns fact=fct-klon5sda
why: The hazard's measured mechanism must be stated against the point where `mounted` listeners fire, after `initial_build` and inside each `poll`, the same `poll` the scope re-verifies at HEAD.
quote: `mounted` listeners fire after `initial_build` and each `poll` (packages/dioxus-native-dom/src/mutation_writer.rs:331-336; packages/dioxus-native-dom/src/dioxus_document.rs:160-168; packages/dioxus-native-dom/src/dioxus_document.rs:232-256)
@carried n=4 label=architecture/cross-cutting-patterns fact=fct-3kximnte
why: Any probe of a `mounted` path, or a fix option brought to the founder, must keep the skip of a queued `mounted` event whose element was removed.
quote: A queued `mounted` event whose element was removed since it was queued is skipped: `flush_queued_mounted_events` reads the element through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)
@carried n=5 label=architecture/cross-cutting-patterns fact=fct-dw3hon6z
why: A link verdict of "exposed" that rests on a bridge-held reader in the snapshot or actionable modules must name the `accessibility` feature it needs and be run with that feature on.
quote: `accessibility` also to `dioxus-native-dom/accessibility`, which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules
@carried n=6 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
why: If link (4), state to the nodes that read it, is measured through the changed set, its verdict must treat that set as engine-touched and a superset of what a snapshot reads, never as the set of nodes that read a state.
quote: the set is engine-touched, a superset of what a snapshot reads as changed (an attribute re-written to itself marks and changes no reading; bounds-only movement changes a reading and marks nothing)
@carried n=6 run=2 label=architecture/cross-cutting-patterns fact=fct-ksdng7dt
quote: so a `resolve` with nothing written marks nothing in either layout mode
@binding n=1
**Shared document and mounted timing ↔ the body's engine-change records**: the founder's reading names the MountedData panic against the shared `Rc<RefCell<BaseDocument>>` and the `mounted` flush inside `poll` (architecture/cross-cutting-patterns fct-dsg4uyo4) (architecture/cross-cutting-patterns fct-klon5sda), and each fix option it brings is stated as a direct engine change for every document, the kind the body records once ratified with its merge surface (architecture fct-7in62t3p). This chunk decides none of them, and the queued-mounted skip holds under every option (architecture/cross-cutting-patterns fct-3kximnte).
@binding n=2
**Listener skip and stable ids ↔ §Standard Contracts → Dioxus DOM bridge**: the APIs that links (1) and (2) name are the bridge's: `data-dioxus-id` routing and `element_id` computed on demand and "written to neither DOM nor vdom" (architecture fct-srb7slak), read alongside the zero-handler skip (architecture/cross-cutting-patterns fct-klon5sda). The probe reads them read-only, and no log or event carries what it reads (architecture fct-tev7ieys).
@binding n=3
**Feature gating and the changed set ↔ §Conventions → Tests and §Standard Contracts → Document core**: a probe check that reads snapshot or actionable modules builds with `accessibility` (architecture/cross-cutting-patterns fct-dw3hon6z). It runs in both layout modes (architecture fct-wqndq65v), and any changed-set reading drains through `take_changed_nodes` with ids read through `get_node`, never by indexing (architecture fct-c5wyjq5i) (architecture/cross-cutting-patterns fct-ksdng7dt).
