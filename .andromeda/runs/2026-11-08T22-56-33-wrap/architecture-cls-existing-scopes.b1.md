@answer n=1 entity=existing-scopes#1.1 v=no
why: accesskit_xplat is not among the session's reached files and its described modules and line citations are unchanged
@answer n=2 entity=existing-scopes#1.2 v=no
why: debug_timer is not among the session's reached files and its described feature gating is unchanged
@answer n=3 entity=existing-scopes#1.3 v=no
why: blitz-traits is not among the session's reached files and its module list is unchanged
@answer n=4 entity=existing-scopes#1.4 v=no
why: blitz-dom is not among the session's reached files and its module list and cfg(test) child modules are unchanged
@answer n=5 entity=existing-scopes#1.5 v=no
why: blitz-dom CSSOM and resolved_style are not reached by the session and their cited line ranges hold
@answer n=6 entity=existing-scopes#1.6 v=no
why: blitz-dom forms, iframes and queries are not reached by the session and their cited line ranges hold
@answer n=7 entity=existing-scopes#1.7 v=no
why: blitz-dom document.rs interaction readings are not reached by the session and their cited line ranges are not changed by it
@answer n=8 entity=existing-scopes#1.8 v=no
why: blitz-dom accessibility and debug modules are not reached by the session
@answer n=9 entity=existing-scopes#1.9 v=no
why: blitz-dom events submodules are not reached by the session
@answer n=10 entity=existing-scopes#1.10 v=no
why: blitz-dom scrolling and selection are not reached by the session
@answer n=11 entity=existing-scopes#1.11 v=no
why: blitz-dom Stylo integration is not reached by the session
@answer n=12 entity=existing-scopes#1.12 v=no
why: blitz-dom tree, traversal and url are not reached by the session
@answer n=13 entity=existing-scopes#1.13 v=no
why: blitz-dom layout is not reached by the session and its cited paths hold
@answer n=14 entity=existing-scopes#1.14 v=no
why: blitz-dom node module is not reached by the session and its select and select_list readings hold
@answer n=15 entity=existing-scopes#1.15 v=no
why: blitz-html is not reached by the session
@answer n=16 entity=existing-scopes#1.16 v=no
why: blitz-net is not reached by the session
@answer n=17 entity=existing-scopes#1.17 v=no
why: blitz-paint is not reached by the session and its module list and citations hold
@answer n=18 entity=existing-scopes#1.18 v=no
why: blitz-vibey-script is not reached by the session and its module list and test files are unchanged
@answer n=19 entity=existing-scopes#1.19 v=no
why: blitz-shell is not reached by the session and its one unit test is unchanged
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate re-exports are not reached by the session
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy is not reached by the session and its module list is unchanged
@answer n=22 entity=existing-scopes#1.22 v=no
why: dioxus-native's module list and its dependency on dioxus-native-dom are unchanged, since the session adds no crate, dependency or feature
@answer n=23 entity=existing-scopes#1.23 v=yes mode=stale
why: the session moves dioxus-native-dom's crate-root re-exports from 35 to 36 with ReachedHandler, its bridge_tests count from 8 to 9, adds the ungated reached module and element_listener_ids absent from this list, and moves the cited lines
@answer n=24 entity=existing-scopes#1.24 v=no
why: blitz-test-harness is not reached by the session and its module list and citations hold
@answer n=25 entity=existing-scopes#1.25 v=no
why: escher-telemetry is not reached by the session, which adds no span, event or log field
