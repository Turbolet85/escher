@answer n=1 entity=visual-design-verification#3.8 v=no
why: the blitz-dom default focus outline is paint, and the chunk changes no style or paint.
@answer n=2 entity=visual-design-verification#3.9 v=no
why: the google fixture's forced-colors focus outline is a fixture style the chunk does not touch.
@answer n=3 entity=visual-design-verification#3.10 v=no
why: focused-input caret and scroll painting is paint the buffered render does not change, and focus is only moved, not painted differently.
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the BBC fixture's reduced-motion media queries are fixture CSS the chunk does not touch.
@answer n=5 entity=visual-design-verification#5.1 v=no
why: an observed absence of motion and colour-scheme rules over the s02 files, and the chunk adds no such rules or touches those files.
@answer n=6 entity=visual-design-verification#5.2 v=yes
why: the screen read after a step must read the new state with no advance, the snapshot the buffered render re-asserts through stand_act_popup under stand_*.
@answer n=7 entity=visual-design-verification#6.1 v=no
why: disabled icon button opacity in browser.css is app chrome styling the chunk does not touch.
@answer n=8 entity=visual-design-verification#6.2 v=yes
why: :open tracks the open attribute's mutator write and removal, and the replay must route falsy boolean attribute removal through the same MutationWriter paths.
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is not an :open state, and the chunk leaves select popup behaviour and its attribute write path's effect on it unchanged.
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner's status word text is runner output the chunk does not touch.
@answer n=11 entity=visual-design-verification#6.5 v=no
why: a group header's synthesized bold in an open list is select popup paint the chunk does not change.
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in the form example about accent colour is example text the chunk does not touch.
@answer n=13 entity=visual-design-verification#7.1 v=no
why: viewport colour scheme and document zoom fields in the shell traits are not touched by the chunk.
@answer n=14 entity=visual-design-verification#7.2 v=no
why: colour-scheme device change tracking in stylo_device.rs is not touched by the chunk.
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale invalidation of text shaping in stylo_device.rs is not touched by the chunk.
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom in the shell window is not touched by the chunk.
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme's prefers-color-scheme stylesheets are app CSS the chunk does not touch.
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC and reference colour-scheme declarations are fixture HTML the chunk does not touch.
@answer n=19 entity=visual-design-verification#7.7 v=no
why: the default stylesheet's inverted-colors rule for images and video is paint the chunk does not change.
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch panning per axis in pointer.rs is a pointer path the chunk does not change.
@answer n=21 entity=visual-design-verification#8.1 v=yes
why: the inert tree exclusion read by the snapshot is re-asserted, and the replay must route falsy inert removal and the inert focus fixup through the same paths.
@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility-based hit-test exclusion in node.rs is hit testing the chunk does not change, and the visibility rule is not needed.
@answer n=23 entity=visual-design-verification#8.3 v=no
why: a non-invertible transform answering no hit is hit testing the chunk does not change.
@answer n=24 entity=visual-design-verification#8.4 v=no
why: pointer-events hit transparency is hit testing the chunk does not change.
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width suppression of overlay scrollbars is paint the chunk does not change.
