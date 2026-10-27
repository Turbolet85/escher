# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.3 · no · the hidpi and zoom scale invalidating text shaping lives in packages/blitz-dom/src/stylo_device.rs, which no reached-files entry names, and the scope lists zoom among the not-needed handling
visual-design-verification#7.4 · yes · the keyboard zoom steps and reset at packages/blitz-shell/src/window.rs:654-662 bind though the scope lists zoom as not needed, since window.rs is a merge surface the redraw routing of 6587e7e79 reaches
visual-design-verification#7.5 · no · the prefers-color-scheme tokens live in apps/readme CSS, which the chunk does not reach, and the scope lists prefers-color-scheme as not needed
visual-design-verification#7.6 · no · the TodoMVC and reference pages' color-scheme declarations live in examples/preact HTML, which the chunk does not reach, and the scope lists color-scheme preference handling as not needed
visual-design-verification#7.7 · no · the inverted-colors un-inversion lives in packages/blitz-dom/assets/default.css, which no reached-files entry names, and the scope lists user-preference handling as not needed
visual-design-verification#7.8 · yes · touch-action per-axis panning at packages/blitz-dom/src/events/pointer.rs binds, since pointer.rs is a merge surface the take moves
visual-design-verification#8.1 · yes · the display none, visibility hidden and inert exclusion from the tree, and inert's hit pass-through, bind through node.rs (conflict surface) and inert_subtree, which the chunk re-reads
visual-design-verification#8.2 · yes · visibility hidden and collapse excluded from hit targets at packages/blitz-dom/src/node/node.rs:1448-1456 binds, since node.rs is a conflict surface the hit resolution of ebacd68bd lands in
visual-design-verification#8.3 · yes · pointer-events none passing hits through to tested descendants at packages/blitz-dom/src/node/node.rs:1458-1462 and 1662-1670 binds, since node.rs is a conflict surface the hit resolution lands in
visual-design-verification#8.4 · no · scrollbar-width none suppressing overlay scrollbars lives in packages/blitz-dom/src/node/scrollbar.rs with its test in scrollbars.rs, neither reached nor a merge surface, and the scope names no scrollbar paint
visual-design-verification#8.5 · yes · the thumb-drag gate on a pointerdown inside a shown overlay scrollbar's band, and its clock-driven fade, live in packages/blitz-dom/src/events/pointer.rs (merge surface) and bind with the chunk's double-click and clock needs
visual-design-verification#8.6 · no · the devtools overlay settings live in packages/blitz-traits/src/devtools.rs, which no reached-files entry names, and the scope names no devtools overlay
