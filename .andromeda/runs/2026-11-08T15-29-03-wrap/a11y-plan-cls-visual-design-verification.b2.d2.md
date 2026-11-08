@answer n=1 entity=visual-design-verification#3.8 v=no
why: the focus outline sits in blitz-dom default.css, which the session did not change, so the claim holds unchanged
@answer n=2 entity=visual-design-verification#3.9 v=no
why: the forced-colors outline rule sits in the google fixture, outside the session's file set, so it holds unchanged
@answer n=3 entity=visual-design-verification#3.10 v=no
why: the caret and selection paint sits in blitz-paint render.rs, which the session did not change
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the reduced-motion queries sit in the bbc_reduced fixture, which the session did not touch
@answer n=5 entity=visual-design-verification#5.1 v=no
why: the absence search covers style files the session did not touch, and the session adds no motion or colour-scheme rule
@answer n=6 entity=visual-design-verification#5.2 v=no
why: the open-list motion claim is pinned in stand_act_popup.rs, which the session did not change, and the screen read it names still reads the new state
@answer n=7 entity=visual-design-verification#6.1 v=no
why: the disabled icon opacity sits in apps/browser/assets/browser.css, which the session did not change
@answer n=8 entity=visual-design-verification#6.2 v=no
why: the :open matching sits in blitz-dom stylo.rs and open_state.rs, both outside the session, and the actionable rule does not read :open
@answer n=9 entity=visual-design-verification#6.3 v=no
why: the select list's non-:open reading is pinned in select_popup.rs and blitz-dom, neither changed by the session
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT status-word print sits in wpt/runner/src/main.rs, which the session did not touch
@answer n=11 entity=visual-design-verification#6.5 v=no
why: the group header weight is pinned in select_popup_painted.rs, which the session did not change
@answer n=12 entity=visual-design-verification#6.6 v=no
why: the form example comment sits in examples/form.rs, which the session did not touch
@answer n=13 entity=visual-design-verification#7.1 v=no
why: the viewport scheme and zoom fields sit in blitz-traits shell.rs, unchanged by the session
@answer n=14 entity=visual-design-verification#7.2 v=no
why: the colour-scheme device change sits in blitz-dom stylo_device.rs, unchanged by the session
@answer n=15 entity=visual-design-verification#7.3 v=no
why: the hidpi and zoom tracking sits in blitz-dom stylo_device.rs, unchanged by the session
@answer n=16 entity=visual-design-verification#7.4 v=no
why: the keyboard zoom steps sit in blitz-shell window.rs, which the session did not touch
@answer n=17 entity=visual-design-verification#7.5 v=no
why: the rdme colour-scheme CSS sits in the readme app assets, which the session did not touch
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC and reference colour-scheme declarations sit in examples/preact html, which the session did not touch
@answer n=19 entity=visual-design-verification#7.7 v=no
why: the inverted-colors rule sits in blitz-dom default.css, which the session did not change
@answer n=20 entity=visual-design-verification#7.8 v=no
why: the touch-action panning rule sits in blitz-dom pointer.rs, which the session did not change
@answer n=21 entity=visual-design-verification#8.1 v=no
why: the inert and visibility exclusion sits in blitz-dom accessibility.rs and node.rs, unchanged by the session, so the snapshot's exclusion still holds
@answer n=22 entity=visual-design-verification#8.2 v=no
why: the visibility hit rule sits in blitz-dom node.rs, which the session did not change
@answer n=23 entity=visual-design-verification#8.3 v=no
why: the non-invertible transform hit rule sits in blitz-dom node.rs, unchanged by the session
@answer n=24 entity=visual-design-verification#8.4 v=no
why: the pointer-events hit passthrough sits in blitz-dom node.rs, unchanged by the session
@answer n=25 entity=visual-design-verification#8.5 v=no
why: the scrollbar-width suppression sits in blitz-dom scrollbar.rs, which the session did not change
