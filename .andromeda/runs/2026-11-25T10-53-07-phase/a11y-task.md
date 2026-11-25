@task_prose part=chunk
Brings a canvas's taken virtual nodes (id {owner}///{key}) into every driver read that today holds DOM elements alone.
The UI graph gains one node per virtual snapshot node with its owner; the actions answer lists a virtual node with its verbs and refusal causes.
Handler coverage reads a virtual node's reach; the layout explanation answers for a virtual id; exploration takes a virtual node as a candidate.
Owns the carried limits of virtual nodes as elements (only click takes a virtual id, clickable reads false, graph/actions/lint/exploration hold none) and lifts or restates them.
Sweeps the literal pins with the words: counts, help texts, the self-description, the verb count, the capability statement's counts and the guards asserting no virtual id.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph, one node per snapshot node, and its exclusion of virtual nodes — edits: the graph build, the virtual-node exclusion
@task_file n=2 path=packages/escher-driver/src/actions.rs mark=edits
what: the actions answer, the actionable nodes listed with the verbs each takes and the cause each would be refused with
@task_file n=3 path=packages/escher-driver/src/lint.rs mark=edits
what: the lint's read of the document's nodes, which today holds no virtual node
@task_file n=4 path=packages/escher-driver/src/explore.rs mark=edits
what: exploration, the candidates it acts on
@task_file n=5 path=packages/escher-driver/src/coverage.rs mark=edits
what: handler coverage, the coverage role's answer of which handlers a kept check reached
@task_file n=6 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's verb execution and its target resolution — edits: resolve, locate, check_target, the arm refusing a virtual id not-found
@task_file n=7 path=packages/escher-driver/src/session.rs mark=edits
what: the session library — edits: Session::layout, the layout explanation read
@task_file n=8 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: the engine's layout explanation, layout_reason
@task_file n=9 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot model and its nodes — edits: clickable
@task_file n=10 path=packages/dioxus-native-dom/src/reached.rs mark=edits
what: the Dioxus document's reached-handler record
@task_file n=11 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description, its counts and help texts — edits: describe()
@task_file n=12 path=docs/capabilities.md mark=edits
what: the capability statement, its counts and their pin, its stated limits pinned by a check
@task_file n=13 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph checks, their guard asserting no virtual id
@task_file n=14 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actions checks, their guard asserting no virtual id
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_declared.rs mark=edits
what: the declared-virtual-node checks, their guard asserting no virtual id
@task_file n=16 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs mark=edits
what: the virtual-nodes-as-elements checks on a fixture canvas in both layout modes — edits: the_graph_actions_and_lint_hold_no_virtual_node
@task_item n=1 part=needs
The UI graph mirrors the snapshot one node per snapshot node; its carve-out excluding virtual nodes is lifted so each virtual snapshot node is a graph node carrying its owner, the graph still read-only and carrying no values.
@task_item n=2 part=needs
The actions answer: its fields and its causes for a virtual node, listing the verbs it takes and the refusal cause each would meet; the actions verb still changes nothing.
@task_item n=3 part=needs
The actionable-node listing and the one actionable rule: which virtual nodes count as actionable and by what key.
@task_item n=4 part=needs
The snapshot node's clickable reading for a virtual node (today false, a provisional delegate word), in the snapshot, the snapshot node and the snapshot text form alike.
@task_item n=5 part=needs
Handler coverage's answer: a virtual node's reach read where a kept check acts on it, through the Dioxus document's reached-handler record (reached_handlers, record_reached).
@task_item n=6 part=needs
The layout explanation answers for a virtual id rather than reading it not-found; the not-found arm for type, scroll, screenshot with an id and canvas is kept or restated per verb.
@task_item n=7 part=needs
Driver refusal causes: the ratified refusal-cause set is not extended; a new refusal cause or a new crossing class is a fork for the founder, not a plan step.
@task_item n=8 part=needs
The served node id's reserved separator /// : a virtual id {owner}///{key} read as such by every read that now carries it, and never minted for a DOM element.
@task_item n=9 part=needs
The virtual-node crossing rule over what the graph, actions answer, lint, canvas answer and exploration may hold is the rule this chunk revisits; every reader stays a reader of one snapshot model.
@task_item n=10 part=needs
Exploration takes a virtual node as a candidate it acts on, and only through the verbs a virtual id is taken by.
@task_item n=11 part=needs
The lint's finding reads (visual lint finding content) with virtual nodes now in its node set, or their exclusion restated.
@task_item n=12 part=needs
A carrier that gains a virtual id or name is read against the ratified founder word that the name and id ride today's exits, and that a declaration change marks and signals nothing; no new carrier, verb, exit, session error or span field.
@task_item n=13 part=needs
The keyed field on the carriers: a virtual node's key as it rides the graph, actions and coverage answers.
@task_item n=14 part=needs
Circle Drawer's drawing area stays one named canvas node on the stand, which declares no virtual node; the proof is a fixture canvas a test declares, in both layout modes.
@task_item n=15 part=needs
Literal pins swept with the words: the self-description's counts and help texts, the verb count, the capability statement's counts and their pin, and every guard asserting no /// id, each turned to the new reading.
@task_item n=16 part=needs
Each new control enters the plan with its red reading, on a guard some input can turn red; a guard refusal of a write is a stop.
@task_standing n=1
convention: each new accessibility check is a Rust test over the AccessKit tree and the snapshot, not a browser or platform-adapter check
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new accessibility test file runs in the accessibility test leg of the slow tier and gates CI
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: expected roles, names and node sets for a test are stated once in a shared table and read by every check that asserts them
home: 7-screen-reader-support · screen-reader-support
@task_item n=1 part=not_needed
Keyboard key contracts of controls (select lists, range input, Cells grid, CRUD list, Enter and Space activation): keyboard reach into a canvas is a later entry.
@task_item n=2 part=not_needed
ARIA state and property reads, role mapping and hidden-content rules of the accessibility tree: the chunk adds no ARIA reading and maps no new role.
@task_item n=3 part=not_needed
The platform accessibility adapter and accessibility feature gating: no adapter, feature or assistive-technology reach changes.
@task_item n=4 part=not_needed
Stand colour choices, scrollbar thumb strokes, scheme and zoom: no colour or viewport setting changes.
@task_item n=5 part=not_needed
Password masking and caret, IME, form event values and select-list choice write paths: no text or form value is read or written for a virtual node.
@task_item n=6 part=not_needed
Cognitive accessibility rows (invalid state beyond colour or class, shared stand tab-order table): untouched.
@task_item n=7 part=not_needed
Open select list visuals and motion, select-list row heights and colours: untouched.
