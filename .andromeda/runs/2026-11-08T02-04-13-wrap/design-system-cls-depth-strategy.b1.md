@answer n=1 entity=depth-strategy#1.1 v=no
why: the browser z-index values sit in apps/browser/assets/browser.css, a file the session did not reach, and no depth or styling value moved
@answer n=2 entity=depth-strategy#1.2 v=no
why: the shared browser menu and suggestion shadow sits in apps/browser/assets/browser.css, which the session did not reach, and no shadow value moved
@answer n=3 entity=depth-strategy#1.3 v=no
why: the seven_guis card and circle-dialog shadows are held in layout-templates, and the session changed only the seven_guis signal registration with no styling moved
@answer n=4 entity=depth-strategy#1.4 v=no
why: the todomvc layered shadows sit in examples/todomvc/src/todomvc.css, which the session did not reach, and no styling moved
@answer n=5 entity=depth-strategy#1.5 v=no
why: the wgpu_texture, custom-widget and transforms z-index files are not among the reached files, and the session moved no z-index
@answer n=6 entity=depth-strategy#1.6 v=no
why: the blitz-dom default dialog backdrop sits in packages/blitz-dom/assets/default.css, which the session did not reach, and no colour or styling moved
@answer n=7 entity=depth-strategy#2.1 v=no
why: the stacking-context build in blitz-dom resolve.rs and lib.rs is not reached, and no layout or paint rule moved
@answer n=8 entity=depth-strategy#2.2 v=no
why: the stacking-context root rule in blitz-dom node.rs is not reached, and no dependency or layout rule moved
@answer n=9 entity=depth-strategy#3.1 v=no
why: the paint-order and hit-walk rules in blitz-dom and blitz-paint are not reached, and the task holds hit-testing at clipped padding boxes unchanged
@answer n=10 entity=depth-strategy#3.2 v=no
why: the select-list hit in blitz-dom document.rs is not reached, and the task holds hit-testing unchanged so the driver covered reading in execute.rs keeps its claim while only the graph verb was added
@answer n=11 entity=depth-strategy#3.3 v=no
why: the non-invertible transform hit rule in blitz-dom node.rs is not reached, and hit-testing is held unchanged by the task
@answer n=12 entity=depth-strategy#3.4 v=no
why: visible_region in blitz-dom scrolling.rs is not reached, and no layout or paint rule moved
@answer n=13 entity=depth-strategy#4.1 v=no
why: the paint order of positioned descendants with z-index auto is pinned by a blitz-paint test that the session did not reach, and no paint rule moved
@answer n=14 entity=depth-strategy#4.2 v=no
why: the select-list paint order in blitz-paint render.rs and select_list.rs is not reached, and no paint rule moved
@answer n=15 entity=depth-strategy#4.3 v=no
why: the outset box-shadow rule in blitz-paint box_shadow.rs is not reached, and no shadow rule moved
@answer n=16 entity=depth-strategy#4.4 v=no
why: the inset box-shadow rule in blitz-paint box_shadow.rs is not reached, and no shadow rule moved
@answer n=17 entity=depth-strategy#4.5 v=no
why: the paints_own_box rule in blitz-dom node.rs and construct.rs is not reached, and no paint rule moved
@answer n=18 entity=depth-strategy#4.6 v=no
why: the opacity and filter layer in blitz-paint render.rs is not reached, and no paint rule moved
@answer n=19 entity=depth-strategy#4.7 v=no
why: the blend module in blitz-paint is not reached, and no paint rule or dependency moved
@answer n=20 entity=depth-strategy#4.8 v=no
why: the blend witnesses in blitz-paint layers.rs and blitz-test-harness contrast.rs are not reached, and no blend rule moved
@answer n=21 entity=depth-strategy#4.9 v=no
why: the select-list paint past an opacity ancestor in blitz-paint select_list.rs is not reached, and no paint rule moved
@answer n=22 entity=depth-strategy#5.1 v=no
why: the filter and backdrop-filter fixtures in examples/assets are not reached, and no styling moved
@answer n=23 entity=depth-strategy#5.2 v=no
why: the google and graphite shadow and z-index fixtures in examples/assets are not reached, and no styling moved
@answer n=24 entity=depth-strategy#5.3 v=no
why: the hr, servo and preact shadow fixtures and examples/box_shadow.rs are not reached, and no styling moved
@answer n=25 entity=depth-strategy#6.1 v=no
why: the session brings no z-index or box-shadow handling into any file and no styling moved, so the absence claim over the s06 files still holds
