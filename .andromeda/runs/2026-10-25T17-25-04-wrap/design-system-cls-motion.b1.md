# Classified — design-system · motion.b1

## Answers
motion#1.1 · yes · stale · its searched glob examples/seven_guis/src/**/*.rs covers reached flight_booker.rs, which the new select changed, so the observed-absent reading can have moved
motion#1.2 · no · its smooth-scroll rules sit in packages/blitz-dom/src/scrolling.rs and tests/blitz-tests/tests/fragment_navigation.rs, neither reached, and no engine crate is reached
motion#1.3 · no · scroll-behaviour parsing sits in packages/blitz-vibey-script and packages/dioxus-native-dom, neither reached, and no engine crate is reached
motion#1.4 · no · the into-view scroll stays instant, and its files (scrolling.rs, packages/blitz-test-harness/src/input.rs, scroll_into_view_nested.rs) are not reached
motion#1.5 · no · fling deceleration sits in packages/blitz-dom/src/scrolling.rs, not reached, and no engine crate is reached
motion#1.6 · yes · add-to · F7, right after a scroll a main-button pointerdown in the band of a shown overlay scrollbar takes a thumb drag and dispatches nothing, and a pointer left on the thumb keeps it shown
motion#2.1 · no · CSS animation and transition sampling sits in packages/blitz-dom/src/stylo.rs, not reached, and the task brings no animation
motion#2.2 · yes · add-to · F7, a pointer left on the thumb keeps the shown overlay scrollbar shown on the virtual clock, so the scrollbar-fade membership read off that clock must carry the held-pointer case beside its 700 ms reading
motion#3.1 · no · the clock's readers sit in packages/blitz-test-harness/src/harness.rs, not reached, its stand tests stand_act_timer.rs and stand_virtual_clock.rs are not reached, the task names no change to the Timer's whole ticks, and click, type and press still leave the clock where it was
motion#4.1 · yes · add-to · F8, the load page's in-process stand-in answers after 200 ms from its own thread while the acting click waits the load out, so a served page now exercises the load wait
motion#4.2 · no · requestAnimationFrame's timer sits in packages/blitz-vibey-script/src/runtime.rs, not reached, and the task changes no script runtime
motion#4.3 · no · animation and transition event data sit in packages/dioxus-native-dom/src/events.rs, not reached
motion#5.1 · no · the reduced-motion queries sit in examples/assets/bbc_reduced.html, which the task does not reach
motion#6.1 · no · its searched slices are named by label only in this batch, and the task brings no reduced-motion query
motion#6.2 · no · its searched slices are named by label only in this batch, and the task brings no transition or animation
motion#7.1 · no · the keyframe fixtures sit in examples/assets/animated_layout.html, animation.html and filters.html, none reached
motion#7.2 · no · the google keyframes and transitions sit in examples/assets/google.html, not reached
motion#7.3 · no · the graphite, gosub and inline-flex-transform transitions sit in examples/assets/graphite.html, gosub.html and inline-flex-transform.html, not reached
motion#7.4 · no · the servo.css spinner and transitions sit in examples/assets/servo.css, not reached
motion#7.5 · no · the svg_native and transforms transitions sit in examples/svg_native.rs and examples/transforms.rs, neither reached
