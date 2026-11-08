@answer n=1 entity=depth-strategy#1.1 v=no
why: browser chrome z-index values; the chunk touches the driver, not the browser app
@answer n=2 entity=depth-strategy#1.2 v=no
why: browser menu box-shadow value; the chunk paints no shadow and does not touch the browser app
@answer n=3 entity=depth-strategy#1.3 v=no
why: seven_guis card and dialog shadows are held in layout-templates, not in this part's values
@answer n=4 entity=depth-strategy#1.4 v=no
why: todomvc layered app and footer shadows; the chunk adds no example or shadow
@answer n=5 entity=depth-strategy#1.5 v=no
why: wgpu_texture, custom-widget and transforms example z-index values; the read is not a stacking feature
@answer n=6 entity=depth-strategy#1.6 v=no
why: blitz-dom default dialog backdrop colour, a paint value the read does not reach
@answer n=7 entity=depth-strategy#2.1 v=no
why: paint-child construction and stacking re-exports; the read calls the hit walk and reads no stacking type
@answer n=8 entity=depth-strategy#2.2 v=no
why: stacking-context root definition for paint layering; the hit walk's order is stated in depth-strategy#3.1, not here
@answer n=9 entity=depth-strategy#3.1 v=yes
why: the hit walk's stop at a paint-clipping box, the select row answered before the walk, and the own-box answers that the read's covered and refusal causes come from
@answer n=10 entity=depth-strategy#3.2 v=yes
why: the open select list answers hit first, so a button under it is refused covered, the cause the read reports
@answer n=11 entity=depth-strategy#3.3 v=yes
why: a non-invertible transform answers no hit for its subtree, which the read's per-verb covered cause must follow
@answer n=12 entity=depth-strategy#3.4 v=yes
why: visible_region through transforms and a select row's scrollport answer bear on the off-screen cause
@answer n=13 entity=depth-strategy#4.1 v=no
why: paint-level sharing for z-index auto is paint order; hit order is stated in depth-strategy#3.1
@answer n=14 entity=depth-strategy#4.2 v=no
why: the list's paint-last order is paint; the chunk reads hit answers, and the covered outcome is stated in depth-strategy#3.2
@answer n=15 entity=depth-strategy#4.3 v=no
why: outset box-shadow paint clipping and shape; paint only
@answer n=16 entity=depth-strategy#4.4 v=no
why: inset box-shadow paint via DestOut; paint only
@answer n=17 entity=depth-strategy#4.5 v=no
why: paints_own_box for inline boxes holding block children; the read does not call the flag or the skip
@answer n=18 entity=depth-strategy#4.6 v=no
why: opacity, filter and backdrop-filter layer paint; the read paints nothing
@answer n=19 entity=depth-strategy#4.7 v=no
why: mix-blend-mode and isolation paint through blitz-paint's blend module; paint only
@answer n=20 entity=depth-strategy#4.8 v=no
why: blending test witnesses; the chunk adds no blend test
@answer n=21 entity=depth-strategy#4.9 v=yes
why: only rows inside the scrollport are laid out, so the read's owner-served select rows and their off-screen answer rest on it
@answer n=22 entity=depth-strategy#5.1 v=no
why: filter and drop-shadow example fixture values; the chunk's minimal fixture is its own work, not these rows
@answer n=23 entity=depth-strategy#5.2 v=no
why: google and graphite fixture shadow and z-index values; not reached by the read
@answer n=24 entity=depth-strategy#5.3 v=no
why: hr, servo and Preact fixture shadow values; not reached by the read
@answer n=25 entity=depth-strategy#6.1 v=no
why: absence record of a z-index or shadow search; it states no rule the read honours
