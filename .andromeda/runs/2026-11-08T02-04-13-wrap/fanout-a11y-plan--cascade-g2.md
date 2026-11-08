@proposal n=1 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-kf4qrij2 fact=F5 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-dwh3o3se
section: ## 7. Screen Reader Support
change: add beside it — since 2026-11-07-ui-graph `stand_graph` asserts in both layout modes that every graph node is a snapshot node, that no served id holds the reserved separator `///` (the reserved `{owner}///{key}` form, which no caller produces), and that a graph read changes nothing and holds no typed text — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
sidecar: screen-reader-support — F5 placed beside the author-id row: stand_graph's graph-node, reserved-separator and read-changes-nothing assertions
rationale: report Outcome (arch) states no served id holds `///` and Coverage states the graph answer carries no typed text; stand_graph is a new stand file
@proposal n=2 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-ge4n5uhw fact=F1 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## 7. Screen Reader Support
change: add beside it — since 2026-11-07-ui-graph `NodeState.clickable` is a `bool` after `focused`, true where the vdom walk's current listeners hold `click` and the element is no disabled form control (`ElementData::is_disabled`), read from the one snapshot model and the walk with no second role mapping; `to_text` writes the token `clickable` after `focused` only when true, the JSON node always writes `clickable` beside `focused`, and `Snapshot::diff` reads it as `Reading::Clickable` (`Reading::ALL` twelve) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
sidecar: screen-reader-support — F1 placed beside the snapshot-model row: the clickable field, its text token, JSON key and diff reading
rationale: report Symbols / APIs (the snapshot) and Coverage (the NodeState.clickable reading, a11y ✓ from the one snapshot model) state the new eighth state field
@proposal n=3 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-ge4n5uhw fact=F4 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## 7. Screen Reader Support
change: add beside it — since 2026-11-07-ui-graph `DioxusDocument::graph` (behind `accessibility`) is one more read-only reader beside the snapshot: a `UiGraph` with one `GraphNode` per snapshot node in pre-order (id, owner, component, handlers, reads) and per registered state its declared writes, confirmed or flagged writers and reading components; an option's or optgroup's `owner` is the stable id of its select; the answer carries ids, component names, state names, event names, closed flag words and booleans only — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
sidecar: screen-reader-support — F4 placed beside the snapshot-model row: the read-only UI graph over the snapshot's nodes
rationale: report Symbols / APIs (the graph reader) and Outcome (security) state the graph's shape and its names-only answer
@proposal n=4 detector=D-cascade severity=warning file=screen-reader-support anchor=fct-yfrcl4qo basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## 7. Screen Reader Support
change: No `expanded` node key exists: `NodeState` holds no such field and `NODE_FIELDS` reads fourteen (`clickable` joined after `focused` at 2026-11-07-ui-graph), so a list's open state reaches the tree and not the snapshot
sidecar: screen-reader-support — the no-expanded-key row's NODE_FIELDS count thirteen to fourteen
rationale: report Schema / config states `NODE_FIELDS` 13 → 14, and Counts / qualifiers moved (pattern node-keys) marks the a11y-plan rows stale
@proposal n=5 detector=D-cascade severity=warning file=screen-reader-support anchor=fct-pfylw46r basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 7. Screen Reader Support
change: replace its clause «no snapshot field says that an element reacts to a click (`NODE_FIELDS` thirteen) — that reading measured by link 2, its snapshot field owed to the route entry «The UI graph» …, this count its starting reading» with: that reading was measured by link 2 and owed to the route entry «The UI graph» on the founder's C3 (2026-10-14); since 2026-11-07-ui-graph it is the snapshot field `NodeState.clickable` (`NODE_FIELDS` fourteen), read from the one snapshot model and the walk's current listeners, not the listener mark, the mark's count staying 677 — residual `res-gvelrxja`'s disposition met
sidecar: screen-reader-support — the actionable-key row's not-built click field restated as built (NodeState.clickable, NODE_FIELDS fourteen)
rationale: report Counts / qualifiers moved, pattern click-field, marks screen-reader-support.md:46 stale: the reading is now NodeState.clickable and res-gvelrxja's disposition is met; Retired removes the matching «Not built» line from docs/capabilities.md
@proposal n=6 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-pfylw46r fact=F3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 7. Screen Reader Support
change: add beside it — since 2026-11-07-ui-graph `stand_non_lean_tasks` asserts a `CLICKABLE` table of the elements that read `clickable` and state no role, per task in both layout modes: Counter, Temperature Converter, Flight Booker, Timer and CRUD 0 each, Circle Drawer 1, Cells 676 — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
sidecar: screen-reader-support — F3 placed beside the actionable-key row: the CLICKABLE table per task
rationale: report Counts / qualifiers moved states the CLICKABLE table as new with no master site, the mark's count staying 677
@proposal n=7 detector=D-cascade severity=warning file=screen-reader-support anchor=fct-7vfy7k5v dependent_of=D-cascade basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 7. Screen Reader Support
change: keep the probe's measured readings as dated, and replace «no snapshot field is added (`NODE_FIELDS` thirteen)» with: no snapshot field was added at that chunk; since 2026-11-07-ui-graph the reading is the snapshot field `NodeState.clickable` (`NODE_FIELDS` fourteen)
sidecar: screen-reader-support — the probe's click-reading row states the field built since 2026-11-07-ui-graph
rationale: report Counts / qualifiers moved, pattern click-field, marks screen-reader-support.md:47 stale; Symbols / APIs states NodeState.clickable
@proposal n=8 detector=D-new-detail severity=warning file=screen-reader-support anchor=fct-7vfy7k5v fact=F2 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-e44heebj
section: ## 7. Screen Reader Support
change: add beside it — on the stand `clickable` reads true on `counter-increment` and on each of Cells' 676 cells, and false on `counter-value` and `cells-grid`, from the one snapshot model and not from the listener mark — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
sidecar: screen-reader-support — F2 placed beside the probe's click-reading row: the stand's clickable readings
rationale: report Outcome (acceptance) states clickable true on counter-increment and the 676 cells, false on counter-value and cells-grid; Outcome (a11y) states it comes from the one snapshot model
@proposal n=9 detector=D-cascade severity=warning file=screen-reader-support anchor=fct-cbzuzrms basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## 7. Screen Reader Support
change: add to it — since 2026-11-07-ui-graph the bridge's vdom walk yields each element's current listeners from those dynamic attributes (`element_listeners`, `mounted` included), the reading `NodeState.clickable` and the graph's handlers take, so neither over-reports as the listener mark does
sidecar: screen-reader-support — the listener over-report row gains the walk's current-listener reading as clickable's and the graph's source
rationale: report Symbols / APIs states the walk now yields current listeners read from VNode.dynamic_attrs / mounted_dynamic_attribute and clickable is true where they hold click; Coverage states not the listener mark
@proposal n=10 detector=D-cascade severity=warning file=ci-integration anchor=fct-bvkme5jk dependent_of=D-cascade basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 9. CI Integration
change: restate its lead as dated — `dioxus_runtime_probe` (10 tests, in both layout modes) measured link 2's «reacts to a click» reading (§7), built since 2026-11-07-ui-graph as `NodeState.clickable` and read by `stand_graph` and `stand_non_lean_tasks`; the probe is none of the a11y leg's fourteen files and carries no `stand_` prefix, so it runs in neither the `a11y` nor the `stand-repeat` leg — the workspace `test` leg, `run all` and `run dioxus_runtime_probe` reach it; the a11y leg and its job are unchanged
sidecar: ci-integration — the probe row's click reading stated as built since 2026-11-07-ui-graph
rationale: report Counts / qualifiers moved, pattern click-field, marks ci-integration.md:21 stale; Gates read the probe green with 10 passed and the a11y leg unchanged
@proposal n=11 detector=D-cascade severity=warning file=body anchor=fct-77ewqepn dependent_of=D-cascade basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 7. Screen Reader Support
change: replace «no snapshot field says an element reacts to a click — that reading measured by link 2, its field owed to the route entry «The UI graph»» with «the snapshot field `clickable` says an element reacts to a click, read from the walk's current listeners and not the listener mark (`NODE_FIELDS` fourteen)», the rest of the line kept
sidecar: body §7 — the actionable-key broad line states the click field built
rationale: report Counts / qualifiers moved, pattern click-field, marks a11y-plan.md:204 stale; Symbols / APIs states NodeState.clickable
@proposal n=12 detector=D-cascade severity=warning file=body anchor=fct-uz4kdss3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## 7. Screen Reader Support
change: add to the line — `clickable` from the vdom walk's current `click` listeners on an element that is no disabled form control; and the read-only UI graph (`DioxusDocument::graph`) holds one node per snapshot node, ids and closed words only
sidecar: body §7 — the snapshot-model broad line names clickable's source and the graph over the snapshot's nodes
rationale: report Symbols / APIs states clickable's source and the graph's one GraphNode per snapshot node; Coverage states no second role mapping
@remark
F6 (slug keyboard-navigation, use_graph_state and its declarations) is group 1's, not this group's; the Cells graph answer figure 293,869 B and residual res-7cc5bi4k have no picked entity here, so the new-detail pass places them
