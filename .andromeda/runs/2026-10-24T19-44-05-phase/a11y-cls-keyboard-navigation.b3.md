# Classified — a11y · keyboard-navigation.b3

## Answers
keyboard-navigation#6.2 · no · a fixture's own markup in examples/assets, which the chunk does not reach; the stand's Tab orders rest on the engine's tabindex rule, already carried by the focusability Need
keyboard-navigation#6.3 · no · the google.html jsaction keydown handlers are page code in an unreached fixture; the chunk's Enter and Space rule covers native controls, not a role=button div
keyboard-navigation#6.4 · no · the google.html :focus-visible outline is focus-indicator styling, which the chunk changes none of (Not needed: focus-ring tokens)
keyboard-navigation#6.5 · no · the graphite.html border-color focus marking is focus-indicator styling, which the chunk changes none of (Not needed: focus-ring tokens)
keyboard-navigation#6.6 · no · the servo.css box-shadow focus ring is focus-indicator styling, which the chunk changes none of (Not needed: focus-ring tokens)
keyboard-navigation#6.7 · no · servo.html's tabindex=-1 headings are markup of an unreached example; the chunk's Tab orders rest on the engine's focusability rule, not on this page
keyboard-navigation#6.8 · yes · the widened accessibility_label witness asserts aria-labelledby outranking a `<label>`, a read that needs for/id and nesting association as this entry states (Need: the merged upstream reads)
keyboard-navigation#7.1 · yes · the focus-agreement rule (engine focus, snapshot focused node and tree focus agree at every step; the first boot Tab on back-btn) is the harness pattern stand_keyboard re-asserts in its reached file
keyboard-navigation#7.2 · yes · stand_keyboard, a reached file, walks each lean task's Tab order with Enter and Space activation on stand buttons, the CRUD list keys and the inert skip, which the chunk re-asserts at boot and in the enabled states
keyboard-navigation#7.3 · yes · the Timer slider's arrow, Home and End model and its click focus are the range model the Tab and activation re-read passes over (Need: the range input)
keyboard-navigation#7.4 · yes · the single-choice select's keyboard model and its one input then one change per choice are what the flight booker select row and the widened select_interaction witness assert (Need: the select model)
keyboard-navigation#8.1 · no · an observed-absent search of `tabindex` over the old s06 files, a historical record and not a rule over the stand checks (Not needed: observed-absent searches)
keyboard-navigation#8.2 · no · an observed-absent search of `tabindex|tabIndex` over the old s10 files, a historical record and not a rule over the stand checks (Not needed: observed-absent searches)
keyboard-navigation#8.3 · no · an observed-absent search of `focus|tabindex|keyboard` over the old s07 files, a historical record and not a rule over the stand checks (Not needed: observed-absent searches)
