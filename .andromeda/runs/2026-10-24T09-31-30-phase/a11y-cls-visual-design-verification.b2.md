# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#7.4 · no · page zoom keys in blitz-shell's window.rs are a zoom preference the Not-needed list excludes, and the shell's take is the adapter path only
visual-design-verification#7.5 · no · rdme's prefers-color-scheme tokens are a colour-scheme preference the Not-needed list excludes, and apps/readme is outside the take
visual-design-verification#7.6 · no · the example pages' color-scheme declarations are a colour-scheme preference the Not-needed list excludes, and examples/preact is outside the take
visual-design-verification#7.7 · no · the inverted-colors rule un-inverts images and video, which no need, standing check or contrast row reads, and default.css is outside the take
visual-design-verification#7.8 · no · touch-action panning is a different path in events/pointer.rs from the click focus and double-click clock the chunk re-reads, and no standing check reads touch
visual-design-verification#8.1 · yes · the hidden-content exclusion (display none, visibility hidden, inert), the select option's exception and inert's hit pass-through are the named accessibility_hidden and inert_subtree re-proofs
visual-design-verification#8.2 · yes · the visibility hit exclusion sits in node/node.rs hit testing beside the inert test, the path the chunk's dropped-node skip lands in
visual-design-verification#8.3 · yes · pointer-events transparency is node/node.rs hit testing, the path the inert and clip-stop merge must keep whole
visual-design-verification#8.4 · no · scrollbar-width suppression is paint in node/scrollbar.rs, which no need names, the chunk's clock re-read reaching the fade's timing and not this width rule, and no standing check reads scrollbars
visual-design-verification#8.5 · no · devtools overlay settings live in blitz-traits, which no need names and the take does not reach
