# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · yes · apps/browser is reached whole (d63ed2c8c moves its toolbar, URL bar and favicon citations), so this browser.css z-index rule is in reach
depth-strategy#1.2 · yes · apps/browser is reached whole (d63ed2c8c moves its toolbar, URL bar and favicon citations), so this browser.css shadow rule is in reach despite the Not needed line on app shadows
depth-strategy#1.3 · no · a pointer to layout-templates, with the stand's stylesheet not reached and its shadows entering no stand_contrast row
depth-strategy#1.4 · no · examples/todomvc is not reached and no example's styles are in the delta (Not needed)
depth-strategy#1.5 · no · the custom-widget, wgpu_texture and transforms examples are not reached and the custom widget pattern is Not needed
depth-strategy#1.6 · yes · the take carries the details styling (53e9d81c6), so the user-agent colours are re-read and this dialog backdrop colour is one of them
depth-strategy#2.1 · no · resolve.rs and lib.rs are not reached and the scope names no change to when paint children and stacking contexts are built
depth-strategy#2.2 · yes · the stacking-context root rule lives in node/node.rs, a predicted conflict surface, so it binds whatever the hunks show
depth-strategy#3.1 · yes · the hit walk order and the padding-box stop at a clipping box live in node/node.rs, a predicted conflict surface, and are the chunk's hit-test need
depth-strategy#4.1 · no · the paint level of positioned z-auto descendants is blitz-paint's and not reached, the scope names no paint-order change, and the hit walk it shapes binds through #3.1
depth-strategy#4.2 · no · outset shadow painting is blitz-paint's box_shadow.rs, not reached, and the scope names no shadow paint change
depth-strategy#4.3 · no · inset shadow painting is blitz-paint's box_shadow.rs, not reached, and the scope names no shadow paint change
depth-strategy#4.4 · no · opacity, filter and backdrop-filter layers are blitz-paint's render.rs, not reached, and the scope names no opacity or filter change
depth-strategy#5.1 · no · examples/assets fixtures are not reached and no example's styles are in the delta (Not needed)
depth-strategy#5.2 · no · the google and graphite examples/assets fixtures are not reached and no example's styles are in the delta (Not needed)
depth-strategy#5.3 · no · the hr, servo, preact and shadow examples and fixtures are not reached and no example's styles are in the delta (Not needed)
depth-strategy#6.1 · no · a search record over the listed s06 files that the take leaves unchanged, since it adds no z-index, box-shadow or border-radius token and the scope names none
depth-strategy#6.2 · no · a search record over the listed s08 files that the take leaves unchanged, since it adds no shadow or elevation token and the scope names none
depth-strategy#6.3 · no · a search record over the s10 slice that the take leaves unchanged, since it adds no shadow, elevation or z-index definition and the scope names none
depth-strategy#6.4 · no · a search record over the listed s11 files that the take leaves unchanged, since it adds no shadow, elevation or z-index token and radius handling is Not needed
