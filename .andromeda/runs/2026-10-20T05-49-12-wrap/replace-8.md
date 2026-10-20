- line: .andromeda/a11y-plan.md:37
old: «a mutation of an in-document node, a focus or checked change and a text control's input mark it;»
new: «a mutation of an in-document node, a focus or checked change, a text control's input and a select's changed choice mark it — `apply_select_choice` marking nothing for a choice of the chosen option (packages/blitz-dom/src/events/keyboard.rs:271-297);»
- line: .andromeda/a11y-plan.md:46
old: «from its ARIA state attributes, on every element, the toggle by role,»
new: «from its ARIA state attributes, on every element — a select's option excepted, its `selected` its chosen state (`select::is_chosen`), true or false, with no `aria-selected` read, and its non-empty `label` attribute its `label` where it has no `aria-label` (packages/blitz-dom/src/accessibility.rs:157-162; packages/blitz-dom/src/accessibility.rs:171-181) —, the toggle by role,»
- line: .andromeda/a11y-plan.md:70
old: «`accessibility_label`, 10 tests (`aria-label`;»
new: «`accessibility_label`, 13 tests (a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, a `<label>` naming no hidden `input` (tests/blitz-tests/tests/accessibility_label.rs:309-432); `aria-label`;»
- line: .andromeda/a11y-plan.md:80
old: «(`Session::lint`, PROVISIONAL: in process only)»
new: «(`Session::lint`, in process only)»
- line: .andromeda/a11y-plan.md:105
old: «read by value on any element since 2026-10-10-snapshot-states;»
new: «read by value on any element — a select's option excepted, its `selected` its chosen state (packages/blitz-dom/src/accessibility.rs:171-181) — since 2026-10-10-snapshot-states;»
- line: .andromeda/a11y-plan.md:112
old: «`hidden`, `display: none` and `visibility: hidden` exclude it;»
new: «`hidden`, `display: none` and `visibility: hidden` exclude it — an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes: it stays in the tree, `hidden`, `inert` and `visibility: hidden` still excluding it (packages/blitz-dom/src/accessibility.rs:257-258; packages/blitz-dom/src/accessibility.rs:260-273);»
- line: .andromeda/a11y-plan.md:155
old: «by the `FOCUS_WITHIN` element state (PROVISIONAL), while»
new: «by the `FOCUS_WITHIN` element state, while»
- line: .andromeda/a11y-plan.md:161
old: «and a click on any other element is unchanged — a CRUD row or the list still clears focus;»
new: «a click on a `select` focuses it through `generate_focus_events` / `set_focus_to`, clearing its typeahead and the document's text selection, a disabled select stopped by the walk's gate first so it neither focuses nor acts, while an option is not clicked (a driver `click` on one is refused `covered`) (packages/blitz-dom/src/events/pointer.rs:857-870; tests/blitz-tests/tests/stand_act_select.rs:172-198; tests/blitz-tests/tests/stand_act_select.rs:235-256), and a click on any other element is unchanged — a CRUD row or the list still clears focus;»
- line: .andromeda/a11y-plan.md:174
old: «Clicking a label runs the default click of its bound input»
new: «Clicking a label runs the default click of its bound labelable element, so a click on a select's label focuses the select (§7)»
- line: .andromeda/a11y-plan.md:234
old: «(`Session::lint`; PROVISIONAL: in process only, no verb carries it)»
new: «(`Session::lint`; in process only, no verb carries it)»
- line: .andromeda/a11y-plan.md:269
old: «are excluded from the accessibility tree, and so from the snapshot;»
new: «are excluded from the accessibility tree, and so from the snapshot — a select's option excepted from `display: none` (§4 Hidden content), so a select's options are its children in the tree and the snapshot;»
- line: .andromeda/a11y-plan.md:284
old: «A `<label>` names the `<input>` it is bound to — its `for`-target by id, else its first nested `<input>` — by being pushed onto that input's `labelled_by` after the whole tree is built, so a label before or after its input both work; the bound input must be an `input` element with a built node;»
new: «A `<label>` names the labelable element it is bound to — its `for`-target by id, else its first nested labelable element, HTML's labelable elements being a `button`, an `input` whose `type` is not `hidden` (ASCII case-insensitive), a `meter`, `output`, `progress`, `select` or `textarea` — by being pushed onto that element's `labelled_by` after the whole tree is built, so a label before or after its element both work; the bound element must have a built node, and a hidden input is named by no label;»
- line: .andromeda/a11y-plan.md:284
old: «packages/blitz-dom/src/document.rs:672-709; tests/blitz-tests/tests/accessibility_names.rs:84-93»
new: «packages/blitz-dom/src/document.rs:677-715; packages/blitz-dom/src/document.rs:3126-3140; tests/blitz-tests/tests/accessibility_names.rs:84-93; tests/blitz-tests/tests/accessibility_label.rs:309-432»
