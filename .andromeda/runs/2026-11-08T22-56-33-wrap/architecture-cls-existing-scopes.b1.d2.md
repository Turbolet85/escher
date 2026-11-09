@answer n=1 entity=existing-scopes#1.1 v=no
why: accesskit_xplat lies outside every file the session reached
@answer n=2 entity=existing-scopes#1.2 v=no
why: debug_timer lies outside every file the session reached
@answer n=3 entity=existing-scopes#1.3 v=no
why: blitz-traits is not among the reached files and its module list is unchanged
@answer n=4 entity=existing-scopes#1.4 v=no
why: no blitz-dom lib or document file is among the reached files, so its module list and citations hold
@answer n=5 entity=existing-scopes#1.5 v=no
why: cssom.rs and resolved_style.rs are not among the reached files
@answer n=6 entity=existing-scopes#1.6 v=no
why: no file of the blitz-dom forms, iframe or query modules is among the reached files
@answer n=7 entity=existing-scopes#1.7 v=no
why: blitz-dom document.rs is not among the reached files, the session touches only dioxus-native-dom
@answer n=8 entity=existing-scopes#1.8 v=no
why: no blitz-dom accessibility or debug file is among the reached files
@answer n=9 entity=existing-scopes#1.9 v=no
why: no blitz-dom events file is among the reached files
@answer n=10 entity=existing-scopes#1.10 v=no
why: no blitz-dom scrolling or selection file is among the reached files
@answer n=11 entity=existing-scopes#1.11 v=no
why: no blitz-dom Stylo integration file is among the reached files
@answer n=12 entity=existing-scopes#1.12 v=no
why: no blitz-dom tree, traversal or url file is among the reached files
@answer n=13 entity=existing-scopes#1.13 v=no
why: no blitz-dom layout file is among the reached files
@answer n=14 entity=existing-scopes#1.14 v=no
why: no blitz-dom node file is among the reached files
@answer n=15 entity=existing-scopes#1.15 v=no
why: blitz-html is not among the reached files and its parsing module is unchanged
@answer n=16 entity=existing-scopes#1.16 v=no
why: blitz-net is not among the reached files and its fetching module is unchanged
@answer n=17 entity=existing-scopes#1.17 v=no
why: no blitz-paint file is among the reached files
@answer n=18 entity=existing-scopes#1.18 v=no
why: blitz-vibey-script is not among the reached files and its modules and test files are unchanged
@answer n=19 entity=existing-scopes#1.19 v=no
why: blitz-shell is not among the reached files and its modules and unit test are unchanged
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz re-export crate is not among the reached files and its re-exports are unchanged
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy is not among the reached files and its modules are unchanged
@answer n=22 entity=existing-scopes#1.22 v=no
why: dioxus-native is not among the reached files, its modules are unchanged and its dependency on dioxus-native-dom is unchanged
@answer n=23 entity=existing-scopes#1.23 v=yes mode=add-to
why: the session adds the ungated reached module with ReachedHandler to the crate root, moves its re-exports from 35 to 36, adds element_listener_ids beside element_id, and moves its bridge tests from 8 to 9
@answer n=24 entity=existing-scopes#1.24 v=no
why: blitz-test-harness is not among the reached files and its modules are unchanged
@answer n=25 entity=existing-scopes#1.25 v=no
why: escher-telemetry is not among the reached files and the session adds no span, event or log field
