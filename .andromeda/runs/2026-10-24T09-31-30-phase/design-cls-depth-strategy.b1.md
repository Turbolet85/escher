# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · browser chrome z-index values (tooltip, menu, suggestions, FPS overlay, status bar) in apps/browser/assets/browser.css, and the take gives apps/browser only an allocator change
depth-strategy#1.2 · no · the browser menu and suggestion box-shadow in apps/browser/assets/browser.css, browser chrome the take does not restyle
depth-strategy#1.3 · yes · the stand's card and circle-dialog box-shadow values paint stand elements that a Stylo move could change, read through the screenshot checks
depth-strategy#1.4 · no · the todomvc example's layered shadows, in a file the task names none of and that no stand check reads
depth-strategy#1.5 · no · root-example z-index values in wgpu_texture, custom_widget and transforms, in files the task names none of and that no stand check reads
depth-strategy#1.6 · yes · the dialog backdrop is a value in packages/blitz-dom/assets/default.css, the user-agent stylesheet CARRY 5 re-reads against the take
depth-strategy#2.1 · yes · the order in which paint children and stacking contexts are built, after layout and transforms, is what the take's hit-walk change to hit_inner reads under the engine depth behaviour need
depth-strategy#2.2 · yes · which nodes are stacking-context roots sets the hoisted and paint-child lists that the hit walk and paint order read, and the take records out-of-flow-ness on the node
depth-strategy#3.1 · yes · hit testing stops at the padding box of a box that clips by overflow or by a contain that includes paint, the root excepted, which the engine depth behaviour need names and the take's hit_inner change (f102663b8) reaches
depth-strategy#4.1 · yes · positioned descendants with z-index auto paint in tree order at one level, pinned by paint_order.rs, which the take's out-of-flow flags and hit-walk change could move
depth-strategy#4.2 · yes · outset box-shadow painting, with its averaged border radius and its opacity and background clipping, paints the stand cards' blurred shadows named by depth-strategy#1.3 under the engine radius and shadow need
depth-strategy#4.3 · no · inset-shadow painting is reached by no stand value in this batch (the stand's shadows are outset) and the take's listed delta names no box-shadow change
depth-strategy#4.4 · no · opacity, filter and backdrop-filter layering is named by no Need (the Needs bind border, shadow and radius values) and the take's listed delta changes none of it
depth-strategy#5.1 · no · drop-shadow, backdrop-filter and the GitHub fixture's z-index are examples/assets fixture values in files the task names none of, and that no stand check reads
depth-strategy#5.2 · no · the google and graphite shadow and z-index values are examples/assets fixture values in files the task names none of, and that no stand check reads
depth-strategy#5.3 · no · the hr, servo, Preact and box-shadow example fixtures are not stand values, sit in files the task names none of, and are read by no stand check
depth-strategy#6.1 · no · an absence of z-index and shadow handling in the s06 files, which binds nothing the chunk must honour since it adds no visual element
depth-strategy#6.2 · no · an absence of shadow or elevation tokens; the elevation scale is recorded as not yet measured and the chunk measures no token set
depth-strategy#6.3 · no · an absence of shadow or elevation definitions in the s10 files, which binds nothing the chunk must honour
depth-strategy#6.4 · no · an absence of shadow, elevation or z-index tokens, with one incidental radius touch-point the entity does not name, so the chunk has nothing to honour
