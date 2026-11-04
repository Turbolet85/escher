@answer n=1 entity=motion#1.1 v=no
why: the session changed no todomvc, browser or seven_guis CSS or source, so the transition claims and the absence reads stand
@answer n=2 entity=motion#1.2 v=no
why: scrolling.rs and fragment_navigation.rs are unchanged, so the 300 ms cubic smooth-scroll claim holds
@answer n=3 entity=motion#1.3 v=no
why: document.rs, scrolling.rs and select_popup.rs are unchanged, so the wheel and key behaviour of the select list holds
@answer n=4 entity=motion#1.4 v=no
why: events.rs is unchanged and the session measured the set_focus hazard and scroll reads, not the Smooth and Instant values at events.rs:246-249 and :285-288, which still hold
@answer n=5 entity=motion#1.5 v=no
why: the re-read of the synchronous MountedData scroll_to found it acts at once, so the Cells grid's Instant scroll claim holds
@answer n=6 entity=motion#1.6 v=no
why: the re-read against the MountedData hazard found the into-view claim unmoved, since only a spawned set_focus panics and a synchronous MountedData call acts
@answer n=7 entity=motion#1.7 v=no
why: touch-fling deceleration in scrolling.rs is unchanged by the session
@answer n=8 entity=motion#1.8 v=no
why: the overlay scrollbar fade constants and paint code in scrollbar.rs and render.rs are unchanged
@answer n=9 entity=motion#1.9 v=no
why: the scrollbar band and driver click and scroll behaviour are unchanged, since the session moved no scroll, click or Cells code
@answer n=10 entity=motion#2.1 v=no
why: the CSS animation and transition clock in stylo.rs is unchanged by the session
@answer n=11 entity=motion#2.2 v=no
why: the settle lines are re-read and unmoved under the harness-lines item, so the keep-animating set and its settle answer still hold
@answer n=12 entity=motion#3.1 v=no
why: the VirtualClock and its driver advance, with click, type and press leaving the clock, are re-read and unmoved, and the probe's press and click do not move it
@answer n=13 entity=motion#3.2 v=no
why: the select list's no-motion claim in stand_act_popup is untouched, since no popup or select code changed
@answer n=14 entity=motion#4.1 v=no
why: the driver's LOAD_WAIT in session.rs is unchanged, so the clock-standing claim during a load wait holds
@answer n=15 entity=motion#4.2 v=no
why: the requestAnimationFrame timer approximation in timers.rs is unchanged
@answer n=16 entity=motion#4.3 v=no
why: the unimplemented animation and transition event data at events.rs:71-73 and :123-125 is unchanged, since the session did not alter events.rs
@answer n=17 entity=motion#5.1 v=no
why: the BBC fixture's reduced-motion queries sit in examples/assets/bbc_reduced.html, which the session did not touch
@answer n=18 entity=motion#6.1 v=no
why: the untouched list confirms reduced-motion handling stays absent, and no stylesheet or media query changed
@answer n=19 entity=motion#6.2 v=no
why: no transition or animation file in the s07 slices changed, and the session added no CSS
@answer n=20 entity=motion#7.1 v=no
why: the @keyframes and transitions in examples/assets/animated_layout.html and animation.html are unchanged
@answer n=21 entity=motion#7.2 v=no
why: the google.html keyframes and transitions in examples/assets are unchanged by the session
@answer n=22 entity=motion#7.3 v=no
why: the graphite, gosub and inline-flex-transform assets are unchanged by the session
@answer n=23 entity=motion#7.4 v=no
why: examples/assets/servo.css is unchanged by the session
@answer n=24 entity=motion#7.5 v=no
why: examples/svg_native.rs and examples/transforms.rs are unchanged by the session
