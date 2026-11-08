@task_prose part=chunk
Builds the UI graph: one read that gives, for the current screen of a held instance, each node, the component that rendered it, its handlers, the state they write and the nodes that read that state (v030-05).
Link 1 and link 2 come from the dioxus-core vdom walk the stable ids already run (dynamic attributes, exact, `mounted` included); links 3 and 4 come from an escher-side signal registry in dioxus-native-dom that the stand's signals move onto.
Adds the «reacts to a click» reading as a closed boolean snapshot field, which absorbs residual res-gvelrxja and moves NODE_FIELDS (thirteen today), the schema help texts, describe() and every pin that counts them.
Keys the graph on the agent-visible accesskit node with an optional owner element, reserves a virtual-node id form that collides with none of the four current id forms, and lets a handler on an owner serve sub-targets.
Values stay masked and are never logged. The read crosses escher-driver's carriers as a new verb, outcome and MCP tool.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
@task_item n=1 part=needs
The snapshot model and its node fields: the new «reacts to a click» boolean joins NODE_FIELDS, the snapshot text form, parts, diff, MCP results and node-key help, and comes from the same snapshot model with no second role or name mapping.
@task_item n=2 part=needs
The actionable-key rule and `DioxusDocument::unkeyed_actionable`: the plan says no snapshot field reports that an element reacts to a click and that the field is owed to «The UI graph». This chunk fulfils that, and the over-inclusive listener mark (a removed listener still counted) stays in force.
@task_item n=3 part=needs
Stable element ids as AccessKit `author_id` on every element node of a Dioxus document: the graph is keyed on the agent-visible accesskit node as the snapshot is built, and link 1 reads through the same vdom walk.
@task_item n=4 part=needs
Author key and id forms: the reserved virtual-node id form must collide with none of today's author key, anchored path, component path or document path, so actionable-key ids stay stable.
@task_item n=5 part=needs
Role derivation: an element that reacts to a click and states no role reads GenericContainer. The new field must say it reacts, and the role must stay untouched.
@task_item n=6 part=needs
Password and file input masking: a value the graph carries for state or a control reads `MASKED_VALUE`, the same as on every other carrier, and typed text appears in no field.
@task_item n=7 part=needs
The obs log allowlist and redaction for escher_* and blitz* targets: no log site, span field or print may carry a node, a component name, a handler, a state or a value from the graph.
@task_item n=8 part=needs
The stand's starting reading pinned by stand_non_lean_tasks (0 on the four lean tasks, Home, Temperature Converter, Circle Drawer 1, Cells 676), in both layout modes, re-pinned for the new field.
@task_item n=9 part=needs
Snapshot diff carriers: the closed boolean sits inside the ratified snapshot and diff carriers and is no boundary widening under the 2026-10-10 rule. A new open value set on a carrier is a widening and the founder's fork.
@task_item n=10 part=needs
Node bounds in the snapshot (client rects, an open list's row bounds): the graph's agent-visible node carries its bounds the way the snapshot builds them.
@task_item n=11 part=needs
The accessibility tree build path through `Document::accessibility_tree` (DioxusDocument's override with stable ids): the graph reads the same tree nodes the snapshot consumes.
@task_item n=12 part=needs
The seven_guis stand tasks whose hook-local signals move onto the registry: their stated roles, names and Tab orders in the shared tables must stay as they are.
@task_standing n=1
convention: A new accessibility or stand test file is listed in the a11y leg's file list, and a stand test that can flake is also named in the stand-repeat leg's three-round list.
home: ci-integration
@task_standing n=2
convention: A stand check asserts in both layout modes, from stated tables with their row counts asserted, and reads roles and names only from the shared module's tables.
home: a11y-strategy
@task_standing n=3
convention: A check of tree-derived readings reads them off the built tree after resolve, the snapshot model being the only consumer, never the element.
home: screen-reader-support
@task_item n=1 part=not_needed
Colour contrast, the `stand_contrast` tables, `Harness::text_contrast` and the visual lint's contrast reading. The graph reads no colour.
@task_item n=2 part=not_needed
Keyboard handling for range inputs, selects and listboxes, the CRUD list keys and Cells' arrows and editor. The chunk changes no key behaviour.
@task_item n=3 part=not_needed
Focus order, focusability, the inert focus fixup and the disabled-ness keying. The chunk changes no focus rule.
@task_item n=4 part=not_needed
The ARIA state attribute mapping (aria-selected, aria-pressed, aria-checked, aria-invalid) and upstream's label, description, expanded, disabled and level reads.
@task_item n=5 part=not_needed
The platform adapter, its creation-before-shown rule and the windowed AT witness owned by res-llkgqygv.
@task_item n=6 part=not_needed
Hidden-content and inert exclusion rules and hit pass-through.
@task_item n=7 part=not_needed
The select list's painted colours and its motion, focus ring, colour scheme and user preferences.
@task_item n=8 part=not_needed
Language and direction, text alternatives in fixtures, IME and cognitive error recovery cues.
@task_item n=9 part=not_needed
Cells grid role and cell semantics (res-wvatzhpe) and the unread aria-activedescendant on the CRUD list.
@task_item n=10 part=not_needed
Double click timing, pointer focus on click and the range input's pointer value setting.
