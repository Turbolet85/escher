# Classified — design · motion.b1

## Answers
motion#1.1 · yes · the absence of transitions in seven_guis styles is searched over cells.rs, a predicted conflict surface the take resolves
motion#1.2 · yes · smooth-scroll timing, the Auto/smooth behaviour and the wheel cancel live in scrolling.rs, a conflict surface, and the requested smooth behaviour reaches the viewport alone
motion#1.3 · yes · the wheel-over-open-list and key-into-view behaviour is scrolling.rs:669-672, the select list the chunk re-reads row by row and edits in select_popup.rs
motion#1.4 · yes · the Smooth and Instant scroll values dioxus-native-dom supports are the values the stand's Instant call and the viewport's Smooth scroll pass
motion#1.5 · yes · the stand_keyboard check the chunk re-reads asserts the Cells grid's synchronous Instant scroll_to
motion#1.6 · yes · scroll_into_view's nested-boxes-at-once and viewport-alone Smooth rule is a Needs item in scrolling.rs, a conflict surface
motion#1.7 · yes · the fling deceleration lives in scrolling.rs, a predicted conflict surface, so any line of it can move under a take
motion#1.8 · yes · the 500 ms and 200 ms fade constants sum to the 700 ms span settle reads after a driver scroll, which the clock invariant re-asserts
motion#1.9 · yes · the shown-scrollbar pointerdown hit runs through the node.rs hit path the take resolves and depends on the driver advance the clock invariant keeps
motion#2.1 · yes · the animation and transition phase during style resolution is stylo.rs:112-130, a predicted conflict surface
motion#2.2 · yes · settle's animating answer and its 700 ms scrollbar-fade membership are the settle behaviour the clock invariant re-asserts
motion#3.1 · yes · the VirtualClock relation that settle, advance and the clock's moves keep is the chunk's own clock-invariant work
motion#3.2 · yes · the select list has no motion, so the open-list key step reads the new state with no advance, which the select model work relies on
motion#4.1 · yes · the driver's load wait holding the VirtualClock still is named in the chunk's clock invariant
motion#4.2 · no · the requestAnimationFrame 16 ms approximation lives in blitz-vibey-script timers.rs, which the chunk does not reach, and no need names it
motion#4.3 · no · animation and transition event data in dioxus-native-dom events.rs is not reached and the chunk emits no such events
motion#5.1 · no · the BBC fixture's reduced-motion queries are fixture content of an example the scope does not change
motion#6.1 · no · the chunk adds no reduced-motion handling and the scope asks for no motion preference
motion#6.2 · no · the absence is over s07 slice fixtures the chunk does not touch
motion#7.1 · no · the keyframes are in example fixtures other than examples/seven_guis, which the scope names no change to
motion#7.2 · no · the google.html keyframes and transitions are fixture values of an example the scope does not change
motion#7.3 · no · the graphite, gosub and inline-flex-transform fixture values are of examples the scope does not change
motion#7.4 · no · the servo.css animation values are a fixture of an example the scope does not change
motion#7.5 · no · the svg_native and transforms example transitions are example values the scope does not change
