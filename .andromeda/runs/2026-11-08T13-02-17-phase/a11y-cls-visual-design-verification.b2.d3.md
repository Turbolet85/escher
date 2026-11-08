@answer n=1 entity=visual-design-verification#3.8 v=no
why: the default focus outline is paint, and the read paints nothing
@answer n=2 entity=visual-design-verification#3.9 v=no
why: the forced-colors focus outline of the google fixture is paint, and the read paints nothing
@answer n=3 entity=visual-design-verification#3.10 v=no
why: the password caret and bullet painting is paint, and the read carries no value to mask
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the BBC reduced-motion fixture's media queries are motion, outside the read
@answer n=5 entity=visual-design-verification#5.1 v=no
why: the absence of prefers-reduced-motion and prefers-color-scheme rules is motion and colour-scheme, not reached by the read
@answer n=6 entity=visual-design-verification#5.2 v=yes
why: an open select's list has no motion, so the owner-served rows' own bounds and covered cause read settled right after a step
@answer n=7 entity=visual-design-verification#6.1 v=no
why: the 0.35 opacity of disabled icon buttons is a rendered state colour, while the read's disabled cause is a refusal
@answer n=8 entity=visual-design-verification#6.2 v=no
why: :open matching on details and dialog is author-style keying, and the read keys disabled, not open
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list takes no :open state, which is author-style matching the read does not key on
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner's status word is a test-runner print, not the read
@answer n=11 entity=visual-design-verification#6.5 v=no
why: a group header's synthesized bold is paint weight, while the scope names the header choosing nothing
@answer n=12 entity=visual-design-verification#6.6 v=no
why: the form example's accent-color comment is a note on a colour the read names none of
@answer n=13 entity=visual-design-verification#7.1 v=no
why: the viewport colour scheme and zoom level are preferences the read does not consult
@answer n=14 entity=visual-design-verification#7.2 v=no
why: colour-scheme change tracking rebuilding the stylist device is preference handling the read does not reach
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale tracking invalidating text shaping is layout preference the read does not reach
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom is a shell input the read does not dispatch
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme's prefers-color-scheme tokens are app styling outside the read
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC page's color-scheme declaration is a fixture colour setting the read does not reach
@answer n=19 entity=visual-design-verification#7.7 v=no
why: un-inverting images under inverted-colors is paint
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action panning is a pointer gesture, and the read dispatches no pointer input
@answer n=21 entity=visual-design-verification#8.1 v=yes
why: display none, visibility hidden and inert exclude nodes from the tree and snapshot, with the select option exception, which bounds which nodes can be listed
@answer n=22 entity=visual-design-verification#8.2 v=yes
why: visibility hidden or collapse makes an element never a hit target, which is what covered means for a listed node
@answer n=23 entity=visual-design-verification#8.3 v=yes
why: a zero-determinant transform gives its subtree no hit, and the read's covered cause comes from the same hit-testing through action_point
@answer n=24 entity=visual-design-verification#8.4 v=yes
why: pointer-events none makes an element transparent to hits while its descendants are tested, which bounds what covered means
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none paints no scrollbar, which is paint independent of hits and the read
