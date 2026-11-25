@task_prose part=session
The session added virtual nodes declared: a canvas widget may now declare keyed, named, role-bearing regions inside its content box, and the Dioxus document reads them in process.
Each declaration carries a key, an AccessKit role taken as AccessKit's own Role type with no second mapping, a name and a region; the read validates them in declaration order and gives each taken one the reserved id {owner}///{key}.
Refusals are closed and per declaration: empty key, key holding a slash, a later duplicate key (the first kept), empty name, region outside the content box; a stale or non-canvas node reads no canvas, an unlaid canvas reads not laid out; no panic, no log site.
Nothing crosses: no declared node reaches the accessibility tree, the snapshot, the snapshot diff, the UI graph or any carrier, and a declaration change writes no changed-set mark and signals no reader.
The reserved id form moves from produced by nothing to produced in process by the declared-node read, still written by no walk and served by no carrier.
Every answer resting on this chunk is a delegate's and PROVISIONAL; the fork of whether a virtual node's name and key ride the accessible-name and stable-id exits stays the founder's, carried to the entry Virtual nodes as elements.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited — the engine's custom widget trait (the canvas widget that paints pixels), behind the custom-widget feature; gains a defaulted method declaring virtual nodes and the declaration type
@task_file n=2 path=packages/blitz-dom/src/node/mod.rs
what: edited — the engine's node module re-exports, the widget trait's public names
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: edited — the engine crate's public surface and its re-exports
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: edited — the Dioxus document crate root, its modules and their accessibility feature gating, and its public re-export list
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
what: edited — the stable element id and the reserved virtual-node id form with its separator ///, the doc stating who produces it
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: added — the Dioxus document's in-process read of a canvas's declared virtual nodes, their validation and closed refusals, outside the accessibility feature
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: added — the integration check that a padded canvas reads its declared nodes in its content box and that a declaring canvas puts nothing on the snapshot or the UI graph, in both layout modes
@task_item n=1 part=touches
decision/snapshot-model-single-source changes (PROVISIONAL): the declared-node read is no snapshot-model reader, and a declared node's role is the accessibility tree's own role type, not a snapshot reading.
@task_item n=2 part=touches
decision/readers-of-one-snapshot-model changes: the canvas read beside the snapshot model now has a sibling, the in-process declared-node read, which feeds no reader of the snapshot model.
@task_item n=3 part=touches
contract/node-id/reserved-separator: the separator /// is now produced in process by the declared-node read for a canvas's taken nodes; no served id holds it yet, so its no-served-id claim stays true but its produced-by-nothing claim moves.
@task_item n=4 part=touches
decision/changed-set-marks: a virtual-node declaration change marks nothing in the changed set; the row stays true but is owed an amendment, carried to Virtual nodes as elements.
@task_item n=5 part=touches
decision/a11y-tree-refresh-on-change: a declaration change signals no reader and refreshes no tree in this chunk; the invalidation half is left to the first reader that refreshes.
@task_item n=6 part=touches
decision/accessibility-feature-gate and decision/agent-readers-behind-accessibility: the new declared-node read stands outside the accessibility feature, ungated, because it is no snapshot reader and names no AccessKit item at run time beyond the role type.
@task_item n=7 part=touches
decision/accesskit-accessibility-tree: a declared virtual node's role is AccessKit's Role type itself, carried on the engine's widget trait.
@task_item n=8 part=touches
decision/role-mapping-html-aam, decision/html-aam-role-mapping and decision/role-attribute-then-element-then-unknown: a declared node's role takes no HTML-AAM or role-attribute mapping; the app states the AccessKit role directly (PROVISIONAL).
@task_item n=9 part=touches
decision/interactive-nodes-named: a declared virtual node with an empty name is refused, so every taken declared node carries a name.
@task_item n=10 part=touches
decision/circle-drawer-canvas-pixels-only: a canvas may now declare named regions inside itself, yet the canvas still reaches the tree, snapshot and graph as one named node and its regions as pixels only; Circle Drawer declares none.
@task_item n=11 part=touches
decision/snapshot-reads-accessibility-tree, decision/snapshot-reads-the-tree and decision/ui-graph-one-node-per-snapshot-node: a declaring canvas adds no node to the snapshot or the UI graph, checked in both layout modes.
@task_item n=12 part=touches
decision/keyed-wire-form: a declared node's key is a separate key, read in process only; it rides no keyed field on the carriers.
@task_item n=13 part=touches
decision/accessibility-tree-through-document-method and decision/adapter-tree-is-document-tree: the widget trait's upstream accessibility-tree hook stays unwritten; declared nodes are not added to the document's tree or the adapter's.
@task_item n=14 part=touches
contract/accessibility-tree/author-id: the stable author id of a canvas is the owner half of each declared node's {owner}///{key} id.
@task_fact n=1 slug=screen-reader-support
fact: A canvas widget may declare virtual nodes, each a key, an AccessKit role, a name and a region in its content box; the Dioxus document reads them in process, validated in declaration order, each taken one given the id {owner}///{key} (PROVISIONAL, a delegate's answer).
@task_fact n=2 slug=screen-reader-support
fact: Absence: no declared virtual node reaches the accessibility tree, the platform adapter, the snapshot, the snapshot diff or the UI graph, and none can be clicked; whether its name and key ride the accessible-name and stable-id exits is the founder's open fork, carried to Virtual nodes as elements.
@task_fact n=3 slug=screen-reader-support
fact: A declared virtual node's role is AccessKit's Role itself, stated by the app, with no second role mapping (PROVISIONAL, a delegate's answer).
@task_fact n=4 slug=screen-reader-support
fact: A declaration is refused with a closed reason and its index alone when its key is empty or holds a slash, its key repeats an earlier one (the first kept), its name is empty, or its region leaves the canvas content box; a stale or non-canvas node reads no canvas and an unlaid canvas reads not laid out, with no panic.
@task_fact n=5 slug=a11y-strategy
fact: A virtual-node declaration change writes no changed-set mark and signals no reader in this chunk; the mark and the invalidation are owed to the first reader that refreshes (PROVISIONAL, a delegate's answer).
@task_item n=1 part=untouched
Keyboard navigation: Tab order, focusability, keyboard activation, range inputs, select lists and the seven_guis CRUD and Cells keys are unchanged; declared nodes take no focus and no key.
@task_item n=2 part=untouched
ARIA state and property reads, accessible name precedence and the label rules: no ARIA attribute is read differently and no element's name changes.
@task_item n=3 part=untouched
Visual design and contrast: the lint, its contrast finding, stand contrast and colour constants are untouched.
@task_item n=4 part=untouched
Hidden and inert content rules and hit-testing: no node is newly hidden or excluded and no hit rule changes.
@task_item n=5 part=untouched
Driver actions and refusals (covered, disabled), the actions answer, handler coverage and the screenshot: no verb gains a reading or a refusal.
@task_item n=6 part=untouched
CI integration and the a11y test leg: no leg, gate or repeat list changed; the stand three-round repeat list is untouched.
@task_item n=7 part=untouched
Cognitive accessibility, password masking and the platform adapter's absence on the stand binaries: untouched.
