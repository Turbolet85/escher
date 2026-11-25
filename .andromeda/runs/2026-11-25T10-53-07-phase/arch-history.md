@history_item n=1 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** — the entry this chunk inherits from. It added contract/accessibility-tree/virtual-nodes, contract/snapshot/virtual-node (clickable false, F12 A), contract/driver-verbs/click-virtual-node (only click takes a virtual id, F10 B), the "a virtual node excepted" carve-out on contract/ui-graph/graph (F9 A), and the virtual-node clauses of contract/refusal-causes/covered, /not-found, contract/snapshot/bounds and /membership. It also widened decision/canvas-placement-at-click-centre to a declared region, and replaced decision/virtual-node-declaration-marks-nothing with decision/declaration-change-marks-nothing. Every one of these carve-outs (F9 A, F10 B, F12 A) is a delegate PROVISIONAL answer that this chunk's CARRY owns and must lift or restate. The rows are what graph, actions, layout, coverage and explore must now cover.

@history_item n=2 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** — added contract/nodes-widgets/virtual-nodes, contract/dioxus-bridge/virtual-nodes (an ungated in-process read: NoCanvas or NotLaidOut, refusals by index) and contract/dioxus-bridge/virtual-node-refusal. It moved contract/stable-element-id/virtual-node-id and contract/dioxus-bridge/virtual-node-id to "produced in process". It also added decision/virtual-node-role-is-accesskit-role (PROVISIONAL). This is the read every new reader of virtual nodes in this chunk rests on. No closed role set of escher's own may be introduced when actions or graph classify a virtual node's role.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — built the UI graph reader (one GraphNode per snapshot node, owner from select_of, readers and writers rules). It also built the graph verb, decision/ui-graph-crossing (ratified: ids, owner ids, names, flags and booleans, never a value or NodeId), the reserved fifth id form {owner}///{key} with VIRTUAL_KEY_SEPARATOR and virtual_node_id, and the snapshot's clickable from the vdom walk's current listeners (never the listener mark). The graph's "one node per snapshot node" promise is the one this chunk extends to virtual nodes. stand_graph asserts that no served id holds `///`, a literal pin to sweep. The graph's owner semantics (select_of) must be given a meaning for a virtual node, likely its canvas.

@history_item n=4 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — graph answers in parts at the snapshot budget (states in part 1 alone), so contract/graph-verb/unpaged and decision/large-screen-in-parts now cover the graph. The reached record is resolved before dispatch and written after it, reading the stop (decision/reached-record-before-dispatch, decision/reached-recorded-before-handlers, contract/handler-coverage/bubbling). A virtual node added to the graph must page within those parts. Its coverage reach must fit the resolve-before, write-after record, which keys on chain nodes' current listeners, and a virtual node has none of its own.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — created the actions verb and decision/actions-answer-crossing (ratified; keyed PROVISIONAL, later ratified). It built the one actionable rule (Readings: focusable, interactive role, current non-mounted listeners) shared by actionable_nodes and unkeyed_actionable, contract/actionable-nodes over the UI graph's nodes, and Session::actions with the shared crate-private check_target and target_of. It also stated "Accepted is not acting" and the unpaged actions answer owned by res-7cc5bi4k. To list a virtual node in actions, extend actionable_nodes (which reads the graph, today virtual-free) and check_target, the one per-verb check the executor and actions share. keyed for a `///` id is a new question, since the id holds `/`.

@history_item n=6 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — ratified decision/actions-keyed-reading and decision/keyed-wire-form (keyed true where the id holds no /). A virtual id {owner}///{key} always holds `/`, so under the standing rule a listed virtual node would read keyed false and fall under unkeyed_actionable's remedy. The chunk must decide whether this ratified reading holds for the reserved form.

@history_item n=7 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — built handler_coverage and the coverage role (one fresh session per check, the graph read at boot and after each replayed step), plus the ungated reached record on the bridge's dispatch. It stated contract/handler-coverage/canvas, "Handlers inside a canvas are not counted", deferred to 0.4.0: the limit this chunk's coverage arm meets. The reached record reads element_listener_ids on chain elements, so a click on a virtual node reaches its canvas's listener, not a virtual handler.

@history_item n=8 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role's kept-check argv reading and its shape, endings and process (decision/coverage-reads-named-checks, decision/coverage-role-reads-named-checks, decision/coverage-role-in-process). A virtual node's reach in coverage must ride these ratified forms, with no new carrier.

