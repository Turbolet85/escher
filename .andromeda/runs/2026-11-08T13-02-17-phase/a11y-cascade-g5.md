@carried n=1 label=a11y-plan/screen-reader-support fact=fct-vee7m5at
why: The owner-served nodes the read lists include a select's options, and their off-screen and covered causes must read the bounds the snapshot gives a closed select's options and an open list's rows.
quote: a closed select's options under it, each with its select's client rect as its `bounds`; a select whose list is open is no closed select, `closed_select_of` answering `None` for it, and the snapshot's builder reads `select_list_bounds(node)` before the client rect, so an open row's `bounds` is its painted row's in the snapshot and the diff
@carried n=2 label=a11y-plan/screen-reader-support fact=fct-kogkylbi
why: Whether an owner-served option can take a click now hangs on its select's open state, which lives in the tree's engine-state reading, and the read's verbs and causes for it must agree with that state.
quote: A single select's tree node takes `expanded` from a closed bool of engine state, reading no `aria-expanded`, its focus staying on the select while the list is open
@carried n=2 run=2 label=a11y-plan fact=fct-sme4dbib
quote: A single select reads `ComboBox` and writes `expanded` from its list's open state, reading no `aria-expanded`, and an author's `open` or `aria-expanded` on it steers nothing; the tree's focus stays on the select and its chosen option reads `selected`; the highlighted row is in no snapshot and no tree; the snapshot holds no `expanded` key, an open list's row reading its own `bounds` in the list
@carried n=3 label=a11y-plan/screen-reader-support fact=fct-axbrqk6r
why: The read lists from the snapshot and graph, so it must state no highlighted row of an open list, keeping this stated limit.
quote: An open list's highlighted row is in no snapshot and no tree, a stated limit
@carried n=4 label=a11y-plan/screen-reader-support fact=fct-qf4gn4dl
why: A multiple select's owner-served rows must take their off-screen and covered causes from their own client rects, in both layout modes.
quote: A `multiple` select's rows and groups read their own client rects as their `bounds` in the snapshot, unlike a closed select's options, in both layout modes
@carried n=4 run=2 label=a11y-plan fact=fct-uo5zb3r6
quote: A `multiple` select carries `set_multiselectable` in the tree and its rows read their own client rects in the snapshot
@carried n=5 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
why: The stable id the read lists per node must be the element's `author_id`, the same key the folded actionable-key reading reports as author key or positional path.
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one
@carried n=5 run=2 label=a11y-plan fact=fct-iq4fjjyz
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one
@carried n=6 label=a11y-plan/screen-reader-support fact=fct-73ao4vi3
why: The read's nodes must be graph nodes that are snapshot nodes, its served ids must keep the reserved `///` form unused, and its read must change nothing and hold no typed text, as `stand_graph` asserts of the graph.
quote: `stand_graph` asserts in both layout modes that every graph node is a snapshot node, that no served id holds the reserved separator `///` (the reserved `{owner}///{key}` form, spelled by `virtual_node_id`, which the walk never calls), and that a graph read changes nothing and holds no typed text
@carried n=7 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: The read's disabled cause must match the call's own refusal rule exactly, on the same reading and in the same order, its stand targets refused while not enabled.
quote: `Session::run` refuses a `click` or a `type` as `disabled` when the snapshot node's `enabled` is `Some(false)`, the first of its three screen-level checks, with nothing dispatched and the ids reading `focused` and the tree's focus as they were before the call; on the stand `crud-delete` and `flight-return-date` are refused while they read not enabled and acted on once they read enabled, in both layout modes
@carried n=7 run=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: `enabled` follows the presence of `disabled` on an element that can be disabled
@carried n=7 run=3 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: a password input reads the fixed marker `MASKED_VALUE` where it holds text and `""` where it holds none
@carried n=7 run=4 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: an element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen — and a shot changes nothing: no focus moves, `has_changes` stays false and the next diff is empty, in both layout modes
@carried n=7 run=5 label=a11y-plan fact=fct-uz4kdss3
quote: its text form, parts, diff, MCP results, node-key help and `screenshot` read that model and nothing else, and the read-only UI graph (`DioxusDocument::graph`) holds one node per snapshot node, names and closed words only — never a second role or name mapping.
@carried n=7 run=6 label=a11y-plan fact=fct-2hrkecxm
quote: the driver refuses a `click` or a `type` as `disabled` where the snapshot's `enabled` reads `Some(false)`, nothing dispatched.
@carried n=8 label=a11y-plan/screen-reader-support fact=fct-6nobpd2c
why: The read's actionable set takes `clickable` as the snapshot gives it, from the walk's current listeners on no disabled form control, with no second role mapping.
quote: `NodeState.clickable` is a `bool` after `focused`, true where the vdom walk's current listeners hold `click` and the element is no disabled form control (`ElementData::is_disabled`), read from the one snapshot model and the walk with no second role mapping
@carried n=9 label=a11y-plan/screen-reader-support fact=fct-gg5bgkx2
why: The listed node must be the graph's node with its owner and handlers, and the read's answer must carry ids and closed words only, never a value.
quote: a `UiGraph` holding one `GraphNode` per snapshot node in pre-order — its id, its `owner` (for an option or an optgroup, the stable id of its select), its component, its handlers with their declared writes, and the states it reads — and per registered state its declared writes, the writes a run confirmed or flagged (`undeclared`, `unconfirmed`) and its reading components; the answer carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value
@carried n=10 label=a11y-plan/screen-reader-support fact=fct-yfrcl4qo
why: An option's open-list state is not a snapshot reading, so the read must take it elsewhere or bring a new field to the founder, which moves the fourteen `NODE_FIELDS`.
quote: No `expanded` node key exists: `NodeState` holds no such field and `NODE_FIELDS` reads fourteen, so a list's open state reaches the tree and not the snapshot
@carried n=11 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
why: The actionable set the read lists and the actionable-key fold the scope names are ruled by this rule and its check, with no second role mapping.
quote: The actionable-key rule: every element an agent can act on — focusable, or carrying an interactive role, or carrying an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it (ratified by the founder, 2026-10-06); under dioxus-native-dom's `accessibility` feature `DioxusDocument::unkeyed_actionable` is the check — it reads focusability from `Node::is_focussable()`, the role from the tree `accessibility_tree` returns (a closed interactive list, so a disabled control is still held) and the listener mark `data-dioxus-id` (so a hidden control is still held, and so is an element whose listener a re-render removed, its mark still standing, so the check over-includes, conservative, while a `mounted` listener writes none — read, not run), with no second role mapping, and returns each actionable element whose id is a positional path, with its tag, its role and the remedy
@carried n=11 run=2 label=a11y-plan fact=fct-77ewqepn
quote: `DioxusDocument::unkeyed_actionable` is the check, with no second role mapping, and returns nothing on every stand task; the snapshot field `clickable` says an element reacts to a click, read from the vdom walk's current listeners and not the listener mark; the listener mark over-includes an element whose listener was removed (conservative).
@carried n=12 label=a11y-plan/screen-reader-support fact=fct-ymr6jwcn
why: The read's actionable set over the stand must agree with the `CLICKABLE` table's per-task counts and keep apart the listener mark's count.
quote: `stand_non_lean_tasks` asserts a `CLICKABLE` table of the elements that read `clickable` and state no role, per task in both layout modes — Counter, Temperature Converter, Flight Booker, Timer and CRUD 0 each, Circle Drawer 1, Cells 676 — beside the listener mark's count, Cells 677
@carried n=13 label=a11y-plan/screen-reader-support fact=fct-7vfy7k5v
why: Elements reacting to `keydown` alone are actionable by listener yet not clickable, so the verbs the read lists for them must follow their actual listeners.
quote: on Cells the 677 elements carrying the listener mark and stating no role are the 676 cells reacting to `click` and `cells-grid` reacting to `keydown`; on Counter `counter-increment` carries `click` and `counter-value` none; CRUD's `crud-list` carries `keydown`
@carried n=14 label=a11y-plan/screen-reader-support fact=fct-lqycjcmz
why: The stand's clickable readings are the values the read's listing must agree with on Counter and Cells.
quote: On the stand `clickable` reads true on `counter-increment` and on each of Cells' 676 data cells, and false on `counter-value` and `cells-grid`, from the one snapshot model and not the listener mark
@carried n=15 label=a11y-plan/screen-reader-support fact=fct-cbzuzrms
why: The read must take listeners from the walk, which reads them exactly, not from the mark, which over-reports; whether the actionable-key check's rule serves the read too is this fork.
quote: The bridge's listener attributes over-report: a listener removed on a re-render leaves its event attribute and the `data-dioxus-id` mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none; the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `VNode::mounted_dynamic_attribute`, public dioxus-core 0.7.10 API) read each element's current listeners exactly, `mounted` included, and the bridge's vdom walk yields them per element (`element_listeners`), the reading `NodeState.clickable` and the graph's handlers take, so neither over-reports as the mark does
@carried n=16 label=a11y-plan/screen-reader-support fact=fct-6q7odjnr
why: A folded actionable-key reading must re-assert the check's empty result on all six served tasks, load page included.
quote: The check also returns nothing on the load page the stand serves beside the lean four (`stand_load_page::the_load_page_is_keyed_and_reads_no_finding_at_boot`, in both layout modes), so it reads empty on all six tasks `escher-session` serves
@carried n=17 label=a11y-plan/screen-reader-support fact=fct-nsojnu3x
why: The CRUD rows are non-focusable options of a listbox, so how the read lists them, own node or owner-served, must agree with what a click on a row does.
quote: the CRUD list states `role="listbox"` and each person row `role="option"` with `aria-selected`, reading `ListBox` / `ListBoxOption` in the tree, and the flight booker's two date inputs state `aria-invalid`; the stand states no `aria-pressed` — the flight booker's mode is one `select#flight-type`, whose options `flight-type-one-way` and `flight-type-return` state the choice by `selected`, `aria-pressed` pinned on a fixture by `accessibility_states`; the actionable-key check still returns nothing; one gap stands open — the rows are not focusable and the tree's focus is the list, never the option
@carried n=17 run=2 label=a11y-plan fact=fct-ldzd7n5j
quote: one gap stands open: the rows are not focusable and the tree's focus is the list, never the option, the missing `aria-activedescendant` kept unread (§5 Per-surface keyboard shortcuts).
@carried n=18 label=a11y-plan/cognitive-accessibility fact=fct-d4aqaj2h
why: The Book button made disabled by an invalid date is a stand target whose disabled cause the read must name exactly as the call refuses it.
quote: the headless-stand check asserts the cue without colour — a typed non-date sets `invalid` on the start field and gives `.flight-btn` a `disabled` attribute it did not carry before
@carried n=18 run=2 label=a11y-plan fact=fct-x32cq65m
quote: Invalid dates get an `invalid` class with red styling and disable the Book button
@binding n=1
**cognitive-accessibility error recovery ↔ screen-reader-support snapshot**: the Book button the invalid-date cue disables must read the `disabled` cause in the read exactly where `Session::run` refuses it on `enabled` `Some(false)`, keyed by the attribute's presence, in both layout modes (a11y-plan/cognitive-accessibility fct-d4aqaj2h) (a11y-plan/screen-reader-support fct-ge4n5uhw)
@binding n=2
**screen-reader-support actionable keys ↔ screen-reader-support snapshot**: the actionable set must be decided over current listeners as `clickable` and the graph's handlers read them, never the over-reporting mark, while a folded key reading keeps the check's own rule and its empty result on every served task; whether one rule serves both is the measured fork (a11y-plan/screen-reader-support fct-pfylw46r) (a11y-plan/screen-reader-support fct-cbzuzrms) (a11y-plan/screen-reader-support fct-6nobpd2c)
@binding n=3
**screen-reader-support tree (select) ↔ screen-reader-support snapshot (graph owner, node fields)**: an owner-served option's verbs and causes must agree with its select's open state, which is in the tree and no snapshot key, and its bounds the snapshot's; a new snapshot field for it moves `NODE_FIELDS` and is the founder's boundary fork (a11y-plan/screen-reader-support fct-kogkylbi) (a11y-plan/screen-reader-support fct-yfrcl4qo) (a11y-plan/screen-reader-support fct-gg5bgkx2)
@binding n=4
**screen-reader-support author ids ↔ screen-reader-support graph**: the id listed per node must be the graph node's stable `author_id`, its served ids free of the reserved `///` form, the read changing nothing and holding no typed text (a11y-plan/screen-reader-support fct-kf4qrij2) (a11y-plan/screen-reader-support fct-73ao4vi3)
