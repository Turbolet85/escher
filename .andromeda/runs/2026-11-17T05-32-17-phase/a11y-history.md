@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — set the name sources (trimmed `aria-label`, a `<label>` bound by `for`-id or nesting), made every Dioxus element node carry its stable id as `author_id` (no TextRun, root or Window node does), and routed the tree through `Document::accessibility_tree` so the wrapper's override reaches it. These are the rows behind `contract/accessibility-tree/author-id`, `decision/a11y-tree-through-document-trait` and `contract/accessible-name/label-element`. An isolated mount has to be built through `DioxusDocument` to get the stable ids, and a `<label for>` whose target sits outside the mounted component will not name it there.

@history_item n=2 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — made the snapshot take role, name and focus from the tree `accessibility_tree` returns, with no second mapping (`decision/snapshot-reads-accessibility-tree`). The in-app vs isolated reading equality P4 writes compares readings that come only from the tree, so any difference has to show up in the tree.

@history_item n=3 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the founder's actionable-key rule and `unkeyed_actionable` (`decision/actionable-key-rule`). Trap: never satisfy the check with a `tabindex` or a role, key the element. A component mounted alone must still read its actionable elements keyed. Its ids are anchored at the component rather than the app, so the check's reading of the isolated mount needs stating.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a falsy boolean attribute (e.g. `hidden: false`) is now removed so the node stays in the tree, and the snapshot reads `enabled`, `checked` and `value` from the element. This matters for props given to an isolated component that toggle `hidden`/`disabled`. It is the source of `contract/snapshot/enabled` and `contract/snapshot/checked`.

@history_item n=5 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree is refreshed on a poll whose drained changed set was non-empty (`decision/a11y-tree-refresh-on-change`, `decision/accessibility-tree-through-document-method`), and the diff is one more reader of the snapshot model. It was ratified at 2026-10-07-driver-session. An isolated document follows the same refresh path.

@history_item n=6 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — trap: the changed set is non-empty on a freshly booted harness document, and settle neither reads nor drains it. A check that reads `has_changes()` or a first diff on the isolated boot has to drain first.

@history_item n=7 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — stated the stand's role and name expectations once, in `tests/blitz-tests/tests/common/mod.rs` (`controls`, `INPUT_NAMES`, `rendered`), as expectation and never as a second mapping. A comparison of a component's isolated reading against its in-app reading should reuse these tables and not copy them.

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on `dioxus-native-dom` unconditionally, so the feature reaches seven_guis's binaries (`decision/driver-requires-accessibility`, `decision/dioxus-accessibility-feature-by-name`). An isolated-mount path inside the driver or seven_guis gets the snapshot readers through this edge. A new crate would get them only by naming the feature.

@history_item n=9 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` cause is read from the snapshot's `enabled` and never from the attribute value (`decision/enabled-by-disabled-presence`). An isolated mount reached through a driver carrier keeps this rule.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the tree states `selected`, `toggled` and `invalid` from ARIA attributes on every element, values trimmed and matched in lower case, and the snapshot takes them from the tree (`decision/aria-states-on-every-element`, `decision/aria-state-lower-case-closed-values`, `decision/boolean-state-limits`). These are among the "states" the isolated/in-app equality compares.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result is one more reader of the snapshot model, with no second mapping (`decision/snapshot-model-single-source`, `decision/readers-of-one-snapshot-model`). Any carrier the founder's fork opens for the isolated mount has to stay a reader of the same model.

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — added the node keys' help and `DiffNode::differs` as readers of the model, closed words only. They are among the readers that `decision/snapshot-model-single-source` binds.

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the screenshot became one more reader of the model's bounds and changes nothing. A password stays masked in every frame. This bears on "bounds where the component's box is the same" and on any shot of an isolated mount.

@history_item n=14 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the bridge removes a framework-written falsy value of 28 boolean attributes, `inert: false` included (`decision/inert-subtree-excluded`'s falsy clause). This holds for props that write `inert` or `hidden` in an isolated mount.

@history_item n=15 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — the `disabled` by presence entry made disabled-ness keyed one way, by the attribute's presence (`decision/disabled-by-attribute-presence`, `decision/disabled-by-presence`). It was the founder's «По наличию».

@history_item n=16 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the enabled-state walks entry: every interactive-role node of the lean tasks is named, and the `&dyn Document` tree equals the document's and carries stable ids only. This is the precedent for asserting tree equality across two readings of the same content.

@history_item n=17 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — `aria-labelledby` targets' text names first, outranking `<label>` and `aria-label` (accname 2B), and each ARIA attribute is read once (`decision/aria-labelledby-outranks-label`, `contract/aria-property-reads/aria-labelledby`, `decision/one-read-per-aria-attribute`, `contract/aria-property-reads/aria-disabled`, `aria-expanded`). Trap: in isolation, a labelledby target outside the component holds no text, so the name falls back to `aria-label` and can differ from the in-app reading.

@history_item n=18 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — two parts. In §2–§4 and §7, the toggle is read per role: `aria-checked` on the five checkable roles, `aria-pressed` elsewhere, and `role="switch"` → `Switch` (`decision/toggle-read-by-role`, `contract/aria-role-mapping/role-switch`). In §5 and §6, `inert` takes focusability, hits and the tree away from its subtree, and `disabled` is narrowed to form controls (`decision/inert-subtree-excluded`). The founder ratified both in the same marker's discharge entry.

@history_item n=19 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a `<label>` names every labelable element and a hidden input none (`decision/label-names-its-labelable-element`). A select's options stay in the tree under `display: none`, and the option's selected state is excepted from `aria-selected` (`decision/hidden-content-excluded`, `contract/accessibility-tree-states/select-option`).

@history_item n=20 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts` is one more reader of the model and is gated by `accessibility` (`decision/agent-readers-behind-accessibility`). An isolated component's snapshot crosses through the same readers.

