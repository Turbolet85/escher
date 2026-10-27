# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser app's z-index values stand, since apps/browser/assets/browser.css is unchanged and the session edits no browser-app value
depth-strategy#1.2 · no · the browser menu and suggestions shadow is untouched, the session making no engine or browser-app change
depth-strategy#1.3 · yes · add-to · the stand's card shadow `0 2px 8px` leaves design-system (move-set row 7, PROVISIONAL), so this record keeps the circle dialog's 0.10 line and carries F3, the card shadow held in layout-templates' desktop-native values
depth-strategy#1.4 · no · todomvc's layered shadows are another example's values, and examples/todomvc is unchanged and not a stand row
depth-strategy#1.5 · no · the wgpu_texture, custom-widget and transforms z-index values are other examples' files, unchanged and not reached
depth-strategy#1.6 · no · the blitz-dom default dialog backdrop is an engine default in packages/blitz-dom/assets/default.css, unchanged and not reached
depth-strategy#2.1 · no · the stacking build and its public re-exports in packages/blitz-dom are unchanged and no engine file is reached
depth-strategy#2.2 · no · the stacking-context rules at packages/blitz-dom/src/node/node.rs:1332-1406 are unchanged and no engine file is reached
depth-strategy#3.1 · no · the hit-test order and the paint-clip rule are untouched, the session making no engine change and reaching no engine or test file
depth-strategy#4.1 · no · positioned descendants sharing one paint level is pinned by tests/blitz-tests/tests/paint_order.rs, unchanged and not reached
depth-strategy#4.2 · no · the outset box-shadow paint code and its shape test are unchanged and no paint file is reached
depth-strategy#4.3 · no · the inset box-shadow paint code in blitz-paint's box_shadow.rs is unchanged and no paint file is reached
depth-strategy#4.4 · no · the opacity, filter and backdrop layer code is unchanged and no paint file is reached
depth-strategy#5.1 · no · filters.html and the GitHub fixture's progress bar are fixtures, unchanged and not reached
depth-strategy#5.2 · no · the google and graphite fixtures' shadows and z-index values are fixtures, unchanged and not reached
depth-strategy#5.3 · no · the hr, servo, Preact and box-shadow fixtures are other examples' files, unchanged and not reached
depth-strategy#6.1 · no · no source, test or manifest changed and no shadow, radius or z-index value moved anywhere in the code, so the absence still holds
depth-strategy#6.2 · no · no source, test or manifest changed and the session adds no shadow or elevation token, so the no-match still holds
depth-strategy#6.3 · no · no source, test or manifest changed and no shadow or elevation definition moved, so the absence still holds
depth-strategy#6.4 · no · no source, test or manifest changed and the session adds no token, so the absence and its one touch-point radius match still hold
