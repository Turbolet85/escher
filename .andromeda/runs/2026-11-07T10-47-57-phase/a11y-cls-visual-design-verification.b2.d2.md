@answer n=1 entity=visual-design-verification#3.8 v=no
why: the default input focus outline is paint the chunk does not change; its focus checks read engine focus, not paint
@answer n=2 entity=visual-design-verification#3.9 v=no
why: the google fixture's forced-colors focus outline is in an example fixture the chunk does not touch
@answer n=3 entity=visual-design-verification#3.10 v=no
why: focused password caret painting and field scrolling are paint behaviour the chunk does not change and no named check reads
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the BBC fixture's prefers-reduced-motion queries are fixture CSS the chunk does not touch
@answer n=5 entity=visual-design-verification#5.1 v=no
why: an absence search over s02 stylesheets for motion and colour-scheme rules, which the chunk does not change
@answer n=6 entity=visual-design-verification#5.2 v=yes
why: the entity's pinned test is a stand_act_popup check, a stand_* check the chunk re-asserts, and a screen read after a step must still read the new state through the replay
@answer n=7 entity=visual-design-verification#6.1 v=no
why: disabled icon button opacity is apps/browser CSS the chunk does not touch
@answer n=8 entity=visual-design-verification#6.2 v=yes
why: open is a boolean attribute written through the mutator path the replay must route, and open_state's tests move it by write and removal and read :open from the OPEN state
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an author's open on a select is engine behaviour the replay routes unchanged, and no select check is named by the scope
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner's status-word output is in wpt/runner, which the chunk does not touch
@answer n=11 entity=visual-design-verification#6.5 v=no
why: the group header's synthesized bold is paint in an open select list, outside the chunk's reach
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in examples/form.rs about input colour, which the chunk does not touch
@answer n=13 entity=visual-design-verification#7.1 v=no
why: the viewport colour scheme and zoom fields are shell traits the chunk does not change
@answer n=14 entity=visual-design-verification#7.2 v=no
why: colour-scheme device-change tracking in stylo_device is outside the render buffer and the chunk does not change it
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale tracking in stylo_device is outside the render buffer and the chunk does not change it
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom in blitz-shell window.rs is not touched by the buffered render
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme's prefers-color-scheme CSS is in apps/readme, which the chunk does not touch
@answer n=18 entity=visual-design-verification#7.6 v=no
why: colour-scheme declarations in the preact example pages, which the chunk does not touch
@answer n=19 entity=visual-design-verification#7.7 v=no
why: the default stylesheet's inverted-colors rule for images and video is CSS the chunk does not change
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action panning in blitz-dom pointer events is input handling the chunk does not change
@answer n=21 entity=visual-design-verification#8.1 v=yes
why: the scope names inert on both attribute paths and the focus fixup under inert, and this rule states that inert excludes the subtree from the tree and that removing inert restores it
@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility hidden and collapse hit-testing is a CSS visibility rule the scope does not name
@answer n=23 entity=visual-design-verification#8.3 v=no
why: non-invertible transform hit-testing is a hit rule the scope does not name
@answer n=24 entity=visual-design-verification#8.4 v=no
why: pointer-events none hit-testing is a hit rule the scope does not name
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none paints overlay scrollbars, and the chunk does not change scrollbar paint
