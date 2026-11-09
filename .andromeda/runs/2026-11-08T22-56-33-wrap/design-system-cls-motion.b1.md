@answer n=1 entity=motion#1.1 v=yes mode=stale
why: examples/seven_guis/src/session_host.rs, a reached file, lies inside this entity's searched seven_guis src scope, and no report shows the transition and animation absence still holds on it
@answer n=2 entity=motion#1.2 v=no
why: it cites blitz-dom scrolling.rs and fragment_navigation.rs, none of which the session reaches
@answer n=3 entity=motion#1.3 v=no
why: it cites blitz-dom document.rs, scrolling.rs and select_popup.rs, none of which the session reaches
@answer n=4 entity=motion#1.4 v=no
why: it cites blitz-vibey-script and dioxus-native-dom events.rs, which the session does not reach
@answer n=5 entity=motion#1.5 v=no
why: it cites stand_keyboard.rs, which the session does not reach
@answer n=6 entity=motion#1.6 v=no
why: it cites blitz-dom scrolling.rs, blitz-test-harness input.rs and scroll_into_view_nested.rs, none of which the session reaches
@answer n=7 entity=motion#1.7 v=no
why: it cites only blitz-dom scrolling.rs fling code, which the session does not reach
@answer n=8 entity=motion#1.8 v=no
why: it cites blitz-dom scrollbar, document, resolve and render files, none of which the session reaches
@answer n=9 entity=motion#1.9 v=no
why: it cites stand_cells_served.rs, which the session does not reach, and no scroll or click path it describes moved
@answer n=10 entity=motion#2.1 v=no
why: it cites blitz-dom stylo.rs, which the session does not reach
@answer n=11 entity=motion#2.2 v=no
why: it cites blitz-dom document.rs, custom_widget.rs, blitz-test-harness settle.rs and stand_settle.rs, none of which the session reaches, and the settle rule is untouched
@answer n=12 entity=motion#3.1 v=no
why: its cited harness, blitz-shell and stand clock files are none reached, and the coverage replay advances no clock beyond each replayed step's own behaviour, as the entity states
@answer n=13 entity=motion#3.2 v=no
why: it cites stand_act_popup.rs, which the session does not reach, and the select list stays motionless
@answer n=14 entity=motion#4.1 v=no
why: it cites escher-driver session.rs for the load wait, which the session does not reach, and the wait moves no clock
@answer n=15 entity=motion#4.2 v=no
why: it cites blitz-vibey-script timers.rs, which the session does not reach
@answer n=16 entity=motion#4.3 v=no
why: it cites dioxus-native-dom events.rs, which the session does not reach
@answer n=17 entity=motion#5.1 v=no
why: it cites examples/assets/bbc_reduced.html, which the session does not reach
@answer n=18 entity=motion#6.1 v=yes mode=stale
why: reached sources (session_host.rs and the escher-driver and dioxus-native-dom files) may sit inside its searched slice sweeps, and no report shows the reduced-motion absence still holds on them
@answer n=19 entity=motion#6.2 v=yes mode=stale
why: reached sources may sit inside its searched s07 slice files, and no report shows the transition and animation absence still holds on them
@answer n=20 entity=motion#7.1 v=no
why: it cites example asset HTML files only, none of which the session reaches
@answer n=21 entity=motion#7.2 v=no
why: it cites examples/assets/google.html only, which the session does not reach
@answer n=22 entity=motion#7.3 v=no
why: it cites graphite.html, gosub.html and inline-flex-transform.html, none of which the session reaches
@answer n=23 entity=motion#7.4 v=no
why: it cites examples/assets/servo.css only, which the session does not reach
@answer n=24 entity=motion#7.5 v=no
why: it cites examples/svg_native.rs and examples/transforms.rs, neither of which the session reaches
