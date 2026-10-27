# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.3 · no · blitz-dom's hidpi or zoom invalidation of text shaping; the chunk changes no scale or shaping code and no lint boot makes a scale change
visual-design-verification#7.4 · no · the shell's keyboard page zoom (window.rs); the chunk reaches no shell code and no lint or sweep step presses a zoom key
visual-design-verification#7.5 · no · rdme's prefers-color-scheme tokens in apps/readme; the chunk reaches no rdme file and the lint reads no rdme page
visual-design-verification#7.6 · no · the colour-scheme declarations of the example pages (examples/preact); the chunk reaches no example page and changes no stand markup
visual-design-verification#7.7 · no · the default stylesheet's inverted-colors rule for images and video (default.css); the lint reads no image or video and paints nothing
visual-design-verification#7.8 · no · touch panning per axis in pointer.rs; the chunk changes no event code and the lint reads no pointer event
visual-design-verification#8.1 · yes · the lint walks the Snapshot, so its exclusion of display:none, visibility:hidden and inert nodes with their descendants, and its keeping a select's options with their select's client rect as bounds, bind the refactored walk (Needs)
visual-design-verification#8.2 · no · the hit-test target rule for visibility hidden or collapse (node.rs); the lint runs no hit test and reads the snapshot, so the chunk does not reach it
visual-design-verification#8.3 · no · the pointer-events: none hit transparency (node.rs); the lint runs no hit test and the chunk changes no hit path
visual-design-verification#8.4 · no · scrollbar-width: none suppression of overlay scrollbar paint (scrollbar.rs, scrollbars.rs); the lint reads snapshot bounds, paints nothing and the scope names no scrollbar
visual-design-verification#8.5 · no · a pointerdown in a shown overlay scrollbar band on a Cells cell (hit_with_scrollbar, stand_cells_served.rs); no lint or sweep step in Needs clicks a Cells cell or scrolls into a band
visual-design-verification#8.6 · no · devtools layout outlines and highlight overlays (blitz-traits devtools.rs); the lint reads no devtools setting and the chunk paints nothing
