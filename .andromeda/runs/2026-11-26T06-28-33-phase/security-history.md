@history_item n=1 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings inside the ratified carriers; the closed-field rule) — added `selected` / `pressed` / `invalid` as closed words in the snapshot text and booleans in the diff, and set the founder's rule that a new field of a closed value set inside a ratified carrier is no widening (decision/closed-value-field-no-widening, decision/boundary-widening-rule, decision/snapshot-state-readings). Why it matters: the founder's B1 has `VirtualNodeDeclaration` gain `selected`, read as the snapshot's existing `selected` with a diff naming `differs: [selected]`. That reading rests on this rule. A field holding authored text or a new carrier would still go to the founder.

@history_item n=2 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (a Markup attributes row for the three control states; the adapter crossing's third class) — control states became the third content class of the platform accessibility adapter crossing, as closed enums and booleans only, never the attribute text (decision/control-states-closed-values-only, decision/accessibility-adapter-content-classes). Why it matters: writing a circle's `selected` on its virtual accessibility node puts a control state on the adapter crossing for a node that has no element. It must stay a closed boolean in the states class.

@history_item n=3 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — ratified keyboard activation as an engine change for every document and a boundary widening: Enter keydown and Space keyup dispatch a click only on a closed set of element names and types, gated by `disabled` presence (decision/enter-activates-focused-control, decision/space-keyup-activates-focused-control, decision/disabled-read-by-presence). Why it matters: a focused canvas is in none of those sets. Keys acting on the current circle belong to the app's own key handler. Widening the engine's Enter or Space arms to a canvas or a virtual node would be a new engine-wide change for the founder.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' `cells-grid` keys) — the precedent for this chunk's design: an app container takes `tabindex` 0 and an `onkeydown` with a closed key match and no role; arrows move `current`, clamped, and Enter opens the editor. Ratified on the founder's F13 (decision/cells-grid-keyboard). Why it matters: the Circle Drawer canvas taking a Tab stop and a key handler that moves a current circle follows the same app-level shape. It is the pattern to reuse, and so is its early-return trap on the pointer path.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus an inert subtree loses, and the tree's `focus`) — the tree's `focus` is written only where `focus_node_id` is among the built nodes, else the window, so the tree never names a node it did not build. The inert focus clear fires no `blur` / `focusout` (decision/accessibility-focus-built-nodes-only). Why it matters: the scope's open question of how the tree's focus names a virtual node while the engine holds the canvas runs straight into this row. Making a virtual node the tree's focus is an engine change for every document behind the feature, so it goes to the founder.

@history_item n=6 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — Cells' current data cell alone states `aria-selected` true (decision/cells-grid-keyboard), and a fourth closed reading, `highlighted`, joined the snapshot and diff for an open list's current row inside the ratified carriers (decision/snapshot-state-readings, decision/select-list-crossing). Why it matters: there is precedent both for a keyboard "current" item stated as `selected` and for a new closed state crossing without a widening. If the current circle needs a reading apart from `selected`, this is the path it would take.

@history_item n=7 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Input Validation: 5 row(s)) — a programmatic focus change now fires blur, focusout, focus and focusin (decision/programmatic-focus-events). The tree refresh moved to the redraw after style resolution, clearing the changed set (decision/accessibility-tree-refresh-on-change). Ratified 2026-11-22 (2026-11-21-dependency-and-ci-debts). Why it matters: focusing the canvas and moving a current circle must keep these focus events and the change-driven tree rebuild consistent, so the engine, the tree and the snapshot agree on focus.

@history_item n=8 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Input Validation: 3 row(s); Error Handling: 1 row(s)) — added a post-layout silent focus clear for a node that no longer shows, now named in the programmatic-focus-events exception list. A key press reaches a re-focused control whose anonymous block was dropped (decision/programmatic-focus-events, decision/dropped-anonymous-block-clears-parent). Why it matters: the canvas's focus is subject to these silent clears. Any new focus path for a virtual node must name its event behaviour against this exception list.

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — reserved `{owner}///{key}` through `virtual_node_id`, a stable-id contract change ratified on the founder's P4 fork 2 (contract/stable-element-id/virtual-node-form). Why it matters: this is the origin of the id that the current circle's focus and selection readings will name on every carrier.

