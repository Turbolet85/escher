@task_prose part=session
The session added virtual nodes declared: an app's custom widget may declare keyed, named, role-bearing regions inside its canvas.
The engine's widget trait gains a defaulted method returning an empty list of declarations, and a declaration type (key, AccessKit role, name, region), behind the custom-widget feature, flagged upstreamable, nothing sent upstream.
The Dioxus native bridge gains an ungated in-process read of a canvas's declared nodes that validates each declaration and gives each taken node the reserved id form {owner}///{key}; a refused declaration carries its index and a closed reason alone.
The read refuses a stale or non-canvas node as NoCanvas and an unlaid canvas as NotLaidOut, with no panic path, no log site, print, span or field, no changed-set mark and no invalidation signal.
No carrier gains a reading: snapshot and graph answers hold no virtual node id or declared name, measured in both layout modes; the names' crossing (F3) stays the founder's fork, deferred, and every answer of the chunk is a delegate's and lands PROVISIONAL.
No dependency, manifest, lock, schema, harness or gate changed; CI green on the final HEAD.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait and its canvas contract in the engine, where an app's widget paints and is pointed in its content box
@task_file n=2 path=packages/blitz-dom/src/node/mod.rs
what: the engine node module's public re-exports of the custom widget types
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: the engine crate's public surface, its re-export list
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus native bridge crate root, its modules and public re-export list
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id derivation in the Dioxus native bridge, holding the reserved virtual-node form and its constructor
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the new in-process read and validation of a canvas's declared virtual nodes in the Dioxus native bridge, with its closed refusal reasons and unit tests
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration check that a padded canvas reads its declared nodes in its content box and that a declaring canvas puts nothing on the snapshot or graph
@task_item n=1 part=touches
contract/stable-element-id/virtual-node-form: the reserved form {owner}///{key} moves from produced by nothing to produced in process for a canvas's taken declared nodes, written by no walk and served by no carrier; no served id holds /// still holds.
@task_item n=2 part=touches
contract/stable-element-id/no-collision and contract/stable-element-id/derivation: a declared key holding a slash or empty is refused, and a later duplicate key refused with the first kept, so a virtual node id cannot collide with a walked id or another virtual node of the same canvas.
@task_item n=3 part=touches
contract/stable-element-id/carriers and decision/stable-id-exits: a new kind of stable id now exists in process; it reaches no ratified exit this chunk.
@task_item n=4 part=touches
decision/accessible-name-exits and decision/attribute-text-leaves-by-ratified-carriers: an app-authored declared name now exists in process; it crosses no exit, and whether it rides the accessible-name exits stays the founder's deferred fork.
@task_item n=5 part=touches
decision/canvas-content-crossing, decision/graph-answer-crossing and contract/graph-answer/nodes-states: a declaring canvas puts nothing on the graph answer; measured in both layout modes.
@task_item n=6 part=touches
contract/snapshot-text/id-field and contract/snapshot-text/name-field: the snapshot holds no virtual node id and no declared name.
@task_item n=7 part=touches
decision/accessibility-adapter-content-classes, decision/author-id-on-accessibility-nodes and decision/accessibility-tree-refresh-on-change: declared nodes are not in the accessibility tree and raise no refresh or invalidation signal this chunk.
@task_item n=8 part=touches
decision/custom-widget-content-box-frame: a declared region is validated against the canvas content box, the final layout size less border and padding per axis floored at 0; a region fitting the border box but not the content box is refused.
@task_item n=9 part=touches
decision/no-panic-on-call-input and decision/stale-node-ids-never-panic: the declared-node read resolves its canvas by element id then node lookup; a stale or non-canvas node reads NoCanvas and an unlaid one NotLaidOut, no panic.
@task_item n=10 part=touches
decision/closed-refusal-cause-set, decision/refusal-closed-cause-set and decision/refusal-holds-no-call-input: a second closed refusal set now stands beside the driver's, per declaration, holding its index and reason alone; it is no driver refusal cause and crosses no carrier.
@task_item n=11 part=touches
decision/one-validation-path-for-every-carrier and decision/single-validation-gate: the declaration validation is an in-process path of its own, reached by no carrier and no driver verb.
@task_item n=12 part=touches
decision/accessible-names-not-logged, decision/stable-id-not-logged and decision/no-screen-content-in-logs: the declared-node read has no log site, print, span or field; declared names and virtual node ids reach no log.
@task_item n=13 part=touches
decision/session-layout-read-in-process and decision/no-process-handle-crosses: the new read takes a NodeId in process, beside the in-process layout read; no process-local handle crosses with a virtual node id.
@task_item n=14 part=touches
decision/unknown-id-refused-stale-or-not-found and contract/stable-element-id/unresolved-node: a stale canvas node is refused in the declared-node read as NoCanvas, the in-process analogue of stale.
@task_item n=15 part=touches
decision/boundary-widening-rule and decision/closed-value-field-no-widening: an engine API addition (a defaulted trait method and a type behind the custom-widget feature) and a public bridge read are added with no carrier field; flagged upstreamable, nothing sent.
@task_item n=16 part=touches
contract/driver-verbs/canvas and name/command/canvas: the canvas verb's answer gains nothing from declared nodes.
@task_fact n=1 slug=input-validation
fact: Virtual nodes on carriers are absent: a canvas's declared virtual nodes, their {owner}///{key} ids and declared names cross no carrier (snapshot, graph, canvas answer, accessibility tree, MCP); whether name and key ride the accessible-name and stable-id exits stays the founder's fork, deferred (PROVISIONAL).
@task_fact n=2 slug=input-validation
fact: App-declared virtual nodes are validated by their reader in declaration order against a closed refusal set (EmptyKey, KeyHoldsSlash, DuplicateKey with the first kept, EmptyName, RegionOutside the canvas content box, NaN and infinities failing); a refusal holds the declaration's index and reason alone, with no panic path (PROVISIONAL on the duplicate rule).
@task_fact n=3 slug=error-handling
fact: The declared-node read refuses a stale or non-canvas node as NoCanvas and a canvas before its first layout as NotLaidOut, a closed set, never panicking.
@task_fact n=4 slug=logging-monitoring
fact: The declared-node read and its validation hold no log site, print, span or field; declared names, keys and virtual node ids reach no log.
@task_fact n=5 slug=input-validation
fact: A declared virtual node's role is AccessKit's Role itself with no second mapping, and validation reads no role (PROVISIONAL).
@task_item n=1 part=untouched
Dependency security: no dependency, manifest, lock, audit, licence or pin changed.
@task_item n=2 part=untouched
Secret management and environment variables: nothing reads an environment variable or secret.
@task_item n=3 part=untouched
API security: no listener, socket bound, outbound request, load wait or idle expiry changed; the session socket and MCP server are unchanged.
@task_item n=4 part=untouched
Authentication and authorization, CI tokens and publish guards: untouched.
@task_item n=5 part=untouched
Data protection: kept checks, recordings, history, cookies, cache and TLS are unchanged.
@task_item n=6 part=untouched
Driver session wire grammar, CLI flags, exit codes, MCP protocol errors and session error tables: no verb, flag, reply or error added.
@task_item n=7 part=untouched
Select list, range, password masking, form controls, script fetch, HTML and CSS parsing rows: no change reaches them.
@task_item n=8 part=untouched
Cold-agent, agent-run, coverage floor, WPT and apt-install script rows: no script or harness changed.
