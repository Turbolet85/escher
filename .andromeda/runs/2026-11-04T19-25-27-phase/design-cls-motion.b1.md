@answer n=1 entity=motion#1.1 v=no
why: todomvc transition timings; the probe styles nothing and the seven_guis stand has no transitions observed
@answer n=2 entity=motion#1.2 v=no
why: smooth scroll timing and scroll-behavior; the probe does no scrolling
@answer n=3 entity=motion#1.3 v=no
why: wheel over an open select list and key into-view; the probe does no scrolling and Counter or CRUD has no select list
@answer n=4 entity=motion#1.4 v=no
why: scroll behaviour value parsing and programmatic Smooth or Instant; the probe makes no scroll call and the events.rs re-check is the line 162 coordinate only
@answer n=5 entity=motion#1.5 v=no
why: Cells grid arrow-key Instant scroll; Cells is not on the Counter or CRUD stand and the probe does no scrolling
@answer n=6 entity=motion#1.6 v=no
why: into-view and driver scroll instant in nested boxes; the probe makes no scroll call and the driver scroll is not one it uses
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration; the probe sends no touch input
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar fade timings; the probe scrolls nothing so no scrollbar is summoned
@answer n=9 entity=motion#1.9 v=no
why: a click landing on a shown scrollbar band right after a scroll; the probe makes no scroll so the band never shows over its clicks
@answer n=10 entity=motion#2.1 v=no
why: CSS animation state by current time during style resolution; the probe reads nodes, handlers, state and readers, not style, and the stand has no CSS animation
@answer n=11 entity=motion#2.2 v=yes
why: settle answers for animating members without waiting on or advancing them and reports the animating flag; settle is a verb the probe's headless runs call, so its reading binds
@answer n=12 entity=motion#3.1 v=yes
why: advance moves the VirtualClock by its ms while click, type and press leave the clock where it was; the scope names this clock behaviour for the probe's steps
@answer n=13 entity=motion#3.2 v=no
why: the select list needs no advance and reads new state at once; the select list is not on the Counter or CRUD stand
@answer n=14 entity=motion#4.1 v=yes
why: a load wait inside an acting step holds the VirtualClock still for up to LOAD_WAIT; the probe's served task and its click or press steps can wait on a load in flight
@answer n=15 entity=motion#4.2 v=no
why: requestAnimationFrame as a 16 ms timer in the script runtime; the Dioxus stand does not run blitz-vibey-script
@answer n=16 entity=motion#4.3 v=no
why: animation and transition event data are unimplemented; the probe reads listener names and dispatches no animation event
@answer n=17 entity=motion#5.1 v=no
why: BBC fixture reduced-motion queries; the fixture is not on the seven_guis stand
@answer n=18 entity=motion#6.1 v=no
why: observed absence of reduced-motion handling across listed fixture and slice files; the probe does not touch those surfaces
@answer n=19 entity=motion#6.2 v=no
why: observed absence of transitions or animation in the s07 slice files; not on the probe's stand
@answer n=20 entity=motion#7.1 v=no
why: example keyframes and transitions in the animated fixtures; not on the seven_guis stand
@answer n=21 entity=motion#7.2 v=no
why: google fixture keyframes and transitions; not on the probe's stand
@answer n=22 entity=motion#7.3 v=no
why: graphite, gosub and inline-flex-transform fixture transitions; not on the probe's stand
@answer n=23 entity=motion#7.4 v=no
why: servo.css spinner keyframes and transitions; not on the probe's stand
@answer n=24 entity=motion#7.5 v=no
why: svg_native and transforms example transitions; not on the probe's stand
