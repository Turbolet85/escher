# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.4 · no · keyboard page zoom lives in blitz-shell window.rs, outside the reached files and every Needs clause
visual-design-verification#7.5 · no · rdme's colour-scheme tokens live in apps/readme, outside the reached files and every Needs clause
visual-design-verification#7.6 · no · the color-scheme declarations live in examples/preact pages, outside the reached files and every Needs clause
visual-design-verification#7.7 · no · the inverted-colors rule lives in blitz-dom's default.css, outside the reached files and every Needs clause
visual-design-verification#7.8 · no · touch-action panning lives in blitz-dom's pointer.rs, outside the reached files and every Needs clause
visual-design-verification#8.1 · yes · a select's option exempt from display: none is kept in the tree and snapshot as a child of its select, which the select Needs and the re-pinned snapshot checks honour
visual-design-verification#8.2 · no · no chunk need hit-tests a visibility: hidden or collapse element, the return-date input being kept disabled rather than hidden
visual-design-verification#8.3 · no · pointer-events transparency in blitz-dom's node.rs hit code lies outside the reached files and every Needs clause
visual-design-verification#8.4 · no · scrollbar-width suppression lives in blitz-dom's scrollbar.rs and its own test, outside the reached files and every Needs clause
visual-design-verification#8.5 · no · devtools overlay settings live in blitz-traits' devtools.rs, outside the reached files and every Needs clause
