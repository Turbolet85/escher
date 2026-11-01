# Classified — design-system · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · the browser z-index values live in apps/browser/assets/browser.css, which the session did not reach
depth-strategy#1.2 · no · the browser menu and suggestions shadow lives in apps/browser/assets/browser.css, not reached
depth-strategy#1.3 · no · the pointer names layout-templates' seven_guis shadows, and no reached file holds them
depth-strategy#1.4 · no · the todomvc shadows live in examples/todomvc, not reached
depth-strategy#1.5 · no · the wgpu_texture, custom_widget example and transforms z-index values live in examples/, not reached (examples/custom_widget.rs is not packages/blitz-dom/src/node/custom_widget.rs)
depth-strategy#1.6 · yes · stale · default.css is a merge surface that lost lines (the picture rule), so the dialog backdrop citation at lines 1034-1037 may shift
depth-strategy#2.1 · no · the re-exports sit at resolve.rs, which is not reached, and at lib.rs:89, above the changed lines past 96-97, so the build-after-layout claim holds
depth-strategy#2.2 · yes · add-to · F1: a blended element is now a stacking root through Node's stacking-context arm, so the root list must carry mix-blend-mode; node.rs is a merge surface and its citations shifted
depth-strategy#3.1 · yes · add-to · F2: render_element's viewport-wide blend layer is a paint-order fact this stacking-order statement omits, and blended roots (F1) widen the hit walk; render.rs and node.rs citations moved
depth-strategy#3.2 · yes · stale · document.rs is a merge surface that gained lines, so the citations at 973-999 and 2436-2438 may move
depth-strategy#3.3 · yes · stale · node.rs is a merge surface that gained lines above line 1485, so the 1485-1490 citation may move
depth-strategy#3.4 · no · visible_region lives in scrolling.rs, which is not reached, and the session changed neither its clip reading nor the select-list scrollport
depth-strategy#4.1 · yes · add-to · F1: a blended element is a stacking root, so positioned z-auto descendants inside it share its paint level, not the root's; F2's blend layer is a paint-order fact this statement omits
depth-strategy#4.2 · no · re-read: the open select list still paints last after the root's render, now past the blend layers, and paint_select_list's call holds no hunk
depth-strategy#4.3 · yes · add-to · F4: render_element now skips the own shadow of an inline box holding only block children, an exception the outset-shadow row must carry; box_shadow.rs citations moved
depth-strategy#4.4 · yes · add-to · F4: render_element now skips the own shadow of an inline box holding only block children, an exception the inset-shadow row must carry; box_shadow.rs citation 90-157 may move
depth-strategy#4.5 · yes · add-to · F2: the layer that carries opacity and filter (maybe_with_layer) now takes a BlendMode, and blend paints through the same layer path; render.rs citations moved
depth-strategy#4.6 · no · the select list's whole paint past opacity and clipping boxes lives in select_list.rs and select_popup_painted.rs, neither reached, and the list still paints last
depth-strategy#5.1 · no · the drop-shadow, backdrop-filter and z-index fixtures live in examples/assets, not reached
depth-strategy#5.2 · no · the google and graphite shadow and z-index fixtures live in examples/assets, not reached
depth-strategy#5.3 · no · the hr, servo, Preact and box-shadow fixtures live in examples/, not reached, and the inline-box skip concerns no row these fixtures name
depth-strategy#6.1 · no · the absence searches z-index and shadow tokens over the s06 files and names no blend or isolation term, so the blend and isolation painting does not reach it
depth-strategy#6.2 · no · the absence searches shadow and elevation tokens and names no blend or isolation term, so the session does not reach it
depth-strategy#6.3 · no · the absence searches shadow, elevation and z-index tokens over the s10 slice and names no blend or isolation term, so the session does not reach it
depth-strategy#6.4 · no · the absence searches radius, shadow, elevation and z-index tokens and names no blend or isolation term, so the session does not reach it
