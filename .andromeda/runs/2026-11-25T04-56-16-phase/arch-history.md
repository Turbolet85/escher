@history_item n=1 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** (Standard Contracts: 5 rows) — landed `contract/nodes-widgets/virtual-nodes`, `contract/dioxus-bridge/virtual-nodes`, `contract/dioxus-bridge/virtual-node-refusal` and re-said `contract/stable-element-id/virtual-node-id` and `contract/dioxus-bridge/virtual-node-id`: `{owner}///{key}` is given in process only to a canvas's taken nodes, "no carrier serves it". This chunk's whole build sits on that read. Putting virtual nodes in the snapshot and its carriers makes "no carrier serves it" false, so both id rows need re-wording. `VirtualNodeDeclaration` gains `disabled` (F8 A), which amends the declaration contract. The three added rows are PROVISIONAL on a delegate's word.

@history_item n=2 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** (Established Decisions: 2 rows) — added `decision/virtual-node-role-is-accesskit-role` and `decision/virtual-node-declaration-marks-nothing`, both PROVISIONAL. The second hands the changed-set mark and the invalidation signal to "the first reader that refreshes", which is this entry. F11 A (the founder's word) keeps the absence: no mark and no signal, with readings current because the snapshot rebuilds the tree on each read. So the row's "yet / first refreshing reader" wording and its provisional state must be settled here.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — reserved the fifth id form `{owner}///{key}` beside the four tiers, spelled by `VIRTUAL_KEY_SEPARATOR` and `virtual_node_id`, and it rested on a measured "no served id holds `///`". `stand_graph` asserts that no served id holds `///`. Under F9 A the graph keeps that true, but a snapshot assertion of the same kind now inverts.

@history_item n=4 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph reader; the graph crosses the ratified carriers) — `DioxusDocument::graph` gives one `GraphNode` per snapshot node in pre-order. This is why a virtual node put in the snapshot reaches the graph unless it is excluded (F9 A, `graph.rs`). The graph's crossing list (ids, names, closed flags) is the precedent this chunk's widening follows.

@history_item n=5 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's `clickable` reading) — added `NodeState.clickable`, read from the vdom walk's current listeners and an `is_disabled` form control. A virtual node has no listeners, so under F12 A its `clickable` reads false, a stated limit. The Reading/NODE_FIELDS growth here is the precedent for the closed-bool shape of any new reading.

@history_item n=6 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — set snapshot membership: a node is an accessibility-tree node with an `author_id` whose element resolves through `get_node`, and each field comes from an existing reader. The standing rule is that the snapshot derives no role or name of its own. This is the reason a virtual tree node reads nothing until the builder gets its own arm (scope premise `snapshot.rs:164-179`), and the arm must take role and name from the tree, not re-derive them.

@history_item n=7 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — `DioxusDocument`'s `accessibility_tree` override sets each element's AccessKit `author_id` to its stable id, so the id reaches both the harness tree and the shell's platform tree. That override is where virtual children of a canvas node and their ids get added. The trap it recorded: a feature-gated item can compile out under `-p blitz-tests`.

@history_item n=8 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the four-tier grammar and its distinctness, where only an anchored path holds `//`. `contract/stable-element-id/tier-distinctness` must still hold once `{owner}///{key}` ids are served. Its trap: `UnkeyedActionable.node` is a process-local `NodeId`, so no wire form may carry it.

@history_item n=9 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — added the changed set and its refresh-then-drain rule (ratified 2026-10-07), with the diff explicitly not derived from it. This is the basis for F11 A: a virtual node's add/remove/change shows in the diff because the diff compares two snapshots, while the changed set stays unmarked (`decision/diff-not-engine-changed-set`, `decision/changed-set-refresh-then-drain`).

@history_item n=10 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Cross-cutting Patterns) — `~ decision/changed-set-write-sites` gained its select-list exception. This is one of the ratified rows a declaration mark would amend for every document (CARRY `cry-t4af3d26`), which F11 A declines.

@history_item n=11 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (Cross-cutting Patterns: 2 rows) — `~ decision/changed-set-write-sites` gained "moving its highlight to a new row" as a write site, ratified. It is the latest precedent of a non-element reading (a list row) being added to the closed write-site list, which is the step this chunk deliberately does not take for virtual nodes. Its Standard Contracts entries added the `highlighted` node key and the tree's active descendant, a closed-bool precedent inside the ratified carriers.

@history_item n=12 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Cross-cutting Patterns: 2 rows) — `~ decision/changed-set-write-sites` except gained "a focus given up after a layout marks its node". This is the row's current text, the one any amendment here must quote. The chunk also added the post-layout focus fixup to `contract/event-dispatch/focus-change` (no events) and extended `decision/hit-focus-form-engine-changes`.

