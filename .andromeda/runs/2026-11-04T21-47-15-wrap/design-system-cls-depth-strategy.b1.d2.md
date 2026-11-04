@answer n=1 entity=depth-strategy#1.1 v=no
why: its browser z-index rule lives in apps/browser/assets/browser.css, which the session does not change
@answer n=2 entity=depth-strategy#1.2 v=no
why: its browser menu and suggestion box-shadow lives in apps/browser/assets/browser.css, which the session does not change
@answer n=3 entity=depth-strategy#1.3 v=no
why: it points to the layout-templates surface entry for the seven_guis stand, and the session changes no stand or layout-templates file
@answer n=4 entity=depth-strategy#1.4 v=no
why: its todomvc layered shadows live in examples/todomvc/src/todomvc.css, which the session does not reach
@answer n=5 entity=depth-strategy#1.5 v=no
why: its wgpu_texture, custom_widget and transforms z-index values live in example files the session does not reach
@answer n=6 entity=depth-strategy#1.6 v=no
why: its blitz-dom default dialog backdrop lives in packages/blitz-dom/assets/default.css, which the session does not change
@answer n=7 entity=depth-strategy#2.1 v=no
why: its paint-children and stacking-context build lives in blitz-dom resolve.rs and lib.rs, which the session does not change
@answer n=8 entity=depth-strategy#2.2 v=no
why: its stacking-context root rule lives in packages/blitz-dom/src/node/node.rs, which the session does not change
@answer n=9 entity=depth-strategy#3.1 v=no
why: its stacking-order paint and hit-test walk lives in blitz-paint render.rs and blitz-dom node.rs, and the session moves no hit-test or paint code
@answer n=10 entity=depth-strategy#3.2 v=no
why: its open select list answering the hit first lives in packages/blitz-dom/src/document.rs, which the session does not change
@answer n=11 entity=depth-strategy#3.3 v=no
why: its non-invertible transform hit rule lives in packages/blitz-dom/src/node/node.rs, which the session does not change
@answer n=12 entity=depth-strategy#3.4 v=no
why: its visible_region clipping rule lives in packages/blitz-dom/src/scrolling.rs, which the session does not change
@answer n=13 entity=depth-strategy#4.1 v=no
why: its positioned-descendant paint order rests on paint_order.rs and blitz-paint code the session does not touch
@answer n=14 entity=depth-strategy#4.2 v=no
why: its open select list paint-last rule lives in blitz-paint render.rs and select_list.rs, which the session does not change
@answer n=15 entity=depth-strategy#4.3 v=no
why: its outset box-shadow clipping lives in packages/blitz-paint/src/render/box_shadow.rs, which the session does not change
@answer n=16 entity=depth-strategy#4.4 v=no
why: its inset box-shadow cut lives in packages/blitz-paint/src/render/box_shadow.rs, which the session does not change
@answer n=17 entity=depth-strategy#4.5 v=no
why: its inline-box paint skip lives in blitz-dom construct.rs and node.rs, which the session does not change
@answer n=18 entity=depth-strategy#4.6 v=no
why: its opacity, filter and backdrop-filter layer lives in packages/blitz-paint/src/render.rs, which the session does not change
@answer n=19 entity=depth-strategy#4.7 v=no
why: its mix-blend-mode and isolation paint lives in blitz-paint blend.rs and render.rs, which the session does not change
@answer n=20 entity=depth-strategy#4.8 v=no
why: its blending witnesses live in blitz-paint layers.rs and blitz-test-harness contrast.rs, which the session does not change
@answer n=21 entity=depth-strategy#4.9 v=no
why: its open select list paint past opacity and clipping boxes lives in blitz-paint select_list.rs, which the session does not change
@answer n=22 entity=depth-strategy#5.1 v=no
why: its filter and backdrop-filter fixtures live in examples/assets and a GitHub fixture the session does not reach
@answer n=23 entity=depth-strategy#5.2 v=no
why: its google and graphite shadow and z-index values live in examples/assets fixtures the session does not reach
@answer n=24 entity=depth-strategy#5.3 v=no
why: its hr, servo, Preact and box-shadow fixture values live in example files the session does not reach
@answer n=25 entity=depth-strategy#6.1 v=no
why: the session adds no z-index or shadow handling, and its three changed files are not among the 17 searched files
