@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the session changes nothing under packages/ (untouched 8), so the blitz-dom hand-written ShellProvider fakes stand as written.
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: interaction_state_teardown.rs is not among the reached files; the session adds only the new dioxus_runtime_probe target.
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup.rs is not among the reached files, and its CountingShell is not touched by the new probe target.
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: packages/blitz-shell is not changed (untouched 8), so the CountedWakes test-local proxy stands.
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: link_rel_attribute.rs is not among the reached files, and RecordingNetProvider is unchanged.
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: render_blocking_stylesheet.rs is not among the reached files, and ManualNetProvider is unchanged.
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: fixtures/mod.rs is not among the reached files and HeldNet is unchanged (untouched 5), so the held-network stand-in stands.
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: stand_load_wait.rs is not among the reached files and LateNet is unchanged (untouched 5).
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: examples/ is not changed and LoadNet is unchanged (untouched 5), so the stand's load-page network stand-in stands.
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: packages/blitz-test-harness is not changed (untouched 7), so the settle unit-test Scripted and Held doubles stand.
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: packages/blitz-net is not changed (untouched 8), so its test-local NetHandler stands.
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: wpt/ is not touched (untouched 10), so WptNetProvider stands.
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: packages/blitz-vibey-script is not changed by the session, and its MapFetcher test double stands.
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: wpt/runner is not touched (untouched 10), so DummyNavigationProvider and its dummy base URL stand.
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: packages/blitz-traits is not changed (untouched 8), so the no-op net, navigation and shell providers stand.
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: blitz-test-harness's API and harness.rs are unchanged (untouched 7), and the new probe only calls the existing Harness::from_vdom.
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: scrollbar_drag.rs is not among the reached files, so NoopEventHandler stands.
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: custom_widget_layout.rs is not among the reached files, so the Probe and SizedProbe stub widgets stand.
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: paint_tree_bench.rs is not among the reached files, so NullScenePainter stands.
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: wpt/ is not touched (untouched 10), so the WPT virtual-time timers stand.
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: TimerTicks is unchanged and no stand-in is added (untouched 5), and examples/ and the seven_guis stand are not changed.
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: packages/debug_timer is not changed (untouched 8), so its zero-cost dummy timer stands.
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: wpt/ is not touched (untouched 10), so the runner-provided testharness and testdriver replacements stand.
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser's tests are not touched (untouched 10), so the capturing publish closure stands.
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser's tests are not touched (untouched 10), so the no-op notify closure stands.