@history_item n=13 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Standard Contracts: 3 rows; Cross-cutting Patterns: 4 rows) — moved the platform-tree refresh to the redraw after resolve, gated on a non-empty changed set (`decision/accessibility-tree-refresh-on-change`, `contract/accessibility-tree/platform-refresh`, `decision/a11y-rebuild-on-nonempty-changed-set`). With F11 A's no-mark, a declaration change alone does not refresh the platform adapter's tree. That is a consequence to state now that virtual nodes ride the adapter exit. The same entry added `decision/events-queued-on-document`.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else the window (`contract/accessibility-tree/focus`). This is one of the three tree-id decoders F7 A says must check the virtual map first. An even-versioned virtual tree id must never be decoded as a DOM `NodeId` here.

@history_item n=15 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — built the refusal order for click/type: `stale` when the session's record holds the id, else `not-found`, then `disabled` · `off-screen` · `covered`. It also added `SeenIds` (`decision/seen-ids-record-bounded`), the click point at the centre of the snapshot's `bounds`, and the rules "a refused call runs nothing" and "visibility read from geometry, never from a hit". This chunk runs a virtual id through exactly this order, and the record must hold virtual ids for `stale` to fire.

@history_item n=16 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` is the one entry point that runs a call. It validates first and resolves ids through `element_ids()` and `get_node`, and nothing resolved outlives a call (`decision/validated-commands-only`). The virtual arm for `click` (F10 B) must live inside this run path, not beside it.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read and the shared per-verb target check) — introduced the crate-private `check_target`, the one per-verb target check called by the four id-taking arms and by `Session::actions`, with `locate` ending in `target_of`. A virtual click target has to pass through it for the disabled/off-screen/covered order to hold "in the run path's one order". `actionable_nodes` reads the UI graph's nodes, so the graph exclusion (F9 A) also keeps virtual nodes out of `actions`.

@history_item n=18 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — causes closed at eight (`decision/refusal-causes-closed-at-eight`), a refusal holding nothing a call supplied, one verb table with no second one. This chunk adds no cause, so a virtual node's limits must be said through the existing causes' fixed texts.

@history_item n=19 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries a call and its answer) — the founder-ratified widening that let ids, names, values, snapshot text and diffs cross the socket (`decision/socket-carries-driver-calls`). F3 A puts virtual names and keys on exactly these exits, with the same "a new surface carrying content asks again" discipline.

@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (`type` replaces; two engine limits measured and said by the tool) — the operator's rule that an agent learns the tool from the tool, so a measured limit is stated in the fixed help and cause texts (`decision/measured-limits-stated-in-the-tool`). F10 B (only `click` resolves a virtual id) and F12 A (`covered` at the region centre only; `clickable` false) are stated limits that must be written there.

@history_item n=21 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the ratified MCP crossing (`decision/mcp-stdio-crossing`): the snapshot text and diffs reach an MCP client. With this chunk, virtual ids and names cross it too.

@history_item n=22 marker=2026-10-12-self-description
**2026-10-12-self-description** — fixed program text rides the ratified carriers (`decision/fixed-program-text-on-carriers`), and `differs` crosses as a closed set (`decision/differs-closed-set`, `Reading`). New stated-limit text for virtual nodes counts as fixed text, but page or canvas text does not, which is why F3 A is a widening.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening) — the closest precedent: a non-element reading, the select list's rows, crossing the ratified carriers only as bounds and closed values, with page text as pixels named a ratified widening (`decision/select-list-crossing`). Virtual nodes follow the same pattern, but with author-written names as text.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's exceptions to scrolling and the hit; the bridge's select tests) — `hit_with_scrollbar` answers the list first, and `visible_region` answers the list's scrollport for a row. The snapshot's `bounds` reads `select_list_bounds` before the client rect. This is the precedent for giving a sub-element thing its own bounds and visible region in the snapshot and refusal checks, as a virtual node's region moved by the canvas content-box origin.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (visible_region through transforms) — `visible_region` reads each clipping box through its transforms, and under a rotation it reads the bounding box. `off-screen` for a virtual node is the region's centre against the canvas's `visible_region` (scope premise), so it inherits that rotation limit.

@history_item n=26 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the geometry readers) — the client rect is read through transforms (`decision/client-rect-under-transforms`, `decision/transformed-element-bounds`), and `covered`'s meaning gained the zero-size reading. A virtual node's bounds must be its region mapped through the canvas's transforms, consistent with these readings. The batch's `ElementData::is_disabled` and `disabled` on form controls alone (`decision/one-disabled-reading`) are what a declared `disabled` on a virtual node stands beside.

@history_item n=27 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a box's own scroll is never subtracted from its client rect (`decision/client-rect-own-box`), and a hit stops at the padding box of a box that clips (`decision/hit-stops-at-clip-padding-box`). These are the geometry rules a virtual node's page bounds and its `covered` reading at the region centre rely on.

@history_item n=28 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `covered`'s meaning gained its one-axis clip limit (`decision/one-axis-clip-readings-kept`). This is the precedent for adding a stated limit to `covered`'s fixed meaning. F12 A's "DOM coverage of the region's centre only" goes there.

@history_item n=29 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — `decision/custom-widget-content-box-paint`: a widget paints in its content box. A virtual node's region is in content-box coordinates, so its page bounds are the region moved by the canvas's content-box origin.

@history_item n=30 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `decision/widget-click-synthesis` (the engine synthesises click on a custom-widget target with no click walk or focus clear), `decision/widget-click-point-content-box`, the `decision/click-focuses-button` exception, and `decision/canvas-placement-at-click-centre` (ratified 2026-11-24). A virtual `click` lands as a pointer at the region's centre, and the widget reads which node was meant (scope premise). This is the path it rides.

@history_item n=31 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (Established Decisions) — retired `decision/canvas-crosses-as-pixels` for `decision/canvas-crossing` and added "canvas readings" to `decision/no-node-id-across-session` and canvas answers to `decision/kept-check-never-logged`. This is the latest state of the closed crossing list that F3 A's virtual-node names and ids must fit (or extend).

@history_item n=32 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (Standard Contracts: 4 of 4) — added `contract/driver-session/canvas` and `contract/target-check/canvas`. `Session::canvas` resolves its id as `Session::layout` does. Under F10 B, `canvas --id` on a virtual id reads `not-found`, a stated limit to write into the canvas verb's help.

@history_item n=33 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the layout read resolves its id "as the run path does" (`contract/driver-session/layout`, `contract/driver-verbs/layout`). F10 B keeps `layout --id` on a virtual id `not-found`, so the virtual arm in the shared resolve must be `click`'s alone.

@history_item n=34 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the `screenshot` verb) — `screenshot` given an `id` resolves it as `click` does and then crops (`contract/target-check/screenshot`). Under F10 B a virtual id must read `not-found` here, even though `click` resolves it.

@history_item n=35 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — the lint reads a canvas's labels by ink box (`contract/lint-verb/truncated`, `contract/driver-verbs/lint`; PROVISIONAL) and records the absence of other canvas lint checks. F9 A has the lint (`lint.rs:238`, which reads the snapshot) exclude virtual nodes, so these canvas findings must stay per label, not per virtual node.

@history_item n=36 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `contract/driver-explore/expansion`: a path gains one child per actionable node and explored verb, and exploration digests the snapshot (`explore.rs:192`). F9 A excludes virtual nodes from exploration until «Virtual nodes in every read».

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage reader and its stated limits) — "handlers inside a canvas not counted, deferred to 0.4.0" is a stated limit. Coverage reads the UI graph, which F9 A keeps free of virtual nodes, so the limit stands.

@history_item n=38 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a screen over `SNAPSHOT_TEXT_BUDGET` crosses in parts (`decision/large-screen-in-parts`, `decision/snapshot-text-budget`). Virtual nodes add snapshot lines, and a part read must record virtual ids the same way.

@history_item n=39 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — kept checks hold each answer whole at rest: ids, names, snapshot text and diffs, owner-only and never logged (`decision/recorded-session-checks`, `decision/kept-check-never-logged`). F3 A names kept checks among the exits virtual names and keys now reach. The two committed Counter checks must stay byte-fresh.

@history_item n=40 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Occupied Resources) — `decision/cold-agent-run-carries-stand-screen` was last re-stated to list what the live run sends to the model provider. F3 A names a cold-agent live run's model provider as one more exit for virtual names and ids. Check whether the row's list must change.

@history_item n=41 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `decision/capability-statement`: `docs/capabilities.md` names and counts are pinned to the schema and CI legs. The scope lists `docs/capabilities.md` among the surfaces F3 A reaches, and the new stated limits (F10 B, F12 A) belong there.

@history_item n=42 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the accessibility builder reads three state attributes; the adapter's third class) — set the rule that adapter crossings carry states as closed enums and booleans, never an attribute's value (`decision/adapter-crossing-carries-states`, ratified). A virtual tree node's `disabled`, written from the declaration (F8 A), must keep that closed shape on the adapter.

@history_item n=43 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Cross-cutting Patterns) — `decision/accessibility-feature-gating` lists the gated modules (`snapshot`, `snapshot_diff`, `snapshot_text`, `actionable`, `graph`) and the ungated ones. The previous chunk's `virtual_nodes` read is ungated, while the tree override and snapshot arm are `accessibility`-gated. Any new module must be placed on the right side of this row.

@history_item n=44 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node) — `decision/bridge-removal-drops-subtree`: a removed node is dropped and no longer resolves. A canvas removed or re-rendered stale makes its virtual ids resolve to nothing, the `stale`/`not-found` path for a virtual id (`NoCanvas` from the read).

@history_item n=45 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip) — `decision/hit-and-paint-skip-dropped-nodes`: a hit and the paint walk skip a node dropped since the last resolve. This bears on `covered` read at a virtual region's centre right after its canvas changes.
