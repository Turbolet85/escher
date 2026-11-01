# Classified — design · motion.b1

## Answers
motion#1.1 · yes · the seven_guis styles' recorded absence of transitions and animations, which the named seven_guis cells fixture must keep accurate; the todomvc transition values are not named
motion#1.2 · yes · smooth scrolls of 300 ms on the cubic curve and the Auto-to-smooth and wheel-cancel rules, all in scrolling.rs, a conflict surface
motion#1.3 · yes · the wheel-over-open-list and key-brings-row-into-view rules, which the select list's rows and scroll_into_view in scrolling.rs keep
motion#1.4 · yes · the Smooth and Instant values carry the requested behaviour that the into-view scroll and its viewport-only smooth scroll name
motion#1.5 · yes · the Cells grid's arrow keys scroll Instant and synchronously, which stand_keyboard re-reads and cells.rs resolves
motion#1.6 · yes · scroll_into_view writes every holding box at once with Smooth applied to the viewport alone, and the select row's list-only scroll, both named in Needs
motion#1.7 · yes · fling deceleration lives in scrolling.rs, a conflict surface
motion#1.8 · yes · the 500 ms delay and 200 ms fade sum to the 700 ms that settle reads off the VirtualClock, so the clock invariant reads these values
motion#1.9 · yes · a shown scrollbar band takes pointerdown as a thumb drag until the driver's advance passes the fade, a hit-testing and clock rule the scope names
motion#2.1 · yes · CSS animation and transition state by current time lives in stylo.rs, a conflict surface
motion#2.2 · yes · settle's animating answer and the scrollbar fade's 700 ms membership read off the document clock, the clock invariant the scope names
motion#3.1 · yes · advance, tick and settle's relation to the VirtualClock is the clock invariant the scope names
motion#3.2 · yes · the select list has no motion, so the select rows' tests read the state after a step, which the select list's re-read rests on
motion#4.1 · yes · the driver's load wait holds the VirtualClock still, the load-wait relation the scope names
motion#4.2 · no · requestAnimationFrame's 16 ms timer lives in blitz-vibey-script timers.rs, which is not reached, and the scope names no script timer
motion#4.3 · no · animation and transition event data in dioxus-native-dom events.rs, which is not reached and not named by the scope
motion#5.1 · no · the BBC fixture's reduced-motion queries, an examples asset the scope does not name
motion#6.1 · no · the recorded absence of reduced-motion handling, which the chunk adds none of and the task lists as not needed
motion#6.2 · no · the recorded absence of transitions over the s07 slice files, which the chunk does not reach
motion#7.1 · no · keyframes in examples assets other than seven_guis, which the scope names no change to
motion#7.2 · no · google.html keyframes and transitions, an example asset the scope names no change to
motion#7.3 · no · graphite, gosub and inline-flex-transform example assets, none named by the scope
motion#7.4 · no · servo.css animations, an example asset the scope names no change to
motion#7.5 · no · svg_native and transforms example transitions, examples the scope names no change to
