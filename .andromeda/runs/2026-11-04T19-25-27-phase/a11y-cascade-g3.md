@carried n=1 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
why: The probe must address every node of its four links by the stable element id it carries as AccessKit author_id, never by a new key, and must expect no author_id on a TextRun, document-root or Window node.
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one
@carried n=1 run=2 label=a11y-plan fact=fct-iq4fjjyz
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one
@carried n=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: Any role, name or focus the probe reads of a node must come off the tree accessibility_tree returns through the snapshot model, with no second role or name mapping and no new snapshot field.
quote: under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` takes each node's role, its name (the `label`, else the names of its `labelled_by` targets — since 2026-10-17-upstream-sync-engine-debts the `label` is also the computed text of an element's `aria-labelledby` targets, which outranks a `<label>` and an `aria-label` (§2 Role derivation), so that text reaches the snapshot `name`) and the focused node from the tree `accessibility_tree` returns, with no second role or name mapping
@carried n=2 run=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: the snapshot's diff `Snapshot::diff` is one more reader of that model and of nothing else — a comparison of two snapshots by stable element id, its entries carrying the role and name the later snapshot holds, with no second role or name mapping — so on the stand, in both layout modes, a text change is reported on the element whose accessible name it changes, by that element's id and never as a text-run entry (`counter-value` after a click, the labelled input after its label is rewritten)
@carried n=2 run=3 label=a11y-plan/screen-reader-support fact=fct-yfrcl4qo
quote: No `expanded` node key exists: `NodeState` holds no such field and `NODE_FIELDS` reads thirteen
@carried n=2 run=4 label=a11y-plan fact=fct-uz4kdss3
quote: The snapshot model consumes the tree: `DioxusDocument::snapshot` takes each node's role, name, focus and `selected`, `pressed` and `invalid` from `accessibility_tree`, never from the element
@carried n=3 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
why: Link (2)'s click answer must be read through the listener mark the actionable-key check already reads, as the reading the residual line owns, without adding the snapshot field that says an element reacts to a click.
quote: The actionable-key rule: every element an agent can act on — focusable, or carrying an interactive role, or carrying an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it (ratified by the founder, 2026-10-06); under dioxus-native-dom's `accessibility` feature `DioxusDocument::unkeyed_actionable` is the check — it reads focusability from `Node::is_focussable()`, the role from the tree `accessibility_tree` returns (a closed interactive list, so a disabled control is still held) and the listener mark `data-dioxus-id` (so a hidden control is still held), with no second role mapping
@carried n=3 run=2 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
quote: and the elements that react to input and state no interactive role are counted on the stand, in both layout modes, and pinned as measured — 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid` (`stand_non_lean_tasks::elements_that_react_to_a_click_and_state_no_role_are_counted`); no snapshot field says that an element reacts to a click (`NODE_FIELDS` thirteen) — that reading owned by a `.andromeda/residuals.md` line (target: next) on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)» (2026-10-14), this count its starting reading
@carried n=3 run=3 label=a11y-plan fact=fct-77ewqepn
quote: no snapshot field says an element reacts to a click — that reading owned by a `.andromeda/residuals.md` line (target: next).
@carried n=4 label=a11y-plan/screen-reader-support fact=fct-nsojnu3x
why: The CRUD list's listbox, its option rows' aria-selected and the list's focus are the stand surface whose handler-written selection links (3) and (4) must read on the CRUD task.
quote: the CRUD list states `role="listbox"` and each person row `role="option"` with `aria-selected`, reading `ListBox` / `ListBoxOption` in the tree
@carried n=4 run=2 label=a11y-plan/screen-reader-support fact=fct-nsojnu3x
quote: one gap stands open — the rows are not focusable and the tree's focus is the list, never the option: since 2026-10-13-stand-keyboard-harness the focused list's keys move the selection (§5 Per-surface keyboard shortcuts)
@carried n=4 run=3 label=a11y-plan fact=fct-ldzd7n5j
quote: On the stand two kinds of control state a role or a state by attribute alone — the CRUD list `listbox` with `option` rows carrying `aria-selected`
@carried n=5 label=a11y-plan/screen-reader-support fact=fct-g5whxvnn
why: The probe's reads through DioxusDocument must reach dioxus-native-dom's accessibility feature as the stand already does, headlessly and without changing a Dioxus crate's manifest or features.
quote: the Dioxus crates state screen-reader exposure only as accesskit support behind the `accessibility` feature (packages/dioxus-native-dom/src/lib.rs:7-10; packages/dioxus-native/src/lib.rs:7)
@carried n=6 label=a11y-plan/cognitive-accessibility fact=fct-2jvvlu33
why: Where the plan takes the load page as the probe's one served task, its run must wait out the stand-in's answer before it reads the click-written status, since the status node stays out of view until the stylesheet lands.
quote: The stand's served load page shows its status in text: the paragraph `load-done` reads "Loaded", `display: none` until the page's stylesheet lands after the Button "Load" (`load-fetch`) is clicked, its in-process stand-in answering after `LOAD_ANSWER_DELAY` (200 ms)
@binding n=1
**screen-reader-support (sr-author-id) ↔ screen-reader-support (sr-actionable-keys)**: the probe must name each node of links (1) and (2) by its `author_id`, and the actionable-key rule makes that id the author key of every element that carries a listener, so a listener the probe reads off the map belongs to a node keyed stably (a11y-plan/screen-reader-support fct-kf4qrij2) (a11y-plan/screen-reader-support fct-pfylw46r)
@binding n=2
**screen-reader-support (sr-actionable-keys) ↔ screen-reader-support (sr-snapshot)**: link (2)'s answer for `click` must come from the listener map or the `data-dioxus-id` mark and stay out of the snapshot, with `NODE_FIELDS` still thirteen, since the field is owned by a residual line and the scope adds no snapshot field (a11y-plan/screen-reader-support fct-pfylw46r) (a11y-plan/screen-reader-support fct-yfrcl4qo)
@binding n=3
**screen-reader-support (sr-stand-states) ↔ screen-reader-support (sr-snapshot)**: on CRUD, the selection that a row's click or the list's arrow keys write must be read as the row's `selected` taken off the tree, and a diff used as link (4)'s witness must name the moved nodes by stable id with no second role or name mapping (a11y-plan/screen-reader-support fct-nsojnu3x) (a11y-plan/screen-reader-support fct-ge4n5uhw)
@binding n=4
**cognitive-accessibility (cognitive-orientation) ↔ the body (§2 changed set)**: a served-task run on the load page must drain the changed set first and read it only after `LOAD_ANSWER_DELAY`, because the status node comes out of `display: none` only once the stylesheet lands (a11y-plan/cognitive-accessibility fct-2jvvlu33) (a11y-plan fct-xhpyuh5z)
