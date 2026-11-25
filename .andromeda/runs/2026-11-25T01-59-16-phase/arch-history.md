@history_item n=1 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — added the fifth, reserved id form `{owner}///{key}` that no tier produces and no walk writes, spelled by the public `VIRTUAL_KEY_SEPARATOR` and `virtual_node_id`, which answers `None` for an empty or `/`-bearing key. This is the founder's P4 fork 2 answer, and `stand_graph` measured no served id holding `///`. It is exactly what this chunk first fills: contract/dioxus-bridge/virtual-node-id and contract/stable-element-id/virtual-node-id. The key rules (non-empty, no `/`) are already the function's own. A taken node must keep "no served tier id holds `///`" true.

@history_item n=2 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers; `clickable`; the graph reader) — `graph` and `clickable` crossed as a ratified widening on the founder's own forks, with the graph's nodes defined as snapshot nodes. This is the precedent for a new reading over the screen's nodes: every graph node is a snapshot node, and `clickable` is a closed bool inside a ratified carrier. Planning should know that nodes with no DOM behind them would reach graph, actions and coverage. Those consumers belong to «Virtual nodes in every read», not this chunk.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — registered `element_id` and `element_ids` and the id grammar: computed on demand, written to neither the DOM nor the vdom, and with no `NodeId`, `ElementId`, `ScopeId` or pointer in an id. The owner part of `{owner}///{key}` is the canvas element's id under this grammar. That makes contract/stable-element-id/no-process-local-values and element-id binding on what a virtual node's id may hold.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the anchored tier `{key}//{segment}` (one-component reach, ratified by the founder) and `unkeyed_actionable`. It set the tier-distinctness rule that only an anchored path holds `//`, which the later `///` form had to stay distinct from. It also left a trap: `UnkeyedActionable.node` is a process-local `NodeId`, so no wire form may carry it. The same applies to any in-process virtual-node record this chunk adds.

@history_item n=5 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed) — Circle Drawer's canvas reads the author id `circle-canvas`, and `unkeyed_actionable()` returns nothing on every stand task. A canvas owner keyed by author key is the owner half of `{owner}///{key}` on the stand. A keyed owner keeps the virtual id stable under the anchored rules.

@history_item n=6 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — added the defaulted `Document::accessibility_tree()`, which `DioxusDocument` overrides to set `author_id` to the stable id, and which the shell calls for the platform tree. contract/document-trait/accessibility-tree is the extension seam later virtual-node entries will use. Its noted trap: a feature-gated item can pass under `--workspace` and be compiled out under `-p blitz-tests`.

@history_item n=7 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot node is every element the accessibility tree keeps, and the standing rule is that the snapshot derives no role or name of its own. Today's snapshot model is element-only. The role set and name this chunk declares must not become a second source that disagrees with the tree once «Virtual nodes as elements» lands.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the adapter's third class) — the builder's state reads widened the platform-adapter crossing by a new author-derived class. This was ratified with the rule that a later attribute read keeps that shape: a closed set, no text carried out. This is the precedent for a closed role set. A declared non-empty name is free author text, so a closed set does not cover it.

@history_item n=9 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (descriptions on the adapter crossing) — a fourth content class, descriptions (free page text), goes to the platform adapter only, ratified as upstream's behaviour, and the snapshot reads no description. This underlies decision/adapter-crossing-carries-descriptions: authored free text has reached the adapter and not the driver's carriers. It is a lead for the names'-crossing fork.

@history_item n=10 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — registered the session socket, ratified by the founder, with the standing rule that the first request carrying an id, a name, a value, snapshot text or a diff is a new crossing question. This is the root of decision/new-carrier-widens-boundary and of the names'-crossing fork this chunk must put to the founder.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries a call and its answer) — the second widening: ids, accessible names and masked values leave by design, ratified by the founder. «The next surface that carries them asks again.» This is the basis of decision/socket-carries-driver-calls and decision/session-over-unix-socket. A virtual node's name riding an existing answer is the question this entry says must be asked again.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP role's stdio crossing was ratified with no listener (decision/mcp-stdio-crossing), with the standing rule that the next surface carrying a call or a screen's content is a new crossing question. If names cross, MCP is one of the carriers the fork must measure.

