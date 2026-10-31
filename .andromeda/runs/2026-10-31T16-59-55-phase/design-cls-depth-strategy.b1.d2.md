# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser app's own z-index scale (apps/browser chrome), which the engine overlay does not read; the layer read is against engine hoisted children and depth
depth-strategy#1.2 · no · the browser menu's shared box-shadow is app chrome; the list's named surface values are padding, row height, gaps, border, corners and colours, with no shadow
depth-strategy#1.3 · no · the seven_guis card and circle-dialog shadows, held in layout-templates; not the select popup's surface
depth-strategy#1.4 · no · todomvc's layered shadows are an example demonstration the new engine default does not read
depth-strategy#1.5 · no · example fixtures' z-index and overlay values are demonstrations the new engine default does not read
depth-strategy#1.6 · yes · the list's colours are read against the engine-painted defaults, and this default.css dialog backdrop is one of them, in a file the change edits
depth-strategy#2.1 · yes · the overlay above the page is read against the plan's hoisted paint children and their public re-exports
depth-strategy#2.2 · yes · whether the open list's node is a stacking-context root decides its paint order against the page
depth-strategy#3.1 · yes · the overlay's paint order and hit walk, and the rule stopping hits at the padding box of a clipped ancestor, which the list's rows must answer under
depth-strategy#3.2 · yes · a list under a degenerate transform answers no hit for its subtree, which the overlay's hit answer must account for
depth-strategy#3.3 · yes · a long list's nested scroll and its into-view scroll read each clipping box through transforms
depth-strategy#4.1 · yes · the list's positioned rows with auto z-index share one paint level and paint in tree order over the page
depth-strategy#4.2 · no · outset box shadows are not named in the chunk's surface values, and the list paints no shadow in scope
depth-strategy#4.3 · no · inset box shadows are not named in scope and the list paints none
depth-strategy#4.4 · no · opacity, filter and backdrop-filter layers are not named in scope; motion is a P4 fork and no effect layer is in the chunk
depth-strategy#5.1 · no · github and filters fixtures' own z-index and filter values are demonstrations the new engine default does not read
depth-strategy#5.2 · no · google and graphite fixtures' own shadows and z-index values are demonstrations the new engine default does not read
depth-strategy#5.3 · no · hr, servo, Preact and box-shadow fixtures are examples the new engine default does not read
depth-strategy#6.1 · yes · records that no z-index layering exists in the listed s06 files, the layer read the overlay's paint order is against
depth-strategy#6.2 · yes · records no shadow or elevation tokens in the s08 files, the layer read against which the overlay sits above the page
depth-strategy#6.3 · yes · records no shadow or elevation definitions in the s10 slice, the layer read for the overlay
depth-strategy#6.4 · yes · records no shadow, elevation or z-index tokens in the s11 files, the layer read; its one radius touch-point is the corners' read
