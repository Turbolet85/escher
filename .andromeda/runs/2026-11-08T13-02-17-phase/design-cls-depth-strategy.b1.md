@answer n=1 entity=depth-strategy#1.1 v=no
why: browser toolbar and menu z-index values are app paint, which the read does not reach
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu and suggestions box-shadow value is paint, which the read does not reach
@answer n=3 entity=depth-strategy#1.3 v=no
why: seven_guis card and dialog shadows are held in layout-templates, and the chunk sets no shadow
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc layered shadows are example paint, which the read does not reach
@answer n=5 entity=depth-strategy#1.5 v=no
why: example z-index stacks for wgpu_texture, custom_widget and transforms are example paint values, not hit or cause rules
@answer n=6 entity=depth-strategy#1.6 v=no
why: blitz-dom default dialog backdrop colour is a paint value the read does not report
@answer n=7 entity=depth-strategy#2.1 v=no
why: paint-tree build order after layout and transforms is paint construction, and the read answers hit and cause, not paint
@answer n=8 entity=depth-strategy#2.2 v=no
why: stacking-context root definition is a paint-tree rule the scope does not name
@answer n=9 entity=depth-strategy#3.1 v=yes
why: hit-testing stops at the padding box of a paint-clipping box and answers an open select row before the walk, which decides the covered cause and why an owner-served select row is not clipped (task item 4)
@answer n=10 entity=depth-strategy#3.2 v=yes
why: the open select list answers the hit first so a button under it is refused covered, the covered cause the read reports through action_point
@answer n=11 entity=depth-strategy#3.3 v=yes
why: a non-invertible transform answers no hit for its whole subtree, which is hit-testing as the depth strategy states it and so decides the covered cause for transformed targets (task item 4)
@answer n=12 entity=depth-strategy#3.4 v=yes
why: visible_region gives the clipping-box region and, for an open select row, the list's scrollport, which the off-screen cause and the owner-served rows read (task items 5 and 6)
@answer n=13 entity=depth-strategy#4.1 v=no
why: tree-order paint of positioned descendants is paint, not hit or cause
@answer n=14 entity=depth-strategy#4.2 v=no
why: the open select list painting last above every box is paint order, while the covered cause is decided by hit (3.2) and not by paint
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow clipping and corner shape are paint only
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow painting with DestOut is paint only
@answer n=17 entity=depth-strategy#4.5 v=no
why: an inline box holding only block children paints no own box, which is paint only and not read by the chunk
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity, filter and backdrop-filter layers are paint only
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode and isolation painting through blend layers is paint only
@answer n=20 entity=depth-strategy#4.8 v=no
why: blending's test witnesses are paint witnesses the chunk does not read
@answer n=21 entity=depth-strategy#4.9 v=yes
why: an open select list paints past a clipping box and only lays out rows in its scrollport, which bounds the owner-served rows the read lists and their off-screen cause (task items 4 and 5)
@answer n=22 entity=depth-strategy#5.1 v=no
why: filter and fixed z-index values in example assets are example paint, not a rule the read honours
@answer n=23 entity=depth-strategy#5.2 v=no
why: google and graphite fixture shadows and z-index values are example assets, not a rule the read honours
@answer n=24 entity=depth-strategy#5.3 v=no
why: hr, servo and Preact shadow fixtures are example assets, not a rule the read honours
@answer n=25 entity=depth-strategy#6.1 v=no
why: the absence record of z-index and shadow handling over the s06 files states no rule the chunk must honour
