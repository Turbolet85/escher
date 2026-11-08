@task_prose part=chunk
Builds one read-only driver read of the current screen's actionable nodes (v030-06, "What can I do here"): for each node an agent can act on, its stable id, its owner where its handler sits on another element, the verbs it takes, and per verb whether a call would be accepted now or the one refusal cause it would return (disabled, covered, off-screen).
The node model is the UI graph's agent-visible node (a GraphNode, one per snapshot node) with an optional owner element, so a select's handler serves its options and groups; nothing is keyed on a DOM NodeId.
The per-node verbs are read from the schema's verb table (VERBS, VerbSpec) with no closed list of verb names in the code; the table may need to say more, and that change, the carrier and the answer's shape are the founder's forks.
Its refusal causes come from the one shared path the executor uses (action_point: disabled, then off-screen, then covered), so they agree with a real call by construction; it runs no step, dispatches nothing and changes nothing, like snapshot, lint and graph.
If it fits, it folds in the actionable-key check (residual L12): each listed node says whether its id is an author key or a positional path; proven on the stand in both layout modes, with a minimal fixture for a covered, an off-screen and an owner-served target.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the verb table VERBS and the new read's verb entry
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: VerbSpec, the per-verb statement of name, level, help, arguments and result fields
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: action_point, the shared refusal path (disabled, off-screen, covered)
@task_file n=4 path=packages/escher-driver/src/json.rs
@task_file n=5 path=packages/escher-driver/src/cli.rs
@task_file n=6 path=packages/escher-driver/src/mcp
@task_file n=7 path=packages/escher-driver/src/recording.rs
@task_file n=8 path=packages/escher-driver/src/replay.rs
@task_file n=9 path=packages/escher-driver/src/describe.rs
@task_file n=10 path=docs/capabilities.md
@task_file n=11 path=.github/scripts/test_capability_statement.py
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=13 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=14 path=packages/dioxus-native-dom/src/actionable.rs
@task_item n=1 part=needs
The snapshot model the read consumes: role, name and focus from the accessibility tree, enabled from the presence of disabled, clickable from the vdom walk's current click listeners on an element that is no disabled form control, and each node's author_id; the read takes these from the model and adds no second role or name mapping.
@task_item n=2 part=needs
The read-only UI graph (DioxusDocument::graph): one node per snapshot node, names and closed words only; the listed node is that graph node with an optional owner, and the new read must hold to names and closed words with no second role or name mapping.
@task_item n=3 part=needs
The actionable-key rule and its check DioxusDocument::unkeyed_actionable: an actionable element is focusable, carries an interactive role or an event listener, and reads an author key; the check returns nothing on every stand task and uses no second role mapping; the listener mark over-includes an element whose listener was removed, while clickable reads current listeners.
@task_item n=4 part=needs
Stable element ids as AccessKit author_id on every element node of a Dioxus document (no TextRun, document-root or Window node one): the id the read lists per node, and the author-key versus positional-path distinction the actionable-key check would report.
@task_item n=5 part=needs
The disabled refusal: the driver refuses a click or a type as disabled where the snapshot's enabled reads Some(false), nothing dispatched; the read's disabled cause must match that rule exactly.
@task_item n=6 part=needs
Disabled-ness keyed one way, by presence of the disabled attribute on a button, input, select or textarea (so disabled="false" still counts), and a falsy Dioxus value of a bridge boolean attribute removing the attribute; the read's disabled cause follows the same keying.
@task_item n=7 part=needs
The focusability rule (tabindex, the natively focusable elements, inert and disabled form controls excluded), which decides the focusable part of the actionable set the read lists.
@task_item n=8 part=needs
Visibility and hit-testing: display none, visibility hidden and inert exclude a node and its descendants from the tree and the snapshot, a select's option excepted from display none; a hit passes through an inert element; this bounds which nodes can be listed and what covered means.
@task_item n=9 part=needs
Select as owner of sub-targets: an option of a closed single select takes no click, a click on a row of an open list chooses it, a disabled row changing nothing and a group header choosing nothing; a multiple select's rows read their own client rects; an optgroup reads Role::Group; an open list's row reads its own bounds and the highlighted row is in no snapshot; the owner-served node and its verbs and causes must agree with these.
@task_item n=10 part=needs
The stand's CRUD list: person rows are options of a listbox by role, not focusable, the list crud-list the focus; how its rows are listed (own node or served by an owner) must agree with what a click on a row does.
@task_item n=11 part=needs
Password and file input values read as the fixed MASKED_VALUE; the read carries ids, verb names and cause words only, never a node's value or typed text.
@task_item n=12 part=needs
Both layout modes: the read's stand proof and its minimal fixture (covered, off-screen and owner-served targets) are asserted in both layout modes, as every stand check of the snapshot and graph is.
@task_item n=13 part=needs
The snapshot's text form, parts, diff, MCP results and node-key help read the snapshot model and nothing else; a new carrier of the read on the socket call, CLI, MCP and a kept check reads the same model.
@task_item n=14 part=needs
The tree's change set and refresh (changed_nodes, take_changed_nodes): a read that runs no step must not drain or depend on the changed set; a check reading has_changes() drains first.
@task_standing n=1
convention: every new stand check of the snapshot, graph or tree runs in both layout modes
home: screen-reader-support
@task_standing n=2
convention: the roles and names a check expects are stated once in the shared module's tables, never a second role or name mapping
home: screen-reader-support
@task_standing n=3
convention: a check over a set of rows states one table with its row count asserted
home: ## 3. A11y Assertion Harness Contract
@task_standing n=4
convention: an accessibility test file joining the a11y leg is named in ci.yml's a11y job file list, and one run under stand-repeat is a named --test of its list
home: ci-integration
@task_standing n=5
convention: a stand check of focus or keyboard reads the engine's focus, the snapshot's focused node and the tree's focus agreeing
home: keyboard-navigation
@task_item n=1 part=not_needed
Colour contrast (Harness::text_contrast, stand_contrast's tables, the lint's contrast reading, the select list's colour constants): the read names no colour and measures no ratio.
@task_item n=2 part=not_needed
ARIA state attribute mapping (aria-selected, aria-pressed, aria-checked, aria-invalid to selected, toggled, invalid) and the merged states aria-expanded, aria-disabled, aria-level: the read lists verbs and refusal causes, not these states.
@task_item n=3 part=not_needed
Accessible name and description computation (aria-label, aria-labelledby, label for, aria-describedby, aria-description): the read lists ids, not names.
@task_item n=4 part=not_needed
The AccessKit platform adapter, accesskit_xplat, its windowed AT witness and residual res-llkgqygv: no adapter is built or read.
@task_item n=5 part=not_needed
Per-key keyboard handling of range inputs, select typeahead and list keys, Enter and Space activation: the read dispatches no key.
@task_item n=6 part=not_needed
Cells grid semantics (cells-grid, residual res-wvatzhpe) and the missing aria-activedescendant on the CRUD list: no role or cell semantics change.
@task_item n=7 part=not_needed
Landmark roles inventory and the HTML-to-AccessKit role mapping, including the strong-native-semantics TODO: the read adds no role mapping.
@task_item n=8 part=not_needed
Language and direction, text alternatives in fixtures, and input method (IME): not reached by a read of actionable nodes.
@task_item n=9 part=not_needed
Cognitive accessibility (the flight booker's invalid-date cue, orientation and status cues): no change to it.
@task_item n=10 part=not_needed
Motion, focus ring, state colour and colour-scheme preferences: the read paints nothing.
@task_item n=11 part=not_needed
Focus restoration on pointer clicks, double-click timing and range pointer setting: the read performs no click.
@task_item n=12 part=not_needed
Drag, virtual-node production, canvas and verbs with two targets: these stay in 0.4.0.
