# Classified — design · motion.b1

## Answers
motion#1.1 · no · todomvc's authored transitions sit in an app the chunk does not reach, Not needed as no debt adds or changes an animation
motion#1.2 · yes · scrolling.rs is reached whole, so its smooth-scroll path (300 ms curve, scroll-behavior smooth on Auto scrolls, auto jumps) binds the transformed-box fix
motion#1.3 · no · the script runtime's scroll-behaviour parsing and dioxus-native-dom's Smooth and Instant support sit in files the chunk does not reach, and no fork passes a scroll behaviour value
motion#1.4 · yes · scroll_into_view writes every holding box at once in scrolling.rs, reached whole, and the driver's scroll is instant in every box, which the transformed-box proof's nested scroll depends on
motion#1.5 · yes · scrolling.rs is reached whole, and the touch fling's per-frame deceleration sits in it
motion#1.6 · yes · the overlay scrollbar's paint fade and hover-summon rule are the paint and hit of a scrolled box, which the transformed-box fix must agree with
motion#1.7 · yes · a click in a shown overlay scrollbar's band right after a scroll lands on the scrollbar until an advance passes the fade, which the transformed-box hit proof must honour
motion#2.1 · no · CSS animation timing in stylo.rs is outside the reached files, and the chunk adds no animation
motion#2.2 · no · standing S3 · a new test that waits or animates takes time from the harness's VirtualClock, and settle never advances it
motion#3.1 · no · standing S3 · a new test that waits or animates takes time from the harness's VirtualClock, which settle never advances and only advance moves
motion#4.1 · no · standing S3 · a new test that waits or animates takes time from the harness's VirtualClock, which the load wait leaves standing
motion#4.2 · no · requestAnimationFrame's 16 ms timer is script-runtime code outside the reached files, and no chunk check or fork calls it
motion#4.3 · no · animation and transition event data are unimplemented in dioxus-native-dom, outside the reached files, and the chunk adds no animation event
motion#5.1 · no · the BBC fixture's reduced-motion queries sit in an example that no served page of this chunk is, Not needed as no debt adds or changes an animation
motion#6.1 · no · the absence of reduced-motion handling, Not needed as no debt adds or changes an animation
motion#6.2 · no · the absent transitions in the s07 slice files sit outside the chunk's reach, Not needed as no debt adds or changes an animation
motion#7.1 · no · authored keyframes and hover transitions in the example assets, Not needed as no debt adds or changes an animation
motion#7.2 · no · google's authored keyframes and transitions in the example assets, Not needed as no debt adds or changes an animation
motion#7.3 · no · graphite, gosub and inline-flex-transform's authored transitions in the example assets, Not needed as no debt adds or changes an animation
motion#7.4 · no · servo.css's loading spinner and authored transitions in the example assets, Not needed as no debt adds or changes an animation
motion#7.5 · no · svg_native's fill transition and the transforms example's button transitions, Not needed as no debt adds or changes an animation
