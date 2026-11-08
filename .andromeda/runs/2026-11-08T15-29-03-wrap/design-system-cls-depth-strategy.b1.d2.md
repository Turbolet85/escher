@answer n=1 entity=depth-strategy#1.1 v=no
why: the browser z-index values sit in apps/browser/assets/browser.css, which the session does not reach
@answer n=2 entity=depth-strategy#1.2 v=no
why: the browser shadow values sit in apps/browser/assets/browser.css, which the session does not reach
@answer n=3 entity=depth-strategy#1.3 v=no
why: the stand shadows are held in layout-templates, a file the session does not reach
@answer n=4 entity=depth-strategy#1.4 v=no
why: the todomvc shadows sit in examples/todomvc/src/todomvc.css, which the session does not reach
@answer n=5 entity=depth-strategy#1.5 v=no
why: the wgpu, custom-widget and transforms z-index values sit in example files the session does not reach
@answer n=6 entity=depth-strategy#1.6 v=no
why: the dialog backdrop sits in packages/blitz-dom/assets/default.css, which the session does not reach
@answer n=7 entity=depth-strategy#2.1 v=no
why: the paint-child build and its re-exports sit in blitz-dom, which the session does not reach
@answer n=8 entity=depth-strategy#2.2 v=no
why: the stacking-context root rule sits in blitz-dom node.rs, which the session does not reach
@answer n=9 entity=depth-strategy#3.1 v=no
why: the task says the hit-testing rule is unchanged, and the walk sits in blitz-dom, which the session does not reach
@answer n=10 entity=depth-strategy#3.2 v=no
why: the open list still answers the hit first and the driver's covered still reads it, since the actions read reuses the unchanged action_point
@answer n=11 entity=depth-strategy#3.3 v=no
why: the non-invertible transform rule sits in blitz-dom node.rs, which the session does not reach
@answer n=12 entity=depth-strategy#3.4 v=no
why: visible_region sits in blitz-dom scrolling.rs, which the session does not reach and the task does not change
@answer n=13 entity=depth-strategy#4.1 v=no
why: the z-index auto paint order sits in blitz-paint, and paint_order.rs is not among the reached files
@answer n=14 entity=depth-strategy#4.2 v=no
why: the select list paint order sits in blitz-paint render.rs, which the session does not reach
@answer n=15 entity=depth-strategy#4.3 v=no
why: the outset shadow rule sits in blitz-paint box_shadow.rs, which the session does not reach
@answer n=16 entity=depth-strategy#4.4 v=no
why: the inset shadow rule sits in blitz-paint box_shadow.rs, which the session does not reach
@answer n=17 entity=depth-strategy#4.5 v=no
why: the paints_own_box rule sits in blitz-dom, which the session does not reach
@answer n=18 entity=depth-strategy#4.6 v=no
why: the opacity and filter layer sits in blitz-paint render.rs, which the session does not reach
@answer n=19 entity=depth-strategy#4.7 v=no
why: the blend-mode paint sits in blitz-paint blend.rs, which the session does not reach
@answer n=20 entity=depth-strategy#4.8 v=no
why: the task names no blended-element test among the reached tests, and the blend witnesses sit outside the reach
@answer n=21 entity=depth-strategy#4.9 v=no
why: the select-list paint past opacity and clip sits in blitz-paint, which the session does not reach
@answer n=22 entity=depth-strategy#5.1 v=no
why: the filter and z-index fixtures sit in examples/assets, which the session does not reach
@answer n=23 entity=depth-strategy#5.2 v=no
why: the google and graphite fixtures sit in examples/assets, which the session does not reach
@answer n=24 entity=depth-strategy#5.3 v=no
why: the hr, servo and box-shadow fixtures sit in examples and examples/assets, which the session does not reach
@answer n=25 entity=depth-strategy#6.1 v=no
why: the task names no z-index or shadow handling added, and the searched s06 files are not among the reached files
