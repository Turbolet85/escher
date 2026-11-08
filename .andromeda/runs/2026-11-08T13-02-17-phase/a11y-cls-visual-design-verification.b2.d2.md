@answer n=1 entity=visual-design-verification#3.8 v=no
why: the focus outline is paint from the default stylesheet, and the read paints nothing (not_needed 10)
@answer n=2 entity=visual-design-verification#3.9 v=no
why: a fixture's forced-colors focus outline is paint, and the read paints nothing (not_needed 10)
@answer n=3 entity=visual-design-verification#3.10 v=no
why: caret and bullet painting in a focused password field is paint, and needs 11 covers values, not paint
@answer n=4 entity=visual-design-verification#4.1 v=no
why: reduced-motion media queries in a fixture are motion, which is not_needed 10
@answer n=5 entity=visual-design-verification#5.1 v=no
why: observed absence of motion and colour-scheme rules across the s02 files is motion, which is not_needed 10
@answer n=6 entity=visual-design-verification#5.2 v=yes
why: an open select list's rows are read with no advance after a step, which the owner-served rows and their bounds (needs 9) and the stand proof depend on
@answer n=7 entity=visual-design-verification#6.1 v=no
why: disabled icon button opacity is paint in browser.css, and the read's disabled cause keys on the attribute, not on opacity
@answer n=8 entity=visual-design-verification#6.2 v=no
why: author :open on details and dialog is open-state styling for nodes the read does not list or key on
@answer n=9 entity=visual-design-verification#6.3 v=yes
why: an open select list is no :open state, so the read must tell an open list from a closed one (needs 9) without keying on :open
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner status word is a test-runner print that the read does not reach
@answer n=11 entity=visual-design-verification#6.5 v=no
why: a group header's synthesized bold is paint, and the read paints nothing (needs 9 covers the header choosing nothing, not its weight)
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in the form example about accent-color reaches no read
@answer n=13 entity=visual-design-verification#7.1 v=no
why: the viewport colour scheme and zoom are shell state the read neither reads nor paints
@answer n=14 entity=visual-design-verification#7.2 v=no
why: a colour-scheme device change rebuilds the stylist, and the read paints and reads no colour scheme
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale invalidating text shaping is shaping, which the read does not reach
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom steps are a key handler, and the read dispatches no key (not_needed 5)
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme colour-scheme tokens are the readme app's paint, which the read does not reach
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC and reference colour-scheme declarations are page styling no read reaches
@answer n=19 entity=visual-design-verification#7.7 v=no
why: the inverted-colors un-invert of images and video is paint, which the read does not reach
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action panning is pointer handling, and the read dispatches no pointer
@answer n=21 entity=visual-design-verification#8.1 v=yes
why: display none, visibility hidden and inert exclude nodes from the tree and snapshot, the select option exception and the pass-through hit bound which nodes can be listed and what covered means (needs 8)
@answer n=22 entity=visual-design-verification#8.2 v=yes
why: visibility hidden and collapse are never hit-test targets, which decides covered for a listed node (needs 8)
@answer n=23 entity=visual-design-verification#8.3 v=yes
why: a non-invertible transform takes no hit for its whole subtree, which the action_point covered reading depends on (needs 8)
@answer n=24 entity=visual-design-verification#8.4 v=yes
why: pointer-events none makes an element hit-transparent while its descendants are still tested, which decides covered (needs 8)
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width suppressing overlay scrollbar paint is paint, and the read paints nothing
