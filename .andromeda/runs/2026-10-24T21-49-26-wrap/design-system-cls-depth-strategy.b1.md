# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser's z-index values sit in apps/browser/assets/browser.css, which the session does not reach
depth-strategy#1.2 · no · the browser menu and suggestion shadow sits in apps/browser/assets/browser.css, which the session does not reach
depth-strategy#1.3 · no · the seven_guis card and dialog shadows sit in the stand's task files, which the session does not reach and no stand code moved
depth-strategy#1.4 · no · the todomvc layered shadows sit in examples/todomvc/src/todomvc.css, which the session does not reach
depth-strategy#1.5 · no · the wgpu_texture, custom-widget and transforms z-index values sit in example files the session does not reach
depth-strategy#1.6 · no · the blitz-dom default dialog backdrop sits in packages/blitz-dom/assets/default.css, which the session does not reach and no stylesheet moved
depth-strategy#2.1 · no · the stacking-context build and its re-exports sit in blitz-dom's resolve.rs and lib.rs, which the session does not reach and no engine code moved
depth-strategy#2.2 · no · the stacking-context root conditions sit in packages/blitz-dom/src/node/node.rs, which the session does not reach and no engine code moved
depth-strategy#3.1 · no · the hit-test order and padding-box stop sit in node.rs, which the session does not reach, and the task's Untouched list keeps the depth hit-testing stop unchanged
depth-strategy#4.1 · no · the z-index auto paint order and its paint_order.rs check sit outside the reached files, and no paint code moved
depth-strategy#4.2 · no · the outset shadow rule sits in blitz-paint's box_shadow.rs and its check, which the session does not reach
depth-strategy#4.3 · no · the inset shadow rule sits in blitz-paint's box_shadow.rs, which the session does not reach
depth-strategy#4.4 · no · the opacity, filter and backdrop layer sits in blitz-paint's render.rs, which the session does not reach
depth-strategy#5.1 · no · the drop-shadow, backdrop-filter and z-index fixtures sit in examples/assets/, which the session does not reach
depth-strategy#5.2 · no · the google and graphite shadows and z-index values sit in examples/assets/, which the session does not reach
depth-strategy#5.3 · no · the hr, servo, preact and shadow fixtures sit in examples/, which the session does not reach
depth-strategy#6.1 · no · the session's edits are test-target names in the reached CI files, none carrying a border-radius, box-shadow, z-index or z_index token, and the task's Touches records this row as no-change
depth-strategy#6.2 · no · the session's edits are test-target names in the reached CI files, none carrying a shadow or elevation token, so the no-match claim still holds
depth-strategy#6.3 · no · the session's edits are test-target names in the reached CI files, none carrying a box-shadow, shadow, elevation or z-index token, so the definitions claim still holds
depth-strategy#6.4 · no · the session's edits are test-target names in the reached CI files, none carrying a radius, shadow, elevation or z-index token, so the absence claim still holds
