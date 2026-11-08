@answer n=1 entity=motion#1.1 v=no
why: the session changed no transition or animation in the apps' CSS, so the todomvc durations stand
@answer n=2 entity=motion#1.2 v=no
why: the session changed no smooth-scroll curve, duration or wheel cancel in blitz-dom
@answer n=3 entity=motion#1.3 v=no
why: the select list wheel and key behaviour is unchanged, and actions opens no list
@answer n=4 entity=motion#1.4 v=no
why: scroll-behaviour parsing and programmatic Smooth or Instant support are untouched
@answer n=5 entity=motion#1.5 v=no
why: the Cells arrow-key handler's Instant scroll_to is unchanged and the session adds no scroll call
@answer n=6 entity=motion#1.6 v=yes mode=add-to
why: the actions read says scroll is accepted on every listed node with no target check, which the driver scroll text must now carry
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration in scrolling.rs is unchanged
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar timings and fade rules are unchanged
@answer n=9 entity=motion#1.9 v=no
why: click's hit-testing and scrollbar-band behaviour is unchanged, since the actions read only reports the existing action_point verdicts
@answer n=10 entity=motion#2.1 v=no
why: CSS animation time resolution in stylo.rs is unchanged
@answer n=11 entity=motion#2.2 v=no
why: the settle set, the animating flag and the 700 ms scrollbar hold are unchanged, and actions calls no settle
@answer n=12 entity=motion#3.1 v=yes mode=add-to
why: actions is a read that moves no clock, so the clause listing verbs that leave the VirtualClock alone must now name actions beside click, type and press
@answer n=13 entity=motion#3.2 v=no
why: the select list still has no motion, and the session changed no select-list read timing
@answer n=14 entity=motion#4.1 v=no
why: the load wait's LOAD_WAIT and clock stand-still are unchanged in session.rs
@answer n=15 entity=motion#4.2 v=no
why: requestAnimationFrame's 16 ms timer approximation is unchanged
@answer n=16 entity=motion#4.3 v=no
why: the unimplemented animation and transition event data in dioxus-native-dom is unchanged
@answer n=17 entity=motion#5.1 v=no
why: the BBC fixture's reduced-motion queries are an asset the session did not touch
@answer n=18 entity=motion#6.1 v=no
why: the session added no reduced-motion handling, so the absence search still holds
@answer n=19 entity=motion#6.2 v=no
why: the session added no transition or animation in the s07 slice files
@answer n=20 entity=motion#7.1 v=no
why: the example keyframes and transitions in examples/assets are untouched
@answer n=21 entity=motion#7.2 v=no
why: the google.html keyframes and transitions are untouched
@answer n=22 entity=motion#7.3 v=no
why: the graphite, gosub and inline-flex-transform animations are untouched
@answer n=23 entity=motion#7.4 v=no
why: the servo.css spinner and transitions are untouched
@answer n=24 entity=motion#7.5 v=no
why: the svg_native and transforms example transitions are untouched