@history_item n=21 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach, and the windowed AT witness is owned by a re-carried residual (`absence/assistive-technology-reach`). If an isolated mount is added to the capability statement, the unmeasured AT reading stays unmeasured.

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the served pages entry: Cells and the load page are booted by `escher-session` on their own boot paths (`boot_cells`, `boot_load`) and are not `LeanTask`s. `unkeyed_actionable` reads empty on every served task. This is the precedent for a document booted at something other than a lean task's root and for the rows such a boot owes. The same marker re-pinned the select's stand readings.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — several entries bear on this chunk. The tree's focus falls back to the window when the focused node was not built (`contract/accessibility-tree/focus`, `contract/accessibility-tree/root`). A native checkbox's or radio's checked state is written to the tree (`decision/native-checkable-toggle`, `contract/accessibility-tree/native-checked`, `decision/checked-reading`). A `multiple` select gets `set_multiselectable`, and its rows read their own client rects while a closed select's options read the select's rect (`contract/accessibility-tree/multiple-select`, `contract/snapshot/multiple-select-rows-and-value`, `contract/snapshot/closed-select-option-bounds`, `contract/snapshot/multiple-select-row-bounds`). `optgroup` reads `Role::Group` (`contract/accessibility-tree/optgroup`). All of these are bound to bounds equality in an isolated mount.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — a single select reads ComboBox and writes `expanded` from its list's open state, with no `aria-expanded` read (`decision/single-select-expanded-from-list`, `contract/accessibility-tree/single-select`, `contract/accessibility-tree/select-expanded`). An open list's row reads its painted bounds, and no `expanded` node key exists (`contract/snapshot/open-list-row-bounds`, `contract/snapshot/select-list-rows`, `absence/snapshot-list-open-state`). Option labels leave only as pixels (`decision/option-labels-leave-only-as-pixels`). Motion on the list is absent (`absence/select-list-motion`).

@history_item n=25 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — the listener-mark entry measured that the bridge's listener mark over-reports and that the vdom walk's dynamic attributes read each element's current listeners exactly. This is the basis of `decision/listeners-from-vdom-walk`, which an isolated mount's `clickable` and graph handlers rest on.

@history_item n=26 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the owner of the stand's platform adapter and AT witness (`res-llkgqygv`) now reads re-carried at 0.3.0 (`absence/assistive-technology-reach`). The buffered render's same-render limit is also relevant if an isolated mount is built from a vdom already rendered.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — this marker added three things. `NodeState.clickable` is read from the vdom walk (`contract/snapshot/clickable`, `contract/snapshot-node/clickable`). The read-only `DioxusDocument::graph` holds one node per snapshot node with owner and component, names and closed words only, and no served id holds the reserved `///` (`decision/ui-graph-one-node-per-snapshot-node`, `decision/ui-graph-carries-no-values`, `contract/node-id/reserved-separator`). Graph state is registered through `use_graph_state` with an ungated signal registry. This is the graph whose component names the isolated mount is keyed by and must name as its root's component.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — two entries matter here. The one actionable rule `Readings` is shared by `unkeyed_actionable` and `actionable_nodes`. The actionable-node reader's `keyed` is true where the id holds no `/`. Trap: ids anchored at a component path, with a prefix, change how `keyed` reads on the isolated mount. The `actions` reader changes nothing.

@history_item n=29 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the reached record and `element_listener_ids` are ungated, while `element_facts` and the graph reader stay behind `accessibility`, because the wasm build compiles dioxus-native-dom without it (`decision/agent-readers-behind-accessibility`'s ungated clause). `handler_coverage` is one more reader of the UI graph.

@history_item n=30 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows: 241 rows verified against the code. This is where every picked row's handle comes from, and from here on an amendment names a row by its handle.

@history_item n=31 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` went from provisional to standing on the founder's ratification. The `keyed` field the isolated mount's ids would be read by is settled.

@history_item n=32 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/changed-set-marks` now says a scroll that closes a list marks its select (this is what `decision/a11y-tree-refresh-on-change` reads). `+ decision/empty-alt-image-decorative` was added and ratified as upstream's behaviour.
