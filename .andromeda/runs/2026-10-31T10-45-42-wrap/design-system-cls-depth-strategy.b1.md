# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser z-index values live in apps/browser/assets/browser.css, which the session does not change
depth-strategy#1.2 · no · the browser menu and suggestions shadow lives in apps/browser/assets/browser.css, which the session does not change
depth-strategy#1.3 · no · the seven_guis change is a Cells focus outline in cells.rs, not a card or circle-dialog shadow, and those shadows stay held in layout-templates, which the session does not reach
depth-strategy#1.4 · no · the todomvc layered shadows live in examples/todomvc/src/todomvc.css, which the session does not change
depth-strategy#1.5 · no · the wgpu_texture, custom-widget and transforms z-index values live in example files the session does not change
depth-strategy#1.6 · yes · stale · the 28 lines added at packages/blitz-dom/assets/default.css :161-188 move the cited :1027-1030 by 28, while the rgba(0, 0, 0, 0.1) backdrop still holds
depth-strategy#2.1 · no · the cited resolve.rs and lib.rs lines are in files the session does not change, and the paint-child build and re-exports hold
depth-strategy#2.2 · yes · stale · node.rs is reached whole, its hit rule gained 6 lines at :1472-1477, and the report does not show the stacking-root rule at :1332-1406 holds on the changed file
depth-strategy#3.1 · yes · add-to · F5 and F6 join its hit and clip rules, since a zero or non-finite transform takes no hit for its subtree (F5) and visible_region reads clipping boxes through their transforms (F6), and its node.rs:1540-1638 citations move 6 lines
depth-strategy#4.1 · no · the paint-order test paint_order.rs is not among the changed files, and blitz-paint, where paint order is built, shows no diff
depth-strategy#4.2 · no · blitz-paint shows no diff, so the outset box-shadow paint rules and outset_box_shadow_shape.rs hold
depth-strategy#4.3 · no · blitz-paint shows no diff, so the inset shadow's blurred cut in box_shadow.rs holds
depth-strategy#4.4 · no · blitz-paint shows no diff, so the filter layer clip at render.rs:496-528 holds
depth-strategy#5.1 · no · the filters and GitHub fixtures are not among the changed files
depth-strategy#5.2 · no · the google and graphite fixtures are not among the changed files
depth-strategy#5.3 · no · the hr, servo, Preact and box-shadow fixtures are not among the changed files
depth-strategy#6.1 · no · the session adds no z-index or shadow handling, its CSS being the default sheet rows and colour pair at default.css :161-188 and the Cells outline at cells.rs :551-554, and blitz-paint shows no diff
depth-strategy#6.2 · no · the session adds no shadow or elevation token, and the session records no elevation scale
depth-strategy#6.3 · no · the session adds no shadow or elevation definition in any changed file
depth-strategy#6.4 · no · the session adds no radius, shadow, elevation or z-index token, its CSS being the default sheet rows and colour pair and the Cells outline
