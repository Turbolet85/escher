@answer n=1 entity=motion#1.1 v=yes
why: the seven_guis styles carry no transitions or animation, which the probe's click and settle readings on Counter or CRUD rely on
@answer n=2 entity=motion#1.2 v=no
why: smooth scroll timing and scroll-behavior govern scrolling, which the probe does not do
@answer n=3 entity=motion#1.3 v=no
why: wheel and key scrolling of the open select list is scrolling, which the probe does not do
@answer n=4 entity=motion#1.4 v=no
why: scroll behaviour values in events.rs:246-249 are a programmatic scroll path the probe does not run, and events.rs is re-read only at the events.rs:162 check
@answer n=5 entity=motion#1.5 v=no
why: the Cells grid's synchronous scroll_to in its arrow-key handler belongs to Cells, which the probe does not run
@answer n=6 entity=motion#1.6 v=no
why: into-view scrolling of nested boxes and the driver's scroll are scrolling, which the probe does not do
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration is touch scrolling, which the probe does not do
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbars appear only after a scroll, which the probe does not do
@answer n=9 entity=motion#1.9 v=no
why: the scrollbar band takes a click only right after a scroll, which the probe does not do, and Cells is not probed
@answer n=10 entity=motion#2.1 v=yes
why: style resolution reads the VirtualClock's time, which advance moves and click leaves alone, so the probe's advance and click readings rest on it
@answer n=11 entity=motion#2.2 v=yes
why: the scope names settle, and this entity states how settle answers for the animating set and the animating flag it returns
@answer n=12 entity=motion#3.1 v=yes
why: it states which verbs move the VirtualClock, advance moves it while click, type and press leave it and settle never advances it
@answer n=13 entity=motion#3.2 v=yes
why: a read after a step on the CRUD select list reads the new state with no advance, the verb behaviour the probe's click and press readings take
@answer n=14 entity=motion#4.1 v=yes
why: the served task's acting steps wait on a load in flight, and during that wait the VirtualClock stands still, so the probe's clock readings depend on it
@answer n=15 entity=motion#4.2 v=no
why: requestAnimationFrame belongs to the blitz-vibey-script runtime, which the Dioxus native stand does not run
@answer n=16 entity=motion#4.3 v=no
why: animation and transition event data are other units of events.rs, not the events.rs:162 check, and the probe reads listener names only
@answer n=17 entity=motion#5.1 v=no
why: the BBC fixture's reduced-motion queries are an example asset the probe does not load
@answer n=18 entity=motion#6.1 v=no
why: the absence of reduced-motion handling concerns a rule the probe does not touch
@answer n=19 entity=motion#6.2 v=no
why: the absence over s07 slice files names no file the chunk reaches, and the stand's own styles are covered by motion#1.1
@answer n=20 entity=motion#7.1 v=no
why: keyframes in the animated_layout and animation example fixtures are not on the seven_guis stand
@answer n=21 entity=motion#7.2 v=no
why: the google fixture's keyframes and transitions are not on the seven_guis stand
@answer n=22 entity=motion#7.3 v=no
why: the graphite, gosub and inline-flex-transform fixture transitions are not on the seven_guis stand
@answer n=23 entity=motion#7.4 v=no
why: the servo.css spinner and transitions belong to a fixture the probe does not load
@answer n=24 entity=motion#7.5 v=no
why: the svg_native and transforms example transitions are not on the seven_guis stand
