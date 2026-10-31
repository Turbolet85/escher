# Classified — a11y-plan · visual-design-verification.b2

## Answers
visual-design-verification#6.3 · no · the WPT runner status-word line (wpt/runner/src/main.rs) is not reached by the session
visual-design-verification#6.4 · no · the form example's accent-color comment (examples/form.rs) is not reached by the session
visual-design-verification#7.1 · no · the viewport colour scheme and zoom (blitz-traits shell.rs, blitz-shell window.rs) are not reached and unchanged
visual-design-verification#7.2 · no · stylo_device.rs is not reached; the session reaches stylo.rs only, so the colour-scheme device claim stands
visual-design-verification#7.3 · no · stylo_device.rs is not reached, so the hidpi and zoom invalidation claim stands unmoved
visual-design-verification#7.4 · no · page zoom in blitz-shell window.rs is not reached by the session
visual-design-verification#7.5 · no · rdme's prefers-color-scheme CSS is not reached by the session
visual-design-verification#7.6 · no · the preact example pages' color-scheme declarations are not reached by the session
visual-design-verification#7.7 · yes · stale · default.css is reached and its citation 1126-1137 is past the session's insert, so the cited line range moved
visual-design-verification#7.8 · yes · stale · events/pointer.rs is reached whole and the report does not show the touch-action citations 161-207 and 254-267 hold on the changed file
visual-design-verification#8.1 · yes · stale · accessibility.rs gained an insert at 238-242 and the citation 281-301 sits past it, so its digits moved (146-156 and 149-150 hold)
visual-design-verification#8.2 · no · node.rs is not reached and hit_inner is not edited, so the visibility-hidden hit rule stands
visual-design-verification#8.3 · no · node.rs is not reached and hit_inner is not edited, so the non-invertible transform hit rule stands
visual-design-verification#8.4 · no · node.rs is not reached and hit_inner is not edited, so the pointer-events hit rule stands
visual-design-verification#8.5 · no · scrollbar.rs and scrollbars.rs are not reached and scrollbar-width none paint suppression is unchanged
visual-design-verification#8.6 · yes · add-to · F24 (the list has no motion and needs no advance) joins the advance rule beside the scrollbar-band hazard, which still stands as written
visual-design-verification#8.7 · no · devtools.rs is not reached by the session
