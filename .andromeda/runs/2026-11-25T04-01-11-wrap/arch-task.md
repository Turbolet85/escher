@task_prose part=session
Virtual nodes declared: an app's custom widget can now declare keyed, named, role-bearing regions inside its canvas, and the Dioxus document reads and validates them, giving each taken one the reserved id {owner}///{key}.
blitz-dom, behind the custom-widget feature, gains the type VirtualNodeDeclaration (key, AccessKit Role, name, region as BoundingRect) and the defaulted trait method Widget::virtual_nodes returning an empty list; nothing in the engine calls it, no existing Widget implementation was edited, and the addition is flagged upstreamable with nothing sent.
dioxus-native-dom, ungated, gains the module virtual_node: DioxusDocument::virtual_nodes(canvas NodeId) returns VirtualNodes (taken nodes and refused declarations by index) or VirtualNodesUnread (NoCanvas for a stale or non-canvas node, NotLaidOut before the first layout); refusals are a closed set EmptyKey, KeyHoldsSlash, DuplicateKey (first kept), EmptyName, RegionOutside (region must lie inside the content box, border and padding subtracted).
virtual_node_id gains its first production caller; the reserved form moves from produced by nothing to produced in process by this read, still written by no id walk and served by no carrier: snapshot and graph carry no virtual node, proven in both layout modes.
No log site, span or field; no changed-set mark and no invalidation signal on a declaration change; no dependency, manifest or crate added; one blitz-tests target (virtual_nodes_declared) and nine unit tests added. Every decision resting on the overseer's answers lands PROVISIONAL; the names' crossing (F3) stays the founder's fork, deferred.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget embedding in blitz-dom, the Widget trait and its defaulted methods behind the custom-widget feature; edited to add VirtualNodeDeclaration and Widget::virtual_nodes
@task_file n=2 path=packages/blitz-dom/src/node/mod.rs
what: blitz-dom's nodes and widgets module and its public re-exports; edited to re-export VirtualNodeDeclaration
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: the blitz-dom crate root and its public re-export list; edited to re-export VirtualNodeDeclaration
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root, its module list and public API re-exports (the Dioxus DOM bridge); edited to add the ungated virtual_node module and re-export five types
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id tiers of the Dioxus DOM bridge, element_id, element_ids and virtual_node_id with the reserved form; edited, docs only, to say the reserved form is now produced in process by the declared-node read
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: new; the Dioxus DOM bridge's declared-node reader DioxusDocument::virtual_nodes, its validation and the types VirtualNode, VirtualNodes, VirtualNodeRefusal, RefusedDeclaration, VirtualNodesUnread, with its unit tests
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: new; a blitz-tests integration test target checking the declared-node read in a padded canvas, a widget declaring nothing, and that a declaring canvas puts nothing on the snapshot or graph, in both layout modes
@task_item n=1 part=touches
contract/nodes-widgets/widget-trait: the Widget trait gains the defaulted method virtual_nodes returning an empty list; no existing implementation changes.
@task_item n=2 part=touches
contract/stable-element-id/virtual-node-id: the row saying no tier produces the reserved form is now stale; the form is produced in process by DioxusDocument::virtual_nodes for a canvas's taken nodes, still written by no id walk and served by no carrier.
@task_item n=3 part=touches
contract/dioxus-bridge/virtual-node-id: virtual_node_id gains its first production caller (the declared-node validation); its body is unchanged.
@task_item n=4 part=touches
decision/accessibility-feature-gating: dioxus-native-dom's ungated list gains the module virtual_node; its unit tests name no accesskit item since accesskit is optional under the accessibility feature.
@task_item n=5 part=touches
decision/feature-gated-widgets and decision/cargo-feature-gating: the new engine type and method stand behind the existing custom-widget feature.
@task_item n=6 part=touches
decision/direct-engine-changes-flagged-upstreamable and decision/fork-additions-in-own-functions: the engine API addition in blitz-dom is an escher addition flagged upstreamable, nothing sent, added lines only.
@task_item n=7 part=touches
decision/accesskit-accessibility-tree and the AccessKit decisions: a declared node's role is AccessKit's Role type itself, with no second role mapping (PROVISIONAL).
@task_item n=8 part=touches
contract/nodes-widgets/try-layout-data: its premise, None before the first layout, is now relied on by a new reader (NotLaidOut) and held under measurement.
@task_item n=9 part=touches
decision/changed-set-write-sites, decision/changed-set-engine-touched and decision/a11y-rebuild-on-nonempty-changed-set: a declaration change writes no changed-set mark and signals no reader; the invalidation half is deferred to the first reader that refreshes, so these rows stay true but gain a stated gap.
@task_item n=10 part=touches
decision/canvas-crossing, decision/ui-graph-crossing, contract/snapshot/membership and contract/graph-verb/node: a declaring canvas puts no virtual node, no ///-form id and no declared name on the snapshot or graph; the crossing of virtual nodes stays absent.
@task_item n=11 part=touches
decision/no-node-id-across-session and decision/node-id-in-public-api: the new read takes a NodeId in process; nothing it yields crosses a session.
@task_item n=12 part=touches
decision/custom-widget-content-box-paint and decision/widget-click-point-content-box: declared regions are measured against the same content box (border and padding subtracted, floored at 0), a region fitting the border box but not the content box is refused.
@task_item n=13 part=touches
decision/typed-results and decision/error-enums: the read returns a typed Result with closed enums for its refusals and unread causes, no panic path.
@task_item n=14 part=touches
contract/dioxus-native-dom-api rows and contract/geometry/bounding-rect: dioxus-native-dom's public re-export list gains five types; blitz-dom's public surface gains VirtualNodeDeclaration, whose region is a BoundingRect.
@task_item n=15 part=touches
decision/blitz-tests-one-target-per-file, decision/integration-tests-in-blitz-tests, decision/both-layout-modes and decision/fixture-precondition-first: a new blitz-tests target, run in both layout modes, asserting its live-needle precondition first.
@task_item n=16 part=touches
decision/driver-logs-no-content, decision/snapshot-text-never-logged and the never-logged decisions: the declared-node read has no log site, print, span or field, and its types are never logged.
@task_item n=17 part=touches
contract/accessibility-tree/build-accessibility-tree and decision/accesskit-accessibility-tree: declared nodes do not enter the accessibility tree this session; that stays the later entry's (virtual nodes as elements).
@task_fact n=1 slug=standard-contracts
fact: blitz-dom nodes and widgets · Widget::virtual_nodes and VirtualNodeDeclaration: behind custom-widget, a widget declares regions as key, AccessKit Role, name and region (BoundingRect) through a defaulted method returning an empty list; the engine never calls it (PROVISIONAL).
@task_fact n=2 slug=standard-contracts
fact: Dioxus DOM bridge · DioxusDocument::virtual_nodes: an ungated in-process read of a canvas's declarations, returning taken nodes with id {owner}///{key} and refused declarations by index, or NoCanvas (stale or non-canvas node) and NotLaidOut (before the first layout); no panic path (PROVISIONAL).
@task_fact n=3 slug=standard-contracts
fact: declared-node validation, in declaration order, refuses with a closed reason set EmptyKey, KeyHoldsSlash, DuplicateKey (the first declaration of a key kept, later ones refused), EmptyName and RegionOutside (region not inside the content box with positive size; NaN and infinities fail), the refusal holding the declaration's index alone (PROVISIONAL).
@task_fact n=4 slug=established-decisions
fact: a declared virtual node's role is AccessKit's Role itself, with no second role mapping of escher's own (PROVISIONAL).
@task_fact n=5 slug=established-decisions
fact: a declaration change neither marks its canvas in the changed set nor signals any reader; invalidation and the changed-set mark are left to the first reader that refreshes (PROVISIONAL).
@task_fact n=6 slug=standard-contracts
fact: virtual nodes on the driver's carriers are absent: no verb, snapshot, graph, diff or accessibility tree carries a declared node, its id or its name; whether name and key ride the accessible-name and stable-id exits stays the founder's open fork.
@task_item n=1 part=untouched
Stack and dependency decisions: no crate, dependency, manifest or lock change, no version or toolchain moved.
@task_item n=2 part=untouched
CI, gate and infrastructure rows: no leg, workflow, coverage floor, mutation tier or script changed.
@task_item n=3 part=untouched
The driver's verbs, arguments, refusal causes, answer JSON, MCP server and session socket wire: no verb, key, cause or bound moved.
@task_item n=4 part=untouched
Telemetry, tracing bootstrap and scrubbing rows: nothing logged, no target or field added.
@task_item n=5 part=untouched
Layout, style, scrolling, hit testing, select list, form controls, event dispatch and paint rows: no engine behaviour changed.
@task_item n=6 part=untouched
Browser, WPT runner, script engine, networking and persistence rows.
@task_item n=7 part=untouched
Occupied names: no binary, crate, environment variable or server name taken or freed.
@task_item n=8 part=untouched
The seven_guis stand, cold-agent, recording and kept-check rows: no stand fixture, check or verdict changed.
