# design-system · Motion
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Motion detail.

## Motion (calibrated to expression level `unrecorded`)

<!-- detail: values · expression-motion -->

- Micro-interactions (apps): todomvc transitions label color over 0.4s and destroy button color over 0.2s ease-out (examples/todomvc/src/todomvc.css:214; examples/todomvc/src/todomvc.css:234); observed absent — transitions or animations in browser chrome and seven_guis styles · searched: `transition|animation|@keyframes` over apps/browser/assets/*.css and examples/seven_guis/src/**/*.rs
- Smooth scrolls run 300 ms on a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:112-122; packages/blitz-dom/src/scrolling.rs:432-433); `scroll-behavior: smooth` makes `Auto` scrolls animate and `auto` behavior jumps (packages/blitz-dom/src/scrolling.rs:528-545; tests/blitz-tests/tests/fragment_navigation.rs:264-293; tests/blitz-tests/tests/fragment_navigation.rs:374-383); a wheel event over a scroller cancels an in-progress smooth scroll (tests/blitz-tests/tests/fragment_navigation.rs:356-372)
- Scroll behaviour values `auto`, `instant`, `smooth` are parsed by the script runtime and passed to blitz-dom (packages/blitz-vibey-script/src/dom/element/scrolling.rs:37-52); programmatic scrolling in dioxus-native-dom supports `Smooth` and `Instant` (packages/dioxus-native-dom/src/events.rs:246-249; packages/dioxus-native-dom/src/events.rs:285-288)
- A programmatic into-view scroll never animates a nested scrolling box: `BaseDocument::scroll_into_view` writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked — the document holds one scroll animation at a time — and the requested behaviour, `Smooth` included, applies to the viewport alone; the driver's `scroll` is instant in every box and in the viewport, made inside one settled step (packages/blitz-dom/src/scrolling.rs:655-734; packages/blitz-test-harness/src/input.rs:235-245; tests/blitz-tests/tests/scroll_into_view_nested.rs:1-10; ratified by the founder, 2026-10-07 — as measured at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md)
- Touch flings decelerate per frame until velocity drops below 0.1 (packages/blitz-dom/src/scrolling.rs:867-885)
- Overlay scrollbars show at full opacity on scroll, stay opaque for `FADE_DELAY` = 500 ms after last activity, then fade linearly over `FADE_DURATION` = 200 ms, documented as Chromium's overlay timings (packages/blitz-dom/src/node/scrollbar.rs:12-26; packages/blitz-dom/src/document.rs:1975-1992; packages/blitz-paint/src/render.rs:725-730; packages/blitz-paint/src/render.rs:752-755); finished fades are dropped each resolve (packages/blitz-dom/src/resolve.rs:67-74); hovering where a hidden thumb would be does not summon it, and the thumb changes appearance on hover and while dragged (tests/blitz-tests/tests/scrollbars.rs:1-4; tests/blitz-tests/tests/scrollbar_drag.rs:136-166; tests/blitz-tests/tests/scrollbar_drag.rs:168-222)
- Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag (`hit_with_scrollbar` while `scrollbar_opacity > 0`) and dispatches nothing; on the virtual clock it stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown — so a driver `click` on a Cells cell in that band right after a `scroll` lands on the scrollbar, and an `advance` past the fade comes first (tests/blitz-tests/tests/stand_cells_served.rs:176-192; tests/blitz-tests/tests/stand_cells_served.rs:91-133; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)

<!-- /detail -->

<!-- detail: values · expression-motion -->

- CSS animations and transitions move from pending to running to finished by the current time during style resolution (packages/blitz-dom/src/stylo.rs:112-130)
- Active CSS animations/transitions, canvases, animating sub-documents, custom widgets, scroll animations and scrollbar fades keep the document animating (packages/blitz-dom/src/document.rs:2262-2279); a custom widget returning `true` from `requires_redraw` causes continuous redraw scheduling (packages/blitz-dom/src/node/custom_widget.rs:100-106); a harness settle answers for every member of that set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag, the scrollbar fade's membership read off the document's clock — after a driver `scroll` it reads true until 700 ms of that clock pass (packages/blitz-test-harness/src/settle.rs:151-186; one member, a CSS animation, exercised at tests/blitz-tests/tests/stand_settle.rs:247-275 — as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md)

