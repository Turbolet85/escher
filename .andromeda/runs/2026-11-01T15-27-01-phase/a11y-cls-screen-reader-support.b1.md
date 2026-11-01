# Classified — a11y · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · parent naming from TextRun children is the tree's name rule, witnessed by the accessibility_* re-reads (accessibility_names) and the tree-identity check; accessibility.rs is not reached
screen-reader-support#1.2 · yes · aria-label and aria-labelledby naming is what the accessibility_names and accessibility_label re-reads assert
screen-reader-support#1.3 · yes · the `<label>` binding, its order, and the aria-labelledby override are asserted by the accessibility_label and accessibility_names re-reads
screen-reader-support#1.4 · yes · the select's ComboBox, ListBoxOption children and value sit in the select model and its list through the mutator.rs and node.rs conflicts, and stand_act_select is named in scope
screen-reader-support#1.5 · yes · the closed-bool `expanded` on a select is the select model's tree reading, which select_popup and stand_act_popup assert
screen-reader-support#1.6 · yes · the open list's tree reading (no highlighted row) is the select list's reading that select_popup and stand_act_popup rule over, and the take must keep it unchanged
screen-reader-support#1.7 · yes · it states the a11y target: SC 2.1.1, 2.4.3 and 1.4.3 asserted in the harness, which the chunk re-proves through its standing checks
screen-reader-support#1.8 · yes · a multiple select's row bounds are the client rects the select list's rows read through absolute_position, both named in scope via stand_act_select
screen-reader-support#1.9 · yes · evt.values() reading multiple_select_values lives in dioxus_document.rs, a predicted conflict surface
screen-reader-support#2.1 · yes · author_id on element nodes and tree identity against accessibility_tree are in dioxus_document.rs, a conflict surface, decided by stand_accessibility_ids and the CRUD list's roles in scope
screen-reader-support#3.1 · yes · the snapshot's role, name, focus, states, value, password masking, text, parts and MCP readings read the tree in dioxus_document.rs, a conflict surface, and the snapshot checks are in scope
screen-reader-support#3.2 · yes · the snapshot's NODE_FIELDS count of thirteen with no expanded key is a value the snapshot checks re-assert under the dioxus_document.rs conflict
screen-reader-support#3.3 · yes · an open list's option and group labels reaching only the snapshot name and a shot is the select list's reading that stand_act_popup asserts
screen-reader-support#3.4 · yes · the tree's toggled reading from a native checkbox or radio is what the accessibility_states re-read asserts
screen-reader-support#4.1 · yes · unkeyed_actionable is named in scope, decided by stand_element_ids, stand_id_persistence, stand_id_edits and the snapshot checks, with Cells' cell ids in the cells.rs conflict
screen-reader-support#4.2 · yes · cells-grid's name and absent role are the Cells surface through the cells.rs conflict, its one Tab stop and name in scope
screen-reader-support#4.3 · yes · the load page's Tab order, Enter and Space, and contrast ratios are named in scope through stand_load_page, LOAD_TAB_ORDER and stand_contrast
screen-reader-support#5.1 · yes · the CRUD list's listbox and option roles, the aria-invalid date inputs, the flight-type select's selected reading and the unread aria-activedescendant are pinned by accessibility_active_descendant_unread, a re-read accessibility_* file
screen-reader-support#6.1 · no · an element's tag name on its tree node is in no reached file and no re-read check rules over it
screen-reader-support#6.2 · yes · the Window root at u64::MAX and tree focus written only where built are what the tree-identity check asserts for root and focus
screen-reader-support#6.3 · no · the role-mapping test's stated goal at accessibility_roles.rs:3-5 is a purpose, not a behaviour or value the chunk's code must honour
screen-reader-support#7.1 · no · accesskit_xplat's event routing is a platform adapter the chunk neither reaches nor re-reads, and no stand binary builds one
screen-reader-support#8.1 · no · the adapter creation from the raw window handle sits in blitz-shell, which the chunk does not reach, and the chunk builds no platform adapter
screen-reader-support#8.2 · no · the accessibility feature gate lives in packages/blitz/Cargo.toml and dioxus-native-dom's lib.rs, neither reached, and the chunk adds no crate or feature of ours
screen-reader-support#9.1 · no · lang and xml:lang setting -x-lang are in lang_attribute.rs, which no Needs entry, reached file or re-read names
