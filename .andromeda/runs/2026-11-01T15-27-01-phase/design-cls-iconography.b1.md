# Classified — design · iconography.b1

## Answers
iconography#1.1 · yes · the cursor is read from the hit node's computed CSS cursor keyword and falls back to Default over no box, so the hit answer that the Needs' node.rs hit-testing changes (ebacd68bd on nodes with no computed styles) feeds it
iconography#1.2 · no · the keyword-to-CursorIcon map, the cursor-icon crate and the shell cursor calls sit in blitz-dom, blitz-traits and blitz-shell, none reached; the scope names winit's version and clipboard API, not its cursor values
iconography#1.3 · no · favicon_url's `<link rel icon>` lookup in document.rs is not reached and the scope names no favicon behaviour
iconography#2.1 · no · a cursor-value fixture in examples/assets, and the scope changes no fixture of an example other than examples/seven_guis
iconography#2.2 · no · inline SVG octicon and google fixtures in examples/assets, not named by the scope
iconography#2.3 · no · Font Awesome fixture classes in examples/assets, not named by the scope
iconography#2.4 · no · graphite atlas and SVG data-URI fixture in examples/assets, not named by the scope
iconography#2.5 · no · currentColor svg fixtures and the svg_native example, other examples the scope does not change; currentColor resolution is not a named value
iconography#3.1 · no · absence of icon handling over the s08 files, none reached; the chunk adds no icon handling
iconography#3.2 · no · absence of icon sets over the s11 files, none reached; the chunk adds no icon set
