@answer n=1 entity=depth-strategy#1.1 v=no
why: browser toolbar and overlay z-index values in browser.css, which the render-path change does not set or read
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu box-shadow value, and the chunk paints no shadow
@answer n=3 entity=depth-strategy#1.3 v=no
why: seven_guis card shadows live in layout-templates, not a design-system value the chunk paints
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc layered shadows in an example stylesheet the chunk does not reach
@answer n=5 entity=depth-strategy#1.5 v=no
why: wgpu_texture and custom widget z-index values in example files the chunk does not reach
@answer n=6 entity=depth-strategy#1.6 v=no
why: blitz-dom default dialog backdrop colour in default.css, which the chunk neither changes nor paints
@answer n=7 entity=depth-strategy#2.1 v=no
why: paint-child and stacking-context build order after layout, which the mutation buffer leaves alone
@answer n=8 entity=depth-strategy#2.2 v=no
why: stacking-context root conditions in node.rs, which the chunk does not read or change
@answer n=9 entity=depth-strategy#3.1 v=no
why: hit-test walk order in node.rs, a file the chunk does not reach and a behaviour its scope does not re-assert
@answer n=10 entity=depth-strategy#3.2 v=no
why: select-list hit precedence, not among the behaviours the chunk re-asserts (document built, ids, snapshot and diff, event order, layout identity)
@answer n=11 entity=depth-strategy#3.3 v=no
why: non-invertible transform hit refusal, untouched by the mutation buffer and not named in the scope
@answer n=12 entity=depth-strategy#3.4 v=no
why: visible_region clipping read, a function the scope does not name and the chunk does not call
@answer n=13 entity=depth-strategy#4.1 v=no
why: CSS 2.1 paint order of positioned descendants, witnessed by a paint test the chunk does not reach
@answer n=14 entity=depth-strategy#4.2 v=no
why: select list paint-last order in paint_scene, which the chunk neither changes nor reads
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow clipping and shape, a paint rule the chunk does not touch
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow cut in box_shadow.rs, a paint rule the chunk does not touch
@answer n=17 entity=depth-strategy#4.5 v=no
why: paints_own_box skip for inline boxes holding block children, a paint rule outside the chunk's scope
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity and filter layer clip in render.rs, which the chunk does not change
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode private blend module, which the chunk does not touch
@answer n=20 entity=depth-strategy#4.8 v=no
why: blending witnesses in layers.rs and contrast.rs, files the chunk does not reach or re-assert
@answer n=21 entity=depth-strategy#4.9 v=no
why: select-list paint past opacity and clipping boxes, not a behaviour the chunk re-asserts
@answer n=22 entity=depth-strategy#5.1 v=no
why: fixture filter and z-index values in examples, which the chunk does not reach
@answer n=23 entity=depth-strategy#5.2 v=no
why: google and graphite fixture shadows and z-index values in examples, which the chunk does not reach
@answer n=24 entity=depth-strategy#5.3 v=no
why: hr, servo and preact fixture shadows in examples, which the chunk does not reach
@answer n=25 entity=depth-strategy#6.1 v=no
why: absence record of z-index and shadow handling in s06 files, a rule the chunk does not act on
