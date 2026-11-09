@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the blitz-dom shell fakes it cites sit in mutator.rs and document.rs, which the session does not reach
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown.rs is a test file the session does not reach
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell in select_popup.rs is a test file the session does not reach
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes in blitz-shell event.rs is outside every file the session reaches
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute.rs is a test file the session does not reach
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider in render_blocking_stylesheet.rs is a test file the session does not reach
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: fixtures/mod.rs is not a reached file and the session leaves the shared fixtures and HeldNet unchanged
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet in stand_load_wait.rs is not reached and stays a self-answering stand-in
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: LoadNet in examples/seven_guis/src/stand.rs is not reached and the session leaves the stand-ins unchanged
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted document and Held net in blitz-test-harness settle.rs are outside every file the session reaches
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: the blitz-net test Handler sits in blitz-net lib.rs, a crate the session does not reach
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider in the wpt runner is outside every file the session reaches
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher in blitz-vibey-script tests is a crate the session does not reach
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider in the wpt runner main.rs is outside every file the session reaches
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op providers in blitz-traits are a crate the session does not reach
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler in blitz-test-harness harness.rs is a separate recorder, not the dioxus-native-dom reached record the session adds
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag.rs is a test file the session does not reach
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: the Probe and SizedProbe widgets in custom_widget_layout.rs are a test file the session does not reach
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter in paint_tree_bench.rs is a test file the session does not reach
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time in the wpt test runners is outside every file the session reaches
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: replay.rs gains only a no-op per-step hook with behaviour unchanged, and TimerTicks, timer_step and advance keep their stated claims
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: the debug_timer dummy timer in packages/debug_timer is outside every file the session reaches
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacements in wpt/runner are outside every file the session reaches
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the apps/browser url_suggestions worker publish closure is a file the session does not reach
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the apps/browser url_suggestions notify closure is a file the session does not reach
