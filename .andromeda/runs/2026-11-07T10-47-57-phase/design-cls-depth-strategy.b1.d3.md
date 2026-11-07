@answer n=1 entity=depth-strategy#1.1 v=no
why: browser UI z-index values in browser.css; the chunk changes no CSS, and documents are re-asserted by ids, events and DOM, not by z-index
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu and suggestion box-shadow values; the chunk paints no shadow and re-asserts no shadow
@answer n=3 entity=depth-strategy#1.3 v=no
why: a pointer to the stand's card and dialog shadows held in layout-templates; states no value the chunk's code or checks read
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc example shadows; the example is not a re-asserted document and the chunk changes no CSS
@answer n=5 entity=depth-strategy#1.5 v=no
why: z-index and stacking values in wgpu_texture, custom_widget and transforms examples; no example is re-read or re-measured by the chunk
@answer n=6 entity=depth-strategy#1.6 v=no
why: blitz-dom default dialog backdrop colour; the chunk does not touch default.css or dialog painting
@answer n=7 entity=depth-strategy#2.1 v=no
why: stacking-context build order after layout and transforms in blitz-dom resolve; the chunk changes no resolve or paint-tree code
@answer n=8 entity=depth-strategy#2.2 v=no
why: stacking-context root conditions on node.rs; the chunk does not change which nodes root a stacking context
@answer n=9 entity=depth-strategy#3.1 v=no
why: hit-testing walk order and clip stop; the chunk's focus, geometry and mounted-event checks do not hit-test, and the named hit witnesses are not re-asserted
@answer n=10 entity=depth-strategy#3.2 v=yes
why: the open select list answers the hit first, witnessed by stand_act_popup.rs, a stand_* check the chunk re-asserts through its existing tests
@answer n=11 entity=depth-strategy#3.3 v=no
why: a non-invertible transform answers no hit; the chunk's checks neither read hit-testing nor pass through a transform
@answer n=12 entity=depth-strategy#3.4 v=yes
why: visible_region's clip answer through transforms may be read by the programmatic into-view scroll the scope keeps as is, a fork the scope leaves open, so the scroll rule must hold over it
@answer n=13 entity=depth-strategy#4.1 v=no
why: paint order of positioned descendants with z-index auto; the chunk changes no paint and no paint_order check is re-asserted
@answer n=14 entity=depth-strategy#4.2 v=no
why: the open select list paints last in paint_scene; no select list is touched and paint_scene is not on the chunk's path
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow clipping and shape in box_shadow.rs; the chunk paints no shadow
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow fill and DestOut hole in box_shadow.rs; the chunk paints no shadow
@answer n=17 entity=depth-strategy#4.5 v=no
why: inline boxes holding only block children paint no box of their own; the chunk's checks read no paint and the skip has no frame test
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity, filter and backdrop-filter layer clipping in blitz-paint render; the chunk changes no paint layer
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode and isolation through the private blend module; the chunk changes no blend path and no blend witness is re-asserted
@answer n=20 entity=depth-strategy#4.8 v=no
why: blending witnesses in layers.rs and contrast.rs; neither is a stand_* or probe check the chunk re-asserts
@answer n=21 entity=depth-strategy#4.9 v=no
why: select list painting past opacity and clipping ancestors, witnessed by select_popup_painted.rs; not a check the chunk re-asserts and no select list is touched
@answer n=22 entity=depth-strategy#5.1 v=no
why: filter and backdrop-filter fixtures and a GitHub fixed progress bar z-index in examples; the chunk re-measures no fixture
@answer n=23 entity=depth-strategy#5.2 v=no
why: google and graphite fixture shadows and z-index values in examples; the chunk re-measures no fixture
@answer n=24 entity=depth-strategy#5.3 v=no
why: hr, servo, Preact and box-shadow example fixtures; the chunk re-measures no fixture and paints no shadow
@answer n=25 entity=depth-strategy#6.1 v=no
why: absence of z-index and shadow handling over the listed s06 files; the chunk adds no depth handling and reaches none of those files as an edited unit or merge surface
