@history_item n=1 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (7. Screen Reader Support, 2 rows) — `~ decision/ui-graph-one-node-per-snapshot-node` gained the carve-out "but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them" (F9 A, delegate escher overseer, PROVISIONAL); `~ contract/snapshot/enabled` reads a virtual node's declared disabled flag (F8 A with F2 A). This chunk retires the F9 A carve-out it owns, so it rewrites that row; the `enabled` reading on a virtual node is what `actions` must answer `disabled` from.

@history_item n=2 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (7. Screen Reader Support, 1 row) — `~ contract/node-id/reserved-separator`: was "no served id holds ///"; now `{owner}///{key}` is served in the accessibility tree, snapshot and diff, no element's id holding `///`. Extending the virtual id into the graph, `actions`, coverage, layout and exploration widens where the form is served, so this row's served-in list moves again; the guards asserting no `///` (`stand_graph`, `stand_actions`) rest on its old reading.

@history_item n=3 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (6. Visual Design Verification, 1 row) — `~ contract/driver-click-refusals/covered`: a virtual node reads `covered` only when an element lies over its region's centre (F12 A, delegate, PROVISIONAL). The `actions` answer's `covered` cause for a virtual node must read through the same rule; `covered` by another virtual node stays out of this chunk.

@history_item n=4 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** (7. Screen Reader Support, 2 entries) — `~ decision/snapshot-model-single-source` and `~ decision/readers-of-one-snapshot-model`: the declared-node read of one canvas is named, beside the canvas read, as no reader of the snapshot model. Twin rows edited together; if the graph, `actions` or layout come to read the declared-node read directly instead of the snapshot, both twins must be re-said in step.

@history_item n=5 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (7. Screen Reader Support, 3 rows) — `decision/circle-drawer-canvas-pixels-only` gained "canvas verb's draw commands", and `decision/readers-of-one-snapshot-model` / `decision/snapshot-model-single-source` gained "the canvas read is no reader of the snapshot model". Precedent for how a new canvas-reading surface is recorded on the twin reader rows; the `canvas` verb reading a virtual id `not-found` (F10 B) is this chunk's to revisit.

@history_item n=6 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** (7. Screen Reader Support) — `~ decision/circle-drawer-canvas-pixels-only`: circles and labels reach no DOM, snapshot, tree or UI graph, leaving as shot pixels, canvas draw commands and lint truncated boxes (F1 A, confirm PROVISIONAL as the delegate's). The lint already reads canvas content as `truncated`; F9 A's "the lint holds no virtual node" must be squared with this row, and the row's "reach no … UI graph" is the stand's Circle Drawer, untouched until «Circle Drawer's circles as virtual nodes».

@history_item n=7 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** (7. Screen Reader Support) — `+ decision/circle-drawer-canvas-pixels-only`, ratified at 2026-11-24T06-11-11-wrap: the drawing area one keyed, clickable canvas node. The stand declares no virtual node yet, so the proving surface is a fixture canvas; this row must stay true through the chunk.

@history_item n=8 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — `decision/circle-drawer-canvas-pixels-only` ratified by the founder (state standing). Bounds any edit: the row's rule stands as landed, so the chunk must not re-word Circle Drawer's reach.

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader `DioxusDocument::graph`) — one `GraphNode` per snapshot node in pre-order, `owner` for an option or optgroup, names and closed words only, gated behind `accessibility`; `stand_graph` asserts every graph node is a snapshot node and that no served id holds `///` (`virtual_node_id`, which the walk never calls). The chunk lands virtual nodes in this graph: the `///` guard is a literal pin to sweep, and `owner` is the field a virtual node's canvas owner rides.

@history_item n=10 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`NodeState.clickable`) — `clickable` read from the vdom walk's current `click` listeners on no disabled form control; text token after `focused` only when true, JSON always; `Reading::Clickable`; `CLICKABLE` table (Circle Drawer 1, Cells 676). Basis of `contract/snapshot/clickable`, `contract/snapshot-node/clickable`, `contract/snapshot-text/clickable`; a virtual node reading `clickable` false (F12 A) sits on this rule, and changing it moves the `CLICKABLE` table and `NODE_FIELDS`.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration and dispatch write record) — `use_graph_state`, `DeclaredWrite`, the registry's dispatch resolving to the first chain element with a listener. A virtual node has no element listener of its own; its handlers and writes in the graph must be reconciled with this resolution rule.

