# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser's z-index values in apps/browser/assets/browser.css, a stylesheet the chunk does not reach; the task's Not-needed list drops depth values and no CSS value changes
depth-strategy#1.2 · no · the browser menu and suggestion box-shadow in apps/browser/assets/browser.css, a stylesheet the chunk does not reach and a shadow value it does not change
depth-strategy#1.3 · no · the seven_guis card and circle-dialog box-shadows are paint only and no stand_lint reading (bounds, text, computed colour, opacity, filter) takes a shadow; the stand's styles are not edited
depth-strategy#1.4 · no · the todomvc layered shadows in examples/todomvc, an example the chunk does not reach whose values no lint reading takes
depth-strategy#1.5 · no · the wgpu_texture, custom_widget and transforms z-index values, examples the chunk does not reach whose stacking no lint reading takes
depth-strategy#1.6 · no · the blitz-dom default dialog backdrop is a ::backdrop paint value in a UA stylesheet the chunk does not touch, and text_contrast composites only ancestors' background-colours
depth-strategy#2.1 · no · the paint-children and stacking-context build order and its public re-exports; the lint's findings come in document order and its overlap reads the snapshot's bounds, so no lint reading takes paint order
depth-strategy#2.2 · no · which nodes root a stacking context is paint order, not a bounds or computed-style reading the lint takes, and the chunk edits no paint code
depth-strategy#3.1 · no · the hit walk's order and its padding-box stop (overflow, contain: paint) are hit testing, while the lint's readings come through text_extent, text_contrast and the snapshot's bounds, and hit_clipped_at_scrolling_box.rs, the witness named here, is not reached
depth-strategy#4.1 · no · positioned descendants with z-index auto painting in tree order, a paint rule witnessed by paint_order.rs, which the chunk does not reach or re-assert
depth-strategy#4.2 · no · outset box-shadow clipping and its corner-for-corner shape in box_shadow.rs, a paint rule the chunk does not touch and no lint reading takes
depth-strategy#4.3 · no · inset box-shadow painting by Compose::DestOut in box_shadow.rs, a paint rule the chunk does not touch and no lint reading takes
depth-strategy#4.4 · no · opacity, filter and backdrop-filter layer painting in render.rs, a paint rule; the contrast exemption past an opacity or filter is text_contrast's own reader rule, which this entry does not state
depth-strategy#5.1 · no · drop-shadow, backdrop-filter and the GitHub fixture's z-index values in examples/assets, fixtures the chunk does not reach
depth-strategy#5.2 · no · the google and graphite shadow and z-index values in examples/assets, fixtures the chunk does not reach
depth-strategy#5.3 · no · the hr, servo, Preact and box-shadow fixture values in examples, none reached by the chunk or read by stand_lint
depth-strategy#6.1 · no · a search record over 17 listed files for border-radius, shadow and z-index handling; the refactor adds no such handling, so the absence holds
depth-strategy#6.2 · no · a search record for shadow or elevation tokens over 16 listed files with no match; the refactor adds no token, so the absence holds
depth-strategy#6.3 · no · a search record for shadow or elevation definitions over 32 s10 slice files; the refactor adds no definition, so the absence holds
depth-strategy#6.4 · no · a search record for radius, shadow, elevation and z-index tokens over 21 s11 files, one touch-point radius match; the refactor adds no token, so the absence holds
