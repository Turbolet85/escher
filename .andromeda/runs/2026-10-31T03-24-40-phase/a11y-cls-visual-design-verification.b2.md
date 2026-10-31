# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.2 · no · colour-scheme device tracking is the task's Not-needed colour-scheme handling, and stylo_device.rs is not reached
visual-design-verification#7.3 · no · hidpi and zoom device scale is the task's Not-needed page zoom handling, not the CSS transform scale P3 measures, and stylo_device.rs is not reached
visual-design-verification#7.4 · no · keyboard page zoom is the task's Not-needed page zoom handling, and the redraw check reads focus_redraw, not zoom keys
visual-design-verification#7.5 · no · rdme's prefers-color-scheme tokens are the task's Not-needed colour-scheme handling, and apps/readme is not reached
visual-design-verification#7.6 · no · colour-scheme declarations on two example pages are the task's Not-needed colour-scheme handling, and neither page is a reached file
visual-design-verification#7.7 · no · inverted-colors un-inversion of images and video is Not-needed user-preference handling in default.css, which is not reached, and the chunk's contrast rows read text
visual-design-verification#7.8 · no · touch-action panning is in no Need, so pointer.rs's touch panning is a re-check every change owes
visual-design-verification#8.1 · yes · inert leaves the tree and the hit and restores on removal, and a select's options stay in the tree despite display none; both are named in the Needs, and inert_subtree.rs's edited test asserts them
visual-design-verification#8.2 · yes · visibility hidden or collapse is never a hit target, the hit-testing the driver's covered and off-screen readings run on (node.rs, reached whole)
visual-design-verification#8.3 · yes · pointer-events none is transparent to hits with descendants still tested, the hit-testing the covered refusal reads (node.rs, reached whole)
visual-design-verification#8.4 · no · scrollbar-width none on overlay bars is a paint rule in scrollbar.rs, which is not reached, and overlay bars take no layout space, so visible_region does not read it
visual-design-verification#8.5 · no · the pointerdown-in-band rule governs driver clicks on a Cells cell, and the chunk's Cells rows are keyboard and contrast only, with no click in a bar
visual-design-verification#8.6 · no · devtools outline and highlight settings are paint overlays in blitz-traits, which is not reached and sits in no Need
