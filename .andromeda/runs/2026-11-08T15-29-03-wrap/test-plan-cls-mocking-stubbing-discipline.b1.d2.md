@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the session changes no blitz-dom file, so the mutator and document ShellProvider fakes it cites stand as they were
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: interaction_state_teardown.rs is not in the session's reach, and RecordingShell is unchanged
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup.rs is not in the session's reach, and its CountingShell is unchanged
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: blitz-shell event.rs is not in the session's reach, and CountedWakes is unchanged
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: link_rel_attribute.rs is not in the session's reach, and RecordingNetProvider is unchanged
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: render_blocking_stylesheet.rs is not in the session's reach, and ManualNetProvider is unchanged
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: the session states HeldNet stays unchanged, and fixtures/mod.rs, where it lives, is not a file the session changed
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: the session states LateNet stays unchanged, and stand_load_wait.rs is not in the session's reach
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: the session states LoadNet stays unchanged, and examples/seven_guis/src/stand.rs is not in the session's reach
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: settle.rs is not in the session's reach, and the harness settle semantics are unchanged
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net is untouched by the session, so its test-local Handler stands as it was
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: the WPT runner and its net provider are unchanged by the session
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: blitz-vibey-script is untouched by the session, so its MapFetcher stands as it was
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: the WPT runner's navigation stand-in is unchanged, since WPT is outside the session
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: blitz-traits is not in the session's reach, so its no-op providers stand as they were
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: the harness API and dispatch_recorded are unchanged, and event dispatch still reads the data-dioxus-id mark
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: scrollbar_drag.rs is not in the session's reach, and NoopEventHandler is unchanged
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: custom_widget_layout.rs is not in the session's reach, and the Probe stub widgets are unchanged
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: paint_tree_bench.rs is not in the session's reach, and NullScenePainter is unchanged
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: the WPT test runners are unchanged, so their virtual-time stand-in stands as it was
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: the session states TimerTicks stays unchanged, and timer.rs and stand_timer.rs are not in the session's reach
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer is not in the session's reach, so its zero-cost dummy timer stands as it was
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness is unchanged by the session
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser is untouched by the session, so url_suggestions.rs stands as it was
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser is untouched by the session, so the no-op notify closure stands as it was
