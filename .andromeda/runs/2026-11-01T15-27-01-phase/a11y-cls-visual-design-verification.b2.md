# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#4.1 · no · a fact about the BBC fixture's media queries; the fixture is not a reached file and no named check reads it
visual-design-verification#5.1 · no · the recorded absence of prefers-reduced-motion and prefers-color-scheme rules, which the task's Not-needed list drops
visual-design-verification#5.2 · yes · names stand_act_popup's no-advance screen read on an open list, the select list and its stand_act_popup assertions the chunk reaches
visual-design-verification#6.1 · no · disabled icon button opacity in apps/browser/assets/browser.css; that file and its app are not reached
visual-design-verification#6.2 · yes · the :open match at stylo.rs:510, a predicted conflict surface, and open_state's rows
visual-design-verification#6.3 · yes · an open select's list is no :open state; the select model and select_popup.rs:628-651 assert it
visual-design-verification#6.4 · no · WPT runner status-word output in wpt/runner/src/main.rs; not reached and no named check reads it
visual-design-verification#6.5 · yes · the open list's group header weight by synthesized bold, asserted by select_popup_painted and bound by the text.rs conflict and the optgroup header weight on every build
visual-design-verification#6.6 · no · a comment in examples/form.rs; not reached and no check reads it
visual-design-verification#7.1 · no · viewport colour scheme and zoom in blitz-traits shell.rs; not reached and not in the needs
visual-design-verification#7.2 · no · colour-scheme device change in stylo_device.rs; not reached and not in the needs
visual-design-verification#7.3 · no · scale change invalidating text shaping in stylo_device.rs; not reached and not in the needs
visual-design-verification#7.4 · no · keyboard page zoom steps in blitz-shell window.rs; not reached and not in the needs
visual-design-verification#7.5 · no · rdme colour-scheme tokens in apps/readme assets; not reached and not in the needs
visual-design-verification#7.6 · no · TodoMVC and reference page colour-scheme declarations in examples/preact; not reached
visual-design-verification#7.7 · yes · the inverted-colors un-invert rule lives in blitz-dom's default.css, a predicted conflict surface, and the default sheet's values bind through it
visual-design-verification#7.8 · no · touch-action panning in events/pointer.rs; not reached and not in the needs
visual-design-verification#8.1 · yes · hidden, inert and select-option exclusion from the accessibility tree and the hit passing through inert, through the node.rs conflict, accessibility.rs, select.rs and inert_subtree
visual-design-verification#8.2 · yes · visibility hidden or collapse never hit-tested, a rule at node.rs:1448-1456, a predicted conflict surface in the hit stops
visual-design-verification#8.3 · yes · a non-invertible transform takes no hit, a rule at node.rs:1472-1477, a predicted conflict surface in the hit stops
visual-design-verification#8.4 · yes · pointer-events none transparent to hits, a rule at node.rs:1458-1462 and 1668-1676, a predicted conflict surface in the hit stops
visual-design-verification#8.5 · no · scrollbar-width none in scrollbar.rs; that file and scrollbars.rs are not reached and not in the needs
visual-design-verification#8.6 · yes · the overlay scrollbar band taken as a thumb drag and the fade on the virtual clock, the clock invariant, witnessed by stand_cells_served and the scrolling.rs conflict
visual-design-verification#8.7 · yes · the open list paints past opacity ancestors and refuses covered clicks, asserted by select_popup_painted, select_popup and stand_act_popup, the select list the chunk reaches
visual-design-verification#8.8 · no · devtools overlay settings in blitz-traits devtools.rs; not reached and not in the needs
