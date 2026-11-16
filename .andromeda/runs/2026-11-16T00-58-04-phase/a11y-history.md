@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — wired dioxus-native-dom's `accessibility` feature and its `accessibility_tree` override, made the shell build and refresh the tree through `Document::accessibility_tree`, and gave every Dioxus element node its stable id as `author_id` (no TextRun, document-root or Window node). This is where decision/accessibility-tree-through-document-method and contract/accessibility-tree/author-id come from. A layout read addressed by stable id resolves through the same `author_id`, so it has to take the id the tree carries and not invent a second id scheme.

@history_item n=2 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — made the snapshot take role, name and focus from the accessibility tree, with no second role or name mapping, and added the snapshot to the `accessibility` feature's gated set. This is the precedent for every later reader: the new layout read reuses the snapshot's bounds (scope fct-tlegbc3w) and must add no role or name mapping of its own.

@history_item n=3 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the actionable-key rule (anything an agent can act on reads an author key) and its `unkeyed_actionable` check. Trap: never satisfy the check by adding a `tabindex` or a role, because that moves the pinned Tab order. A minimal layout fixture (flex child, grid item, positioned or transformed box) that adds controls must key them and leave focus order alone.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — made a falsy `hidden` stop dropping a node, so an element rendered `hidden: false` is laid out and stays in the tree and the snapshot, while a truthy `hidden` still excludes it. This bears on the hidden-content rows: an element the layout read is asked about may be laid out yet excluded, or excluded by `display: none`.

@history_item n=5 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — added `Snapshot::to_text` as one more reader of the snapshot model, put it behind the `accessibility` feature, and had the crate doc name each gated item. This is a precedent for how a new reader joins the gated set (decision/agent-readers-behind-accessibility) and for keeping the crate doc's list in step.

@history_item n=6 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the tree refresh on a non-empty changed set and stated what marks that set: node creation, hover, active, scroll and layout do not. The diff became one more reader of the snapshot model, with the `accessibility` feature gating it. This is the origin of decision/a11y-tree-refresh-on-change and decision/changed-set-marks. A read-only layout read must mark nothing, and a layout-only move marks no changed set.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — at the time, the session library `escher-driver` named no dioxus-native-dom feature, and the decision was left to the first entry that reads the snapshot through a session. The founder also ratified the refresh on change and the changed-set contract (2026-10-07). This is the history behind decision/driver-requires-accessibility and decision/a11y-tree-refresh-on-change.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set, so a check that reads the set as "what this step changed" must drain it first. This is the source of decision/harness-settle-leaves-changed-set (guard `stand_settle`). A check that the layout read "changes nothing" by reading `has_changes` after boot must drain first.

@history_item n=9 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's schema defines no role or name set of its own: `role` and `name` are field names of its result nodes. This is the source of decision/driver-schema-has-no-role-set. If the layout answer is a driver verb, its schema adds closed words (layout modes, placement kinds) without a role or name vocabulary of its own.

@history_item n=10 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — made `escher-driver` name `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table of its own, because its executor reads the snapshot. The feature reaches both seven_guis binaries, and still no platform adapter is built. This is the source of decision/driver-requires-accessibility. A layout verb in `execute.rs` can rely on the snapshot's bounds under that feature.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result reads the snapshot model and nothing else, with no second role or name mapping. This is the precedent for the scope's `mcp/` listing and results if the layout read becomes a verb on every carrier.

@history_item n=12 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the chunk that worked a scrolling box's bounds and hit. Its a11y amendment only recorded that a click on a plain button leaves nothing focused. It is a prior nearby change in the snapshot's bounds of scrolled boxes, which the layout read's "scroll that moves it from where layout put it" case extends.

@history_item n=13 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `DiffNode::differs` (closed words) and the node keys' help (`help`) as readers of the snapshot model. The Flight Booker's Book names nodes that only moved as `[bounds]`. This is a precedent for describing a bounds-only move in closed words, and for the self-description (`describe.rs`) that the scope says a new verb must extend.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — made the driver's `screenshot` one more reader of the model's bounds: an element shot by stable id is its bounds rounded outward and cropped to what can be seen. A shot changes nothing (no focus move, `has_changes` false, empty next diff). This is the source of contract/driver-screenshot/element-shot and decision/screenshot-changes-nothing, the model for the layout read's "changes nothing" proof and its bounds reading.

@history_item n=15 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast`, which reads an element's pair and font metrics from computed styles (backgrounds composited to the first opaque ancestor, `None` under opacity, filter or background image). This is a precedent reader of Stylo computed style per element, which the layout read's "inputs from the computed style" leg parallels.

