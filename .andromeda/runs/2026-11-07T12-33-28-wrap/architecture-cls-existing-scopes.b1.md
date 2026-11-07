@answer n=1 entity=existing-scopes#1.1 v=no
why: the session's reached files lie outside accesskit_xplat and its manifest did not move.
@answer n=2 entity=existing-scopes#1.2 v=no
why: the session's reached files lie outside debug_timer and its enable feature is untouched.
@answer n=3 entity=existing-scopes#1.3 v=no
why: the session reaches no blitz-traits file and its module list holds.
@answer n=4 entity=existing-scopes#1.4 v=no
why: the session's reached files are in dioxus-native-dom and the probe test, none in blitz-dom, so its module list holds.
@answer n=5 entity=existing-scopes#1.5 v=no
why: no reached file touches cssom.rs or resolved_style.rs.
@answer n=6 entity=existing-scopes#1.6 v=no
why: no reached file touches form.rs, iframe.rs or query_selector.rs.
@answer n=7 entity=existing-scopes#1.7 v=no
why: no reached file touches document.rs, and the MountedData calls stay synchronous and unchanged in blitz-dom.
@answer n=8 entity=existing-scopes#1.8 v=no
why: no reached file touches accessibility.rs or debug.rs.
@answer n=9 entity=existing-scopes#1.9 v=no
why: no reached file touches the blitz-dom events submodules.
@answer n=10 entity=existing-scopes#1.10 v=no
why: the scroll calls stay synchronous and unchanged, and no reached file is scrolling.rs or selection.rs.
@answer n=11 entity=existing-scopes#1.11 v=no
why: no reached file touches the Stylo modules.
@answer n=12 entity=existing-scopes#1.12 v=no
why: no reached file touches tree.rs, traversal.rs or url.rs.
@answer n=13 entity=existing-scopes#1.13 v=no
why: no reached file touches the layout directory.
@answer n=14 entity=existing-scopes#1.14 v=no
why: no reached file touches the node directory.
@answer n=15 entity=existing-scopes#1.15 v=no
why: no reached file is in blitz-html.
@answer n=16 entity=existing-scopes#1.16 v=no
why: no reached file is in blitz-net and its fetching is unchanged.
@answer n=17 entity=existing-scopes#1.17 v=no
why: no reached file is in blitz-paint.
@answer n=18 entity=existing-scopes#1.18 v=no
why: no reached file is in blitz-vibey-script.
@answer n=19 entity=existing-scopes#1.19 v=no
why: no reached file is in blitz-shell, whose one test module is unchanged.
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate only re-exports modules and no reached file is in it.
@answer n=21 entity=existing-scopes#1.21 v=no
why: no reached file is in stylo_taffy.
@answer n=22 entity=existing-scopes#1.22 v=no
why: dioxus-native's lib.rs module list and its Cargo.toml dependency on dioxus-native-dom did not move, and can_create_surfaces is unchanged.
@answer n=23 entity=existing-scopes#1.23 v=yes mode=add-to
why: fact F5, the ported task_polled_during_render_can_access_document joining the bridge tests, and the new private module mutation_buffer both join this crate's module list and its dioxus_document tests.
@answer n=24 entity=existing-scopes#1.24 v=no
why: Harness::from_vdom calls the bridge as before and no reached file is in the harness crate.
@answer n=25 entity=existing-scopes#1.25 v=no
why: the replay re-fires the writer's own trace calls, so no new log site appears in the telemetry crate.
