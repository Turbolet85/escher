@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the shell fakes sit in blitz-dom's mutator and document sources, a crate the session does not reach
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell sits in interaction_state_teardown.rs, a test file outside every file the session changed
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell sits in select_popup.rs, a test file outside every file the session changed
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes sits in blitz-shell's event source, a crate outside every file the session changed
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider sits in link_rel_attribute.rs, a test file outside every file the session changed
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider sits in render_blocking_stylesheet.rs, a test file outside every file the session changed
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet sits in the shared fixtures module, which the session leaves unchanged with its stand-in count and readers
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet sits in stand_load_wait.rs, a stand-in the session leaves unchanged and no file it changed
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: LoadNet sits in examples/seven_guis/src/stand.rs, outside every file the session changed, and stays the load page's stand-in
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted Document and Held NetProvider sit in blitz-test-harness settle.rs, a crate the session does not reach
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: the blitz-net test Handler sits in blitz-net's own test module, a crate the session does not reach
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider sits in the WPT runner, which the session does not reach
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher sits in blitz-vibey-script's tests, a crate the session does not reach
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider sits in the WPT runner, which the session does not reach
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op net, navigation and shell providers sit in blitz-traits, a crate outside every file the session changed
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler sits in blitz-test-harness harness.rs, a crate the session does not reach, and the new reached record is a dioxus-native-dom path, not this one
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler sits in scrollbar_drag.rs, a test file outside every file the session changed
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe sit in custom_widget_layout.rs, a test file outside every file the session changed
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter sits in paint_tree_bench.rs, a test file outside every file the session changed
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time sits in the WPT runner, which the session does not reach
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: TimerTicks is one of the stand-ins the session leaves unchanged, and timer_step and the driver's advance stay as written
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer sits in packages/debug_timer, a crate outside every file the session changed
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT harness replacement sits in the WPT runner, which the session does not reach
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the capturing publish closure sits in apps/browser, which the session does not reach
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the no-op notify closure sits in apps/browser, which the session does not reach
