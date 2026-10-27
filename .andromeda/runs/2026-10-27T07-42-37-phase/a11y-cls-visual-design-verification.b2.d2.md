# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.3 · no · stylo_device.rs is neither reached nor a conflict or merge surface, and the scope names no hidpi or zoom scale path
visual-design-verification#7.4 · yes · the zoom keys' rule lives in blitz-shell/src/window.rs, a merge surface the redraw routing and winit beta.4 reach
visual-design-verification#7.5 · no · apps/readme's CSS is not reached and the take adds no prefers-color-scheme rule (listed Not needed)
visual-design-verification#7.6 · no · the examples/preact pages are not reached and the scope names no color-scheme rule
visual-design-verification#7.7 · no · default.css's inverted-colors rule lives in an unreached file and the scope names no inverted-colors behaviour
visual-design-verification#7.8 · yes · the touch-action panning rule lives in events/pointer.rs, a merge surface that 0980cce5f edits
visual-design-verification#8.1 · yes · the display-none, visibility-hidden and inert exclusion and the inert hit pass-through live in node/node.rs (conflict surface, 1437-1446), with inert_subtree.rs re-read
visual-design-verification#8.2 · yes · the visibility-hidden and collapse hit exclusion lives in node/node.rs (conflict surface, 1448-1456), the hit path the resolution keeps
visual-design-verification#8.3 · yes · the pointer-events hit transparency lives in node/node.rs (conflict surface, 1458-1462 and 1662-1670), the hit path the resolution keeps
visual-design-verification#8.4 · no · the scrollbar-width paint rule lives in node/scrollbar.rs and scrollbars.rs, neither reached nor a conflict or merge surface, and the scope names no scrollbar paint
visual-design-verification#8.5 · yes · the thumb-drag hit rule lives in events/pointer.rs (merge surface, hit_with_scrollbar) on the document's virtual clock the chunk keeps
visual-design-verification#8.6 · no · the devtools settings live in blitz-traits/src/devtools.rs, which is neither reached nor a merge surface, and the scope names no devtools overlay
