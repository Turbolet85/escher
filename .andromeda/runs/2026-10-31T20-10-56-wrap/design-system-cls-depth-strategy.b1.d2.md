# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser stylesheet is under apps/browser, which the session did not reach
depth-strategy#1.2 · no · the browser menu shadow lives in apps/browser/assets/browser.css, which the session did not reach
depth-strategy#1.3 · no · the seven_guis shadows are held in layout-templates, and the session changed examples/seven_guis tests alone
depth-strategy#1.4 · no · the todomvc stylesheet is not in the reached files
depth-strategy#1.5 · no · the wgpu_texture, custom_widget and transforms z-index sources are not in the reached files
depth-strategy#1.6 · yes · stale · default.css gained lines past 183, so the dialog backdrop rule cited at 1038-1041 has moved
depth-strategy#2.1 · yes · stale · lib.rs is reached and gains the new public read-type re-exports, so its cited line 89 may have moved; the build order itself is unchanged
depth-strategy#2.2 · no · the stacking-context rule lives in node/node.rs, which the session did not reach
depth-strategy#3.1 · yes · add-to · the open list answers the hit first (F10), so a point outside the select's padding box can reach a list row, which is not a child of the node
depth-strategy#3.2 · no · the non-invertible transform rule lives in node/node.rs, which the session did not reach
depth-strategy#3.3 · yes · contradict · a list row's visible region is the list's scrollport (scrolling.rs:754-756), no longer read through its ancestors' transforms
depth-strategy#4.1 · yes · add-to · the open list paints last in paint_scene, above every positioned descendant (F8)
depth-strategy#4.2 · no · the outset shadow rule lives in blitz-paint render/box_shadow.rs, which the session did not reach
depth-strategy#4.3 · no · the inset shadow rule lives in blitz-paint render/box_shadow.rs, which the session did not reach
depth-strategy#4.4 · yes · add-to · the list paints whole past an opacity ancestor and past a clipping box, outside the layer this rule describes (F9)
depth-strategy#5.1 · no · the example fixtures under examples/assets are not in the reached files
depth-strategy#5.2 · no · the example fixtures under examples/assets are not in the reached files
depth-strategy#5.3 · no · the example fixtures under examples/assets are not in the reached files
depth-strategy#6.1 · no · the list adds no z-index: its paint order comes from paint_scene, and the new default.css rules set no z-index or shadow
depth-strategy#6.2 · no · the session adds no shadow or elevation token, and the engine holds no tokens
depth-strategy#6.3 · no · the session adds no shadow or elevation definition
depth-strategy#6.4 · no · the session adds no radius, shadow, elevation or z-index token
