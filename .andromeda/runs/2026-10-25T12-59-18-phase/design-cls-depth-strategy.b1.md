# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · browser z-index values in apps/browser; the chunk touches no app under apps/
depth-strategy#1.2 · no · browser menu and suggestion shadow in apps/browser; the chunk touches no app under apps/
depth-strategy#1.3 · yes · the stand's card shadow (0 2px 8px at 0.08) that the select and Cells' grid stack over, which the Needs' depth item re-reads; flight_booker.rs:140 holds it outside the edited choice unit
depth-strategy#1.4 · no · todomvc's layered shadows in examples/todomvc; todomvc is neither the stand nor reached
depth-strategy#1.5 · no · wgpu_texture, custom_widget and transforms example z-index values; those examples are neither the stand nor reached
depth-strategy#1.6 · no · the UA dialog backdrop in blitz-dom's default.css, which the change does not reach; the chunk's scope names no modal dialog and the select paints no popup
depth-strategy#2.1 · yes · the engine's build order of paint children and stacking contexts after layout, which sets how the select and the scrolling grid stack over the frame
depth-strategy#2.2 · yes · the stacking-context root conditions (positioning, z-index on grid items, transforms, opacity) that set the layer the select, the grid and its items stack in
depth-strategy#3.1 · yes · paint and hit order with a hit stopping at a clipping box's padding box, which the Needs' hit-testing item and Cells' covered and off-screen refusals rest on
depth-strategy#4.1 · yes · positioned z-index auto descendants share one paint level in tree order, the stack the select and Cells' cells paint in; paint_order.rs witnesses it
depth-strategy#4.2 · yes · outset shadow painting (clipping under opacity or a translucent background, averaged blur radius, hiding behind its element) that paints the card shadow of #1.3
depth-strategy#4.3 · no · inset shadow painting is neither elevation nor stacking, and no select, grid or frame is stated to carry an inset shadow
depth-strategy#4.4 · no · the opacity, filter and backdrop-filter layer clip is paint detail that binds only where a select, grid or frame carries one, which the Needs do not state; its stacking effect is #2.2
depth-strategy#5.1 · no · drop-shadow, backdrop-filter and a fixed bar's z-index in examples/assets fixtures, which the chunk does not reach
depth-strategy#5.2 · no · google and graphite fixture shadows and z-index in examples/assets, which the chunk does not reach
depth-strategy#5.3 · no · hr, servo, Preact and box-shadow example fixtures, which the chunk does not reach
depth-strategy#6.1 · no · an absence of z-index and shadow handling in 17 listed s06 files; it states no value the chunk paints or tests
depth-strategy#6.2 · no · an absence of shadow or elevation tokens in 16 listed s08 files; it states no value the chunk reads
depth-strategy#6.3 · no · an absence of shadow and elevation definitions in 32 listed s10 files; it states no value the chunk reads or tests
depth-strategy#6.4 · no · an absence of shadow, elevation and z-index tokens in 21 listed s11 files, a touch-point radius only; it states no value the chunk reads or tests
