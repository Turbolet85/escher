# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#6.3 · no · the WPT runner's result lines in wpt/runner are not reached, and the list's not-colour-alone rule over its own options does not reach that runner
visual-design-verification#6.4 · no · a comment in examples/form.rs, a file the chunk does not reach
visual-design-verification#7.1 · yes · the viewport's light or dark scheme is an input of the list's new colour default, which the contrast reader measures under SC 1.4.3 (the colours themselves are the P4 fork)
visual-design-verification#7.2 · no · the stylist device rebuild on a scheme change sits in stylo_device.rs, which is not reached, and the scope does not name it
visual-design-verification#7.3 · no · hidpi and zoom shaping invalidation is engine mechanism in stylo_device.rs, not named by the scope and not a family or size the contrast rows read
visual-design-verification#7.4 · no · the page zoom key is a shell binding in blitz-shell window.rs, not reached, and the select's key rule covers document keys only
visual-design-verification#7.5 · no · rdme's prefers-color-scheme tokens in apps/readme are not reached
visual-design-verification#7.6 · no · the TodoMVC and reference pages' color-scheme declarations in examples/preact are not reached
visual-design-verification#7.7 · no · the default.css un-invert rule covers images and video, which the list adds none of, so the list's new default styles do not reach it
visual-design-verification#7.8 · no · touch-action panning in pointer.rs is another unit that must keep holding, a re-check every change owes, not a click unit the change edits
visual-design-verification#8.1 · yes · the option's display none carve-out under select, inert still excluding, and a hit passing through inert with inert removed restoring it, are named by the hidden-content carve-out and the inert hit need
visual-design-verification#8.2 · yes · the list's options take hits only where they paint, so visibility hidden and collapse must stop a hit on an overlay option, which the overlay hit code must honour
visual-design-verification#8.3 · no · the zero-determinant transform hit answer is not named by the scope and is pinned by its own existing test, a re-check every change owes
visual-design-verification#8.4 · no · pointer-events none is not named by the scope, and the existing hit path already honours it
visual-design-verification#8.5 · no · scrollbar-width none is in scrollbars.rs, not reached, and the list's scrolling is not named by the scope
visual-design-verification#8.6 · yes · a driver click on an option inside a just-scrolled overlay scrollbar band is swallowed until an advance passes the fade, which the driver acting on an open list's option must honour
visual-design-verification#8.7 · no · devtools layout outlines and overlays in blitz-traits are not reached and the chunk does not name them
