# Classified — design · motion.b1

## Answers
motion#1.1 · no · authored todomvc transitions in an example app, which the task's Not needed list drops since no debt adds or changes an animation
motion#1.2 · yes · smooth-scroll timing and scroll-behaviour code in scrolling.rs, which is reached whole
motion#1.3 · no · the scroll-behaviour value parse lives in the script runtime and the dioxus adapter, neither reached nor named by the scope
motion#1.4 · yes · scroll_into_view writes every scrolling box that holds a target (scrolling.rs:655-734), the target-in-a-box case the transformed-box fix covers
motion#1.5 · yes · touch-fling deceleration in scrolling.rs, which is reached whole
motion#1.6 · yes · overlay-scrollbar hover summon and thumb paint on a scrolled box, the paint and hit-test behaviour the scope names
motion#1.7 · yes · a pointerdown in an overlay scrollbar band over a scrolled box goes to the scrollbar (hit_with_scrollbar), the hit-test behaviour the scope names
motion#2.1 · no · animation state during style resolution lives in stylo.rs, neither reached nor named, and no check the chunk adds or re-asserts reads an animation
motion#2.2 · no · standing S3 · a new test that waits or animates takes time from the harness's VirtualClock, settle never advances it, and only advance moves it
motion#3.1 · yes · the shell's frame redraw while the document animates (window.rs:438-440), the windowed redraw route the new check adds
motion#4.1 · no · standing S3 · a new test that waits takes time from the harness's VirtualClock, and the load wait leaves that clock unmoved
motion#4.2 · no · the requestAnimationFrame timer approximation lives in the script runtime, which the chunk does not reach or name
motion#4.3 · no · animation and transition event data are stubbed in dioxus-native-dom, neither reached nor named, and no animation event is in scope
motion#5.1 · no · an authored reduced-motion media query in an example fixture, which the task's Not needed list drops
motion#6.1 · no · the absence of reduced-motion handling, which the task's Not needed list drops since no debt adds or changes an animation
motion#6.2 · no · the absence of transitions or animation in the s07 slice files, which the task's Not needed list drops since no debt adds or changes an animation
motion#7.1 · no · authored keyframes and transitions in example assets, which the task's Not needed list drops
motion#7.2 · no · authored keyframes and transitions in the google example asset, which the task's Not needed list drops
motion#7.3 · no · authored transitions and marker rotation in example assets, which the task's Not needed list drops
motion#7.4 · no · authored spin keyframes and transitions in the servo.css example asset, which the task's Not needed list drops
motion#7.5 · no · authored transitions in the svg_native and transforms examples, which the task's Not needed list drops
