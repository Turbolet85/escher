# Classified — a11y-plan · visual-design-verification.b2

## Answers
visual-design-verification#7.4 · no · page zoom sits in packages/blitz-shell/src/window.rs, which is not among the reached files, and no reached check drives zoom
visual-design-verification#7.5 · no · the rdme color-scheme CSS under apps/readme/assets is not among the reached files, and the session changes no rdme file
visual-design-verification#7.6 · no · the examples/preact pages it cites are not among the reached files, and the session changes no color-scheme declaration
visual-design-verification#7.7 · no · blitz-dom's default.css is not among the reached files and no engine crate moved, so the inverted-colors un-invert holds
visual-design-verification#7.8 · no · pointer.rs is not among the reached files, and F4 is a mouse main-button pointerdown on a scrollbar band, not touch panning or touch-action
visual-design-verification#8.1 · no · the hidden-content exclusion, inert and the select-option exception are unchanged, and the flight booker's two options stay its children in the tree and the snapshot
visual-design-verification#8.2 · no · the visibility: hidden and collapse hit rule sits in node.rs, which is not among the reached files, and the session changes no visibility rule
visual-design-verification#8.3 · no · the pointer-events hit rule sits in node.rs, which is not among the reached files, and the session changes no pointer-events rule
visual-design-verification#8.4 · yes · add-to · F4 adds that a click right after a scroll lands on the shown overlay scrollbar, its band's main-button pointerdown taken as a thumb drag and the bar shown until an advance passes its fade on the virtual clock
visual-design-verification#8.5 · no · packages/blitz-traits/src/devtools.rs is not among the reached files, and devtools outlines and highlight overlays are unrelated to the session
