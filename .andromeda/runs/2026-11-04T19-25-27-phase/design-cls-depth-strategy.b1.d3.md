@answer n=1 entity=depth-strategy#1.1 v=no
why: browser app z-index values; the probe paints and reads no depth and the browser is not the stand
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu box-shadow value; the probe paints no shadows
@answer n=3 entity=depth-strategy#1.3 v=no
why: seven_guis card and dialog shadows are held in layout-templates, and the probe paints and styles nothing
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc layered shadows; todomvc is not the stand and the probe paints no shadows
@answer n=5 entity=depth-strategy#1.5 v=no
why: z-index values in the wgpu_texture, custom_widget and transforms examples; none is the stand and the scope excludes engine depth
@answer n=6 entity=depth-strategy#1.6 v=no
why: blitz-dom default dialog backdrop colour; the probe paints nothing
@answer n=7 entity=depth-strategy#2.1 v=no
why: paint children and stacking-context build order; engine depth behaviour is not needed per the scope and the probe paints nothing
@answer n=8 entity=depth-strategy#2.2 v=no
why: stacking-context root conditions; engine depth behaviour is not needed per the scope
@answer n=9 entity=depth-strategy#3.1 v=no
why: hit-testing walk order and clip rule; the scope says the probe does not read hit-testing or the hit-testing clip rule, and the click step is named as stepping handlers
@answer n=10 entity=depth-strategy#3.2 v=no
why: open select list answering hit and the covered refusal; hit-testing is excluded by the scope and no select list is named for the Counter or CRUD task
@answer n=11 entity=depth-strategy#3.3 v=no
why: non-invertible transform hit answer; hit-testing is excluded by the scope and the scope names no transform
@answer n=12 entity=depth-strategy#3.4 v=no
why: visible_region through transforms and the select scrollport; it serves scrolling and visibility, which the probe does not do
@answer n=13 entity=depth-strategy#4.1 v=no
why: tree-order paint of positioned z-index auto descendants; paint only and the probe paints nothing
@answer n=14 entity=depth-strategy#4.2 v=no
why: open select list paint order above every box and past blend layers; paint only
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow clipping and shape; paint only
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow cut with Compose::DestOut; paint only
@answer n=17 entity=depth-strategy#4.5 v=no
why: inline box with only block children paints no own box; paint only, and no frame test reads the skip
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity, filter and backdrop-filter layer clipping; paint only
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode and isolation blend layers; paint only
@answer n=20 entity=depth-strategy#4.8 v=no
why: blending witnesses in layers.rs and contrast.rs; test witnesses for paint the chunk does not run
@answer n=21 entity=depth-strategy#4.9 v=no
why: select list paint past opacity and clipping boxes, and the rows laid out in the scrollport; paint and a select list the scope does not name, and hit-testing is excluded
@answer n=22 entity=depth-strategy#5.1 v=no
why: filter and backdrop-filter values in an example fixture; fixtures are not reached
@answer n=23 entity=depth-strategy#5.2 v=no
why: google and graphite fixture shadows and z-index values; fixtures are not reached
@answer n=24 entity=depth-strategy#5.3 v=no
why: hr, servo and Preact shadow fixtures; fixtures are not reached
@answer n=25 entity=depth-strategy#6.1 v=no
why: absence of z-index or shadow handling in the s06 files; the scope names no depth or shadow handling, so the search is not a check the chunk owes