@history_item n=16 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks entry) — proved both halves of the refresh headlessly on the stand, and proved that the `&dyn Document` tree equals the document's, carrying stable ids only. It also re-homed the stand's platform adapter and AT witness to a residual. This bears on decision/a11y-tree-through-document-trait and on absence/assistive-technology-reach.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6 entry) — an element carrying `inert`, with its subtree, leaves the tree and the snapshot, and a hit passes through it. The founder ratified this on 2026-10-18. This is part of the hidden-content rows (contract/accessibility-tree/hidden-and-inert-nodes): an inert or hidden element the layout read is asked about has layout yet no snapshot node.

@history_item n=18 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — `text_contrast`'s first non-test caller is the in-process lint `Session::lint`, and its `contrast` reading adds no second formula. This is the lint whose findings name an element but not its cause, the observed gap behind v030-11 (scope fct-o7ne25b4).

@history_item n=19 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — the founder ratified the `lint` verb on every carrier (F1). A finding is written as `id` · `defect` · `bounds` · `ratio` · `required`: ids, closed words and numbers only. This is a precedent for a new read-only verb's carriage as a boundary widening, and for the closed-words-and-numbers field form that the scope's widening rule (fct-l235qet4) uses.

@history_item n=20 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — an option under a `select` stays in the tree under `display: none` (it was excluded before), while `hidden`, `inert` and `visibility: hidden` still exclude it. This is the origin of the select-option exception in decision/hidden-content-excluded and its twins. A layout read of an option that computes `display: none` has no Taffy box of its own, yet the snapshot holds it.

@history_item n=21 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `Snapshot::text_parts` as a reader behind the `accessibility` feature, so the gated set grew to five. The driver's `snapshot` pages an over-budget screen (`SNAPSHOT_TEXT_BUDGET`). This is a precedent for a reader joining the gated set, and for bounding an answer's size if the layout answer can grow large.

@history_item n=22 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach under `## Accessibility reach` (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, AT reading unmeasured). Its pin reads none of those claims, so a change to the reach is made by hand. This is the source of decision/accessibility-reach-tier-0 and decision/capability-statement-reach-by-hand. The scope edits `docs/capabilities.md` and its pin `.github/scripts/test_capability_statement.py` for a new verb, and any reach change there is hand-written.

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages entry) — the snapshot gives each closed select's `ListBoxOption` its select's client rect, which made the lint read a false `overlap`. It also recorded the overlay scrollbar's band right after a `scroll`. Trap: a closed select's options read borrowed bounds and not their own layout, so a layout answer for an option must not present the select's rect as the option's own placement.

