# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser z-index values live in apps/browser/assets/browser.css, which the session did not reach
depth-strategy#1.2 · no · the browser menu shadow lives in apps/browser/assets/browser.css, not reached
depth-strategy#1.3 · no · the seven_guis card and dialog shadows live in layout-templates, and the session edited only the seven_guis tests
depth-strategy#1.4 · no · the todomvc shadows live in examples/todomvc/src/todomvc.css, not reached
depth-strategy#1.5 · no · the wgpu_texture, custom_widget and transforms z-index values live in example sources, none reached
depth-strategy#1.6 · yes · stale · default.css changed past line 183, so the citation at lines 1038-1041 moved while the backdrop value holds
depth-strategy#2.1 · no · resolve.rs:121-125 precedes the past-133 changes and lib.rs:89 is not named as moved, so the re-exports and the after-layout build still hold
depth-strategy#2.2 · no · the stacking-root rules live in node/node.rs, not reached, and the list adds no stacking property
depth-strategy#3.1 · yes · contradict · a list row stands outside a clipping padding box, since hit_with_scrollbar answers select_list_hit first (document.rs:2404-2406), and F8 (the list paints last), F9 and F10 (it answers the hit first) extend this order
depth-strategy#3.2 · no · the zero-determinant rule lives in node/node.rs and its test, neither reached, and the list is engine state outside the node's subtree
depth-strategy#3.3 · yes · contradict · visible_region answers the list's scrollport for a row or group (scrolling.rs:754-756), so a row no longer reads its clipping boxes through its ancestors, and the 798-820 citation moved too
depth-strategy#4.1 · no · positioned descendants keep tree order inside the root render, the list painting after it, and paint_order.rs is not reached
depth-strategy#4.2 · no · the outset shadow rules live in box_shadow.rs and its test, neither reached, and the list draws no shadow
depth-strategy#4.3 · no · the inset shadow rules live in box_shadow.rs, not reached, and the list draws no shadow
depth-strategy#4.4 · yes · stale · render.rs:499-531 moved past the render.rs changes (past line 7), and F9 says the list paints whole past an opacity ancestor
depth-strategy#5.1 · no · the drop-shadow, backdrop-filter and z-index values sit in example fixtures, none reached
depth-strategy#5.2 · no · the google and graphite fixture shadows and z-index values sit in examples/assets, not reached
depth-strategy#5.3 · no · the hr, servo, Preact and box-shadow fixtures sit in examples, not reached
depth-strategy#6.1 · no · the list adds no z-index or shadow text, its stacking being paint order, so the absence holds
depth-strategy#6.2 · no · the list adds no shadow or elevation token, so the absence holds
depth-strategy#6.3 · no · no shadow or elevation definition enters the session
depth-strategy#6.4 · no · the list carries no radius and no z-index token, so the touch-point radius stays the only match