@history_item n=12 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule) — `unkeyed_actionable` and `actionable_nodes` share the crate-private `Readings`: focusable, an interactive role, or the vdom walk's exact current listeners holding an event other than `mounted`; the `data-dioxus-id` mark serves dispatch alone. Basis of `decision/one-actionable-rule` and `decision/listeners-from-vdom-walk`; a virtual node has no vdom listeners, so whether it is actionable needs a stated leg without a second role mapping.

@history_item n=13 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — `actionable_nodes` lists the UI graph's nodes in snapshot pre-order that the rule holds for or whose `owner` is set, each `id`, `owner`, `keyed` (`keyed` true where the id holds no `/`); domain is the graph's nodes. Basis of `contract/actionable-nodes/listing` and `decision/keyed-wire-form`; a `{owner}///{key}` id holds slashes, so `keyed` would read false for every virtual node unless the wire form is re-said.

@history_item n=14 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — `Session::actions` answers each verb taking an id with the cause `check_target` would refuse it with, `stale` for no element; changes nothing (an open list stays open); ids, owner ids, verb and cause words and booleans only. Basis of `contract/actions-answer/causes`, `/fields`, `decision/actions-verb-changes-nothing`; for a virtual node the causes must equal what `click` and the F10 B `not-found` arms return.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actionable counts and the Cells bound) — `ACTIONABLE_AT_BOOT` (Counter 2, Flight Booker 7, Timer 3, CRUD 11, Cells 678, load page 2) and the unpaged Cells `actions` answer at 106,088 bytes, owned by residual `res-7cc5bi4k`. Literal counts to sweep if virtual nodes join the listing; the stand declares none yet, so they should hold.

@history_item n=16 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading, PROVISIONAL) — the `keyed` reading on the `actions` answer is the actionable-key check's first wire form. Origin of `decision/keyed-wire-form`, ratified at 2026-11-13T07-43-19-wrap.

@history_item n=17 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` state provisional → standing (founder's «Утвердить keyed»). The "no slash" wire form is ratified; extending it to virtual ids (whose key the declaration authors) changes a founder-ratified row, which makes it a fork.

@history_item n=18 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record) — `reached` module (`ReachedHandler`, `record_reached`, `reached_handlers`) ungated, written on `handle_event`'s path before `runtime.handle_event`, each chain node whose current listeners hold the event inserting its stable id; reads `element_listener_ids`, not the gated `element_facts` (wasm build). Basis of `contract/dioxus-document/record-reached` and `decision/agent-readers-behind-accessibility`'s ungated clause; a virtual node's click is dispatched to its canvas element, so its reach is not on this path as built.

@history_item n=19 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`handler_coverage`, a further reader of the UI graph) — reads the graph at boot and after each step, `mounted` left out, deduplicated by (id, event); answer ids, owner ids, event names and two counts only. Basis of `contract/handler-coverage/answer`; a virtual node joining the graph joins this answer, and the scope asks its reach read where a kept check acts on it.

@history_item n=20 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`stand_handler_coverage`) — 8 tests, a refused step exercising nothing and a keyboard activation its click; `stand-repeat` by prefix, no `a11y` leg. A refused virtual-node click must likewise exercise nothing.

@history_item n=21 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (5. Keyboard Navigation) — `~ contract/dioxus-document/reached-handlers`: ids resolved before the handlers run; a stopped dispatch or panic counts the innermost listener alone, a non-bubbling event its target alone. Constraint on how a virtual node's reach could be recorded: the record holds element ids from the dispatch chain, so a virtual id would need its own rule.

@history_item n=22 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report) — the bridge's listener attributes over-report a listener removed on a re-render; the vdom walk's dynamic attributes read current listeners exactly. Origin of `decision/listeners-from-vdom-walk` (guard `a_listener_removed_on_a_re_render_is_read_by_neither_reader`): no virtual-node reading may fall back on the mark.

@history_item n=23 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule (founder's): every element an agent can act on reads an author key; `unkeyed_actionable` its check. Trap recorded: never satisfy the check by adding a `tabindex` or role. Basis of `decision/actionable-key-rule`; a virtual node's key is its declaration's, so whether the rule reads it keyed is the point to settle.

@history_item n=24 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's `disabled` cause worded on the snapshot's presence reading; rule: the entry that detects `disabled` reads the snapshot's `enabled`, never the attribute. For a virtual node `enabled` reads the declaration's flag, so `actions`' `disabled` cause must read `enabled`, not the declaration directly.

@history_item n=25 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `disabled` detected on `enabled == Some(false)`, the first screen-level check, nothing dispatched, focus as it was. Basis of `contract/driver-refusal/disabled` and `contract/driver-actions/disabled`; the check order that `actions` mirrors.

@history_item n=26 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` one more reader of the model's bounds; an element shot by stable id is its bounds rounded outward and cropped; a shot changes nothing. Basis of `contract/driver-screenshot/element-shot` and `decision/screenshot-changes-nothing`; `screenshot` with a virtual id reads `not-found` today (F10 B), which `actions` must answer.