@history_item n=9 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — added contract/reached-record/take-step-reached and narrowed decision/driver-library-side-effect-free and decision/driver-spans-only to "no global subscriber". The reached record's per-step drain is the coverage arm's input, and any virtual-node reach must stay inside these driver side-effect rules.

@history_item n=10 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — built BaseDocument::layout_reason (None for an unresolved id or a non-element), the closed PlacedBy, ShiftBy, SizeCause and Length sets, and Session::layout (resolves its id as the run path does, opens no span of its own). An answer for a virtual id cannot come from layout_reason, which takes a NodeId and has no element. The answer must be built from the declared region, inside the closed placing words or the fallback.

@history_item n=11 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — made layout the fifteenth verb on every carrier (decision/layout-answer-crossing replacing layout-read-in-process-only). It added the answer JSON layout rows (placing words with the `unexplained` fallback, contract/driver-session/layout-unexplained), layout as a target verb of actions with no cause, and excluded the layout reading kinds from contract/driver-explore/explored-verbs. A virtual id's layout answer must fit contract/answer-json/layout and contract/answer-json/layout-placing (perhaps placed by `app` or `unexplained` with canvas). The actions target-verb list (layout with no cause) changes once layout accepts a virtual id.

@history_item n=12 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirms of the layout-on-the-carriers rows. The layout rows this chunk amends are ratified, so a change to their promise is a founder-level amendment, not a delegate one.

@history_item n=13 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added explore, path expansion (one child per actionable node and explored verb), StepBudget, PathEnding and contract/driver-explore/explored-verbs (selected by result field kind, never name). It also added absence/exploration-on-carriers and absence/exploration-follows-navigation. Exploration's candidates come from the actionable list, so a virtual node becomes a candidate only through actions. Exploration stays in process only.

@history_item n=14 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — added the canvas verb and decision/canvas-crossing, made canvas the sixth target verb of actions ("canvas with none on a document that can paint text"), and extended explored-verbs to leave out the commands and command-counts kinds. The current target-verb list and its causes are the literal this chunk's actions arm extends. canvas reads a virtual id not-found under F10 B.

@history_item n=15 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — the lint reads a canvas element's labels as truncated by ink box (contract/lint-verb/truncated, contract/lint-verb/finding, contract/driver-verbs/lint). It added decision/canvas-label-region (PROVISIONAL) and absence/canvas-lint-checks-beyond-truncation. F9 A says the lint holds no virtual node. If this chunk makes the lint see virtual nodes, it must keep the canvas-label reading and the stated absence apart from per-node lint of virtual regions.

@history_item n=16 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added decision/widget-click-synthesis (no click walk and no focus clear on a custom-widget target, guard stand_canvas_click), decision/widget-click-point-content-box and decision/canvas-placement-at-click-centre. A click on a virtual node lands on its canvas through this synthesis. A coverage reach or an explored click of a virtual node is the canvas's click handler firing at a region point.

@history_item n=17 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — the founder ratified decision/canvas-placement-at-click-centre, a measured limit "until the drag verb". The virtual-nodes-as-elements widening of the row (a declared region) is still delegate-PROVISIONAL, which matters when this chunk restates it.

@history_item n=18 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — decision/custom-widget-content-box-paint: a widget paints at its content box, which is why a virtual node's region is moved by the canvas's content-box origin in contract/snapshot/bounds. Any layout or bounds answer for a virtual id must use the same origin.

@history_item n=19 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — NodeState gained highlighted (nine fields), NODE_FIELDS fifteen keys and Reading::ALL thirteen. These are the current shapes behind contract/snapshot/node-state and contract/answer-json/node. Lifting F12 A, a virtual node's clickable reading false, changes a snapshot-node reading inside these fixed shapes, not their count.

@history_item n=20 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — measured that the bridge's listener attributes over-report while the vdom walk reads the current listeners exactly, and that a node's owning component reads through Dioxus public API. These are the reasons behind decision/listeners-read-from-vdom and decision/owning-component-via-public-api. A virtual node has no vdom listeners or owning component of its own, so clickable, graph handlers and owner for it cannot come from these readers.

@history_item n=21 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the four-tier id grammar (an anchored path the only id holding `//`) and unkeyed_actionable. Trap: its NodeId must not cross. The reserved `///` form sits outside these tiers (contract/stable-element-id/tier-distinctness), and any reader that splits ids on `/` (keyed, unkeyed_actionable) must treat it apart.

@history_item n=22 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — set the rules that a refused call runs nothing, that visibility is read from geometry and never from a hit (decision/visibility-from-geometry-not-hit), and that a verb joins VERBS, never a second table. It also created the seen-ids record (decision/seen-ids-record-bounded). Lifting F10 B for type, scroll, screenshot or layout on a virtual id must reuse the one check order and record and read visibility geometrically.

