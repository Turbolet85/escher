# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · no · the predicate lives in unedited element.rs and node.rs, stand_keyboard.rs:889-930 sits above the 1055 append and holds, and the Tab walk holds with a list open (F7)
keyboard-navigation#1.2 · no · the flush clear_focus lives in unedited mutator.rs, and a list closed by inertness fires no event (F5), so the no-blur claim does not move
keyboard-navigation#1.3 · yes · add-to · F4: with a list open, Tab chooses and closes it before moving focus in either direction, and the walk holds with it open (F7)
keyboard-navigation#1.4 · no · the recompute lives in unedited mutator.rs and focusability_updates.rs, and no select's tabindex, href or disabled rule moved
keyboard-navigation#1.5 · no · the disabled toggle lives in element.rs, node.rs and focusability_updates.rs, none of which the session reached
keyboard-navigation#1.6 · yes · stale · the Enter and Space activation reads sit in reached keyboard.rs and pointer.rs, whose cited lines can have moved; the presence reading holds (a disabled select opens nothing, F6)
keyboard-navigation#1.7 · no · the inner button's tabindex lives in mutator.rs, which the session did not reach
keyboard-navigation#1.8 · no · the autofocus rule lives in mutator.rs and autofocus_attribute.rs, neither reached, and no autofocus behaviour moved
keyboard-navigation#1.9 · yes · stale · document.rs is reached and its cited lines 2249-2254 can have moved; the focus-state set and clear hold, focus staying on a select while its list is open (F7)
keyboard-navigation#1.10 · yes · stale · stylo.rs is reached and its cited lines 488-492 can have moved; the :focus and :focus-within reading holds
keyboard-navigation#1.11 · no · the bubbling rule lives in blitz-traits events.rs, which the session did not reach, and no focus or blur event rule moved
keyboard-navigation#2.1 · yes · add-to · F6: a select removed while its list is open closes the list with no event, beside the focus reset to the body and the blur side effects
keyboard-navigation#2.2 · yes · add-to · F5: a click outside an open list closes it with the value unchanged and no event, beside the clear of focus on a non-interactive area
keyboard-navigation#2.3 · yes · add-to · F7 and F8: a click on a single select now opens its list and focus stays on the select, and a press on an enabled row starts no drag with its release choosing that row; the closed-option refusal stands
keyboard-navigation#2.4 · yes · stale · click_select_row and the multiple-row rule hold, but the cited pointer.rs:1121-1162, select_interaction.rs and stand_act_select.rs:363-388 sit in reached files whose digits can have moved
keyboard-navigation#2.5 · yes · stale · the double-click rule holds, but pointer.rs:397-407 sits in a reached file whose cited digits can have moved
keyboard-navigation#2.6 · yes · stale · pointer.rs:920-946 sits in a reached file whose cited digits can have moved; the summary rule itself is unmoved
keyboard-navigation#2.7 · yes · stale · stylo.rs:510 is a citation into a reached file whose digit can have moved; :open and flush_open_state are unmoved, so the claim holds
keyboard-navigation#2.8 · yes · stale · pointer.rs:854-919 is a citation into a reached file whose digits can have moved; no touch names the checkbox and radio toggle rule
keyboard-navigation#2.9 · yes · stale · pointer.rs:865-871, 883 and 899-908 sit in a reached file whose digits can have moved; no touch names the checkbox and radio change rule
keyboard-navigation#2.10 · yes · stale · pointer.rs:462-466, 522-524, 725-753 and 840-852 sit in a reached file whose digits can have moved; no touch names the range rule
keyboard-navigation#3.1 · no · the text-input key rule lives in node/text.rs, which the session did not reach, and no text-input key moved
keyboard-navigation#3.2 · yes · stale · keyboard.rs:106-111, 156-173 and 532-553 are citations into a reached file whose lines can have moved; no touch names the range arrow and Home/End rule
keyboard-navigation#3.3 · yes · contradict · Enter and Space now open the list (F2), so "Enter and Space open nothing" and "no popup is painted" are retired; the arrow, Home, End and typeahead keys still hold
keyboard-navigation#3.4 · yes · add-to · F11: on a multiple select with a valid size, a key also brings the current row into view through scroll_into_view, and its other row semantics are unchanged
