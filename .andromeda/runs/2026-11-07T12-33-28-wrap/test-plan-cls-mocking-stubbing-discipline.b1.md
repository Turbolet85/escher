@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the session's changed files are dioxus-native-dom's mutation_buffer, dioxus_document, lib and the probe, not blitz-dom's hand-written ShellProvider fakes in mutator.rs and document.rs
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: interaction_state_teardown.rs and its RecordingShell are not among the session's changed files
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup.rs and its CountingShell are not among the session's changed files
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: blitz-shell's event.rs and its CountedWakes are not among the session's changed files
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: link_rel_attribute.rs and its RecordingNetProvider are not among the session's changed files
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: render_blocking_stylesheet.rs and its ManualNetProvider are not among the session's changed files
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: fixtures/mod.rs is not a changed file and HeldNet's shape and callers are unmoved by the session
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: stand_load_wait.rs and its self-answering LateNet are not among the session's changed files
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: seven_guis stand.rs and its LoadNet are not among the session's changed files
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: blitz-test-harness settle.rs is not a changed file, and the Document trait and its poll signature are unmoved so the Scripted document stands
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's lib.rs test module and its Handler are not among the session's changed files
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: the WPT runner's net_provider.rs is not among the session's changed files
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: blitz-vibey-script's tests and its MapFetcher are not among the session's changed files
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: the WPT runner's main.rs and its DummyNavigationProvider are not among the session's changed files
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: blitz-traits' net, navigation and shell no-op providers are not among the session's changed files and no dependency moved
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: blitz-test-harness harness.rs is not a changed file and the Harness::from_vdom signature is unmoved
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: scrollbar_drag.rs and its NoopEventHandler are not among the session's changed files
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: custom_widget_layout.rs and its Probe and SizedProbe widget stubs are not among the session's changed files, and the widget stubs are not the runtime probe file
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: paint_tree_bench.rs and its NullScenePainter are not among the session's changed files
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: the WPT runner's test_runners/mod.rs and its virtual time are not among the session's changed files
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: the seven_guis timer and stand files holding TimerTicks are not among the session's changed files
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's lib.rs and its dummy timer are not among the session's changed files and no dependency moved
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT harness_test.rs and its replaced JS harness files are not among the session's changed files
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser's url_suggestions.rs and its capturing publish closure are not among the session's changed files
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser's url_suggestions.rs and its no-op notify closure are not among the session's changed files
