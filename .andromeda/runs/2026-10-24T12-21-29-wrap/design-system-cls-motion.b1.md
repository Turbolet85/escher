# Classified — design-system · motion.b1

## Answers
motion#1.1 · no · the session changes no CSS, and the todomvc stylesheet it cites is untouched
motion#1.2 · no · scrolling.rs and the fragment-navigation checks it cites are untouched, and smooth scrolling is unchanged
motion#1.3 · no · the script runtime's scroll-behaviour parse and dioxus-native-dom's programmatic scroll are not in the session's touched files
motion#1.4 · no · scroll_into_view, its harness input and its nested-scroll check are untouched, and the into-view scroll is unchanged
motion#1.5 · no · the fling code in scrolling.rs is not in the session's touched files
motion#1.6 · yes · stale · the session adds two lines to resolve.rs at 29 and 32, so the finished-fade drop cited at resolve.rs:67-74 now reads 69-76
motion#2.1 · yes · stale · the session adds seven lines to stylo.rs at 29-35, so the animation-state lines cited at stylo.rs:105-123 now read 112-130
motion#2.2 · no · document.rs, custom_widget.rs and the settle code it cites are untouched, and the session moves no animating-set rule
motion#3.1 · no · the shell's clock in window.rs and the harness's VirtualClock drive are untouched, and advance and settle are unchanged
motion#4.1 · no · escher-driver is byte-unchanged, so the load wait and its clock rule stand
motion#4.2 · no · the requestAnimationFrame approximation in blitz-vibey-script's runtime.rs is untouched
motion#4.3 · no · dioxus-native-dom's events.rs, where the unimplemented animation event data sits, is not in the session's touched files
motion#5.1 · no · the BBC fixture's reduced-motion queries sit in examples/assets, which the session does not touch
motion#6.1 · no · the session adds no reduced-motion read or handling and changes no CSS, so the absence stands
motion#6.2 · no · the session adds no transition or animation and changes no CSS, so the absence stands
motion#7.1 · no · the animation fixtures under examples/assets are untouched
motion#7.2 · no · the google fixture under examples/assets is untouched
motion#7.3 · no · the graphite, gosub and inline-flex-transform fixtures under examples/assets are untouched
motion#7.4 · no · servo.css under examples/assets is untouched
motion#7.5 · no · svg_native.rs and transforms.rs, the files cited, are untouched
