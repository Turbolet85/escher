# Classified — design · motion.b1

## Answers
motion#1.1 · yes · the seven_guis absence of transitions is read over cells.rs, a conflict surface the take resolves; the todomvc part is out of scope
motion#1.2 · yes · the 300 ms smooth-scroll curve and scroll-behavior smooth sit in scrolling.rs, a conflict surface, and the scope names a requested smooth behaviour reaching the viewport alone
motion#1.3 · yes · a wheel over an open select list and a key bringing the current row into view at once are the select model and open list the scope names, with scrolling.rs:669-672 and select_popup.rs
motion#1.4 · yes · the smooth and instant values parsed by the script runtime and passed through dioxus-native-dom are the smooth value the scope names reaching the viewport alone
motion#1.5 · yes · stand_keyboard's Cells arrow keys calling scroll_to Instant is a re-read keyboard check the chunk re-proves
motion#1.6 · yes · nested boxes written at once, Smooth applied to the viewport alone, and a select row's scroll_y write are the into-view behaviour the scope names
motion#1.7 · yes · the touch fling decay sits in scrolling.rs, a conflict surface whose rules bind whole
motion#1.8 · no · the overlay fade constants live in scrollbar.rs, document.rs and blitz-paint, none reached, and the scope names no fade timing
motion#1.9 · yes · the shown-scrollbar band that takes a click depends on hit testing in node.rs, a conflict surface, and on advance passing the fade on the clock the scope names
motion#2.1 · yes · pending-to-finished animation resolution in stylo.rs, a conflict surface the take touches through the Stylo preferences and size hint
motion#2.2 · yes · the settle answer for animating members reads the clock the scope names, and the custom widget redraw is the a26c6ddeb surface
motion#3.1 · yes · the harness clock that settle, the driver's advance and the step read is the clock invariant the scope names
motion#3.2 · yes · the select list's no-motion and read-after-step behaviour is the open list the scope names, with the select_popup.rs row edit
motion#4.1 · yes · the driver's load wait holding VirtualClock still is the load-wait relation the scope names
motion#4.2 · no · the requestAnimationFrame 16 ms approximation lives in blitz-vibey-script timers.rs, not reached, and no clause of the scope names rAF or its timestamp
motion#4.3 · no · animation and transition event data are unimplemented in dioxus-native-dom events.rs, not reached, and the scope adds no animation event use
motion#5.1 · no · the BBC fixture's reduced-motion queries sit in examples/assets, a fixture the scope does not name
motion#6.1 · no · the absence of reduced-motion handling is named under Not needed, and the chunk adds none
motion#6.2 · no · the absent transitions over the s07 slice files are outside the reached files and the scope names no such file
motion#7.1 · no · the keyframes of the examples/assets animation fixtures are not named by the scope
motion#7.2 · no · the google.html keyframes and transitions are an examples/assets fixture the scope does not name
motion#7.3 · no · the graphite, gosub and inline-flex-transform fixtures are not named by the scope
motion#7.4 · no · the servo.css animations and transitions are a fixture the scope does not name
motion#7.5 · no · the svg_native and transforms examples are not named by the scope
