@carried n=1 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
why: Any element the probe's test or example code renders that carries a listener (the Counter or CRUD handlers it measures) must read an author key, and `unkeyed_actionable` must still return nothing on every stand task with the probe in place.
quote: The actionable-key rule: every element an agent can act on — focusable, or carrying an interactive role, or carrying an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it (ratified by the founder, 2026-10-06); under dioxus-native-dom's `accessibility` feature `DioxusDocument::unkeyed_actionable` is the check — it reads focusability from `Node::is_focussable()`, the role from the tree `accessibility_tree` returns (a closed interactive list, so a disabled control is still held) and the listener mark `data-dioxus-id` (so a hidden control is still held), with no second role mapping
@carried n=2 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
why: The probe's "reacts to a click" answer (link 2 for `click`) is the reading this residual owns; it must agree with the pinned count as its starting reading and must add no snapshot field, `NODE_FIELDS` staying thirteen.
quote: and the elements that react to input and state no interactive role are counted on the stand, in both layout modes, and pinned as measured — 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid` (`stand_non_lean_tasks::elements_that_react_to_a_click_and_state_no_role_are_counted`); no snapshot field says that an element reacts to a click (`NODE_FIELDS` thirteen) — that reading owned by a `.andromeda/residuals.md` line (target: next) on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)» (2026-10-14), this count its starting reading
@carried n=3 label=a11y-plan fact=fct-77ewqepn
why: The probe advances the click reading only as a verdict for the founder; the snapshot gains no click field in this chunk, and the actionable-key check must keep returning nothing on every stand task.
quote: `DioxusDocument::unkeyed_actionable` is the check, with no second role mapping, and returns nothing on every stand task; no snapshot field says an element reacts to a click — that reading owned by a `.andromeda/residuals.md` line (target: next).
@carried n=4 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
why: The probe's node side of each link must name a node by the stable element id the bridge already holds, the id the tree and snapshot read, and never by a second id of its own.
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one
@carried n=5 label=a11y-plan fact=fct-2hrkecxm
why: Values the probe reads from a handler or a signal must stay masked and never logged, and a password's typed text must surface in no field, tree node or frame the probe touches.
quote: A password input's value reads as the fixed `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no snapshot field, tree node or painted frame
@binding n=1
**The click count and residual ↔ the scope's "reacts to a click" reading (link 2)**: the probe's `click` answer from the listener map is the residual's reading, and the measured count of elements that react to a click with no role is its starting reading; no node key is added (a11y-plan/screen-reader-support fct-pfylw46r), (a11y-plan fct-77ewqepn)
@binding n=2
**The actionable-key rule ↔ the probe's stand code**: any probe element carrying a listener reads an author key, and `unkeyed_actionable` keeps returning nothing on the stand (a11y-plan/screen-reader-support fct-pfylw46r)
@binding n=3
**Masking ↔ the scope's nothing-crosses rule (values masked, never logged)**: a value the probe reads (a signal, a handler's write) is never printed or put in a field, and a password reads only `MASKED_VALUE` (a11y-plan fct-2hrkecxm)
@binding n=4
**Stable element id ↔ the probe's node addressing**: each link's node is read by its stable element id, the `author_id` the tree and snapshot carry (a11y-plan/screen-reader-support fct-kf4qrij2)
