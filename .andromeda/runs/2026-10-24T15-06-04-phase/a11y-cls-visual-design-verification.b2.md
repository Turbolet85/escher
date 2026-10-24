# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.4 · no · page zoom is a windowed key reading in blitz-shell's window.rs; zoom is a Not-needed area and no shell file is reached
visual-design-verification#7.5 · no · rdme's prefers-color-scheme tokens are colour scheme, a Not-needed area, and apps/readme is not reached
visual-design-verification#7.6 · no · the TodoMVC and reference pages' color-scheme declarations are colour scheme, a Not-needed area, and examples/preact is not reached
visual-design-verification#7.7 · no · the inverted-colors un-invert of images and video is user-preference media, a Not-needed area, and blitz-dom's default.css is not reached
visual-design-verification#7.8 · no · touch-action panning in pointer.rs is no witness the sweep names (its click focus and double click are a separate path), and pointer.rs is not reached
visual-design-verification#8.1 · yes · the inert rule (out of the tree and snapshot, passed through by a hit, restored on removal) is v020-03's witness, and display none keeps a select's options in the tree and snapshot, as the Visibility and hit-testing need names
visual-design-verification#8.2 · no · the visibility need names visibility hidden only for tree and snapshot exclusion (8.1), so its hit rule for visibility hidden or collapse is no named witness, and node.rs is not reached
visual-design-verification#8.3 · no · pointer-events transparency to hits is no need the chunk names, and node.rs is not reached
visual-design-verification#8.4 · no · scrollbar-width suppression of overlay scrollbars is no need the chunk names, and no scrollbar file is reached
visual-design-verification#8.5 · no · devtools overlay settings are no need the chunk names, and blitz-traits' devtools.rs is not reached
