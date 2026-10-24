# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the Document trait's default accessibility_tree, which DioxusDocument overrides with stable ids for the a11y checks, and the changed set that stand_accessibility_ids holds in the leg
standard-contracts#2.1 · yes · the document clock that the select typeahead row reads its 1 s buffer on and that the harness tick and advance_time move
standard-contracts#2.2 · yes · BaseDocument::select_value, the public reader the flight booker select row reads its choice through, beside the crate-private writers that settle the choice
standard-contracts#3.1 · yes · build_accessibility_tree's naming sources and its aria-selected, aria-pressed, aria-checked, aria-invalid, aria-expanded, aria-disabled and aria-describedby reads, which the widened accessibility checks witness
standard-contracts#4.1 · yes · keyboard activation (a queued Click on Enter and on a released Space), a click focusing a button, the blur, focusout, focus, focusin order and change only after input on a select, all read by the Tab, Enter, Space and select rows
standard-contracts#5.1 · no · the scroll API, scroll_into_view, visible_region and the hit walk are the task's Not needed list, and the Tab rows bind focusability rules rather than the NodeTree, traversal or style-resolve helpers
standard-contracts#6.1 · no · transformed client rects are the task's Not needed list and no stand row re-read here depends on them
standard-contracts#6.2 · no · text_extent feeds only the visual lint, which the task's Not needed list excludes
standard-contracts#6.3 · no · the hit walk's skip of a dropped node is hit testing, which the task's Not needed list excludes
standard-contracts#7.1 · no · the layout pass's Taffy traits, style accessors and paint-tree API are the task's Not needed layout surface
standard-contracts#8.1 · yes · the range input's RangeInput element and range_input_value, the upstreamable change the re-read reads against (the Timer's slider and its snapshot value); the widget and serialization items bind no row
standard-contracts#9.1 · yes · ElementData::is_disabled, read by presence on a form control alone, which decides focusability, the enabled-state Tab sequences and the Enter and Space gates
standard-contracts#9.2 · yes · the select's rules in crate-private node::select (SelectStep, Typeahead, the 1 s TYPEAHEAD_TIMEOUT and the readers) that the select row exercises, read through select_value or the snapshot value since the module is crate-private
standard-contracts#9.3 · no · the out-of-flow flag for absolutely positioned boxes is layout-pass state, the task's Not needed layout surface, and no row or check the chunk makes reads it
standard-contracts#9.4 · no · the class atoms are Stylo's selector input, which no chunk row, widened check or code path reads or writes directly
standard-contracts#10.1 · yes · the headless stand's pinned 800x600 Light Viewport and its offline boot (NetProvider is_noop), the ground every row reads; the navigation, devtools and parser providers bind no row
standard-contracts#11.1 · no · HTML parsing, networking and resource loading are the task's Not needed list, and the offline stand takes no network path