@history_item n=13 marker=2026-10-12-self-description
**2026-10-12-self-description** (fixed program text on the ratified carriers) — the program's own fixed `'static` table text rides the ratified carriers, while text from a page, a call or a screen still widens. This is decision/fixed-program-text-on-carriers. A declared virtual-node name is app-authored text, not program text, so it falls on the widening side.

@history_item n=14 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `caused` began carrying an app's own authored messages and fields across every carrier, ratified on the founder's word. decision/no-node-id-across-session's list gained "a step's caused text". This is the nearest precedent for app-authored text crossing a ratified carrier as an explicit widening. It is a measured-cost lead for the names'-crossing fork.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening) — a select list's painted labels crossing as pixels in a shot was named a boundary widening and ratified by the founder. A row's `bounds` stayed inside the ratified crossing, and no `expanded` node key was added. This is the precedent for content with no DOM box of its own reaching the carriers, and for keeping that content off the node keys.

@history_item n=16 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — added decision/canvas-crosses-as-pixels, ratified for all carriers, and decision/harness-paints-custom-widgets (the harness turns on blitz-paint's `custom-widget`). This was the first statement of how a canvas's content crosses, and the base the later draw-command crossing replaced.

@history_item n=17 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — added decision/custom-widget-content-box-paint, later restated as ratified: the widget paints at its content box times the scale, at the content origin inside the element's transform, unclipped. It also added `Widget::fonts` and `BaseDocument::font_ctx`. The scope holds a virtual node's region in the content-box frame on the strength of this decision. Its region rule must use the same box (border and padding off, floored at 0).

@history_item n=18 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added decision/widget-click-point-content-box (a click point is read against the content box, measured off by border and padding when read against the client rect) and decision/widget-click-synthesis. It also added Circle Drawer's in-process-only boot, served by no role, and decision/canvas-placement-at-click-centre, which was provisional and later ratified. The pointer frame the scope cites for the region is this one. The stand's Circle Drawer is the canvas a later entry turns into virtual nodes.

@history_item n=19 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** — added a canvas-only shot held byte-equal on three CI hosts as an exception to decision/no-screenshot-in-cross-os-byte-check, and restated committed-stand-checks' reason. This explains the test-plan CARRY cry-ax6q6t4w (the one-host clause) that this chunk's wrap must carry.

@history_item n=20 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (canvas crossing) — retired canvas-crosses-as-pixels for decision/canvas-crossing: draw commands cross every carrier as the `canvas` verb's answer, ratified. It added "canvas readings" to decision/no-node-id-across-session and decision/canvas-reading-diffable, plus provisional decision/canvas-dump-in-parts. These are the standing crossing rows a virtual node's id, role and name would extend. The draw-commands-grouped-by-node work belongs to the Circle Drawer entry.

@history_item n=21 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (the canvas verb and `custom_widget_scene`) — added contract/paint/custom-widget-scene: one widget's scene outside a raster, behind `custom-widget`, flagged upstreamable. It also added the harness's `canvas_scene`, `Session::canvas`, the canvas target-check arm and the verb table at sixteen. This is the in-process read path and the widget-scene seam beside which a declaration read would sit.

@history_item n=22 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — added decision/canvas-label-region (provisional): a label's region is the widget's content box cut by each pushed clip, layer or mask path's bounding box, mapped through its transform, and by the element's clip region. It also added absence/canvas-lint-checks-beyond-truncation and decision/skrifa-glyph-outline-bounds (provisional). This is the nearest existing in-canvas region rule. A virtual node's region "inside the canvas's content box" must be stated consistently with it.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage reader and its limits) — states "handlers inside a canvas not counted, deferred to 0.4.0" as a limit with its owner. Virtual nodes are this version's path to that deferred limit. The scope assigns coverage to «Virtual nodes in every read», so this chunk must not claim the limit's discharge.

@history_item n=24 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule; `actions`) — added `actionable_nodes`, the one actionable rule, `ActionableNode { id, owner, keyed }`, and `keyed` as the actionable-key check's first wire form (decision/keyed-wire-form, decision/actionable-elements-keyed). `keyed` reads true where the id holds no `/`. A `{owner}///{key}` id holds `/`, so a virtual node read through this rule would read unkeyed. That is a trap for the later «every read» entry and for how this chunk states the form.

@history_item n=25 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — decision/keyed-wire-form and decision/actions-keyed-reading went from provisional to standing on the founder's word. The keyed reading's `/` test is now a ratified wire form, so the virtual form's interaction with it is not loose.

@history_item n=26 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (a node's component through Dioxus public API) — added decision/owning-component-via-public-api: the owning component is read through Dioxus 0.7.10's public API, and a deleted row resolves to none. Hook-local `use_signal` state is unreachable without a fork-level change. This bounds where a declaration living in the Dioxus renderer can read the owner from, one of the forks P4 may find.

@history_item n=27 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — added decision/bridge-buffered-render: a ratified port of upstream `286874cc1`. The vdom renders into a mutation buffer with the document unborrowed, then replays it. Its limit: a node created in the same render is not yet in the document. A declaration made at render or paint time must respect this ordering, and the merge surface is owned by the upstream-sync CARRY.

@history_item n=28 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node) — a node the bridge removes is dropped with its descendants and no longer resolves, giving `get_node` `None` and `element_id` `None` (decision/bridge-removal-drops-subtree). This sets what happens to a canvas owner's id and to declared nodes when the canvas element goes away.

@history_item n=29 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — added the changed set (has_changes / take_changed_nodes, node creation marking nothing, drained ids read through `get_node`) and the diff kept apart from the changed set. Both were later ratified. These are the changed-set rows in the picked set. A declaration list changing between paints is a write the written-sites list does not hold.

