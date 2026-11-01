# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · browser z-index values in apps/browser/assets/browser.css; that file is not reached and the Not needed list drops its depth values
depth-strategy#1.2 · no · browser menu and suggestion box-shadow in apps/browser/assets/browser.css; not reached and in Not needed
depth-strategy#1.3 · no · a pointer to the seven_guis card and circle-dialog shadows held in layout-templates; it states no value, and the cells.rs resolution does not reach them
depth-strategy#1.4 · no · todomvc layered shadows; examples other than seven_guis are in Not needed
depth-strategy#1.5 · no · wgpu_texture, custom-widget and transforms z-index values; those examples are not reached and the scope names no change to them
depth-strategy#1.6 · yes · the blitz-dom default dialog backdrop rgba in default.css, a conflict surface, is a recorded engine default depth value that the Needs read against the resolved sheet
depth-strategy#2.1 · no · paint-after-layout behaviour in resolve.rs, not reached; the re-exports at lib.rs:89 sit outside the edited lines 96-97, a re-check only
depth-strategy#2.2 · yes · the stacking-context root conditions in node.rs, a conflict surface, include isolation and the atomic paint effects that 6fca6308d's mix-blend-mode and isolation touch
depth-strategy#3.1 · yes · the hit walk order and padding-box stop in node.rs, a conflict surface, which the Needs keep through a resolution
depth-strategy#3.2 · yes · an open select list answering the hit first and refusing a covered button is the rows-answered-before-the-walk the Needs name
depth-strategy#3.3 · yes · a non-invertible transform answering no hit in node.rs, where 0980cce5f's hit_inner hunk conflicts with the non-invertible answer
depth-strategy#3.4 · yes · visible_region in scrolling.rs, a conflict surface, with its transform handling and select-list scrollport, which the into-view Need keeps
depth-strategy#4.1 · no · z-index auto paint order lives in render.rs and paint_order.rs, neither reached; no Needs entry names paint order
depth-strategy#4.2 · no · the open list painting last in paint_scene lives in render.rs, not reached; the scope names no change to paint order
depth-strategy#4.3 · no · outset box-shadow clipping in box_shadow.rs, not reached; no Needs entry names shadows
depth-strategy#4.4 · no · inset box-shadow painting in box_shadow.rs, not reached; no Needs entry names shadows
depth-strategy#4.5 · no · opacity, filter and backdrop-filter layers in render.rs, not reached; the compositing Need names mix-blend-mode and isolation, not this layer
depth-strategy#4.6 · yes · the open list's non-finite unpaint and scrollport-only row layout, which the e42ee1f25 off-screen culling change named beside visible_region must keep
depth-strategy#5.1 · no · filter and z-index fixture values in examples/assets; examples other than seven_guis are not in scope
depth-strategy#5.2 · no · google and graphite fixture shadows and z-index values; examples other than seven_guis are not in scope
depth-strategy#5.3 · no · hr, servo and Preact fixture shadows and the box-shadow examples; examples other than seven_guis are not in scope
depth-strategy#6.1 · no · an absence search over listed files the chunk does not reach; it states no value the chunk's code, tests or re-reads need
depth-strategy#6.2 · no · an absence of shadow or elevation tokens; the chunk defines no tokens, per Not needed
depth-strategy#6.3 · no · an absence of shadow definitions over 32 slice files the chunk does not reach
depth-strategy#6.4 · no · an absence of shadow, elevation and z-index tokens with one radius touch-point the scope does not name; it states no value the chunk reads