@history_item n=23 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the one 'static table every driver surface reads; causes closed (contract/driver-session/command-schema, decision/refusal-causes-closed-at-eight). The scope says a new refusal cause would be a fork. Help texts, causes and describe() counts change only in the one table.

@history_item n=24 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — decision/validated-commands-only: nothing public runs a Command that did not come from validation, and nothing resolved outlives a call. Exploration and coverage acting on a virtual node must go through Session::run like every other call.

@history_item n=25 marker=2026-10-12-self-description
**2026-10-12-self-description** — describe() and the help tables are the program's fixed text on the ratified carriers (decision/fixed-program-text-on-carriers, decision/one-self-description-for-cli-and-mcp). Restated help texts (graph, actions, layout, click) and counts are literal pins this chunk sweeps.

@history_item n=26 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the capability-statement pin (CapabilityStatementTest) holds docs/capabilities.md's verb and cause names and five counts to the schema. The scope's literal-pin sweep includes this statement's virtual-node limits from the last wrap.

@history_item n=27 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — decision/capability-statement now also holds the a11y leg file count and the stand-repeat named-target count to the CI leg runner. A new stand target or a11y file this chunk adds moves those pinned counts.

@history_item n=28 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — decision/masked-input-values gained the guard stand_masks_on_new_answers: masked values stay masked on the graph, actions and layout answers, kept checks, explored paths and MCP. Adding virtual nodes to those answers must keep that guard green.

@history_item n=29 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer is kept whole and replay stops at the first differing answer (contract/check-replay/first-difference, decision/kept-check-replays-through-run). Making graph, actions or layout answers include virtual nodes changes kept answers, so the committed checks and stand_replay must be re-read.

@history_item n=30 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — kept checks are owner-only at rest and never logged (decision/recorded-session-checks, decision/kept-check-masking). A virtual id and name now at rest inside graph, actions and layout answers ride this ratified rule. They must not be logged or fielded.

@history_item n=31 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the session socket carries calls and answers (decision/socket-carries-driver-calls, ratified). Its standing rule: the next surface that carries an id, a name or a value asks again. A virtual name crossing in a new answer class is read against this and against the founder's F3 A exits.

@history_item n=32 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — serde_json is a reader only (decision/serde-json-reader-only), and no span field holds what a call supplied or a screen reads. New answer content (virtual nodes in graph and actions) is hand-written by the driver's own writer and never fielded.

@history_item n=33 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — decision/large-screen-in-parts: a screen over the budget crosses in parts with no new verb, cause or field. Virtual nodes added to graph or snapshot parts must page under it.

@history_item n=34 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — measured the overlay-scrollbar hazard behind decision/click-after-scroll-advances-past-fade (a click in a just-scrolled box needs an advance first). Explored or replayed clicks on virtual nodes inside a scrolled canvas meet the same hazard.

@history_item n=35 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — is_multiple_select and closed_select_of were re-exported from dioxus-native-dom because escher-driver has no blitz-dom dependency (is_masked_input the precedent). Any engine fact the driver needs about a virtual node (for example a region or a canvas test) must come through a dioxus-native-dom re-export, not a new driver dependency.

@history_item n=36 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — retired contract/driver-forms/is-select because "which function reads it" (locate or target_of in execute.rs) is the code's to hold. Precedent: a row naming execute.rs internals like resolve, locate or check_target is read as code-held mechanism, not a master row.

@history_item n=37 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — restated contract/driver-verbs/click (a drop-down opens its list, a list box's row is chosen) and closed_select_of. It is the most recent amendment of the click contract this chunk's virtual-id click arm sits beside.

@history_item n=38 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — `disabled` is read only on a form control (decision/disabled-read-on-form-control-only), read by the snapshot's clickable and the click walk. A virtual node's disabled is its declared flag, not this reading, so actions' disabled cause for it must come from the declaration.

@history_item n=39 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — standing rule: the snapshot derives no role or name of its own, so it cannot disagree with the accessibility tree. Virtual nodes enter the snapshot through the tree (contract/accessibility-tree/virtual-nodes), and new readers must keep taking role and name from it.

@history_item n=40 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — decision/accessibility-feature-gating: graph, actionable and snapshot are gated by `accessibility`, while reached, signal_registry, seed and boundary stay ungated (reached compiles on wasm with accessibility off). A virtual-node reach in the ungated reached record cannot call the gated virtual-node readers.