@history_item n=30 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (the changed set's write sites) — decision/changed-set-write-sites' exception gained "a focus given up after a layout marks its node". Together with the select-list highlight site, this is the latest form of the closed write-site list a new marking site would amend.

@history_item n=31 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (the platform tree refresh moved to redraw) — changed decision/accessibility-tree-refresh-on-change and contract/accessibility-tree/platform-refresh: on a redraw, after resolve, the shell hands the adapter a fresh tree when the changed set is non-empty, and a poll refreshes nothing. This is the refresh path virtual nodes would ride once they reach the tree.

@history_item n=32 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — redraw requests now also follow a focus set or cleared (decision/redraw-requests). This is a lead for whether a declaration change must request a redraw.

@history_item n=33 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the geometry readers) — the client rect is read where transforms move it, and absolute position subtracts only the ancestors' scroll (decision/client-rect-under-transforms, decision/absolute-position-ancestor-scroll-only, decision/transformed-element-bounds). Both were ratified 2026-10-18. A region held in the canvas's content-box frame is later mapped into the page through these readers.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (visible_region through transforms) — added contract/geometry/visible-region-transforms: exact under translate and scale, and the rotated padding box's bounding box under a rotation. This is the standing limit any later visibility reading of a virtual node's region inherits.

@history_item n=35 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — added contract/scroll-hit/hit-clip's content-box frame (a hit stopped at a clipping box answers in that box's content-box frame) and contract/nodes-widgets/try-layout-data (public, read-only, None before the first layout). The content box a region check reads comes from layout data that may be absent before the first layout. A declaration must not panic then.

@history_item n=36 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — added `BaseDocument::text_extent`, a read-only reader with stated limits (RTL and vertical text read as LTR, a transformed box read at its client rect). This is contract/basedocument-readers/text-extent in the picked rows: an example of a no-panic, mutate-nothing reader shape for an in-process declaration read.

@history_item n=37 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — added decision/no-wall-time-outside-clock: no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs`. Ratified 2026-10-18. This binds a declaration form with invalidation or timing that lands in blitz-dom.

@history_item n=38 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — added contract/nodes-widgets/widget-drop-acceptance and decision/os-drop-input, both ratified 2026-11-24. These are the latest widenings of the `Widget` trait's surface beside contract/nodes-widgets/widget-trait. They precede the CARRY cry-tjzqz3hz (OS drag-and-drop as untrusted input) that this chunk's wrap must carry.

@history_item n=39 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — added absence/fixture-state-on-carriers: a start in a state was ratified as in process only. This is the precedent for the conservative names'-crossing option, a capability in process only with its carriers stated as an absence.

@history_item n=40 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — added absence/component-alone-on-carriers beside the fixture-state absence. This is a second in-process-only precedent, and part of the same picked absences set the plan reads.

@history_item n=41 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added `Session::layout` as an in-process read with decision/layout-read-in-process-only, provisional on a delegate's answer pending the founder. It also amended decision/driver-spans-only: an in-process read outside the run path opens no span. This is the precedent and constraint for an in-process declaration read: no span, no event, and in-process-only status recorded as provisional when a delegate answers.

@history_item n=42 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — retired layout-read-in-process-only for decision/layout-answer-crossing. It widened decision/no-node-id-across-session ("layout readings ... numbers") and the verb set on the founder's own fork. This is the path an in-process read took to the carriers in a later chunk, and the measured cost shape for the names'-crossing fork.

@history_item n=43 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (eight causes) — closed `Cause` at eight by retiring `time-unavailable`. This is decision/refusal-causes-closed-at-eight. The scope adds no cause, and a refused declaration is not a driver refusal.

@history_item n=44 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — added the ungated `boundary` module to decision/accessibility-feature-gating's list. It also restated decision/driver-spans-only as "no global subscriber". The gating row lists dioxus-native-dom's gated modules (`snapshot`, `graph`, `actionable` …) and its ungated ones (`signal_registry`, `reached`, `seed`, `boundary`). A new declaration module in the bridge must take a side in this list.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the inline box's paint skip) — decision/node-holds-structure now allows a node one recorded style fact and one box-construction fact, with everything else on element data. Virtual-node state must not be added to `Node`.

@history_item n=46 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** — added decision/fork-additions-in-own-functions, on the founder's word: a fork addition to an upstream function over the complexity ceiling goes into escher's own function, with one call in upstream's body. This binds any edit to the `Widget` trait's surrounding code or to the bridge.

@history_item n=47 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — recorded the dev-host cognitive-complexity reading (rust-code-analysis-cli, ceiling 15, never in CI). This is decision/cognitive-complexity-ceiling. A new validation function is read against it.

@history_item n=48 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (std Result under the Dioxus prelude) — added decision/std-result-under-dioxus-prelude: a file glob-importing the dioxus_native prelude spells `std::result::Result`, a failure only the Android CI job saw. This is a trap for a declaration type in a prelude-globbing file (stand or app code).
