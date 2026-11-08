@answer n=1 entity=visual-design-verification#3.8 v=no
why: focus outline paint in the blitz-dom default stylesheet, and the chunk paints nothing and changes no focus
@answer n=2 entity=visual-design-verification#3.9 v=no
why: forced-colors focus outline on the google fixture, a paint rule the chunk does not reach
@answer n=3 entity=visual-design-verification#3.10 v=no
why: caret and bullet painting of a focused password input, paint the chunk does not touch, and MASKED_VALUE redaction is not a caret rule
@answer n=4 entity=visual-design-verification#4.1 v=no
why: reduced-motion media queries in the BBC fixture, a motion rule over a fixture the chunk does not read
@answer n=5 entity=visual-design-verification#5.1 v=no
why: absence of reduced-motion and colour-scheme rules, motion and preference rules the chunk paints around
@answer n=6 entity=visual-design-verification#5.2 v=no
why: no motion on an open select list, a motion rule over a select popup the scope does not name
@answer n=7 entity=visual-design-verification#6.1 v=no
why: disabled icon buttons at 0.35 opacity, a paint rule, and the driver's disabled refusal reads the snapshot, not opacity
@answer n=8 entity=visual-design-verification#6.2 v=no
why: author styles keyed on :open for details and dialog, a CSS open state the chunk does not read
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is no :open state, the chunk reads no :open match and the select popup is outside its scope
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner prints status words as text, and the chunk does not reach the WPT runner
@answer n=11 entity=visual-design-verification#6.5 v=no
why: synthesized bold on a group header in an open list, a paint rule over a select popup the chunk does not touch
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in the form example on accent colour, and the chunk measures no colour
@answer n=13 entity=visual-design-verification#7.1 v=no
why: viewport colour scheme and zoom level, a user-preference state the chunk does not read or paint
@answer n=14 entity=visual-design-verification#7.2 v=no
why: colour-scheme device change rebuilding the stylist, a preference rule with no bearing on the coverage read or the replay
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale tracking for text shaping, a preference and paint rule the chunk does not reach
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom steps in the shell window, a shell input rule the chunk does not touch
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme prefers-color-scheme tokens, colour and preference rules in the readme app the chunk does not read
@answer n=18 entity=visual-design-verification#7.6 v=no
why: TodoMVC and reference page colour-scheme declarations, colour scheme on example pages the chunk does not read
@answer n=19 entity=visual-design-verification#7.7 v=no
why: inverted-colors un-inversion of images and video, a preference paint rule
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch panning per axis under touch-action, pointer panning the chunk does not reach
@answer n=21 entity=visual-design-verification#8.1 v=no
why: display none, visibility hidden and inert exclusion from the tree and snapshot, the chunk changes no exclusion and only reads the graph built from the snapshot as it stands
@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility hidden and collapse are never hit targets, a hit-test rule the chunk does not reach
@answer n=23 entity=visual-design-verification#8.3 v=no
why: a non-invertible transform takes no hit, a hit-test rule the chunk does not reach
@answer n=24 entity=visual-design-verification#8.4 v=no
why: pointer-events none makes an element hit-transparent, a hit-test rule the chunk does not reach
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none suppresses overlay scrollbars, a paint rule the chunk does not reach