@history_item n=10 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** — added decision/virtual-node-declarations-validated (a closed refusal set over app-authored declarations, no panic path; still PROVISIONAL on the escher overseer's delegate word) and the retired absence/virtual-nodes-on-carriers. Why it matters: `VirtualNodeDeclaration` gains `selected` (and possibly a focus field) here. That extends this provisional validated surface, so the new field must keep the closed refusal set and carry no authored text.

@history_item n=11 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (Input Validation: 3 row(s) (1 of 2)) — virtual nodes are served in the accessibility tree with author id `{owner}///{key}`, and from the tree in the snapshot and diff (decision/author-id-on-accessibility-nodes, contract/stable-element-id/virtual-node-form, contract/stable-element-id/no-collision). Why it matters: the circle's `selected` and any focus reading ride this tree-to-snapshot path. The shared reader `focused` asserts the snapshot's focus and the tree's focus agree.

@history_item n=12 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (Input Validation: 1 row(s) (2 of 2); Error Handling: 1 row(s)) — a click on a virtual node is refused disabled, off-screen, covered in the fixed order through the one target check (decision/target-refusal-order, PROVISIONAL on F10 B). `covered`'s fixed meaning says a virtual node reads covered only by an element over its region (contract/refusal-causes/covered, F12 A). Why it matters: this chunk owes the stand's `covered`, `off-screen` and `disabled` readings on a circle. The F4 B question (a later sibling virtual node read as covering) would change `covered`'s fixed meaning and this contract.

@history_item n=13 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (Threat Model Summary: 1 row(s); Input Validation: 1 row(s)) — added decision/virtual-node-crossing (a virtual node's id and declared name cross the tree, snapshot, diff, MCP, kept checks and the answer line, a ratified widening) and removed absence/virtual-nodes-on-carriers. Why it matters: a circle's `selected` reading and focus cross inside this ratified carrier set. A new carrier, or the name in a read that holds only the id, would widen it.

@history_item n=14 marker=2026-11-25-virtual-nodes-in-every-read
**2026-11-25-virtual-nodes-in-every-read** — the graph, actions and layout answers carry a virtual node's id and never its name (decision/virtual-node-crossing on F13 A). contract/actions-answer/verbs says a listed virtual node reads not-found on every verb but click and layout (PROVISIONAL, delegate). Why it matters: `press` on a focused canvas acts on the element, and a virtual node takes no `press`. If keyboard reach adds a verb to virtual nodes, it changes this provisional contract and the actions answer.

@history_item n=15 marker=2026-11-25-circle-drawer-virtual-nodes
**2026-11-25-circle-drawer-virtual-nodes** — added decision/canvas-answer-node-index (each canvas command line carries only the 1-based node index, never the id or name). The coverage answer was added to the reads carrying the id (decision/virtual-node-crossing, contract/stable-element-id/virtual-node-form). Why it matters: this is the prior chunk, where F1 A has a stand click still adding a circle. B1's selection repaints the chosen circle and moves the canvas pins, and `GRAPH_DRIVES` in `stand_handler_coverage` is re-read under B1.

@history_item n=16 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added decision/custom-widget-click-synthesis: behind `custom-widget`, the engine synthesizes click, context menu and double click on a widget after a pointer event, with no click walk and no focus clear. Why it matters: Circle Drawer's `onclick` and `ondoubleclick` reach the canvas through this path. Keyboard reach of the diameter dialog (today reached by a double click) must not lean on a widened synthesis without a new ruling.

@history_item n=17 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — added decision/custom-widget-content-box-frame: a widget is painted and pointed in its content box, and a sub-document in its border box. Why it matters: virtual-node regions and the `off-screen` and `covered` hit readings on a circle are read in this frame. A rotated or scaled canvas is out of scope («Pan and zoom»).

@history_item n=18 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (Threat Model Summary: 4 row(s); Data Protection: 1 row(s)) — replaced the pixels-only crossing with decision/canvas-content-crossing (closed words, numbers and digests on every carrier) and added decision/kept-check-canvas-answer. contract/actions-answer/verbs gained canvas. Why it matters: a selected circle painted filled changes the canvas answer's digests and the kept-check replay. That is a literal-pin sweep, and it must not add text or a glyph id to the answer.

@history_item n=19 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** (Dependency Security: 1 row(s)) — added decision/harness-blitz-paint-custom-widget: the harness turns on blitz-paint's custom-widget feature in every build selection, with no lock package added. Why it matters: the canvas shots and stand pins this chunk re-reads depend on that feature set staying as it is.

@history_item n=20 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — added decision/os-drag-and-drop-untrusted (Threat Model) and decision/dropped-file-read-unrestricted (Input Validation), both provisional at first. Why it matters: CARRY `cry-d3tqol54` names exactly these two rows (`dec-5vvljfpf`, `dec-fjqcv7ai`) as the gaps in `.claude/docs/security-summary.md`. This entry's wrap leaf pass must write them, not by hand.

@history_item n=21 marker=2026-11-24T21-15-59-wrap
**2026-11-24T21-15-59-wrap** — ratified decision/dropped-file-read-unrestricted and decision/os-drag-and-drop-untrusted (state set to standing) on the founder's «Подтвердить все 10». Why it matters: the two rows the folded CARRY `cry-d3tqol54` carries to the leaf pass now stand ratified. The leaf summary must state them as standing.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms, and `off-screen`'s meaning) — added decision/visible-region-through-transforms: a clipping box's visible region is read through its transforms, and a rotation reads the bounding box. Why it matters: a circle refused `off-screen` on the stand is read by this reader, through `in_view` in `packages/escher-driver/src/execute.rs`.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§Input Validation: `disabled` on form controls) — `disabled` is read by presence on a form control alone (`is_disabled`), for focusability, pointer gates and the activation arms (decision/disabled-read-by-presence). Ratified 2026-10-18. Why it matters: the stand's `disabled` reading on a circle cannot come from a `disabled` attribute on the canvas element, which is no form control. A virtual node's disabled state needs its own closed source.

@history_item n=24 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the shared target check and the actions read; the one actionable rule) — added one crate-private `check_target`, shared by every id-taking verb and the actions read, and one actionable rule, so a listed cause equals the call's refusal (decision/actions-read-shares-target-check, decision/one-actionable-rule). The actions answer crossing was also set (decision/actions-answer-crossing). Why it matters: a focusable canvas, a `press` on it and the circles' covered, off-screen and disabled causes must go through the one check and rule, so the actions answer and the refusals agree.

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — `clickable` was added as a closed bool after `focused` in the snapshot and diff, with no widening under the closed-field rule (contract/snapshot-text/clickable, contract/diff-node/clickable). Why it matters: a canvas that gains a key handler and `tabindex` changes its `focused` and `clickable` readings and the diff's `differs`. These are literal pins to sweep.

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record and handler_coverage as readers of the stable id) — added the reached-handler record and `handler_coverage` as readers of the id, with no log site and no NodeId (decision/reached-handler-record-without-log, decision/reached-record-deduplicated-survives-removal, decision/coverage-line-no-user-data). Why it matters: a new canvas `onkeydown` handler joins the coverage answer. `GRAPH_DRIVES`' first click now selects under B1, so the coverage readings are re-read.

@history_item n=27 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (Input Validation: 3 row(s) (1 of 2); Data Protection: 2 row(s)) — the reached record keeps only the innermost listener when a dispatch was stopped. A kept check holds `press` keys and the `ctrl` flag as call content (decision/reached-record-deduplicated-survives-removal, decision/kept-check-call-content). Why it matters: the keys that move the current circle cross as `press` calls kept in checks, and the canvas key handler's reached record follows the innermost-listener rule.