@history_item n=27 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot model takes role, name and focus from the accessibility tree with no second role or name mapping. Origin of the readers-of-one-snapshot-model rule; a virtual node reaches every new reader through the snapshot, never through a second mapping.

@history_item n=28 marker=2026-10-12-self-description
**2026-10-12-self-description** — the node keys' help and `DiffNode::differs` added as readers of the snapshot model, closed words only. The `help` / `describe()` texts are literal pins to sweep when virtual ids enter more reads.

@history_item n=29 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on `dioxus-native-dom` unconditionally, so the stand binaries compile the snapshot model and tree override. Basis of `decision/agent-readers-behind-accessibility`: graph, actions and coverage readers live behind that feature, the reached record ungated.

@history_item n=30 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (both entries: in-process, then the gated `lint` verb part) — the lint's `contrast` reads through `text_contrast`, exempt where `enabled` is `Some(false)`, no second formula; a finding written `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only. Basis of `contract/lint-contrast-finding/*`, `contract/visual-lint-finding/content`; any lint over a virtual node keeps this shape.

@history_item n=31 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (6. Visual Design Verification) — `~ decision/lint-contrast-single-formula` and `~ decision/lint-contrast-shares-one-reader`: the highlight reader first, then text contrast, one formula; `− decision/highlighted-row-contrast-unread`. Current shape of the lint rows picked; a virtual node has no computed text style, so a lint over it must not add a second reader.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — the reader and the lint read `None` past a `mix-blend-mode`, "so a blended text's pair is unread by the lint as well". The clause `decision/lint-contrast-shares-one-reader` still carries.

@history_item n=33 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — `Harness::text_contrast`, WCAG 2.x, 3.0 large else 4.5, the exempt rule. Origin of `decision/text-contrast-required-ratio`.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — `overlaps` skips a closed select's options by element (`closed_select_of`), never the role; the stand reads no finding of any kind at boot. Basis of `decision/stand-lint-clean-at-boot` (guard `the_stand_at_boot_reads_its_measured_findings`); a virtual node's region overlapping its canvas element is the same trap if the lint reads virtual nodes.

@history_item n=35 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the lint's option overlap) — the stand's boot finding was the flight booker's options overlapping because the snapshot gave each option its select's rect. Precedent: nodes sharing or nesting in another node's bounds read as `overlap` unless excepted.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`; the lint over an open list) — a driver `click` on an element under the list is refused `covered`; the lint's exclusion widened to `Entry.in_select_box`. Origin of `contract/driver-click-refusals/covered`'s first clause and the precedent for excepting nodes from the lint's overlap by an element test.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `Target.select`, `focus_select` closing any list, no new verb, argument or cause. Basis of `decision/driver-type-focuses-select-no-click`; `type` on a virtual id reads `not-found` (F10 B), and adding a new refusal cause would be a fork.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — `type` focuses a `multiple` select with no click; the driver's closed sets hold no new verb, argument, key name, result field, cause, session error or exit. The closed-set clause `decision/driver-type-focuses-select-no-click` carries; the scope expects no new carrier, verb, exit or cause.

@history_item n=39 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (1. A11y Scope Summary) — `~ decision/single-select-engine-overlay-list`: a drop-down alone opens the overlay list, a single list box none. Current wording of the picked row; the overlay list is what a virtual node's `covered` reads against.

@history_item n=40 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (1. A11y Scope Summary) — `~ decision/capability-statement-reach-by-hand` why: the pin reads only the a11y leg's file count; no check reads the rest. A change of reach (virtual nodes in every read) is written into `docs/capabilities.md`'s `## Accessibility reach` by hand.

@history_item n=41 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach under `## Accessibility reach`; its pin reads none of the claims (since narrowed). Origin of the by-hand rule; the carried stated limits («Pinned by a check») live there.

@history_item n=42 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable` empty on every stand task; the click-without-role count pinned (Circle Drawer 1, Cells 676). Circle Drawer's canvas is that one clickable element; the counts are literal pins that hold while the stand declares no virtual node.
