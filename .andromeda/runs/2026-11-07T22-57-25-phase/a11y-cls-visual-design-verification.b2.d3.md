@answer n=1 entity=visual-design-verification#3.8 v=no
why: the blitz-dom default focus outline is focus paint, which the chunk's focus-rule exclusion leaves untouched
@answer n=2 entity=visual-design-verification#3.9 v=no
why: forced-colors focus outlines in the google fixture; no graph read touches focus paint
@answer n=3 entity=visual-design-verification#3.10 v=no
why: caret and bullet paint of a focused password input; the graph's masking is a MASKED_VALUE field, not paint
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the BBC fixture's reduced-motion queries; the chunk adds no motion
@answer n=5 entity=visual-design-verification#5.1 v=no
why: an absence of reduced-motion and colour-scheme rules across the s02 files; no chunk code or test reads them
@answer n=6 entity=visual-design-verification#5.2 v=no
why: absent motion on an open select list is select-list motion, which the scope leaves untouched; the named test witnesses a select popup read, not the graph read
@answer n=7 entity=visual-design-verification#6.1 v=no
why: disabled icon button opacity in browser.css; the chunk paints no icon button
@answer n=8 entity=visual-design-verification#6.2 v=no
why: `:open` matching on details and dialog reads the element's OPEN state, which the graph's handler and state reads do not carry
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is no `:open` state; the chunk adds no `:open` read
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner's status word beside its colour; the chunk touches no runner output
@answer n=11 entity=visual-design-verification#6.5 v=no
why: a group header's synthesized bold in an open list is select-list painting, which the scope leaves untouched
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in the form example about accent colour; no chunk code or test reads it
@answer n=13 entity=visual-design-verification#7.1 v=yes
why: the viewport's document zoom default of 1.0 unzoomed is the default the graph's node bounds are measured at, and the chunk re-measures them
@answer n=14 entity=visual-design-verification#7.2 v=no
why: a colour-scheme change rebuilding the stylist device; the chunk changes no device tracking
@answer n=15 entity=visual-design-verification#7.3 v=no
why: a scale change invalidating text shaping; the chunk changes no scale path and the scope names no scale input
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom in the shell window; the chunk adds no key behaviour
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme's prefers-color-scheme tokens in the readme app; the chunk does not touch that app
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC and reference pages' colour-scheme declarations; no chunk code or test reads them
@answer n=19 entity=visual-design-verification#7.7 v=no
why: un-inversion of images and video under inverted-colors; the graph reads no colour or paint
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch panning per axis under touch-action; pointer panning is outside the graph read
@answer n=21 entity=visual-design-verification#8.1 v=yes
why: the graph's nodes are the agent-visible accessibility-tree nodes the snapshot consumes, so the display none, visibility hidden and inert exclusion decides which nodes the graph may list
@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility hidden or collapse as a hit-test target rule; the chunk runs no hit test
@answer n=23 entity=visual-design-verification#8.3 v=no
why: a non-invertible transform taking no hit; hit testing is outside the scope
@answer n=24 entity=visual-design-verification#8.4 v=no
why: pointer-events none passing hits through; hit pass-through is outside the scope
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none suppressing overlay scrollbars; it paints no scrollbar and reads no bounds the graph carries
