@task_prose part=chunk
Brings a canvas's taken virtual nodes (id {owner}///{key}) into every driver read that today holds DOM elements alone: the UI graph (one graph node per virtual snapshot node, with its owner, in process and as the graph verb), the actions answer (a virtual node with the verbs it takes and the refusal cause each would meet), handler coverage (a virtual node's reach where a kept check acts on it), the layout explanation (an answer for a virtual id) and exploration (a virtual node as a candidate it acts on).
Owns the carried provisional limits of the virtual-nodes-as-elements entry: lifts the exclusion that kept the graph, actions, lint and exploration free of virtual nodes, and re-decides which verbs take a virtual id (only click today; type, scroll, screenshot with an id, layout and canvas read it not-found) and whether a virtual node reads clickable (false today).
Sweeps the literal pins with the words: counts, help texts, the driver's self-description, the verb count and the committed capability statement's counts, and turns the guards that assert no virtual node in graph, actions and lint.
Proves on a fixture canvas a test declares, in both layout modes; adds no carrier, verb, exit, session error or span field, and any new refusal cause or crossing class is written as a fork for the founder.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph, one graph node per snapshot node, and its virtual-node exclusion — edits: the graph build, one node per virtual snapshot node with its owner
@task_file n=2 path=packages/escher-driver/src/actions.rs mark=edits
what: the actions read and its answer of nodes, ids, owners and verbs — edits: the listing of a virtual node with the verbs it takes and each verb's refusal cause
@task_file n=3 path=packages/escher-driver/src/lint.rs mark=edits
what: the lint and its findings over the page and canvas labels — edits: the exclusion of virtual nodes
@task_file n=4 path=packages/escher-driver/src/explore.rs mark=edits
what: exploration and its report of the candidates it acts on — edits: the candidate set, a virtual node a candidate
@task_file n=5 path=packages/escher-driver/src/coverage.rs mark=edits
what: handler coverage, the coverage role reading kept checks into its answer line — edits: the reach read for a virtual node a kept check acts on
@task_file n=6 path=packages/escher-driver/src/execute.rs mark=edits
what: the target check shared by the verbs and the actions read, with its fixed refusal order — edits: resolve, locate and check_target, the arm refusing a virtual id as not-found on verbs other than click
@task_file n=7 path=packages/escher-driver/src/layout.rs mark=edits
what: the in-process layout read on a session and the layout verb's answer — edits: Session::layout, an answer for a virtual id
@task_file n=8 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: the engine's layout explanation the layout read answers from — edits: layout_reason
@task_file n=9 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot and its per-node state readings — edits: the clickable reading of a virtual node
@task_file n=10 path=packages/dioxus-native-dom/src/reached.rs mark=edits
what: the reached-handler record, deduplicated, kept past removal, holding no NodeId and writing no log
@task_file n=11 path=docs/capabilities.md mark=edits
what: the committed capability statement — edits: its counts and their pin, the stated limits pinned by a check
@task_file n=12 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's graph check — edits: the guard asserting no virtual id in the graph
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actions check — edits: the guard asserting no virtual id in the actions answer
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs mark=edits
what: the virtual-node declaration check, declarations read and validated in process — edits: the guard asserting no virtual id
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs mark=edits
what: the virtual-nodes-as-elements check on a fixture canvas — edits: the_graph_actions_and_lint_hold_no_virtual_node
@task_item n=1 part=needs
The virtual node crossing rule: today the graph, the actions answer, the lint, the canvas answer and exploration hold no virtual node; this chunk changes what crosses on the graph, actions, lint and exploration, and keeps or re-decides the canvas answer's exclusion.
@task_item n=2 part=needs
The graph answer: its crossing, its nodes-and-states shape, one graph node per snapshot node now counting virtual snapshot nodes with their owner, and its paging as the answer grows.
@task_item n=3 part=needs
The actions answer: its crossing and its id, owner, verbs, keyed and nodes items for a virtual node; the actions read runs nothing, moves no session state, shares one target check with the verbs and stays unpaged.
@task_item n=4 part=needs
The target check and refusal set: which verbs take a virtual id (click alone today, the rest reading not-found), the fixed order of target refusals, unknown ids refused as stale or not-found, the closed refusal cause set — a new cause is a founder's fork, not a build step.
@task_item n=5 part=needs
The clickable state reading of a virtual node (false today, provisional): the snapshot's three state readings per control, the one actionable rule for both readings, and clickable as a snapshot text state word and a diff node key.
@task_item n=6 part=needs
Handler coverage of a virtual node: the coverage answer's crossing and answer line (checks, held, unexercised) carrying no user data, kept check files read as untrusted, and an unusable check refused as malformed.
@task_item n=7 part=needs
The reached-handler record a virtual node's reach is read from: deduplicated, kept past element removal, holding no NodeId and writing no log; dispatch recorded only where a signal registry stands.
@task_item n=8 part=needs
The layout read of a virtual id: the in-process layout read on a session and the layout verb's answer, read against what the kept check's layout answer holds.
@task_item n=9 part=needs
Exploration acting on a virtual node: its report holds closed content only, and it acts through the same session verbs and target check.
@task_item n=10 part=needs
The lint over virtual nodes: lint findings hold no page content and cross only ratified carriers, the canvas label lint crossing included.
@task_item n=11 part=needs
The stable id's virtual-node form {owner}///{key}: its ratified exits and carriers, no NodeId and no process-local handle crossing with it, ids looked up anew each call, ids kept out of logs.
@task_item n=12 part=needs
A virtual node's name riding a new read: accessible names leave only by their ratified exits (the founder's F3 A, ratified) and stay out of logs; any carrier gaining a virtual id or name is read against those exits and the boundary widening rule.
@task_item n=13 part=needs
No new carrier, verb, exit, session error or span field: the closed driver verb schema, the driver verbs graph, actions, layout and canvas, the content-free command span, the fixed session error table and exit codes stay as ratified.
@task_item n=14 part=needs
Literal pins swept: the driver's describe self-description and verb count, the fixed MCP listing text and server help, the command line's help and usage texts, and the committed capability statement of the crossing and its counts.
@task_item n=15 part=needs
Declared virtual nodes as validated by their closed refusal set are what the reads consume; a declaration change marks and signals nothing (the founder's F11 A, ratified).
@task_item n=16 part=needs
Answer bounds as virtual nodes enlarge the graph, actions and exploration answers: the driver command line's answer bounds and the MCP tool result size bound.
@task_item n=17 part=needs
Kept checks holding the graph, actions and layout answers whole: a check kept before this chunk replays against answers that now carry virtual nodes.
@task_item n=18 part=needs
The proving surface: a fixture canvas the test declares, in both layout modes; the stand declares no virtual node; each turned guard enters the plan with its red reading, on an input that can turn it red.
@task_item n=19 part=needs
Refusals and outcomes of a virtual id stay out of logs: a refusal logs its cause name only and holds nothing a call supplied; driver outcomes are never logged.
@task_standing n=1
convention: each new log line or span names no stable id, accessible name or call content; content-named fields are withheld and only a refusal's cause name is logged
home: logging-monitoring
@task_standing n=2
convention: each new test runs under the test harness that takes no tracing and no style, adding no dependency
home: dependency-security
@task_standing n=3
convention: evidence the plan adds holds no credential and no raw log carrying a temp or home path
home: secret-management
@task_item n=1 part=not_needed
Authentication and authorization rows: the reads add no peer, login or token, and session access stays as ratified.
@task_item n=2 part=not_needed
Outbound HTTP, script fetch, CORS, same-origin, navigation scheme and sub-resource request rows: no read here fetches or navigates.
@task_item n=3 part=not_needed
Select list, multiple select, range input, password painting and masked input value rows: virtual nodes on a canvas are none of those controls.
@task_item n=4 part=not_needed
HTML, CSS, SVG, font and stylesheet parsing and painting rows of the engine: the chunk reads layout, it does not parse or paint content.
@task_item n=5 part=not_needed
Session socket wire grammar, lifecycle replies, connection and idle-expiry rows: the binary-level reading of a virtual id on the socket host is another entry's.
@task_item n=6 part=not_needed
MCP protocol error codes, stdio transport and initialize revision rows: no protocol behaviour changes.
@task_item n=7 part=not_needed
Recording directory, state directory and recorder file-removal rows: the chunk changes no recording or cleanup.
@task_item n=8 part=not_needed
Cold-agent script, agent-run script and apt-install argument rows: no harness script is changed.
@task_item n=9 part=not_needed
WPT runner, WPT quarantine and WPT post-results rows: no web-platform test path is touched.
@task_item n=10 part=not_needed
History store, cookie store, HTTP cache, TLS and clipboard or file dialog rows: no stored data or platform integration is touched.
@task_item n=11 part=not_needed
Release publishing, signing material and CI secret rows: the chunk publishes and signs nothing.
@task_item n=12 part=not_needed
Keyboard reach into a canvas, rotated or scaled canvases, and Circle Drawer's circles as virtual nodes: other entries' scope.
