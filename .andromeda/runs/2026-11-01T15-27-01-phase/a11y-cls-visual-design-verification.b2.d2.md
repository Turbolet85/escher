# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#4.1 · no · the BBC fixture's reduced-motion queries sit in examples/assets, which the chunk does not reach, and the Not-needed list drops the reduced-motion records
visual-design-verification#5.1 · no · observed absence of prefers-reduced-motion and prefers-color-scheme rules, dropped by the Not-needed list
visual-design-verification#5.2 · yes · no motion on an open select list, pinned by stand_act_popup.rs, which the scope names as asserting the select model and its list
visual-design-verification#6.1 · no · disabled icon button opacity in apps/browser/assets/browser.css, a file the chunk does not reach
visual-design-verification#6.2 · yes · `:open` matching in packages/blitz-dom/src/stylo.rs, a predicted conflict surface
visual-design-verification#6.3 · yes · an open select list is no `:open` state, asserted by select_popup.rs, which the scope names under the select model and its list
visual-design-verification#6.4 · no · WPT runner status text in wpt/runner/src/main.rs, outside the chunk's scope
visual-design-verification#6.5 · yes · the open list's group header weight, painted through blitz-paint's text.rs conflict and pinned by select_popup_painted.rs, the font-embolden witness
visual-design-verification#6.6 · no · a comment in examples/form.rs, a file the chunk does not reach
visual-design-verification#7.1 · no · viewport colour scheme and zoom in blitz-traits shell.rs, which the chunk does not reach
visual-design-verification#7.2 · no · colour-scheme device change in stylo_device.rs, which the chunk does not reach
visual-design-verification#7.3 · no · total scale and text shaping in stylo_device.rs, which the chunk does not reach
visual-design-verification#7.4 · no · keyboard page zoom in blitz-shell window.rs, which the chunk does not reach
visual-design-verification#7.5 · no · rdme colour-scheme tokens in apps/readme, which the chunk does not reach
visual-design-verification#7.6 · no · TodoMVC and reference colour-scheme declarations in examples/preact, which the chunk does not reach
visual-design-verification#7.7 · yes · the inverted-colors rule lives in blitz-dom's default.css, a predicted conflict surface
visual-design-verification#7.8 · no · touch-action panning in events/pointer.rs, which the chunk does not reach
visual-design-verification#8.1 · yes · display none, visibility hidden and inert exclusion from the tree, and hits passing through inert, in accessibility.rs and node.rs (a conflict surface), with inert_subtree's rows
visual-design-verification#8.2 · yes · visibility hidden and collapse never hit-tested, in node.rs, a conflict surface under the hit-stops need
visual-design-verification#8.3 · yes · a non-invertible transform takes no hit, in node.rs, a conflict surface under the hit-stops need
visual-design-verification#8.4 · yes · pointer-events none transparent to hits, in node.rs, a conflict surface under the hit-stops need
visual-design-verification#8.5 · no · scrollbar-width none in scrollbar.rs, which the chunk does not reach
visual-design-verification#8.6 · yes · the clock invariant and the overlay scrollbar fade on the document's clock, witnessed by stand_cells_served.rs, which the scope names beside stand_keyboard
visual-design-verification#8.7 · yes · the open list paints past an opacity ancestor and refuses a covered click, asserted by select_popup_painted.rs, select_popup.rs and stand_act_popup.rs under the select model and its list
visual-design-verification#8.8 · no · devtools overlay settings in blitz-traits devtools.rs, which the chunk does not reach
