# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser app's own z-index values in its stylesheet; the shell is not reached and an app value is not an engine default the list reads
depth-strategy#1.2 · no · the browser menu's shadow is app CSS, and the scope names no shadow for the list
depth-strategy#1.3 · no · the seven_guis card and circle-dialog shadows are layout-template values, not the select or the list
depth-strategy#1.4 · no · todomvc's own layered shadows are example styling, a demonstration a new engine default does not read
depth-strategy#1.5 · no · wgpu_texture and custom-widget z-index values are example fixtures, demonstrations
depth-strategy#1.6 · yes · the list's new colour default is read against the engine-painted defaults the scope names, and this dialog backdrop is one of them, in default.css which the chunk edits
depth-strategy#2.1 · yes · the list's place in paint order is an overlay above the page, read against the plan's hoisted paint children
depth-strategy#2.2 · yes · whether a select's ancestor is a stacking-context root decides the layer its open list paints in, against the overlay above the page
depth-strategy#3.1 · yes · the padding-box clip rule that stops hits over a select inside an overflow or contain: paint ancestor, which the overlay must answer on its rows through, and the stacking order of hit testing over the list
depth-strategy#3.2 · no · the non-invertible transform no-hit rule is not named in the scope, and the new code must act on nothing it sets
depth-strategy#3.3 · no · visible_region's transform reading is not named in the scope; the chunk's clip reading is the padding-box rule in 3.1 with no transform on the chain
depth-strategy#4.1 · yes · a positioned overlay with z-index auto shares one paint level and paints in tree order, which sets the list's place above the page
depth-strategy#4.2 · no · outset box-shadow handling; the scope's surface values name padding, row height, gaps, border and corners, no shadow
depth-strategy#4.3 · no · inset box-shadow handling; no inset shadow is named for the list
depth-strategy#4.4 · yes · the opacity and filter layer clipped to the border box is a layer the list paints through when a select sits under such an ancestor, the scope's "layers" reading
depth-strategy#5.1 · no · drop-shadow, backdrop-filter and z-index values in example fixtures, demonstrations a new engine default does not read
depth-strategy#5.2 · no · google and graphite shadow and z-index values are example fixtures
depth-strategy#5.3 · no · hr, servo, preact and box-shadow fixture values are demonstrations
depth-strategy#6.1 · yes · the record that no z-index handling exists in the listed s06 files bears on the list's new paint level, which has no token to follow
depth-strategy#6.2 · no · the record of no shadow or elevation tokens; the scope names no shadow or elevation for the list
depth-strategy#6.3 · yes · the record of no z-index or shadow definitions in the s10 slice bears on the list's new paint level, which has no existing definition to follow
depth-strategy#6.4 · yes · the record of no z-index tokens and one touch-point radius in the s11 files bears on the list's paint level and on its corners, read against engine radius handling
