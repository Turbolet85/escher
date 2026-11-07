@answer n=1 entity=existing-scopes#1.1 v=no
why: the session's changes stay in dioxus-native-dom's bridge, buffer and probe, not the accesskit adapter
@answer n=2 entity=existing-scopes#1.2 v=no
why: no debug_timer file or feature is in the session's reached files
@answer n=3 entity=existing-scopes#1.3 v=no
why: no blitz-traits module or re-export is in the session's reached files
@answer n=4 entity=existing-scopes#1.4 v=no
why: the bridge and buffer live in dioxus-native-dom, and no blitz-dom module list or test module is reached
@answer n=5 entity=existing-scopes#1.5 v=no
why: no blitz-dom CSSOM or resolved-style file is in the session's reached files
@answer n=6 entity=existing-scopes#1.6 v=no
why: no blitz-dom form, iframe or query-selector file is in the session's reached files
@answer n=7 entity=existing-scopes#1.7 v=no
why: blitz-dom's document.rs, which holds the hit, focus and geometry code, is not in the session's reached files
@answer n=8 entity=existing-scopes#1.8 v=no
why: no blitz-dom accessibility or debug file is in the session's reached files
@answer n=9 entity=existing-scopes#1.9 v=no
why: no blitz-dom events submodule is in the session's reached files
@answer n=10 entity=existing-scopes#1.10 v=no
why: no blitz-dom scrolling or selection file is in the session's reached files
@answer n=11 entity=existing-scopes#1.11 v=no
why: no blitz-dom Stylo integration file is in the session's reached files
@answer n=12 entity=existing-scopes#1.12 v=no
why: no blitz-dom tree, traversal or url file is in the session's reached files
@answer n=13 entity=existing-scopes#1.13 v=no
why: no blitz-dom layout file is in the session's reached files
@answer n=14 entity=existing-scopes#1.14 v=no
why: no blitz-dom node file is in the session's reached files, so the select and range types are unchanged
@answer n=15 entity=existing-scopes#1.15 v=no
why: blitz-html is not in the session's reached files
@answer n=16 entity=existing-scopes#1.16 v=no
why: blitz-net is not in the session's reached files
@answer n=17 entity=existing-scopes#1.17 v=no
why: blitz-paint is not in the session's reached files, so its select_list painter is unchanged
@answer n=18 entity=existing-scopes#1.18 v=no
why: blitz-vibey-script is not in the session's reached files
@answer n=19 entity=existing-scopes#1.19 v=no
why: blitz-shell is not in the session's reached files
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate's re-exports are unchanged and no file under packages/blitz is reached
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy is not in the session's reached files
@answer n=22 entity=existing-scopes#1.22 v=no
why: dioxus-native's module list and DioxusNativeApplication::can_create_surfaces are unchanged, and no file under packages/dioxus-native is reached
@answer n=23 entity=existing-scopes#1.23 v=yes mode=add-to
why: F5 joins the ported task_polled_during_render_can_access_document to the DioxusDocument bridge tests, and the new private module mutation_buffer joins this module list, with the cited lines 15-29 and 888-890 shifted
@answer n=24 entity=existing-scopes#1.24 v=no
why: Harness::from_vdom and the test harness API are unchanged, and no blitz-test-harness module is reached
@answer n=25 entity=existing-scopes#1.25 v=no
why: the session adds no log site or span, and the replay re-fires the writer's own trace calls, so the telemetry modules are unchanged
