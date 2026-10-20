- line: .andromeda/security-plan.md:125
old: «a fifth in-process reader of the id (PROVISIONAL: in process only)»
new: «a fifth in-process reader of the id (in process only)»
- line: .andromeda/security-plan.md:125
old: «on the in-process `Session::lint`, which takes no argument (PROVISIONAL)»
new: «on the in-process `Session::lint`, which takes no argument»
- line: .andromeda/security-plan.md:126
old: «each built `<label>` resolves through `label_bound_input_element` (its `for`-target `<input>` by id, else its first nested `<input>`) and is pushed onto that input's `labelled_by` only when both have built nodes, so a hidden label or input never associates»
new: «each built `<label>` resolves through `label_bound_input_element` (its `for`-target labelable element by id, else its first nested labelable element — HTML's labelable elements, `button`, an `input` whose `type` is not `hidden` (ASCII case-insensitive), `meter`, `output`, `progress`, `select` and `textarea`, by the private `is_labelable`, a closed element-name match) and is pushed onto that element's `labelled_by` only when both have built nodes, so a hidden label or element never associates and a hidden input is never named by one, and a click on a select's label re-dispatches to the select»
- line: .andromeda/security-plan.md:126
old: «pinned by `accessibility_label`'s 10 tests»
new: «pinned by `accessibility_label`'s 13 tests»
- line: .andromeda/security-plan.md:126
old: «tests/blitz-tests/tests/accessibility_label.rs:30-297); no lookup panics»
new: «tests/blitz-tests/tests/accessibility_label.rs:30-297; tests/blitz-tests/tests/accessibility_label.rs:299-440); no lookup panics»
- line: .andromeda/security-plan.md:126
old: «packages/blitz-dom/src/document.rs:672-709; tests/blitz-tests/tests/accessibility_names.rs:1-11»
new: «packages/blitz-dom/src/document.rs:677-715; packages/blitz-dom/src/document.rs:3126-3140; tests/blitz-tests/tests/accessibility_names.rs:1-11»
- line: .andromeda/security-plan.md:129
old: «`BaseDocument::build_accessibility_node`, on every element, the toggle by role,»
new: «`BaseDocument::build_accessibility_node`, on every element — a select's option excepted, whose `selected` is its chosen state with no `aria-selected` read (the `select` row) —, the toggle by role,»
- line: .andromeda/security-plan.md:132
old: «a `disabled` attribute on any other element not and a click on any other element unchanged»
new: «a `disabled` attribute on any other element not, a click on a `select` focusing it (the `select` row) and a click on any other element unchanged»
- line: .andromeda/security-plan.md:252
old: «the partial take is F1 answered by the operator as delegate, PROVISIONAL —»
new: «the partial take ratified —»
- line: .andromeda/security-plan.md:363
old: «and `type`'s help still says that it replaces (»
new: «and `type`'s help still says that it replaces and that on a select the text chooses the first option whose label starts with it (packages/escher-driver/src/schema.rs:208-209; »
