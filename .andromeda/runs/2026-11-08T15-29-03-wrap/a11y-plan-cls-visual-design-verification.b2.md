@answer n=1 entity=visual-design-verification#3.8 v=no
why: the session changed no stylesheet or focus outline, and the cited blitz-dom default.css is not among its files
@answer n=2 entity=visual-design-verification#3.9 v=no
why: the google fixture's forced-colors focus outlines live in examples/assets/google.html, which the session does not reach
@answer n=3 entity=visual-design-verification#3.10 v=no
why: the caret and selection paint in blitz-paint render.rs and the password test are untouched by the driver verbs
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the BBC reduced-motion queries sit in examples/assets/bbc_reduced.html, which the session does not reach
@answer n=5 entity=visual-design-verification#5.1 v=no
why: the absence of reduced-motion and color-scheme rules over the s02 files holds, since the session adds no CSS and touches none of those files
@answer n=6 entity=visual-design-verification#5.2 v=no
why: the open select list has no motion, and stand_act_popup.rs and the select popup are not among the session's files
@answer n=7 entity=visual-design-verification#6.1 v=no
why: the disabled icon button opacity is in apps/browser/assets/browser.css, which the session does not reach
@answer n=8 entity=visual-design-verification#6.2 v=no
why: the :open styling and open_state tests are in blitz-dom stylo.rs and open_state.rs, and the session changes no open state
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select list still has no :open state, and select_popup.rs and the select engine are not reached by the session
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner status word is printed by wpt/runner/src/main.rs, which the session does not reach
@answer n=11 entity=visual-design-verification#6.5 v=no
why: the group header weight is pinned by select_popup_painted.rs and the select paint, neither reached by the session
@answer n=12 entity=visual-design-verification#6.6 v=no
why: the form example comment is in examples/form.rs, which the session does not reach
@answer n=13 entity=visual-design-verification#7.1 v=no
why: the viewport color scheme and zoom are in blitz-traits shell.rs, which the session does not reach
@answer n=14 entity=visual-design-verification#7.2 v=no
why: the color-scheme device change is in blitz-dom stylo_device.rs, which the session does not reach
@answer n=15 entity=visual-design-verification#7.3 v=no
why: the hidpi and zoom scale tracking is in blitz-dom stylo_device.rs, which the session does not reach
@answer n=16 entity=visual-design-verification#7.4 v=no
why: the keyboard page zoom is in blitz-shell window.rs, which the session does not reach
@answer n=17 entity=visual-design-verification#7.5 v=no
why: the rdme color-scheme tokens live under apps/readme assets, which the session does not reach
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC and reference color-scheme declarations are in examples/preact, which the session does not reach
@answer n=19 entity=visual-design-verification#7.7 v=no
why: the inverted-colors rule is in the blitz-dom default stylesheet, which the session does not change
@answer n=20 entity=visual-design-verification#7.8 v=no
why: the touch-action panning is in blitz-dom events pointer.rs, which the session does not reach
@answer n=21 entity=visual-design-verification#8.1 v=no
why: the display, visibility and inert exclusion and the hit rule are in blitz-dom accessibility and node code, which the session leaves unchanged while the driver's covered check only reads hits
@answer n=22 entity=visual-design-verification#8.2 v=no
why: the visibility hit-target rule is in blitz-dom node.rs, which the session does not change
@answer n=23 entity=visual-design-verification#8.3 v=no
why: the non-invertible transform hit rule is in blitz-dom node.rs, which the session does not change
@answer n=24 entity=visual-design-verification#8.4 v=no
why: the pointer-events hit rule is in blitz-dom node.rs, which the session does not change
@answer n=25 entity=visual-design-verification#8.5 v=no
why: the scrollbar-width suppression is in blitz-dom scrollbar.rs and scrollbars.rs, neither reached by the session
