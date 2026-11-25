@task_prose part=session
The session serves a canvas's declared virtual nodes as elements: each taken virtual node now stands in the accessibility tree under its canvas, and from it in the snapshot, its text and its diff, carrying its {owner}///{key} id and its declared name.
The click verb now takes a virtual node id, locates it at its owning canvas and acts on its region through the one target check, refused in the fixed order (disabled, off-screen, covered, not-found, stale); every other id verb reads a virtual id not-found.
The graph, the actions answer, the lint and exploration hold no virtual node; the canvas answer is unchanged; no verb, argument, key, result field, cause, session error, exit, log field or span was added, and no dependency moved.
The virtual node declaration gains a disabled flag; the declared-node reader and its closed refusal set are unchanged.
The id and name now cross the MCP results, the kept checks (which replay a virtual click) and, by construction, the socket and command-line answer line: a boundary widening ratified by the founder (F3 A).
Measured: the a11y CI leg runs twenty files (20 · 199 · 0 · 0); the act-spans guard reads 46 child calls and no virtual id or declared name on any log line.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget's virtual node declaration (key, name, role, region), now with a disabled flag
@task_file n=2 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the in-process declared virtual node reader and its closed refusal set; now also a virtual node's page bounds and its accessibility tree id
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document's accessibility tree build, which now adds each canvas's virtual nodes under the canvas with author id, role, name and disabled
@task_file n=4 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot reader (id, role, name, state readings, bounds) that feeds the snapshot text, the diff, MCP results and kept checks; now reads virtual nodes
@task_file n=5 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph answer builder, which now skips every node whose id holds the virtual separator
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id derivation and its reserved virtual-node form, documented as now served
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver's command execution and id resolution (stale and not-found refusals, target check), now resolving click on a virtual id and refusing it on other id verbs
@task_file n=8 path=packages/escher-driver/src/lint.rs
what: the canvas and layout lint and its findings; doc only, its stated limits gain that virtual nodes are not linted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the closed driver verb schema and its fixed help texts (the click help and the clickable key help changed)
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the unit check pinning every verb's help, arguments and result
@task_file n=11 path=packages/escher-driver/src/refusal.rs
what: the closed refusal cause set with fixed meanings and remedies; the not-found and covered meanings changed
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: new integration check of virtual nodes in the tree, snapshot, click, refusals, MCP and a kept check's replay, and their absence from graph, actions and lint
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: integration check of the declared virtual node reader and its refusals
@task_file n=14 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the log guard that no id or name reaches a span line on the stand apps' acting calls
@task_file n=15 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixture pages, now with a canvas page declaring five virtual nodes
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the CI leg script; the a11y leg's list of test files
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the check of the CI workflow legs and their test lists
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the check that the committed capability statement states the leg counts
@task_file n=19 path=docs/capabilities.md
what: the committed capability statement, its not-built list and stated limits
@task_item n=1 part=touches
absence/virtual-nodes-on-carriers is now stale: the accessibility tree, snapshot, diff, MCP results and kept checks hold a declared virtual node; the absence narrows to the graph, actions answer, lint, canvas answer and exploration.
@task_item n=2 part=touches
contract/stable-element-id/virtual-node-form is stale: the reserved {owner}///{key} form is now produced and served on the tree, snapshot and diff, no longer in process only.
@task_item n=3 part=touches
contract/stable-element-id/carriers and decision/stable-id-exits: the virtual-node id now leaves by the snapshot text, diff, MCP results, kept checks and the socket and command-line answer line.
@task_item n=4 part=touches
decision/accessible-name-exits, decision/attribute-text-leaves-by-ratified-carriers and decision/accessibility-adapter-content-classes: an author-declared virtual node name now crosses the accessibility adapter, snapshot, diff, MCP results and kept checks (F3 A).
@task_item n=5 part=touches
decision/boundary-widening-rule: the session is a boundary widening (virtual node id and declared name on every carrier), answered by the founder himself (F3 A).
@task_item n=6 part=touches
decision/virtual-node-declarations-validated: the declaration gains a disabled flag; the reader's closed refusal set is unchanged and now feeds the tree and snapshot, a canvas whose read errs taking none.
@task_item n=7 part=touches
decision/author-id-on-accessibility-nodes: a virtual node's tree node carries the author id {owner}///{key}; its tree id is the canvas slot with an even high half, equal to no DOM id.
@task_item n=8 part=touches
decision/accessibility-focus-built-nodes-only and decision/inert-subtree: tree focus is untouched; a hidden, collapsed or inert canvas contributes no virtual node.
@task_item n=9 part=touches
decision/snapshot-state-readings and contract/snapshot-text/clickable, id-field, name-field: a virtual node reads id, role, name, enabled from its declared disabled, clickable false, highlighted false, no checked, value, selected or pressed.
@task_item n=10 part=touches
contract/diff-node/differs and the diff node keys: a click on a virtual node yields a diff naming the virtual node as differs name.
@task_item n=11 part=touches
contract/refusal-causes/not-found and contract/refusal-causes/covered (and contract/refusal-json/meaning): their fixed meaning texts changed to cover virtual nodes; remedies unchanged.
@task_item n=12 part=touches
decision/target-refusal-order, decision/unknown-target-refused and decision/actions-read-shares-target-check: click on a virtual id runs the same check, refused disabled, off-screen, covered, not-found, stale in the one order, nothing run.
@task_item n=13 part=touches
decision/unknown-id-refused-stale-or-not-found and contract/driver-id-refusal/not-found: type, scroll, screenshot with an id, layout and canvas read a virtual id not-found, recording the screen first.
@task_item n=14 part=touches
decision/closed-driver-verb-schema and decision/single-validation-gate: the click help and the clickable key help changed; no verb, argument or key added; a virtual id passes the same validation.
@task_item n=15 part=touches
decision/graph-answer-crossing and contract/graph-answer/nodes-states: the graph skips every id holding the virtual separator.
@task_item n=16 part=touches
decision/actions-answer-crossing and contract/actions-answer/id, nodes: the actions answer holds no virtual node.
@task_item n=17 part=touches
decision/lint-findings-crossing and decision/lint-findings-closed-content: the lint names no virtual id, by construction (a virtual entry has no element, so no visible part); no skip in code.
@task_item n=18 part=touches
decision/mcp-result-crossing and decision/mcp-results-client-responsibility: MCP results now carry a virtual node's id and declared name.
@task_item n=19 part=touches
Kept checks (decision/kept-check-answer-whole, decision/kept-check-call-content, contract/kept-check-file/contents, contract/recorded-check/arguments, decision/replay-through-session-run): a kept check holds a virtual click with its id and name and replays it.
@task_item n=20 part=touches
decision/socket-carries-screen-reading and decision/session-socket-carries-calls: the socket and command-line answer carry the virtual id and name by construction, not yet read on the binary (owed to the first served app declaring one).
@task_item n=21 part=touches
decision/cold-agent-screen-to-provider: a cold-agent run would carry virtual ids and names to the provider by construction; none served yet.
@task_item n=22 part=touches
decision/stable-id-not-logged, decision/accessible-names-not-logged and decision/command-span-content-free: no log site, span or field added; the act-spans guard reads no virtual id or declared name, its call count 43 to 46.
@task_item n=23 part=touches
decision/custom-widget-click-synthesis and decision/custom-widget-content-box-frame: a virtual click acts on its region in the canvas content box and focuses the canvas; virtual bounds are the region moved by the content-box origin.
@task_item n=24 part=touches
name/command/canvas and contract/driver-verbs/canvas, layout: these verbs read a virtual id not-found; the canvas answer holds no virtual node.
@task_item n=25 part=touches
decision/no-process-handle-crosses and contract/stable-element-id/no-node-id: the accessibility tree id is built from the canvas slot internally; only the {owner}///{key} id crosses.
@task_fact n=1 slug=threat-model-summary
fact: A canvas's virtual node id {owner}///{key} and its author-declared name cross the accessibility tree, snapshot text, diff, MCP results, kept checks and the socket and command-line answer line, a widening ratified by the founder (F3 A); the graph, actions answer, lint, canvas answer and exploration hold none.
@task_fact n=2 slug=input-validation
fact: Click is the only verb that takes a virtual node id, locating it at its owning canvas and acting on its region through the one target check; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
@task_fact n=3 slug=input-validation
fact: A virtual node in the snapshot reads enabled from its declared disabled flag, clickable false, highlighted false, no checked, value, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
@task_fact n=4 slug=input-validation
fact: A hidden, collapsed or inert canvas, or one whose declared-node read errs, contributes no virtual node to the accessibility tree or the snapshot.
@task_fact n=5 slug=input-validation
fact: The lint holds no virtual node by construction: a virtual entry has no element, hence no visible part to pair or read; no code skip guards it, a behaviour check does.
@task_fact n=6 slug=error-handling
fact: A virtual node reads covered only by an element over its region and is refused in the same fixed order as an element; the not-found and covered meanings state this.
@task_item n=1 part=untouched
Dependency security: no dependency, lockfile, audit ignore, licence or pin moved.
@task_item n=2 part=untouched
Authentication, authorization and secret management: no credential, token, environment variable or secret read was added.
@task_item n=3 part=untouched
API security listeners and bounds: no listener, socket grammar, request or result size bound, idle expiry or load wait changed.
@task_item n=4 part=untouched
Outbound network, fetch schemes, CORS, CSP, file URLs, iframes and stylesheet or font loading: not reached.
@task_item n=5 part=untouched
Value masking, password and file inputs, select lists, range inputs and their exits: not reached; a virtual node carries no value.
@task_item n=6 part=untouched
Protocol and exit contracts: no MCP protocol error, session error, exit code, CLI flag, role or refusal cause was added or removed.
@task_item n=7 part=untouched
Logging sink, target allowlists and telemetry: no log site, span field or target added.
@task_item n=8 part=untouched
CI token permissions, CI secrets, publish jobs, WPT workflows and coverage floors: unchanged.
@task_item n=9 part=untouched
Data protection storage (history database, HTTP cache, cookies, TLS, recording directory permissions): unchanged.
