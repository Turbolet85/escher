# Classified — a11y · keyboard-navigation.b4

## Answers
keyboard-navigation#5.12 · no · Android Activity focus in touch mode lives in host files (apps/browser, examples/todomvc) the chunk does not reach and outside its focus-order and activation needs
keyboard-navigation#6.1 · no · the docs.rs toolbar tabindex=-1 is an examples/assets fixture the chunk neither reaches nor re-reads
keyboard-navigation#6.2 · no · the google fixture's role=button and tabindex=0 on non-buttons is an examples/assets fixture the chunk does not reach or re-read
keyboard-navigation#6.3 · no · the google fixture's jsaction keydown binding is an examples/assets fixture outside the reached files and the stand checks
keyboard-navigation#6.4 · no · the google fixture's :focus-visible outline is an examples/assets fixture; no chunk check reads focus-visible styling
keyboard-navigation#6.5 · no · the graphite fixture's outline removal is an examples/assets fixture no chunk check reads
keyboard-navigation#6.6 · no · servo.css's button focus ring is an examples/assets fixture no chunk check reads
keyboard-navigation#6.7 · no · servo.html's heading tabindex=-1 is an examples/assets fixture no chunk check reads
keyboard-navigation#6.8 · yes · label association by for/id and nesting is the accessible-name source the re-read accessibility_label check witnesses
keyboard-navigation#7.1 · yes · the first Tab to back-btn and the Tab and Shift+Tab focus steps, with snapshot focused and tree focus agreeing, are the tier-0 focus-order and tree-identity proof the task re-proves, and the mutator's flush focus fixup can move them
keyboard-navigation#7.2 · yes · stand_keyboard's walks of the stated Tab orders, Enter and Space activation, and the keys that do not activate, re-read in both layout modes
keyboard-navigation#7.3 · yes · LOAD_TAB_ORDER and CELLS_TAB_ORDER are the stated Tab orders the Needs name, read by stand_keyboard
keyboard-navigation#7.4 · yes · the range input's keys and clicks are the arm the Needs name beside the select model in the mutator.rs and node.rs conflicts, and stand_act_range drives it
keyboard-navigation#7.5 · yes · the range key and pointerdown firing input and not change is the range arm's event behaviour inside the conflicted mutator and node code
keyboard-navigation#7.6 · yes · select_interaction and stand_act_select's keys, typeahead on the document's clock, and list rows are the select model and list the mutator.rs and node.rs conflicts and the clock invariant reach
keyboard-navigation#7.7 · yes · select_popup, select_popup_painted, stand_act_popup and select_list's unit tests are the select list's merge surface and the font-embolden witness the Needs name
keyboard-navigation#7.8 · yes · stand_keyboard's flight-type select arrows, Enter and Space are the single-choice select model the mutator.rs and node.rs conflicts reach
keyboard-navigation#8.1 · no · a past tabindex absence search over slice files the chunk's named files do not include, a record the task drops as not needed
keyboard-navigation#8.2 · no · a past tabindex absence search over s10 slice files the chunk does not include, a record the task drops as not needed
keyboard-navigation#8.3 · no · a past focus and keyboard absence search over s07 slice files the chunk does not include, a record the task drops as not needed
