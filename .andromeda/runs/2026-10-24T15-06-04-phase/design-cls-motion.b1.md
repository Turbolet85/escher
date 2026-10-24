# Classified — design · motion.b1

## Answers
motion#1.1 · no · todomvc is outside the reach, and the stand's CSS absence holds untouched since the chunk changes no stand CSS (a re-check every change owes)
motion#1.2 · yes · the Smooth into-view scroll, which applies to the viewport alone, runs on this 300 ms curve that the scroll witnesses re-prove
motion#1.3 · no · neither the script runtime's behaviour parse nor dioxus-native-dom's programmatic scroll is reached, the sweep's into-view path being blitz-dom's scroll_into_view (a re-check every change owes)
motion#1.4 · yes · the named Need: the into-view scroll writes every nested scrolling box at once and applies Smooth to the viewport alone, re-proven by the scroll witnesses
motion#1.5 · no · a touch fling needs touch input the sweep never drives, and its deceleration runs per frame rather than on the clock (scrolling.rs is not reached; a re-check every change owes)
motion#1.6 · yes · the overlay fade reads the document's clock headlessly (motion#3.1), so its 500 ms delay and 200 ms fade bind the re-proven virtual-clock witnesses and the 700 ms settle reading after a driver scroll
motion#2.1 · yes · the click beside a running CSS animation and the CSS-animation settle witness need the pending-to-finished progression by the clock
motion#2.2 · yes · the named Need: settle answers Settled with the animating flag for every animating member, the scroll fade's 700 ms reading included, and never advances time
motion#3.1 · yes · the named Need: the VirtualClock that settle reads moves by exactly advance's ms while click, type and press leave it still (the windowed shell clock is not reached)
motion#4.1 · yes · the named Need: the load wait moves no clock, so the VirtualClock stands still for up to LOAD_WAIT
motion#4.2 · no · requestAnimationFrame is a script-runtime timer, no Need names a script, and blitz-vibey-script is not reached (a re-check every change owes)
motion#4.3 · no · no Need names an animation or transition event, and dioxus-native-dom's event bridge is not reached (a re-check every change owes)
motion#5.1 · no · the BBC fixture's reduced-motion queries are outside the reach, and reduced-motion handling is in the Not-needed list
motion#6.1 · no · the chunk adds no reduced-motion handling (the Not-needed list), so the observed absence holds untouched
motion#6.2 · no · the absence is measured over s07 slice files the chunk does not reach
motion#7.1 · no · the keyframes on examples/assets demo pages are outside the reach and no Need names them, the sweep's fixtures being new minimal pages
motion#7.2 · no · google.html's keyframes and transitions are an example page outside the reach
motion#7.3 · no · graphite.html, gosub.html and inline-flex-transform.html are demo pages outside the reach, and no Need names a transition or hover state
motion#7.4 · no · servo.css's spinAround and its transitions are outside the reach
motion#7.5 · no · svg_native.rs and transforms.rs are example apps outside the reach, and no Need names a transition or hover state
