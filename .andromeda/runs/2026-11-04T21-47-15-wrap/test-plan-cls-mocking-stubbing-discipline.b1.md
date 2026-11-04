@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the ShellProvider fakes live in packages/blitz-dom, which the session did not reach (only dioxus_runtime_probe.rs, Cargo.lock and deny.toml moved), and the probe adds no shell stand-in
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell is in tests/blitz-tests/tests/interaction_state_teardown.rs, which the session did not reach, and the probe adds no shell stand-in
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: CountingShell is in tests/blitz-tests/tests/select_popup.rs, which the session did not reach, and the probe adds no shell stand-in
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes is in packages/blitz-shell/src/event.rs, which the session did not reach, and the probe adds no event-loop stand-in
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in tests/blitz-tests/tests/link_rel_attribute.rs is not in any reached file, and the probe adds no network stand-in
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider in tests/blitz-tests/tests/render_blocking_stylesheet.rs is not in any reached file, and the probe adds no network stand-in
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet in tests/blitz-tests/tests/fixtures/mod.rs is not reached, and the session states HeldNet is unchanged with no stand-in added
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet in tests/blitz-tests/tests/stand_load_wait.rs is not reached, and the session states it is unchanged
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: LoadNet in examples/seven_guis/src/stand.rs is not reached (examples/ did not move), and the session states LoadNet is unchanged
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted Document and Held NetProvider are in packages/blitz-test-harness/src/settle.rs, which the session did not reach
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: the test-local Handler is in packages/blitz-net/src/lib.rs, which the session did not reach
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider is in wpt/runner/src/net_provider.rs, and WPT and its runner are untouched by the session
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is in packages/blitz-vibey-script/tests/dom.rs, which the session did not reach
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider is in wpt/runner/src/main.rs, and WPT and its runner are untouched by the session
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op providers are in packages/blitz-traits, which the session did not reach
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler is in packages/blitz-test-harness/src/harness.rs, which the session did not reach, and the Harness contract is unchanged (the probe is one more caller only)
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler is in tests/blitz-tests/tests/scrollbar_drag.rs, which the session did not reach
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe/SizedProbe are in tests/blitz-tests/tests/custom_widget_layout.rs, which the session did not reach
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is in tests/blitz-tests/tests/paint_tree_bench.rs, which the session did not reach
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual-time timers are in wpt/runner/src/test_runners/mod.rs, and WPT is untouched by the session
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: TimerTicks is in examples/seven_guis/src/tasks/timer.rs and stand.rs, neither reached, and the session states TimerTicks is unchanged
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is in packages/debug_timer/src/lib.rs, which the session did not reach
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT testharness replacements are in wpt/runner/src/test_runners/harness_test.rs, and WPT is untouched by the session
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the capturing publish closure is in apps/browser/src/url_suggestions.rs, which the session did not reach
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the no-op notify closure is in apps/browser/src/url_suggestions.rs, which the session did not reach
