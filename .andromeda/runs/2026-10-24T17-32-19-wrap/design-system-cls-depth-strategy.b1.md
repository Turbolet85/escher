# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser's z-index values sit in apps/browser/assets/browser.css, which the session does not reach
depth-strategy#1.2 · no · the browser menu's shared shadow sits in apps/browser/assets/browser.css, which the session does not reach
depth-strategy#1.3 · no · the seven_guis card and dialog shadows sit in examples/seven_guis/src/tasks, and the one seven_guis file reached is a test
depth-strategy#1.4 · no · the todomvc layered shadows sit in examples/todomvc/src/todomvc.css, which the session does not reach
depth-strategy#1.5 · no · the wgpu_texture, custom_widget and transforms z-index values sit in files the session does not reach
depth-strategy#1.6 · no · the dialog backdrop sits in packages/blitz-dom/assets/default.css, which the session does not reach
depth-strategy#2.1 · no · the stacking-context re-exports sit in blitz-dom's resolve.rs and lib.rs, neither reached, and no engine file moved
depth-strategy#2.2 · no · the stacking-context root rules sit in blitz-dom's node.rs, which the session does not reach
depth-strategy#3.1 · no · the hit rule is engine code the session did not reach, its cited test is unreached, the 2026-10-14 WITNESSES record is byte-unchanged, and the entity names no runner
depth-strategy#4.1 · no · paint_order.rs is unreached, the paint-level rule is unchanged, and the entity names no runner
depth-strategy#4.2 · no · the outset-shadow rule sits in blitz-paint's box_shadow.rs and outset_box_shadow_shape.rs, neither reached
depth-strategy#4.3 · no · the inset-shadow code sits in blitz-paint's box_shadow.rs, which the session does not reach
depth-strategy#4.4 · no · the opacity and filter layer sits in blitz-paint's render.rs, which the session does not reach
depth-strategy#5.1 · no · the drop-shadow, backdrop-filter and z-index fixtures sit in examples/assets, which the session does not reach
depth-strategy#5.2 · no · the google and graphite shadows and z-index values sit in examples/assets, which the session does not reach
depth-strategy#5.3 · no · the hr, servo, Preact and box-shadow fixture values sit in files the session does not reach, and its new test pages carry no shadow
depth-strategy#6.1 · no · the claim holds on the changed files: the session adds no z-index, shadow or radius token, its new pages holding overflow, width, padding, direction, writing-mode and transform only
depth-strategy#6.2 · no · the claim holds on the changed files: the session adds no shadow or elevation token
depth-strategy#6.3 · no · the claim holds on the changed files: the session adds no shadow, elevation or z-index definition
depth-strategy#6.4 · no · the claim holds on the changed files: the session adds no radius, shadow, elevation or z-index token, and radius is untouched per the task's Untouched list
