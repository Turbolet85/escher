@answer n=1 entity=motion#1.1 v=yes
why: the seven_guis stand's styles carry no transitions or animations, so a settled click on the stand reads no running motion the probe has to account for.
@answer n=2 entity=motion#1.2 v=no
why: smooth-scroll timing is scroll behaviour, excluded by the scope; the probe does no scrolling.
@answer n=3 entity=motion#1.3 v=no
why: a wheel over the open select list is scroll behaviour, and no select list is on the Counter or CRUD path.
@answer n=4 entity=motion#1.4 v=no
why: scroll behaviour values and programmatic scrolling (events.rs:246-249, 285-288) are scroll behaviour, excluded by the scope; events.rs is re-checked only at line 162.
@answer n=5 entity=motion#1.5 v=no
why: the Cells grid's scroll_to is programmatic scrolling, excluded by the scope, and Cells is not on the probe's path.
@answer n=6 entity=motion#1.6 v=no
why: into-view and driver scroll behaviour are excluded by the scope; the probe does no scrolling.
@answer n=7 entity=motion#1.7 v=no
why: touch-fling deceleration is scroll behaviour the probe never reaches.
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar fade timing only starts after a scroll, and the probe does no scrolling.
@answer n=9 entity=motion#1.9 v=no
why: scrollbar capture of a click right after a scroll needs a prior scroll, which the probe never makes.
@answer n=10 entity=motion#2.1 v=no
why: stylo's time-driven animation states are not named by the scope, and the stand has no CSS animations (motion#1.1) for them to change what the probe reads.
@answer n=11 entity=motion#2.2 v=yes
why: it states the settle answer (Settled with the animating flag) for the animating set, and the scope names settle as the verb the probe's clicks and advances return through.
@answer n=12 entity=motion#3.1 v=yes
why: it states the VirtualClock contract the scope names: advance moves the clock, while click, type and press leave it where it was.
@answer n=13 entity=motion#3.2 v=no
why: the no-motion, no-advance rule is specific to the select list, which the probe does not touch.
@answer n=14 entity=motion#4.1 v=yes
why: a served task's acting steps can wait on a load in flight, and the clock stands still during that wait, so the probe's click readings depend on it.
@answer n=15 entity=motion#4.2 v=no
why: the requestAnimationFrame timer belongs to the script runtime, which the Dioxus seven_guis stand does not run.
@answer n=16 entity=motion#4.3 v=no
why: the probe reads listener names only and never builds animation or transition event data; events.rs:71-73 and 123-125 are outside the re-check at line 162.
@answer n=17 entity=motion#5.1 v=no
why: the BBC fixture's reduced-motion queries are in an example asset the probe does not load.
@answer n=18 entity=motion#6.1 v=no
why: the reduced-motion absence is a styling finding over other surfaces; the probe adds and reads no reduced-motion handling.
@answer n=19 entity=motion#6.2 v=no
why: the transition and animation absence in the s07 slice files is outside the seven_guis stand the probe runs on.
@answer n=20 entity=motion#7.1 v=no
why: the @keyframes and transitions in the animation example assets are not on the probe's path.
@answer n=21 entity=motion#7.2 v=no
why: the google.html keyframes and transitions are a fixture the probe does not load.
@answer n=22 entity=motion#7.3 v=no
why: the graphite, gosub and inline-flex-transform example animations are not on the probe's path.
@answer n=23 entity=motion#7.4 v=no
why: the servo.css spinner and transitions are a fixture the probe does not load.
@answer n=24 entity=motion#7.5 v=no
why: the svg_native and transforms example transitions are not on the probe's path.
