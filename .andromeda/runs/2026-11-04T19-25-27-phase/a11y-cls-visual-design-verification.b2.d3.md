@answer n=1 entity=visual-design-verification#3.8 v=no
why: focus outline paint on inputs; the probe reads focus from the accessibility tree and paints no ring
@answer n=2 entity=visual-design-verification#3.9 v=no
why: forced-colors focus outline on the google fixture; the probe reads no page's colour and no fixture
@answer n=3 entity=visual-design-verification#3.10 v=no
why: caret and selection paint of a focused password input; the probe paints nothing and masks values only in logs
@answer n=4 entity=visual-design-verification#4.1 v=no
why: prefers-reduced-motion queries in the BBC fixture; the probe reads no motion preference
@answer n=5 entity=visual-design-verification#5.1 v=no
why: absence of prefers-reduced-motion and prefers-color-scheme rules over the s02 files; no preference rule is read or added by the probe
@answer n=6 entity=visual-design-verification#5.2 v=yes
why: a screen read after a step reads the new state with no advance, the reading link (4) takes through the tree after a handler writes state; the open select list motion part is out of scope
@answer n=7 entity=visual-design-verification#6.1 v=no
why: opacity of disabled icon buttons in the browser chrome; the probe reads no button's opacity
@answer n=8 entity=visual-design-verification#6.2 v=no
why: author styles keyed on :open for details and dialog; the probe writes no attribute path and reads no open state
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is no :open state; the probe reads no select list
@answer n=10 entity=visual-design-verification#6.4 v=no
why: WPT runner status word printed beside its colour; no WPT runner is run or changed by the probe
@answer n=11 entity=visual-design-verification#6.5 v=no
why: synthesized bold on a group header in an open list; the probe reads no select list
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in the form example about accent-color; the probe touches no form example
@answer n=13 entity=visual-design-verification#7.1 v=no
why: viewport colour scheme and document zoom level; the probe reads no viewport preference
@answer n=14 entity=visual-design-verification#7.2 v=no
why: colour-scheme device change rebuilds the stylist; no device change is read or driven by the probe
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale changes invalidate text shaping; the probe drives no scale change
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom in 0.1 steps; the probe sends no zoom key
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme dark and light tokens under prefers-color-scheme; the readme app is not reached
@answer n=18 entity=visual-design-verification#7.6 v=no
why: TodoMVC and reference colour-scheme declarations; neither page is reached by the probe
@answer n=19 entity=visual-design-verification#7.7 v=no
why: inverted-colors un-inversion of images and video; the probe reads no image or video
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action panning per axis; the probe drives no touch or pointer pan
@answer n=21 entity=visual-design-verification#8.1 v=no
why: exclusion of display none, visibility hidden and inert nodes from the tree and of hit-testing through inert; the task lists hidden content and hit-testing through inert as not needed, and the select option exception is out of scope
@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility hidden and collapse never hit-test; the probe takes no hit, it presses through the keyboard harness
@answer n=23 entity=visual-design-verification#8.3 v=no
why: non-invertible transform takes no hit; the probe takes no hit
@answer n=24 entity=visual-design-verification#8.4 v=no
why: pointer-events none passes hits through; the probe takes no hit
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none paints no overlay scrollbar; the probe paints nothing and reads no scrollbar
