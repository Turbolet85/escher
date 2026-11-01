# Classified — a11y · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · a parent's name from its TextRun children feeds the names and the text-change diff that the stand checks and the accessibility_* re-reads assert
screen-reader-support#1.2 · yes · aria-label and aria-labelledby name precedence feeds the stand's names (CRUD "People", "Flight type") and is asserted by accessibility_names and accessibility_label, re-read under accessibility_*
screen-reader-support#1.3 · yes · the label-to-control association names the stand's labelled inputs, asserted by stand_accessibility_ids and the accessibility_names and accessibility_label re-reads
screen-reader-support#1.4 · yes · the select model's ComboBox and ListBoxOption tree reading, select_value, closed_select_of and select_list_bounds in the snapshot, asserted by stand_act_select and the select list's bounds
screen-reader-support#1.5 · yes · the single select's `expanded` in the tree, through the select model the Needs name, asserted by select_popup and stand_act_popup
screen-reader-support#1.6 · yes · the open list's highlight absent from the tree and snapshot is the stated limit of the select model the chunk keeps through the node.rs conflict; the take must not add a highlighted row to either
screen-reader-support#1.7 · yes · the a11y target's SC 2.1.1, 2.4.3 and 1.4.3 are re-proven through the harness alone, the tier-0 target the Needs name
screen-reader-support#1.8 · yes · a multiple select's rows and groups read their own client rects as bounds, the scrolled-box bounds and absolute_position the Needs name; stand_act_select asserts them
screen-reader-support#1.9 · yes · evt.values() through multiple_select_values is read through the dioxus_document.rs conflict, while the select's value stays select_value
screen-reader-support#2.1 · yes · author_id on element nodes, the 14 controls' roles and names and the accessibility_tree identity sit in the dioxus_document.rs conflict and are decided by stand_accessibility_ids; its tables-once clause is the S3 convention and is not carried
screen-reader-support#3.1 · yes · the snapshot model reads the tree's roles, names, focus, states and the password's MASKED_VALUE through the dioxus_document.rs and text.rs conflicts, asserted by the snapshot, text, parts and diff checks
screen-reader-support#3.2 · yes · the snapshot's thirteen NODE_FIELDS with no expanded key read the tree the chunk keeps through the select model, and the snapshot checks assert that count
screen-reader-support#3.3 · yes · an open list's option labels leave it only as painted pixels and the snapshot name, asserted by stand_act_popup, which the Needs name
screen-reader-support#3.4 · yes · the native checkbox and radio toggled reading on checkable roles is asserted by accessibility_states, a file re-read under accessibility_*
screen-reader-support#4.1 · yes · unkeyed_actionable is the dioxus_document.rs conflict surface, and the stand keys it reads (CRUD rows and list, Cells) are asserted by stand_actionable_keys
screen-reader-support#4.2 · yes · cells-grid's aria_label Cells and no role sit in cells.rs, the Cells conflict surface, whose cells-grid the Needs name
screen-reader-support#4.3 · yes · the load page's Tab order, Enter and Space, and its contrast ratios are LOAD_TAB_ORDER, stand_load_page and stand_contrast rows the Needs name
screen-reader-support#5.1 · yes · the CRUD list's listbox and option roles, and the unread aria-activedescendant gap, sit under the list walks of stand_keyboard and accessibility_active_descendant_unread (re-read under accessibility_*)
screen-reader-support#6.1 · no · element tag names are read by no check the scope names, and accessibility.rs:268 is no reached file or unit
screen-reader-support#6.2 · yes · the tree's Window root with id u64::MAX and the focus fallback are what the tree-identity check compares (same focus and root), and the mutator's focus fixup reaches them
screen-reader-support#6.3 · no · a goal statement of accessibility_roles that asserts no value or behaviour the take must honour
screen-reader-support#7.1 · no · accesskit_xplat's handler routing is upstream adapter code outside the reached files and the scope; the chunk adds no platform adapter
screen-reader-support#8.1 · no · the windowed adapter's creation from the raw window handle is blitz-shell code the chunk does not reach, and no windowed witness is run
screen-reader-support#8.2 · yes · the a11y checks run behind the accessibility feature and Cargo.toml is reached whole, so the feature gate the take's merge must keep binds
screen-reader-support#9.1 · no · lang inheritance is named by no reached file, Need or check, and the fixture lang records are not needed
