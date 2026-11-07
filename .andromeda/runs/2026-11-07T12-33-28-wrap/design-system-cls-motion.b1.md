@answer n=1 entity=motion#1.1 v=no
why: the session touched no transition, animation or stylesheet, and the todomvc CSS it cites is unchanged
@answer n=2 entity=motion#1.2 v=no
why: the smooth-scroll and scroll-behavior rules live in blitz-dom scrolling.rs, which the session does not reach, and no scroll behaviour moved
@answer n=3 entity=motion#1.3 v=no
why: the wheel and key behaviour of the open select list lives in blitz-dom files the session does not reach
@answer n=4 entity=motion#1.4 v=no
why: the script runtime parsing of scroll behaviour values and the Smooth and Instant support are unchanged, events.rs is not a reached file, and the claim does not assume a synchronous handler call
@answer n=5 entity=motion#1.5 v=no
why: the Cells grid keeps its synchronous scroll_to handles, so its arrow-key handler call still holds, and the session only adds a call path for tasks without removing this one
@answer n=6 entity=motion#1.6 v=no
why: the into-view rule concerns BaseDocument scroll_into_view and the driver scroll rather than MountedData calls, and the session does not reach scrolling.rs
@answer n=7 entity=motion#1.7 v=no
why: touch fling deceleration in blitz-dom scrolling.rs is unchanged and the session does not reach that file
@answer n=8 entity=motion#1.8 v=no
why: overlay scrollbar fade timings in blitz-dom and blitz-paint are unchanged and the session reaches none of those files
@answer n=9 entity=motion#1.9 v=no
why: scrollbar band hit-testing and the driver click behaviour are unchanged, since the session left hit-testing and paint as they were
@answer n=10 entity=motion#2.1 v=no
why: CSS animation and transition timing during style resolution in stylo.rs is unchanged and the session does not reach that file
@answer n=11 entity=motion#2.2 v=no
why: the document animating set and the harness settle are unchanged, and the session reaches no animation or settle file
@answer n=12 entity=motion#3.1 v=no
why: the shell animation clock and the VirtualClock the harness drives are unchanged, as the task states for harness settle and clock
@answer n=13 entity=motion#3.2 v=no
why: the select list has no motion and the session does not reach the select popup path, so the screen read after a step is unaffected
@answer n=14 entity=motion#4.1 v=no
why: the driver load wait and the clock standing still during it are unchanged, as the task states for the driver advance and load wait
@answer n=15 entity=motion#4.2 v=no
why: the requestAnimationFrame timer approximation in the script runtime is unchanged and the session does not reach that file
@answer n=16 entity=motion#4.3 v=no
why: animation and transition event data in dioxus-native-dom events.rs is unchanged, and events.rs is not a file the session reached
@answer n=17 entity=motion#5.1 v=no
why: the BBC fixture reduced-motion media queries are a fixture asset the session did not touch
@answer n=18 entity=motion#6.1 v=no
why: the observed absence of reduced-motion handling is a search over app and asset trees the session did not change, so it still holds
@answer n=19 entity=motion#6.2 v=no
why: the observed absence of transitions or animation over the s07 slice files is unchanged, since the session edited no slice file
@answer n=20 entity=motion#7.1 v=no
why: the keyframe and transition examples are example assets the session did not change
@answer n=21 entity=motion#7.2 v=no
why: the google.html keyframes and transitions are an example asset the session did not change
@answer n=22 entity=motion#7.3 v=no
why: the graphite, gosub and inline-flex-transform example assets are unchanged by the session
@answer n=23 entity=motion#7.4 v=no
why: the servo.css keyframes and transitions are an asset file the session did not change
@answer n=24 entity=motion#7.5 v=no
why: the svg_native and transforms examples are unchanged, and the session touched no stylesheet or example
