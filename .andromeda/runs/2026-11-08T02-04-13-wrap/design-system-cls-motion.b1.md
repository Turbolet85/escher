@answer n=1 entity=motion#1.1 v=no
why: the only seven_guis change is signal registration and the report says no styling changed, so the absence of transitions in seven_guis styles holds
@answer n=2 entity=motion#1.2 v=no
why: smooth-scroll timing lives in blitz-dom scrolling.rs, which the session does not reach, and the report keeps the scroll rules unchanged
@answer n=3 entity=motion#1.3 v=no
why: the select-list wheel rule lives in blitz-dom document.rs and scrolling.rs, which the session does not reach
@answer n=4 entity=motion#1.4 v=no
why: scroll-behaviour parsing lives in blitz-vibey-script and dioxus-native-dom events.rs, neither reached by the session
@answer n=5 entity=motion#1.5 v=no
why: the report keeps the into-view scroll rules unchanged, so the Cells arrow-key Instant scroll in cells.rs still holds
@answer n=6 entity=motion#1.6 v=no
why: the report keeps the into-view scroll rules and the driver scroll step unchanged, and BaseDocument::scroll_into_view in blitz-dom is not reached
@answer n=7 entity=motion#1.7 v=no
why: touch-fling deceleration lives in blitz-dom scrolling.rs, which the session does not reach
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar fade timings live in blitz-dom scrollbar.rs and document.rs, which the session does not reach
@answer n=9 entity=motion#1.9 v=no
why: the scrollbar-band hit rule lives in blitz-dom, not reached, and the report keeps the advance and clock rules unchanged
@answer n=10 entity=motion#2.1 v=no
why: CSS animation timing lives in blitz-dom stylo.rs, which the session does not reach, and the report says no styling changed
@answer n=11 entity=motion#2.2 v=no
why: the settle membership rule lives in blitz-test-harness settle.rs, which the session does not reach, and the report keeps settle unchanged
@answer n=12 entity=motion#3.1 v=no
why: the report keeps the VirtualClock and advance rules unchanged and states that a graph read moves no clock, so the timer tick through advance still holds
@answer n=13 entity=motion#3.2 v=no
why: the select list has no motion, a rule in blitz-dom and stand_act_popup, neither reached by the session
@answer n=14 entity=motion#4.1 v=no
why: the load-wait rule lives in escher-driver session.rs, which the session does not reach, and the report keeps load wait unchanged
@answer n=15 entity=motion#4.2 v=no
why: the requestAnimationFrame timer approximation lives in blitz-vibey-script timers.rs, which the session does not reach
@answer n=16 entity=motion#4.3 v=no
why: the unimplemented animation event data lives in dioxus-native-dom events.rs, which the session does not reach, and the new dispatch wraps only handle_event
@answer n=17 entity=motion#5.1 v=no
why: the BBC fixture media queries live in examples/assets, which the session does not reach
@answer n=18 entity=motion#6.1 v=no
why: the report says no styling changed and the motion rules are unchanged, so the absence of reduced-motion handling holds
@answer n=19 entity=motion#6.2 v=no
why: the report says no styling changed, so the absence of transitions or animation holds
@answer n=20 entity=motion#7.1 v=no
why: the keyframes and transitions live in examples/assets HTML, which the session does not reach
@answer n=21 entity=motion#7.2 v=no
why: the google keyframes and transitions live in examples/assets, which the session does not reach
@answer n=22 entity=motion#7.3 v=no
why: the graphite, gosub and inline-flex-transform examples live in examples/assets, which the session does not reach
@answer n=23 entity=motion#7.4 v=no
why: the servo.css animations live in examples/assets, which the session does not reach
@answer n=24 entity=motion#7.5 v=no
why: the svg_native.rs and transforms.rs transitions are examples the session does not reach
