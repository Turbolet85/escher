@carried n=1 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: the «reacts to a click» boolean must join the snapshot model's fields beside the seven NodeState readings, taken through the model with no second role or name mapping.
quote: The snapshot model is a consumer of this output: under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` takes each node's role, its name
@carried n=1 run=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: since 2026-10-10-snapshot-states the tree gives an element node `selected`, `toggled` and `invalid` (§2 Role derivation), and the snapshot takes those three from the tree, never from the element — `NodeState.selected` and `.pressed` (read off the five checkable roles), each an `Option<bool>`, and `.invalid`, a `bool`, the model's seven fields in the order `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused`
@carried n=1 run=3 label=a11y-plan fact=fct-uz4kdss3
quote: its text form, parts, diff, MCP results, node-key help and `screenshot` read that model and nothing else — never a second role or name mapping.
@carried n=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: the new field must reach the text form, the diff, the parts, the MCP results and the node keys' help only as readers of the one model.
quote: the snapshot's diff `Snapshot::diff` is one more reader of that model and of nothing else
@carried n=2 run=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: an MCP tool result is a further reader of that model and of nothing else
@carried n=2 run=3 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: the node keys' help, which `help` serves — each key's reading stated by the tool
@carried n=3 label=a11y-plan/screen-reader-support fact=fct-yfrcl4qo
why: NODE_FIELDS reads thirteen today and the closed boolean must move it to fourteen in every pin that counts it, with no `expanded` key added alongside.
quote: No `expanded` node key exists: `NodeState` holds no such field and `NODE_FIELDS` reads thirteen, so a list's open state reaches the tree and not the snapshot
@carried n=3 run=2 label=a11y-plan fact=fct-sme4dbib
quote: the snapshot holds no `expanded` key, an open list's row reading its own `bounds` in the list
@carried n=4 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: a state or control value the graph carries must read masked exactly as the snapshot masks it, a password's and a file input's as `MASKED_VALUE`.
quote: a password input reads the fixed marker `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no field of the snapshot or of the tree
@carried n=4 run=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: a file input reads the same marker where its `value` attribute is non-empty, its path in no field of the snapshot
@carried n=4 run=3 label=a11y-plan fact=fct-2hrkecxm
quote: A password input's value reads as the fixed `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no snapshot field, tree node or painted frame
@carried n=5 label=a11y-plan/screen-reader-support fact=fct-ljepgzwi
why: the graph's new read and its log sites must open no further exit for an option's or a group's label, which leaves only as pixels and the snapshot name, with no log site.
quote: An option's label and a group's `label` leave an open list only as painted pixels in a `screenshot` and the snapshot's `name`, with no log site, ratified
@carried n=6 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
why: the actionable-key count is the «reacts to a click» field's starting reading, owed to this chunk, and the re-pin must restate it for the field.
quote: and the elements that react to input and state no interactive role are counted on the stand, in both layout modes, and pinned as measured — 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid` (`stand_non_lean_tasks::elements_that_react_to_a_click_and_state_no_role_are_counted`); no snapshot field says that an element reacts to a click (`NODE_FIELDS` thirteen) — that reading measured by link 2, its snapshot field owed to the route entry «The UI graph» on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)» (2026-10-14), this count its starting reading
@carried n=6 run=2 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
quote: The actionable-key rule: every element an agent can act on — focusable, or carrying an interactive role, or carrying an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it (ratified by the founder, 2026-10-06); under dioxus-native-dom's `accessibility` feature `DioxusDocument::unkeyed_actionable` is the check — it reads focusability from `Node::is_focussable()`, the role from the tree `accessibility_tree` returns (a closed interactive list, so a disabled control is still held) and the listener mark `data-dioxus-id` (so a hidden control is still held, and so is an element whose listener a re-render removed, its mark still standing, so the check over-includes, conservative, while a `mounted` listener writes none — read, not run), with no second role mapping
@carried n=6 run=3 label=a11y-plan/screen-reader-support fact=fct-6q7odjnr
quote: The check also returns nothing on the load page the stand serves beside the lean four (`stand_load_page::the_load_page_is_keyed_and_reads_no_finding_at_boot`, in both layout modes), so it reads empty on all six tasks `escher-session` serves
@carried n=6 run=4 label=a11y-plan fact=fct-77ewqepn
quote: no snapshot field says an element reacts to a click — that reading measured by link 2, its field owed to the route entry «The UI graph»; the listener mark over-includes an element whose listener was removed (conservative).
@carried n=7 label=a11y-plan/screen-reader-support fact=fct-7vfy7k5v
why: the field must carry the probe's measured link-2 reading, the 676 cells reacting to `click` and `cells-grid` to `keydown` only.
quote: The «reacts to a click» reading is measured by `dioxus_runtime_probe`'s link 2, in both layout modes: on Cells the 677 elements carrying the listener mark and stating no role are the 676 cells reacting to `click` and `cells-grid` reacting to `keydown`; on Counter `counter-increment` carries `click` and `counter-value` none; CRUD's `crud-list` carries `keydown`; no snapshot field is added (`NODE_FIELDS` thirteen)
@carried n=7 run=2 label=a11y-plan/screen-reader-support fact=fct-cbzuzrms
quote: The bridge's listener attributes over-report: a listener removed on a re-render leaves its event attribute and the `data-dioxus-id` mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none; the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `VNode::mounted_dynamic_attribute`, public dioxus-core 0.7.10 API) read each element's current listeners exactly, `mounted` included
@carried n=8 label=a11y-plan/screen-reader-support fact=fct-pofw2owv
why: the new field must say `cells-grid` reacts as it measures while its role stays untouched, role and cell semantics staying out under the residual.
quote: Cells' `cells-grid` takes `aria_label` `Cells` and no role; role and cell semantics stay out by residual `res-wvatzhpe`, re-carried at 0.3.0 (target: next)
@carried n=9 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
why: the graph must be keyed on the tree node carrying its stable element id as `author_id`, read from the same tree `accessibility_tree` returns.
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one
@carried n=9 run=2 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
quote: and the tree the shell hands the adapter, `Document::accessibility_tree` through `&dyn blitz_dom::Document` (the call `AccessibilityState::update_tree` makes), equals `harness.doc.accessibility_tree()` by id, node for node, with the same focus and root, every `author_id` on it the node's `element_id`, none empty, all digits or holding `0x`
@carried n=9 run=3 label=a11y-plan/screen-reader-support fact=fct-jn5hbj7a
quote: Text nodes become `TextRun` nodes carrying their text, and the parent is labelled by them
@carried n=9 run=4 label=a11y-plan fact=fct-iq4fjjyz
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one
@carried n=10 label=a11y-plan/screen-reader-support fact=fct-vee7m5at
why: the graph's agent-visible node must carry its bounds the way the snapshot builds them, a closed select's option its select's rect and an open row its painted row's.
quote: a closed select's options under it, each with its select's client rect as its `bounds`; a select whose list is open is no closed select, `closed_select_of` answering `None` for it, and the snapshot's builder reads `select_list_bounds(node)` before the client rect, so an open row's `bounds` is its painted row's in the snapshot and the diff, `differs` naming `bounds`
@carried n=10 run=2 label=a11y-plan/screen-reader-support fact=fct-qf4gn4dl
quote: A `multiple` select's rows and groups read their own client rects as their `bounds` in the snapshot, unlike a closed select's options, in both layout modes
@carried n=11 label=a11y-plan/visual-design-verification fact=fct-lq4s5uty
why: the graph keyed on the agent-visible node must hold no node the tree excludes, hidden or inert content standing outside it as outside the snapshot.
quote: Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree, and so from the snapshot
@carried n=11 run=2 label=a11y-plan fact=fct-xkzcu6wr
quote: Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot
@carried n=12 label=a11y-plan/screen-reader-support fact=fct-nsojnu3x
why: moving the stand's hook-local signals onto the registry must move no element, class, style, key, id or order, the attribute-stated roles and states reading as they do.
quote: On the stand two kinds of control state a role or a state by attribute alone since 2026-10-10-snapshot-states, with no element, class, style, key, id or order moving: the CRUD list states `role="listbox"` and each person row `role="option"` with `aria-selected`, reading `ListBox` / `ListBoxOption` in the tree, and the flight booker's two date inputs state `aria-invalid`
@carried n=12 run=2 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
quote: on the stand the 14 interactive controls carry an HTML-AAM role and a non-empty accessible name
@carried n=12 run=3 label=a11y-plan/cognitive-accessibility fact=fct-d4aqaj2h
quote: since 2026-10-10-snapshot-states the two date inputs `flight-start` and `flight-return-date` also state `aria-invalid`, so the invalid state is in the accessibility tree and the snapshot and not carried by the class alone, the class and its styling staying
@carried n=12 run=4 label=a11y-plan/cognitive-accessibility fact=fct-d4aqaj2h
quote: CRUD's sequence restated there to six stops, `back-btn`, `crud-filter`, `crud-list`, `crud-name`, `crud-surname`, `crud-create`, and the flight booker's to four, `back-btn`, `flight-type`, `flight-start`, `flight-book` (`flight-return-date` joining in `ENABLED_TAB_ORDER`, five stops), the error-recovery cue unchanged
@carried n=12 run=5 label=a11y-plan fact=fct-x32cq65m
quote: the two date inputs also state `aria-invalid`, so the invalid state is in the tree and the snapshot and not carried by the class alone; the headless stand asserts the cue without colour.
@carried n=13 label=a11y-plan/screen-reader-support fact=fct-g5whxvnn
why: the graph reader and the snapshot field read the accessibility tree, so they must sit behind the Dioxus crates' `accessibility` feature as the snapshot does.
quote: the Dioxus crates state screen-reader exposure only as accesskit support behind the `accessibility` feature
@binding n=1
**sr-actionable-keys ↔ sr-snapshot**: the «reacts to a click» field must be read by link 2's exact vdom walk, `mounted` included, never by the over-reporting listener mark that `unkeyed_actionable` keeps, and must move `NODE_FIELDS` from thirteen in the model and every reader of it, `to_text`, parts, diff, MCP results and node-key help (a11y-plan/screen-reader-support fct-cbzuzrms) (a11y-plan/screen-reader-support fct-yfrcl4qo) (a11y-plan/screen-reader-support fct-ge4n5uhw)
@binding n=2
**sr-actionable-keys starting reading ↔ the scope's re-pin**: the plan's pinned count reads Cells 677, the 676 data cells and `cells-grid`, elements that react to input, while the probe measures 676 reacting to `click` and `cells-grid` to `keydown` only; the re-pin of `stand_non_lean_tasks` must state which reading the field carries, so a click-only field reading `cells-grid` false keeps its role untouched (a11y-plan/screen-reader-support fct-pfylw46r) (a11y-plan/screen-reader-support fct-7vfy7k5v) (a11y-plan/screen-reader-support fct-pofw2owv)
@binding n=3
**sr-author-id ↔ visual-visibility ↔ sr-tree bounds**: the graph's nodes must be exactly the snapshot's agent-visible tree nodes, keyed by `author_id`, with no hidden or inert node and with each node's bounds as the snapshot builds them; whether `TextRun` nodes, which carry no `author_id`, are graph nodes is a fork the plan must settle (a11y-plan/screen-reader-support fct-kf4qrij2) (a11y-plan/visual-design-verification fct-lq4s5uty) (a11y-plan/screen-reader-support fct-vee7m5at) (a11y-plan/screen-reader-support fct-jn5hbj7a)
@binding n=4
**sr-stand-states ↔ cognitive-error-recovery**: moving the flight booker's, CRUD's and the other stand tasks' signals onto the registry must leave their attribute-stated roles and states, their stated Tab orders and the not-colour-alone error cue as they are, the actionable-key check still empty on all six served tasks (a11y-plan/screen-reader-support fct-nsojnu3x) (a11y-plan/cognitive-accessibility fct-d4aqaj2h) (a11y-plan/screen-reader-support fct-6q7odjnr)
@binding n=5
**sr-snapshot masking ↔ the scope's never-logged rule**: a value the graph carries must read `MASKED_VALUE` as on every carrier, and neither the graph's read nor any log site may open a new exit for a value or an option's label (a11y-plan/screen-reader-support fct-ge4n5uhw) (a11y-plan/screen-reader-support fct-ljepgzwi)
