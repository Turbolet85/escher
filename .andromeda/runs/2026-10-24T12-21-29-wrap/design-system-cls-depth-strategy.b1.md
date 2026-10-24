# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser z-index values sit in browser.css, which the session does not touch
depth-strategy#1.2 · no · the browser menu and suggestion shadows sit in browser.css, which the session does not touch
depth-strategy#1.3 · no · the seven_guis card and dialog shadows sit in task files the session does not touch, and no stand CSS or markup moved
depth-strategy#1.4 · no · the todomvc shadows sit in todomvc.css, which the session does not touch
depth-strategy#1.5 · no · the wgpu_texture, custom-widget and transforms z-index values sit in files the session does not touch, and the new multi_doc example records no depth value
depth-strategy#1.6 · no · the dialog backdrop sits in blitz-dom's default.css, which the session does not touch, and no engine colour default changed
depth-strategy#2.1 · yes · stale · resolve.rs gained two import lines at 29 and 32, so its cited 121-125 now sit at 123-127
depth-strategy#2.2 · yes · stale · node.rs gained lines at 125-128, 425 and 1271-1279 above its cited 1323-1366 and 1368-1397, so both ranges moved
depth-strategy#3.1 · yes · add-to · the hit walks over hoisted, paint and inline children now skip nodes dropped since the last resolve, a new fact pinned by hit_test_dropped_nodes, and its node.rs:1521-1617 citations moved
depth-strategy#4.1 · no · the paint-order claim sits in paint_order.rs, which the session does not touch, and the new out-of-flow state changes no paint
depth-strategy#4.2 · no · the outset shadow claim sits in blitz-paint's box_shadow.rs and its test, which the session does not touch
depth-strategy#4.3 · no · the inset shadow claim sits in blitz-paint's box_shadow.rs, which the session does not touch
depth-strategy#4.4 · no · the opacity and filter layer claim sits in blitz-paint's render.rs, which the session does not touch
depth-strategy#5.1 · no · the filter and z-index fixtures sit in examples/assets files the session does not touch
depth-strategy#5.2 · no · the google and graphite shadow and z-index values sit in fixture pages the session does not touch
depth-strategy#5.3 · no · the hr, servo, preact and box-shadow fixtures sit in files the session does not touch, and the new multi_doc example records no shadow value
depth-strategy#6.1 · no · the session adds no z-index or shadow handling and changes no border-radius handling, so the absence stands
depth-strategy#6.2 · no · the session adds no shadow or elevation token, so the absence stands
depth-strategy#6.3 · no · the session adds no shadow or elevation definition, so the absence stands
depth-strategy#6.4 · no · the session adds no shadow, elevation or z-index token and changes no border-radius handling, so the radius touch-point stays and the absence stands