<!-- /detail -->

<!-- detail: values · expression-motion -->

- The shell's animation clock is seconds since the first animation-time query, and frames keep redrawing while the document is animating (packages/blitz-shell/src/window.rs:281-289; packages/blitz-shell/src/window.rs:438-440); the test harness drives animation from the `VirtualClock` its documents read — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — moved by `tick`, `advance_time` and the driver's `advance` (packages/blitz-test-harness/src/harness.rs:152-174), and settle reads animation time only through that clock, never advancing it; the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was: a driver click beside a running CSS animation returns settled without waiting on it (tests/blitz-tests/tests/stand_act_timer.rs:1-12; tests/blitz-tests/tests/stand_virtual_clock.rs:50-96 — as measured at escher-0.2.0/chunks/2026-10-18-one-virtual-clock/report.md)

<!-- /detail -->

<!-- detail: values · expression-motion -->

- The driver's load wait moves no clock: while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait (packages/escher-driver/src/session.rs:16-53; as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md)
- `requestAnimationFrame` is a timer approximated as 16ms away, passing 16 as the timestamp (packages/blitz-vibey-script/src/runtime/timers.rs:98-123)
- Animation and transition event data are `unimplemented!()` in dioxus-native-dom (packages/dioxus-native-dom/src/events.rs:71-73; packages/dioxus-native-dom/src/events.rs:123-125)

<!-- /detail -->

<!-- detail: values · expression-motion -->

- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)

<!-- /detail -->

<!-- detail: absence · expression-motion -->

- Observed absent — reduced-motion handling · searched: `prefers-reduced-motion` over the 21 s02 files, the 32 s03 slice files and the 86 s04 slice files; `prefers-reduced-motion|reduced_motion` over the 15 s05 files; `prefers-reduced-motion|reduced_motion|prefers_reduced` over the 17 listed s06 files; `reduced.motion|prefers` over the 32 listed s09 files
- Observed absent — transitions or animation · searched: `transition|animation` over the 8 s07 slice files

<!-- /detail -->

<!-- detail: values · expression-motion -->

- `@keyframes` animations (big-small 2s alternate, gradient-animation 20s, spin 4s linear) and transitions on transform and filter with `:hover`/`:active` nesting (examples/assets/animated_layout.html:21; examples/assets/animated_layout.html:28-43; examples/assets/animation.html:32-38; examples/assets/animation.html:17-21; examples/assets/animation.html:46-51; examples/assets/filters.html:16-17)
- google keyframes gb__a, g-bubble-show, g-bubble-hide, g-snackbar-show/hide and qli spinner, with transitions such as box-shadow 250ms and transform/opacity/visibility .3s ease-in-out (examples/assets/google.html:49-56; examples/assets/google.html:1589; examples/assets/google.html:1597; examples/assets/google.html:1700; examples/assets/google.html:1709; examples/assets/google.html:2757; examples/assets/google.html:597; examples/assets/google.html:1457)
- graphite carousel transform .5s and details marker rotating 90deg (examples/assets/graphite.html:498-500; examples/assets/graphite.html:1243-1245); gosub link border-bottom 0.3s ease-in-out (examples/assets/gosub.html:49); inline-flex-transform scale/translate/rotate/transform over 0.15s, scale 1.2 on hover (examples/assets/inline-flex-transform.html:27-28; examples/assets/inline-flex-transform.html:37-38)
- servo.css `@keyframes spinAround` on `.button.is-loading::after` at 500ms infinite linear and 86ms ease-out transitions on background-color, opacity, transform (examples/assets/servo.css:82-83; examples/assets/servo.css:328-335; examples/assets/servo.css:6096-6098)
- svg_native transitions `fill 0.15s` on hover (examples/svg_native.rs:14-15); the transforms example `.button` transitions `filter, scale` over 0.15s and on hover applies `brightness(90%)` and `scale: 1.2` (examples/transforms.rs:302-303; examples/transforms.rs:312-315)

<!-- /detail -->
