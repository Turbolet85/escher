@answer n=1 entity=motion#1.1 v=no
why: todomvc label and button transitions are CSS in an example the chunk does not render or touch
@answer n=2 entity=motion#1.2 v=yes
why: smooth programmatic scrolls run 300 ms and Auto scrolls animate under scroll-behavior, the Smooth case of the scroll_to call scope item 3 names
@answer n=3 entity=motion#1.3 v=no
why: wheel and key-into-view on an open select list; no select list is touched and the key path is not reached through MountedData
@answer n=4 entity=motion#1.4 v=yes
why: programmatic scroll_to supports Smooth and Instant in dioxus-native-dom, the calls scope item 3 names
@answer n=5 entity=motion#1.5 v=yes
why: the stand's Cells arrow handler calls scroll_to(Instant) synchronously, a MountedData call scope item 3 names, re-asserted by the stand_keyboard tests
@answer n=6 entity=motion#1.6 v=yes
why: scroll_into_view writes every nested box at once with the viewport alone taking Smooth, the rule scope item 3 must keep
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration is not reached by MountedData scroll, set_focus or the probe's tests
@answer n=8 entity=motion#1.8 v=yes
why: overlay scrollbar fade timings are read off the VirtualClock scope item 8 names and raised by the scroll calls scope item 3 names, re-asserted by the stand tests
@answer n=9 entity=motion#1.9 v=yes
why: a driver click right after a driver scroll lands on the scrollbar until an advance passes the fade, the scroll and clock scope items 3 and 8 name
@answer n=10 entity=motion#2.1 v=no
why: stylo's animation-time resolution is not changed and no probe test drives a CSS animation
@answer n=11 entity=motion#2.2 v=yes
why: settle's answer for animating members and the 700 ms scroll fade read off the clock, the settle behaviour scope item 8 names
@answer n=12 entity=motion#3.1 v=yes
why: VirtualClock, advance and settle reading animation time only through that clock, the behaviour scope item 8 names for the inverted pins and the ported test
@answer n=13 entity=motion#3.2 v=no
why: the select list has no motion and is not touched by the chunk
@answer n=14 entity=motion#4.1 v=no
why: the driver's LOAD_WAIT clock freeze belongs to the escher-driver session, which the probe's tests do not drive
@answer n=15 entity=motion#4.2 v=no
why: requestAnimationFrame timer approximation in the script runtime, which the chunk does not touch
@answer n=16 entity=motion#4.3 v=no
why: animation and transition event data are not mounted events and the chunk does not change event dispatch
@answer n=17 entity=motion#5.1 v=no
why: BBC fixture media queries are untouched by the chunk
@answer n=18 entity=motion#6.1 v=no
why: observed absence of reduced-motion handling is a search record no chunk rule reads
@answer n=19 entity=motion#6.2 v=no
why: observed absence of transitions in the s07 files is a search record no chunk rule reads
@answer n=20 entity=motion#7.1 v=no
why: keyframes in example HTML fixtures the chunk does not render or change
@answer n=21 entity=motion#7.2 v=no
why: google fixture keyframes and transitions lie outside the chunk's render and test paths
@answer n=22 entity=motion#7.3 v=no
why: graphite, gosub and inline-flex-transform fixture transitions lie outside the chunk's render and test paths
@answer n=23 entity=motion#7.4 v=no
why: servo.css spinner and transitions, a fixture the chunk does not render or change
@answer n=24 entity=motion#7.5 v=no
why: svg_native and transforms example transitions are CSS values the chunk does not change
