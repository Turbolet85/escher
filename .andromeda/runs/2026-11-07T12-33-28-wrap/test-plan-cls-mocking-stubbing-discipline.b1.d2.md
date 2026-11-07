@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: its ShellProvider fakes live in blitz-dom's mutator.rs and document.rs, which the session does not reach
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell is in blitz-tests interaction_state_teardown.rs, a file the session does not touch
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell is in select_popup.rs, which the session does not reach
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes lives in blitz-shell's event.rs, which the session does not reach
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider is in link_rel_attribute.rs, a file the session does not touch
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider is in render_blocking_stylesheet.rs, a file the session does not touch
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet is in the blitz-tests fixtures/mod.rs, which the session does not reach, and its net stand-in is unchanged
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet is in stand_load_wait.rs, a file the session does not touch
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: LoadNet is in the seven_guis stand.rs, which the session does not reach
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted Document and Held NetProvider are settle.rs test doubles in blitz-test-harness, a file the session does not reach
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: the test-local Handler is in blitz-net's unit tests, which the session does not reach
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider lives in the wpt runner, which the session does not reach
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is in blitz-vibey-script's tests, which the session does not reach
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider is in the wpt runner main.rs, which the session does not reach
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op providers are in blitz-traits net, navigation and shell, none of which the session reaches
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler is in blitz-test-harness harness.rs, a file the session does not reach, and dispatch_recorded is unchanged
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler is in scrollbar_drag.rs, a file the session does not touch
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe are in custom_widget_layout.rs, a file the session does not touch
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is in paint_tree_bench.rs, a file the session does not touch
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: the WPT virtual time sits in the wpt runner's test_runners, which the session does not reach
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: TimerTicks lives in the seven_guis timer and stand files, which the session does not reach
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer is in packages/debug_timer, which the session does not reach
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness files are in the wpt runner, which the session does not reach
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the capturing publish closure is in apps/browser url_suggestions.rs, which the session does not reach
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the no-op notify closure is in apps/browser url_suggestions.rs, which the session does not reach
