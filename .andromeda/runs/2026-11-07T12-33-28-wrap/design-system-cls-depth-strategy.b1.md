@answer n=1 entity=depth-strategy#1.1 v=no
why: browser z-index values live in apps/browser/assets/browser.css, which the session does not reach, and depth is untouched.
@answer n=2 entity=depth-strategy#1.2 v=no
why: the browser menu and suggestion shadow lives in apps/browser/assets/browser.css, which the session does not reach, and no shadow value changed.
@answer n=3 entity=depth-strategy#1.3 v=no
why: it points to the layout-templates seven_guis card and dialog shadows, and the session changed no shadow, layout template or stand stylesheet.
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc layered shadows in examples/todomvc/src/todomvc.css are outside the reached files, and no stylesheet changed.
@answer n=5 entity=depth-strategy#1.5 v=no
why: the wgpu_texture, custom-widget and transforms z-index values sit in example files the session does not reach, and no z-index value moved.
@answer n=6 entity=depth-strategy#1.6 v=no
why: the blitz-dom default dialog backdrop in packages/blitz-dom/assets/default.css is not reached, and no CSS default changed.
@answer n=7 entity=depth-strategy#2.1 v=no
why: the paint-children and stacking-context build in blitz-dom resolve.rs and lib.rs is not reached, and the task leaves paint unchanged.
@answer n=8 entity=depth-strategy#2.2 v=no
why: the stacking-context root rules in blitz-dom node.rs are not reached, and no style or node rule moved.
@answer n=9 entity=depth-strategy#3.1 v=no
why: child paint order and the hit-testing walk in blitz-paint and blitz-dom are not reached, and the task states hit-testing and paint are unchanged.
@answer n=10 entity=depth-strategy#3.2 v=no
why: the open select list hit in blitz-dom document.rs is not reached, and its hit answers are unchanged.
@answer n=11 entity=depth-strategy#3.3 v=no
why: the non-invertible transform hit rule in blitz-dom node.rs is not reached, and no transform or hit code changed.
@answer n=12 entity=depth-strategy#3.4 v=no
why: visible_region in blitz-dom scrolling.rs is not reached, and the session changed no clipping box or scrollport read.
@answer n=13 entity=depth-strategy#4.1 v=no
why: the positioned-descendant paint order test and CSS 2.1 Appendix E sharing sit in blitz-dom and blitz-tests files the session does not reach, and paint order is unchanged.
@answer n=14 entity=depth-strategy#4.2 v=no
why: the paint_scene select-list paint in blitz-paint render.rs is not reached, and paint is unchanged.
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow painting in blitz-paint box_shadow.rs is not reached, and no shadow paint changed.
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow painting in blitz-paint box_shadow.rs is not reached, and no paint changed.
@answer n=17 entity=depth-strategy#4.5 v=no
why: the inline-box paints_own_box rule in blitz-dom node.rs and construct.rs is not reached, and no box painting changed.
@answer n=18 entity=depth-strategy#4.6 v=no
why: the opacity, filter and backdrop-filter layer in blitz-paint render.rs is not reached, and paint is unchanged.
@answer n=19 entity=depth-strategy#4.7 v=no
why: the mix-blend-mode and isolation blend module in blitz-paint is not reached, and no blend code changed.
@answer n=20 entity=depth-strategy#4.8 v=no
why: the blend witnesses in blitz-paint layers.rs and the harness contrast.rs fixture are not reached, and no blend test or fixture changed.
@answer n=21 entity=depth-strategy#4.9 v=no
why: the open select list paint past opacity and clipping boxes in blitz-paint select_list.rs is not reached, and paint is unchanged.
@answer n=22 entity=depth-strategy#5.1 v=no
why: the filter and backdrop-filter example fixtures are not reached, and no fixture changed.
@answer n=23 entity=depth-strategy#5.2 v=no
why: the google and graphite fixture shadows and z-index values are example assets the session does not reach, and no fixture changed.
@answer n=24 entity=depth-strategy#5.3 v=no
why: the hr, servo, Preact and box-shadow fixture shadows are example assets the session does not reach, and no fixture changed.
@answer n=25 entity=depth-strategy#6.1 v=no
why: the session adds no z-index or box-shadow text to any reached file, and the task leaves depth untouched.
