# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the session changes no browser stylesheet, and apps/browser/assets/browser.css is not reached
depth-strategy#1.2 · no · the browser menu and suggestion shadow sits in apps/browser/assets/browser.css, which the session does not reach
depth-strategy#1.3 · yes · stale · flight_booker.rs is reached whole and gained lines 66-80 and 83-86 above line 148, so the :148 citation moved
depth-strategy#1.4 · no · the todomvc layered shadows sit in examples/todomvc/src/todomvc.css, which the session does not reach
depth-strategy#1.5 · no · the z-index values sit in the wgpu_texture, custom_widget and transforms examples, none of which the session reaches
depth-strategy#1.6 · no · the dialog backdrop is blitz-dom's default stylesheet, an engine default, and no engine crate changed
depth-strategy#2.1 · no · the stacking-context build is blitz-dom code (resolve.rs, lib.rs), and no engine crate changed
depth-strategy#2.2 · no · the stacking-context root rule sits in blitz-dom node.rs, which the session does not reach
depth-strategy#3.1 · no · the hit-testing rule at a paint-clipping box is untouched (no engine crate changed, node.rs not reached), and the overlay-scrollbar fact is motion
depth-strategy#4.1 · no · the shared paint level is pinned by tests/blitz-tests/tests/paint_order.rs, which the session does not reach
depth-strategy#4.2 · no · the outset shadow paint sits in blitz-paint box_shadow.rs and its shape test, neither reached
depth-strategy#4.3 · no · the inset shadow paint sits in blitz-paint box_shadow.rs, which the session does not reach
depth-strategy#4.4 · no · the opacity and filter layers sit in blitz-paint render.rs, which the session does not reach, and the scrollbar fade is motion
depth-strategy#5.1 · no · the drop-shadow, backdrop-filter and z-index fixtures are examples/assets files the session does not reach
depth-strategy#5.2 · no · the google and graphite shadows and z-index values are example fixtures the session does not reach
depth-strategy#5.3 · no · the hr, servo, preact and box-shadow fixtures are not reached by the session
depth-strategy#6.1 · no · the session adds no z-index or shadow handling, and its new CSS names none
depth-strategy#6.2 · no · the session adds no shadow or elevation token, and its new CSS names none
depth-strategy#6.3 · no · the session adds no shadow, elevation or z-index definition
depth-strategy#6.4 · no · no shadow, elevation or z-index is added, and the select's radius 6 sits in flight_booker.rs, which carries a box-shadow the no-shadow reading would have matched
