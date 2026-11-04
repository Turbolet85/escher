@answer n=1 entity=depth-strategy#1.1 v=no
why: browser app z-index values; the chunk touches no browser app and paints nothing
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu and suggestion box-shadow values; the probe styles nothing
@answer n=3 entity=depth-strategy#1.3 v=no
why: seven_guis card and circle-dialog shadows are held in another section; the probe runs the stand but does not read or paint its shadows
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc layered shadows; the chunk does not touch todomvc
@answer n=5 entity=depth-strategy#1.5 v=no
why: wgpu_texture, custom-widget and transforms z-index values; none is reached and the probe reads no stacking
@answer n=6 entity=depth-strategy#1.6 v=no
why: blitz-dom default dialog backdrop colour; the probe paints no backdrop and reads no style
@answer n=7 entity=depth-strategy#2.1 v=no
why: paint children and stacking contexts are built after layout; the probe reads node, handler and state links, not paint order
@answer n=8 entity=depth-strategy#2.2 v=no
why: stacking-context root definition; a paint rule the probe neither reads nor changes
@answer n=9 entity=depth-strategy#3.1 v=no
why: hit-testing walk and clip rule; the scope states the probe does not read or change hit-testing, and click steps handlers rather than hit-testing a point
@answer n=10 entity=depth-strategy#3.2 v=no
why: open select list answers the hit first and refuses a covered button; the seven_guis Counter, CRUD and served-task runs open no select list and the scope excludes hit-testing
@answer n=11 entity=depth-strategy#3.3 v=no
why: a non-invertible transform answers no hit for its subtree; hit-testing and transforms are outside the probe
@answer n=12 entity=depth-strategy#3.4 v=no
why: visible_region clipping boxes through transforms; the probe does no scrolling and reads no visible region
@answer n=13 entity=depth-strategy#4.1 v=no
why: positioned descendants with z-index auto paint in tree order; paint order is not read by the probe
@answer n=14 entity=depth-strategy#4.2 v=no
why: the open select list paints last in paint_scene; paint is outside the probe and no select list is opened
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow clipping and shape; the probe paints and styles nothing
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow fill and DestOut hole; the probe paints and styles nothing
@answer n=17 entity=depth-strategy#4.5 v=no
why: an inline box holding only block children paints no own box; the probe reads no paint flag
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity, filter and backdrop-filter layer clipping; paint is outside the probe
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode and isolation painting through the blend module; paint is outside the probe
@answer n=20 entity=depth-strategy#4.8 v=no
why: the witnesses for blending are unit tests and a fixture; the chunk adds no blend test and reads no blend
@answer n=21 entity=depth-strategy#4.9 v=no
why: the open select list paints past opacity ancestors and clipping boxes; paint and select lists are not in the probe
@answer n=22 entity=depth-strategy#5.1 v=no
why: drop-shadow, backdrop-filter and z-index fixture values in example assets; no example asset is reached by the chunk
@answer n=23 entity=depth-strategy#5.2 v=no
why: google and graphite shadow and z-index fixture values; no such fixture is reached by the chunk
@answer n=24 entity=depth-strategy#5.3 v=no
why: hr, servo, Preact and box-shadow fixture values; the probe reads no shadow and no such fixture is reached
@answer n=25 entity=depth-strategy#6.1 v=no
why: an absence record of z-index and shadow search over s06 files; it states no rule the probe must honour
