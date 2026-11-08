@answer n=1 entity=motion#1.1 v=no
why: observed absence of transitions in seven_guis styles; the chunk's signal move adds no style, and the todomvc transitions sit in an unreached app
@answer n=2 entity=motion#1.2 v=no
why: smooth-scroll timing of viewport and scroller boxes; the graph read scrolls nothing
@answer n=3 entity=motion#1.3 v=no
why: wheel over the open select list; the chunk touches no select list scrolling or painting
@answer n=4 entity=motion#1.4 v=no
why: scroll-behaviour values parsed by the script runtime and dioxus programmatic scrolling; the chunk adds no scroll call
@answer n=5 entity=motion#1.5 v=yes
why: the Cells arrow-key handler in stand.rs calls scroll_to(ScrollBehavior::Instant) synchronously, and the Cells signals move onto the registry in that handler, so the call must stay
@answer n=6 entity=motion#1.6 v=no
why: scroll_into_view and the driver's scroll; the chunk calls neither
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration in blitz-dom; unreached and the chunk drives no fling
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar fade timings; the chunk draws no scrollbar
@answer n=9 entity=motion#1.9 v=yes
why: a driver click on a Cells cell in the scrollbar band right after a scroll lands on the scrollbar; the scope names the Cells task and its signals move, so that click path must keep this reading
@answer n=10 entity=motion#2.1 v=no
why: CSS animation and transition state during style resolution; the chunk touches no style resolution
@answer n=11 entity=motion#2.2 v=no
why: the settle answer's animating set and the 700 ms scrollbar read; the chunk changes no settle and drives no animation
@answer n=12 entity=motion#3.1 v=yes
why: the driver's advance hands the stand's app time through the step so whole Timer ticks land together; stand.rs is reached whole and its signals, the Timer's included, move onto the registry, so the tick must stay whole
@answer n=13 entity=motion#3.2 v=yes
why: a screen read after a step reads the new state with no advance; the graph read is a screen read of the current screen and must not need an advance for the select list
@answer n=14 entity=motion#4.1 v=no
why: the load wait holds the VirtualClock still in session.rs; the graph read takes no acting step and waits on no load
@answer n=15 entity=motion#4.2 v=no
why: requestAnimationFrame timer approximation in the script runtime; unreached
@answer n=16 entity=motion#4.3 v=no
why: animation and transition event data are unimplemented in dioxus-native-dom events.rs; the graph read lists handlers and dispatches none, so no animation event data is built
@answer n=17 entity=motion#5.1 v=no
why: prefers-reduced-motion media queries in the BBC fixture; the chunk re-reads no fixture
@answer n=18 entity=motion#6.1 v=no
why: observed absence of reduced-motion handling; the chunk adds no motion or media query
@answer n=19 entity=motion#6.2 v=no
why: observed absence of transitions or animation in the s07 slice files; the chunk adds none
@answer n=20 entity=motion#7.1 v=no
why: keyframes and transitions in example assets; the chunk touches no asset
@answer n=21 entity=motion#7.2 v=no
why: google.html keyframes and transitions; unreached
@answer n=22 entity=motion#7.3 v=no
why: graphite, gosub and inline-flex-transform assets; unreached
@answer n=23 entity=motion#7.4 v=no
why: servo.css spinner and transitions; unreached
@answer n=24 entity=motion#7.5 v=no
why: svg_native and transforms examples; unreached
