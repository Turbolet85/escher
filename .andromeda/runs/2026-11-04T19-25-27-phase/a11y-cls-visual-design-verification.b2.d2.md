@answer n=1 entity=visual-design-verification#3.8 v=no
why: the default focus outline is paint; the probe reads focus from the snapshot, not outline paint
@answer n=2 entity=visual-design-verification#3.9 v=no
why: a forced-colors focus outline in the google fixture; no forced-colors read is in scope
@answer n=3 entity=visual-design-verification#3.10 v=no
why: caret and bullet paint of a focused password input; paint is not reached and the masking rule covers logs only
@answer n=4 entity=visual-design-verification#4.1 v=no
why: reduced-motion queries in the BBC fixture; motion is not needed by the chunk
@answer n=5 entity=visual-design-verification#5.1 v=no
why: absent reduced-motion and colour-scheme rules across the s02 files; motion and preference tokens are not reached
@answer n=6 entity=visual-design-verification#5.2 v=yes
why: the probe's press-then-read of the snapshot rests on a screen read after a step reading the new state with no advance
@answer n=7 entity=visual-design-verification#6.1 v=no
why: disabled icon button opacity in browser.css is paint; not reached
@answer n=8 entity=visual-design-verification#6.2 v=no
why: author :open styles on details and dialog; the probe writes and reads no open attribute
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is no :open state; the probe reads no select list
@answer n=10 entity=visual-design-verification#6.4 v=no
why: WPT runner status word printed as text; runner output is not reached
@answer n=11 entity=visual-design-verification#6.5 v=no
why: group header weight in an open select list; select lists are not reached
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in the form example about accent-color; colour is not reached
@answer n=13 entity=visual-design-verification#7.1 v=no
why: viewport colour scheme and zoom level are user-preference tokens, not needed by the chunk
@answer n=14 entity=visual-design-verification#7.2 v=no
why: colour-scheme device change rebuilding the stylist; the probe changes no colour scheme
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale changes invalidating text shaping; scale changes are not reached
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom in the windowed shell; the probe is headless and presses no zoom key
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme prefers-color-scheme tokens and backgrounds; colour preferences are not reached
@answer n=18 entity=visual-design-verification#7.6 v=no
why: TodoMVC and reference page colour-scheme declarations; not reached
@answer n=19 entity=visual-design-verification#7.7 v=no
why: inverted-colors image rule in the default stylesheet; colour preferences are not reached
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action panning per axis; touch panning is not reached
@answer n=21 entity=visual-design-verification#8.1 v=no
why: display none, visibility hidden and inert exclusion from the tree and hit-testing are not needed; the probe reads no hidden or inert node
@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility hidden and collapse never being hit targets; visibility exclusion is not needed
@answer n=23 entity=visual-design-verification#8.3 v=no
why: non-invertible transform hit answering None; transform hit-testing is not reached
@answer n=24 entity=visual-design-verification#8.4 v=no
why: pointer-events none hit transparency; hit-testing is not reached
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none suppressing overlay scrollbars; paint is not reached
