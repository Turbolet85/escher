@answer n=1 entity=motion#1.1 v=no
why: todomvc label and button transitions and their absence in browser chrome; the chunk renders and animates no app's CSS
@answer n=2 entity=motion#1.2 v=no
why: smooth scroll timing and scroll-behavior; the chunk adds no scrolling and the scope names no scroll step
@answer n=3 entity=motion#1.3 v=no
why: wheel over the open select list; the scope names no select list and no wheel step
@answer n=4 entity=motion#1.4 v=no
why: scroll behaviour values parsed by the script runtime; the chunk passes no scroll behaviour value
@answer n=5 entity=motion#1.5 v=no
why: the Cells grid arrow-key handler scrolls instantly in its own code; the count reads only whether that handler ran and the chunk does not act on the scroll
@answer n=6 entity=motion#1.6 v=no
why: scroll_into_view over nested boxes; the chunk adds no into-view scroll and the scope names no scroll verb
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration; no touch input is replayed or counted
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar fade timings; neither the coverage read nor the owed test reads scrollbar state
@answer n=9 entity=motion#1.9 v=no
why: a pointerdown in a shown scrollbar band is a thumb drag that dispatches nothing; a component behaviour the scope does not name, and the count reads the dispatch outcome without the chunk acting on it
@answer n=10 entity=motion#2.1 v=no
why: CSS animation state advancing during style resolution; the read-only coverage read resolves no style it must act on and counts no animation state
@answer n=11 entity=motion#2.2 v=no
why: settle's animating flag and its 700 ms scroll window; neither the coverage answer nor the owed test's answer reads the animating flag, and the scope names no scroll or settle
@answer n=12 entity=motion#3.1 v=yes
why: the driver's advance moves the VirtualClock and the app's whole Timer ticks while click, type and press leave it; the owed replay runs advance, click, type and press steps through Session::run, and timer-reached handlers reach the count's record
@answer n=13 entity=motion#3.2 v=no
why: the select list has no motion and needs no advance; no select list is in the chunk's scope
@answer n=14 entity=motion#4.1 v=yes
why: Session::run's load wait stands the VirtualClock still for up to LOAD_WAIT while an acting step waits on a load; the owed replay runs each instance-verb step through Session::run on a fresh session, which starts with a load in flight
@answer n=15 entity=motion#4.2 v=no
why: requestAnimationFrame is a 16 ms timer in the script runtime; no script callback is counted or replayed by the scope
@answer n=16 entity=motion#4.3 v=yes
why: animation and transition event data are unimplemented!() in dioxus-native-dom; the record of dispatched (element, event) handlers must take those event names on every app without building their data
@answer n=17 entity=motion#5.1 v=no
why: the BBC fixture's reduced-motion queries; the chunk loads no such fixture
@answer n=18 entity=motion#6.1 v=no
why: observed absence of reduced-motion handling in the listed apps; the chunk adds no motion and reads no such check
@answer n=19 entity=motion#6.2 v=no
why: observed absence of transitions or animation in the s07 files; the chunk reads none of those files
@answer n=20 entity=motion#7.1 v=no
why: example keyframes and transitions in examples/assets; the coverage command and the owed test replay no example asset
@answer n=21 entity=motion#7.2 v=no
why: google.html keyframes and transitions; the coverage command and the owed test replay no google asset
@answer n=22 entity=motion#7.3 v=no
why: graphite and gosub transitions and inline-flex transforms; the chunk replays no such fixture
@answer n=23 entity=motion#7.4 v=no
why: servo.css spinner and transitions; the chunk replays no servo fixture
@answer n=24 entity=motion#7.5 v=no
why: svg_native and transforms example transitions; the chunk builds and replays no such example
