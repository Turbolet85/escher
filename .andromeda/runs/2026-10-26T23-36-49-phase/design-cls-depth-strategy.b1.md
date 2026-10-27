# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · browser z-index values in apps/browser/assets/browser.css; apps/browser is not reached and the lint reads no stacking order
depth-strategy#1.2 · no · browser menu and suggestion box-shadow in apps/browser/assets/browser.css; a shadow value the task's Not needed depth line excludes, in an app the chunk does not reach
depth-strategy#1.3 · no · seven_guis card box-shadow values in examples/seven_guis/src/tasks; the stand is re-read, but no shadow is an input to a stand_lint finding (bounds, text extent, contrast colors), and no value changes
depth-strategy#1.4 · no · todomvc example shadows in examples/todomvc/src/todomvc.css; the example is not reached and no shadow is an input to any lint finding
depth-strategy#1.5 · no · example z-index values in examples/wgpu_texture/src/styles.css, examples/custom_widget.rs and examples/transforms.rs; no example is reached and the lint reads no stacking order
depth-strategy#1.6 · no · blitz-dom default dialog backdrop in packages/blitz-dom/assets/default.css; the stylesheet is not reached, and the contrast reader reads ancestors' background-color, never a dialog ::backdrop
depth-strategy#2.1 · no · stacking-context build after layout and the StackingContext and HoistedPaintChild re-exports in packages/blitz-dom/src; the chunk changes no engine code and the lint reads no stacking
depth-strategy#2.2 · no · stacking-context root conditions in packages/blitz-dom/src/node/node.rs; the chunk calls no stacking code, and the lint's contrast exemption is no part of this rule
depth-strategy#3.1 · no · paint and hit-test stacking walk with its clip and own-box answers; the chunk calls no hit test or paint, and the scope names no stacking or hit reading
depth-strategy#4.1 · no · z-index auto positioned descendants paint in tree order, witnessed by paint_order.rs; a paint rule the lint never runs, in a test the chunk does not touch
depth-strategy#4.2 · no · outset box-shadow clipping in packages/blitz-paint/src/render/box_shadow.rs; paint only, and the chunk touches no paint code
depth-strategy#4.3 · no · inset box-shadow fill and DestOut cut in packages/blitz-paint/src/render/box_shadow.rs; paint only, and the chunk touches no paint code
depth-strategy#4.4 · no · opacity, filter and backdrop-filter layers in packages/blitz-paint/src/render.rs; paint only, and the opacity exemption of the lint's contrast reader is a rule this entity does not state
depth-strategy#5.1 · no · drop-shadow, backdrop-filter and z-index values in examples/assets/filters.html and examples/assets/github_profile_reduced2.html; the chunk reaches no fixture
depth-strategy#5.2 · no · google and graphite fixture shadows and z-index values in examples/assets/google.html and examples/assets/graphite.html; the chunk reaches no fixture
depth-strategy#5.3 · no · hr, servo, Preact and box-shadow fixture values in examples/assets, examples/preact and examples/box_shadow.rs; the chunk reaches none
depth-strategy#6.1 · no · an absence recorded by a search over 17 s06 files; the chunk adds no z-index or shadow token, so nothing in its code must honour the absence
depth-strategy#6.2 · no · a no-match search over 16 s08 files; the chunk adds no shadow or elevation token, so nothing in its code must honour the absence
depth-strategy#6.3 · no · an absence recorded by a search over 32 s10 slice files; the chunk adds no shadow or elevation definition, so nothing in its code must honour the absence
depth-strategy#6.4 · no · a search over 21 s11 files that matched only a touch-point radius; the chunk adds no depth or radius token, so nothing in its code must honour the absence
