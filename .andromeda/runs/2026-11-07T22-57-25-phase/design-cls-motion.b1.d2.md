@answer n=1 entity=motion#1.1 v=no
why: todomvc and seven_guis transition observation; the chunk adds no transition or style and the scope names none
@answer n=2 entity=motion#1.2 v=no
why: smooth scroll timing in blitz-dom; the chunk makes no scroll call
@answer n=3 entity=motion#1.3 v=no
why: wheel behaviour over the open select list; the chunk touches no select list input
@answer n=4 entity=motion#1.4 v=no
why: scroll behaviour value parsing and the Smooth and Instant support; the chunk passes no new scroll value and the Cells call it carries is witnessed by motion#1.5
@answer n=5 entity=motion#1.5 v=yes
why: the stand's Cells arrow-key handler calls scroll_to(ScrollBehavior::Instant) synchronously, and the stand.rs signal move reaches that handler, so the new code must keep the synchronous instant call
@answer n=6 entity=motion#1.6 v=no
why: nested into-view scroll and the driver's scroll; the chunk runs no scroll
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration; the chunk handles no touch input
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar fades are not graph nodes; the graph read keys on accesskit nodes and bounds
@answer n=9 entity=motion#1.9 v=no
why: the outcome of a driver click in a scrollbar band after a scroll; the graph read clicks nothing
@answer n=10 entity=motion#2.1 v=no
why: CSS animation state during style resolution; the graph read moves no clock and drives no animation
@answer n=11 entity=motion#2.2 v=no
why: settle's animating flag and its scrollbar fade membership; settle is used unchanged and the scope names no settle read
@answer n=12 entity=motion#3.1 v=no
why: which verbs move the virtual clock; the graph read moves no clock, and the scope names no advance or clock
@answer n=13 entity=motion#3.2 v=no
why: the select list has no motion; the scope names no select list read and paints none
@answer n=14 entity=motion#4.1 v=no
why: the load wait holds the clock during an acting step; the graph read is not an acting step and the scope names no load wait
@answer n=15 entity=motion#4.2 v=no
why: requestAnimationFrame as a 16ms timer in the script runtime; the chunk touches no script timers
@answer n=16 entity=motion#4.3 v=no
why: animation and transition event data are unimplemented in events.rs; the graph reads handler kinds and dispatches none, and events.rs is not reached
@answer n=17 entity=motion#5.1 v=no
why: reduced-motion queries in the BBC fixture; the fixture is outside the chunk's reach
@answer n=18 entity=motion#6.1 v=no
why: observed absence of reduced-motion handling in the listed slices; the chunk adds no motion handling and the scope names none
@answer n=19 entity=motion#6.2 v=no
why: observed absence of transitions and animation in the s07 slices; the chunk touches no such CSS
@answer n=20 entity=motion#7.1 v=no
why: keyframes and transitions in the examples/assets fixtures; no fixture is reached
@answer n=21 entity=motion#7.2 v=no
why: google fixture keyframes and transitions; no fixture is reached
@answer n=22 entity=motion#7.3 v=no
why: graphite and gosub fixture animations; no fixture is reached
@answer n=23 entity=motion#7.4 v=no
why: servo.css spinner and transitions; no fixture is reached
@answer n=24 entity=motion#7.5 v=no
why: svg_native and transforms example transitions; neither example is reached