@history_item n=24 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (CSS properties as `CSSStyleDeclaration` accessors) — a script's `element.style` read or write of `display` or `visibility` goes through the inline style and reaches the tree's hidden-content exclusion. The inline style is one more input to the computed style that the layout read traces.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree, the snapshot and the form values) — narrowed borrowed bounds to a closed select. A `multiple` select's rows and groups read their own client rects, and its `value` reads its first chosen option in tree order. This is the source of contract/snapshot/multiple-select-rows-and-value, contract/snapshot/multiple-select-row-bounds and contract/snapshot/closed-select-option-bounds, which the layout read's "bounds the snapshot already reads" inherits.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — widened the `display: none` exception to every option of a select, as its child or under its `optgroup` (`is_select_option`), with `hidden`, `inert` and `visibility: hidden` still excluding it. This is part of contract/accessibility-tree-hidden-content/display-none.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and chosen-row pair) — in the user-agent layout, a single select's group reads `display: contents` and its chosen option `display: inline`. A `multiple` select lays out its groups and rows as blocks. These are user-agent-sheet layout modes a "why it sits there" answer will meet on a select.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — `overlaps` skips a closed select's options through `Entry.in_closed_select`, which reads the element (`closed_select_of`) and never the role. This is a precedent for telling a select's borrowed bounds from a real placement, and is part of the lint's link (scope fct-o7ne25b4).

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — a transform whose determinant is 0 or not finite takes no hit for the whole subtree (`hit_non_invertible_transform`). This bears on the layout read's transformed-box case (scope fct-tlegbc3w, fct-bdtm4oct).

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a single select's list opening or closing marks the set, and with a list open a page scroll or a move that closes it marks its select. A highlight or a wheel move inside the list marks nothing. This is the history of decision/changed-set-marks, which the read-only layout read must not disturb.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — an open list's row reads its painted row's bounds through `select_list_bounds` in the snapshot and the diff, and `differs` names `bounds`. No `expanded` node key exists, and the highlighted row is in no snapshot and no tree. The answer card ruled a row's own `bounds` a closed set inside the ratified crossing, by the field rule of 2026-10-10. This is the source of contract/snapshot/open-list-row-bounds, contract/snapshot/select-list-rows and absence/snapshot-list-open-state. An open list's row is placed by the engine's overlay and not by Taffy, so it is another placement kind outside the box tree.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a long list and a sized `multiple` select) — a row brought into view writes the list's own `scroll_y` and moves no scroll chain or viewport. On a `multiple` select with a valid `size`, a key scrolls through `scroll_into_view`. This bears on the layout read's scroll-offset leg.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — the list paints whole past an `opacity` ancestor and past a box that clips its select, and a driver `click` under it is refused `covered`. The list is an overlay placed outside author layout, which the answer's open placement kind may need to name.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list) — `Entry.in_select_box` reads an open row through `select_list_bounds`, and `clip_region` takes an open row's region from `visible_region` alone. This is a precedent for the lint's reading of placement and clipping, which the lint's link to the layout read touches.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list has no motion) — motion on an open list is observed absent and needs no `advance` (`a_screen_read_after_a_step_reads_the_new_state_with_no_advance`). This is the source of absence/select-list-motion.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — the named limits in `docs/capabilities.md` under `## Accessibility reach` were edited by hand, and the pin still reads none of them. This is a precedent for decision/capability-statement-reach-by-hand when this chunk edits the capability statement.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a key press past a dropped anonymous block) — `BaseDocument::deallocate_anonymous_block` sets `layout_parent` to `None` on each child still naming the block (`key_press_after_container_lost_its_box`). Trap: a node's layout parent can be absent or an anonymous block, so walking the layout chain to name a containing block or parent mode must handle a `None` layout parent and anonymous boxes.

@history_item n=38 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — added a snapshot field: `NODE_FIELDS` went from 13 to 14 and `Reading::ALL` from 11 to 12. This is a precedent for what adding a field to the snapshot or diff costs: the text token, the JSON field, the diff reading and the help. If the layout read stays a separate verb, the snapshot's fields stay as they are.

@history_item n=39 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader DioxusDocument::graph) — the graph is read-only and gated, and carries names and closed words only, never a value. `stand_graph` asserts that no served id holds the reserved `{owner}///{key}` separator. This is the source of decision/ui-graph-carries-no-values and contract/node-id/reserved-separator, which the scope's open placement kind follows («placed by the app inside canvas X», reserved like `///`).

@history_item n=40 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — added `DioxusDocument::actionable_nodes` to the `accessibility` feature's gated list, re-exported from the crate root. This is the latest example of a read-only reader joining decision/agent-readers-behind-accessibility.

@history_item n=41 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — `actions` changes nothing (no focus, snapshot or diff moves, and an open list stays open) and writes ids, verb and cause words and booleans only. The founder ratified its crossing onto every carrier as a new verb («Новый глагол actions»). This is the source of decision/actions-verb-changes-nothing and the scope's direct precedent for a new read-only verb as the founder's widening.

@history_item n=42 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading, provisional on the carriers) — a new field on the carriers, decided by the founder's delegate, stood PROVISIONAL until the founder's own word. This is a precedent for a fork that is the founder's while he is away (scope fct-l235qet4), and for how it is marked.

@history_item n=43 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable counts at boot and the Cells answer's bound) — the unpaged `actions` answer on Cells is 106,088 bytes, a stated limit owned by residual `res-7cc5bi4k`. Trap: a per-element read is bounded, but any answer that lists many nodes can reach the same size on Cells.

@history_item n=44 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record on handle_event's path) — the reached record is ungated because the wasm build compiles dioxus-native-dom without `accessibility`, and `element_facts` stays gated. Constraint: a layout reader placed in dioxus-native-dom behind `accessibility` is absent from the wasm build, so the gating decision must be made deliberately.

@history_item n=45 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the a11y master's text into rows (241 rows), each verified against the code. This is the origin of every picked row's handle and of the plan's absences and overviews that this chunk extends.

@history_item n=46 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` went from provisional to standing on the founder's ratification of the actions answer's `keyed` field. This shows how a delegate-made carrier field is later ratified, which applies to any closed-word field this chunk adds.

@history_item n=47 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks`: "scroll does not mark" was qualified to "any other scroll", because a scroll that closes a list marks its select. This is the current wording of the picked row, which the layout read leaves untouched.
