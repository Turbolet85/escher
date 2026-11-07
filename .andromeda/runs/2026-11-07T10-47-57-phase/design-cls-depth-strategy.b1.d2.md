@answer n=1 entity=depth-strategy#1.1 v=no
why: browser chrome z-index values in browser.css; the chunk changes no browser UI and no CSS, and the browser app's documents, ids and mounted events are not z-index values
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu and suggestion box-shadow values in browser.css; no shadow is set, read or painted by the chunk
@answer n=3 entity=depth-strategy#1.3 v=no
why: seven_guis card and circle-dialog shadows held in layout-templates; the stand's shadows are not touched and no shadow is read by the buffered render
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc layered shadows in its own stylesheet; example CSS the chunk does not touch
@answer n=5 entity=depth-strategy#1.5 v=no
why: wgpu_texture, custom_widget and transforms example z-index values; example styling outside the chunk's files and tests
@answer n=6 entity=depth-strategy#1.6 v=no
why: blitz-dom default dialog backdrop colour in default.css; a colour value the chunk neither sets nor reads
@answer n=7 entity=depth-strategy#2.1 v=no
why: paint children and stacking contexts are built after layout in blitz-dom's resolve; the chunk changes only how dioxus-native-dom replays mutations and reads no stacking context
@answer n=8 entity=depth-strategy#2.2 v=no
why: stacking-context root rules for opacity, position, z-index and transforms in blitz-dom node.rs; the buffered render sets none of these properties and the chunk's tests read none of them
@answer n=9 entity=depth-strategy#3.1 v=no
why: hit-testing walk order in blitz-dom node.rs; the chunk's needs are focus, element geometry, programmatic scroll and mounted events, none of which is a hit-test answer, and no named check reads hit order
@answer n=10 entity=depth-strategy#3.2 v=no
why: open select list answers the hit first in hit_with_scrollbar; no select list is touched, and stand_act_popup witnesses this select hit rule, not a behaviour the chunk re-asserts (document built, element ids, snapshot and diff, mounted event order, layout identity)
@answer n=11 entity=depth-strategy#3.3 v=no
why: a non-invertible transform answers no hit for its subtree; a hit-test rule the chunk's code and tests do not reach
@answer n=12 entity=depth-strategy#3.4 v=no
why: visible_region's transform and select-list scrollport rules in scrolling.rs; the chunk's scope names scroll_to, scroll, get_scroll_offset and get_scroll_size, not visible_region, and the into-view rule the scope names is not this entity's rule
@answer n=13 entity=depth-strategy#4.1 v=no
why: CSS 2.1 Appendix E paint order for positioned descendants in blitz-paint; paint is not changed by the chunk, and the scope's 'unchanged in what it paints' is carried by blitz-paint's own code, with no named check over this order
@answer n=14 entity=depth-strategy#4.2 v=no
why: open select list paints last in paint_scene; no select list is touched, and the paint order of select_list.rs is blitz-paint's own guarantee the chunk does not act on
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow clipping and shape rules in box_shadow.rs; the chunk paints no shadow and no named check reads these pixels
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow fill and DestOut hole in box_shadow.rs; blitz-paint's own code the chunk does not reach
@answer n=17 entity=depth-strategy#4.5 v=no
why: paints_own_box skip for inline boxes holding only block children, read through a public Node method the chunk does not call; no frame test reads the skip
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity, filter and backdrop-filter layer clipping in render.rs; the chunk applies no effect and reads no layer
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode and isolation through blitz-paint's private blend module; the chunk sets no blend and the module is not reached by the buffered render
@answer n=20 entity=depth-strategy#4.8 v=no
why: names the blending witnesses (layers.rs unit tests, contrast.rs fixture); none of these is a check the chunk re-asserts or names
@answer n=21 entity=depth-strategy#4.9 v=no
why: open select list paints whole past opacity ancestors and clipping boxes; no select list is touched and no named check reads this paint
@answer n=22 entity=depth-strategy#5.1 v=no
why: filter drop-shadow, backdrop-filter and a z-index fixture in example assets; example content the chunk does not load or test
@answer n=23 entity=depth-strategy#5.2 v=no
why: google and graphite fixture shadows and z-index values; fixture styling the chunk does not render or assert
@answer n=24 entity=depth-strategy#5.3 v=no
why: hr, servo and Preact fixture shadows and the box-shadow examples; fixture styling the chunk does not render or assert
@answer n=25 entity=depth-strategy#6.1 v=no
why: absence record that the s06 files hold no z-index or shadow handling; it states no rule the chunk must honour
