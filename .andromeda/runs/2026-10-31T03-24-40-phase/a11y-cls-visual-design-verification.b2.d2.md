# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.2 · no · colour-scheme change tracking is colour-scheme handling, on the Not-needed list, and stylo_device.rs is not reached
visual-design-verification#7.3 · no · total-scale invalidation from hidpi or zoom is page zoom, on the Not-needed list, and not the transform scale the Needs name; stylo_device.rs is not reached
visual-design-verification#7.4 · no · keyboard page zoom is page-zoom handling, on the Not-needed list; the new redraw check reads focus_redraw, not the zoom keys
visual-design-verification#7.5 · no · prefers-color-scheme tokens in the rdme app are colour-scheme handling, on the Not-needed list, and apps/readme is not reached
visual-design-verification#7.6 · no · the TodoMVC and reference pages' declared color-scheme is colour-scheme handling, on the Not-needed list; no Need names these pages
visual-design-verification#7.7 · no · un-inverting media under inverted-colors is user-preference handling, on the Not-needed list, and blitz-dom's default.css is not reached
visual-design-verification#7.8 · no · touch-action panning is named by no Need, whose event rows are keys, activation, clicks and change, and no conflict or merge surface is marked
visual-design-verification#8.1 · yes · inert nodes are left out of the tree, passed through by a hit and restored on removal; the Needs name inert as a whole and inert_subtree asserts it
visual-design-verification#8.2 · yes · visibility hidden or collapsed targets are never hits, the answer a driver covered refusal reads, and the Needs name visibility and hit-testing as the driver reads them
visual-design-verification#8.3 · yes · pointer-events none makes an element transparent to hits, the answer a driver covered refusal reads, and the Needs name hit-testing as the driver reads it
visual-design-verification#8.4 · no · scrollbar-width none and overlay scrollbar paint are named by no Need, and scrollbar.rs and scrollbars.rs are not reached
visual-design-verification#8.5 · no · the scrollbar-band thumb drag governs driver clicks on Cells cells, which the chunk does not take (its Cells rows are keyboard and contrast only; selected-cell state is residual line 24)
visual-design-verification#8.6 · no · devtools overlay settings in blitz-traits are named by no Need, and that crate is not reached
