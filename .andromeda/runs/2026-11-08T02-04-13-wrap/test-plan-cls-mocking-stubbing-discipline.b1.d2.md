@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's mutator and document fakes are byte-unchanged and the session reaches no file cited by this row
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: interaction_state_teardown.rs and its RecordingShell are not among the session's reached files
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup.rs and its CountingShell are not among the session's reached files
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: blitz-shell's event.rs and its CountedWakes are not reached, the session touching only the dioxus-native-dom and driver graph paths
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: link_rel_attribute.rs and its RecordingNetProvider are not among the session's reached files
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: render_blocking_stylesheet.rs and its ManualNetProvider are not among the session's reached files
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet sits in the shared fixtures module, which stays unchanged, and the session adds no network stand-in and leaves the three-stand-in count as it is
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: stand_load_wait.rs and its in-file LateNet are not among the session's reached files
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: stand.rs, which holds LoadNet, is not among the reached files, its signals stay plain use_signal and its options stay unchanged
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: blitz-test-harness's settle.rs is untouched and the Harness surface is unchanged
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net is untouched and its test-local Handler sits in a file the session does not reach
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: the WPT runner and its net provider are untouched
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: blitz-vibey-script is untouched and its MapFetcher is not reached
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: the WPT runner's main.rs is untouched, so its DummyNavigationProvider and base URL stand unchanged
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: blitz-traits' no-op net, navigation and shell providers are untouched
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: blitz-test-harness's harness.rs is untouched and its RecordingHandler stays as it is
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: scrollbar_drag.rs and its NoopEventHandler are not among the session's reached files
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: custom_widget_layout.rs and its Probe stubs are not among the session's reached files
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: paint_tree_bench.rs and blitz-paint are not reached, so NullScenePainter stands unchanged
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: the WPT runner's test_runners module is untouched and its virtual timers are not reached
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: timer.rs, stand.rs, stand_timer.rs and stand_settle.rs are not among the reached files, and the graph's signal wrapper in tasks/mod.rs leaves the TimerTicks step and its clock untouched
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer is untouched and its zero-cost dummy timer is not reached
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness files are untouched and not reached
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser is untouched and its url_suggestions.rs is not among the reached files
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser is untouched and the synchronous nucleo driver's no-op notify closure is not reached
