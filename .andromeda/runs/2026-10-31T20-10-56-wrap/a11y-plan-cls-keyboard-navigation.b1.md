# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · no · the focusability rule lives in unreached element.rs, the session changes no focusability reading, and its test citation stand_keyboard.rs:889-930 sits above the 1055 append and holds
keyboard-navigation#1.2 · yes · add-to · F6, a select made inert, hidden, disabled or removed while its list is open now closes the list, which this flush clear of focus does not mention
keyboard-navigation#1.3 · yes · add-to · F4 and F7, with the list open Tab chooses and closes before moving focus in either direction, and focus stays on the select while it is open
keyboard-navigation#1.4 · no · the cached focusability recompute lives in unreached mutator.rs, and closing a list on disable changes no focusability
keyboard-navigation#1.5 · no · the disabled toggles and button focusability live in unreached element.rs and node.rs and the session changes neither, the list closing on disable being carried by #1.6
keyboard-navigation#1.6 · yes · add-to · F6, a disabled select opens nothing and a disabled row changes nothing and leaves the list open, on the is_disabled presence the Enter and Space activation already reads
keyboard-navigation#1.7 · no · the file input's inner button tabindex is set in unreached mutator.rs, which the session does not touch
keyboard-navigation#1.8 · no · autofocus is read in unreached mutator.rs and no autofocus fact about a select or list is new
keyboard-navigation#1.9 · yes · stale · document.rs is reached whole and holds the FOCUS and FOCUSRING setter at 2249-2254, and the report does not show that claim holds on the changed file, so its citation may have moved
keyboard-navigation#1.10 · yes · stale · stylo.rs is reached whole and holds the :focus and :focus-within match at 488-492, and the report does not show that claim holds on the changed file, so its citation may have moved
keyboard-navigation#1.11 · no · focus and blur bubbling is set in unreached blitz-traits events.rs, which the session does not touch
keyboard-navigation#2.1 · yes · add-to · F6, removing a focused select whose list is open now also closes that list, beside the body-focus reset and the blur side effects
keyboard-navigation#2.2 · yes · add-to · F5, a click outside an open list closes it with the value unchanged and no event, beside the non-interactive click that clears focus
keyboard-navigation#2.3 · yes · add-to · F7 and F8, a click on a single select now also opens its list with focus staying on it, and a row under the pointer takes the highlight with the release choosing it, while the closed-option refusal stands
keyboard-navigation#2.4 · no · the held claim click_select_row covers a multiple select's row click, which the session leaves unchanged, and only citation digits may have moved
keyboard-navigation#2.5 · yes · stale · pointer.rs is reached whole and holds the double-click timing at 397-407, and the report does not show that claim holds on the changed file, so its citation may have moved
keyboard-navigation#2.6 · no · the report states the rule for summaries is unmoved, so the first summary's toggle and focus hold
keyboard-navigation#2.7 · no · the report states that :open and flush_open_state are unmoved, so the details and dialog open-state claim holds
keyboard-navigation#2.8 · yes · stale · pointer.rs is reached whole and holds the checkbox and radio click arms at 854-919, and the report does not show that claim holds on the changed file, so its citation may have moved
keyboard-navigation#2.9 · yes · stale · pointer.rs is reached whole and holds the checkbox and radio Change dispatch at 865-908, and the report does not show that claim holds on the changed file, so its citation may have moved
keyboard-navigation#2.10 · yes · stale · pointer.rs is reached whole and holds the range-input click and pointerdown arms at 462-753 and 840-852, and the report does not show that claim holds on the changed file, so its citation may have moved
keyboard-navigation#3.1 · no · text-input key handling lives in unreached node/text.rs, and the report states the Enter and Space rule for inputs is unmoved
keyboard-navigation#3.2 · yes · stale · keyboard.rs is reached whole and holds the range-input key arms at 106-111, 156-173 and 532-553, and the report does not show that claim holds on the changed file, so its citation may have moved
keyboard-navigation#3.3 · yes · contradict · a closed single select's Enter and Space now open its list, a released Space does nothing more, Escape and the other close triggers close it, and the move keys highlight (F2, F3, F5, F9), so "every other key does nothing" and "no popup is painted" no longer hold
keyboard-navigation#3.4 · yes · add-to · F11, on a multiple select with a valid size a key now brings the current row into view through scroll_into_view, and the other row semantics stand
