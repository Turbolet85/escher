@answer n=1 entity=visual-design-verification#3.8 v=no
why: the blitz-dom default focus outline is paint; the chunk paints nothing and the focusability rule it needs is not stated here
@answer n=2 entity=visual-design-verification#3.9 v=no
why: the google fixture's forced-colors focus outline is paint in a fixture the chunk does not reach
@answer n=3 entity=visual-design-verification#3.10 v=no
why: the caret and selection paint over a password input's bullets is painting; the chunk's MASKED_VALUE read carries no paint
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the BBC fixture's prefers-reduced-motion queries are a motion preference the read does not touch
@answer n=5 entity=visual-design-verification#5.1 v=no
why: an absence of motion and colour-scheme rules across the s02 files; the read paints nothing and states no preference
@answer n=6 entity=visual-design-verification#5.2 v=yes
why: it states that a screen read after a step reads the new state with no advance, the call the read makes over an open list's owner-served rows in the stand proof
@answer n=7 entity=visual-design-verification#6.1 v=no
why: a disabled icon button's 0.35 opacity is a visual state colour; the disabled refusal keys on the snapshot's enabled, not opacity
@answer n=8 entity=visual-design-verification#6.2 v=no
why: the :open author-style keying of details and dialog is a state the read lists nowhere; the read keys disabled by its own attribute presence
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is no :open state, and the read takes the open list's rows from the snapshot model, not :open
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner's status word beside its colour is a runner output the read does not produce
@answer n=11 entity=visual-design-verification#6.5 v=no
why: a group header's synthesized bold is paint; the group header choosing nothing is a click rule the entity does not state
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a form example comment on the input accent colour names no colour the read uses
@answer n=13 entity=visual-design-verification#7.1 v=no
why: the viewport's colour scheme and zoom level are preferences the read does not name, and the off-screen cause is the shared action_point rule, which this entity does not state
@answer n=14 entity=visual-design-verification#7.2 v=no
why: a colour-scheme change rebuilding the stylist device is a preference the read does not reach
@answer n=15 entity=visual-design-verification#7.3 v=no
why: total scale changes invalidating text shaping are paint and layout, and the read runs no step to change scale
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom in 0.1 steps is a key handling the read does not dispatch and a zoom it never changes
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme's prefers-color-scheme tokens are a colour preference outside the read
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC and reference pages' color-scheme declarations are colour-scheme preferences the read does not reach
@answer n=19 entity=visual-design-verification#7.7 v=no
why: the inverted-colors image un-inversion is paint of images, which the read does not paint
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action panning is a pointer scroll rule; the read performs no pointer action
@answer n=21 entity=visual-design-verification#8.1 v=yes
why: display none, visibility hidden and inert exclude nodes from the tree and snapshot, with the select's option exception and inert passing hits, which bound which nodes the read lists and what covered means
@answer n=22 entity=visual-design-verification#8.2 v=yes
why: visibility hidden or collapse is never a hit-test target, which decides the covered cause the read returns
@answer n=23 entity=visual-design-verification#8.3 v=yes
why: a zero-determinant transform makes hit_inner answer None for its whole subtree, which changes what covered means for nodes under it
@answer n=24 entity=visual-design-verification#8.4 v=yes
why: pointer-events none makes an element transparent to hits while its descendants are still tested, which sets the covered answer the read reports
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none suppresses overlay scrollbar paint, which the read does not paint or measure
