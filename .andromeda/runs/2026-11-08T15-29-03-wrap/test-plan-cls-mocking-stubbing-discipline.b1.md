@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the hand-written ShellProvider fakes in blitz-dom's mutator.rs and document.rs are outside every file the session reached
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown.rs is not in a file the session reached
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell is not in a file the session reached and the read changes no redraw
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes in blitz-shell's event.rs is not in a file the session reached
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute.rs is not in a file the session reached
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider in render_blocking_stylesheet.rs is not in a file the session reached
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet lives in fixtures/mod.rs, which the session does not reach (only the fixtures declarer and reader counts move) and the task lists HeldNet as unchanged
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet in stand_load_wait.rs is not in a file the session reached and the task lists LateNet as unchanged
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: LoadNet in seven_guis src/stand.rs and stand_load_page.rs is not in a file the session reached and the task lists LoadNet as unchanged
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted and Held stand-ins in blitz-test-harness settle.rs are not in a file the session reached and settle semantics are unchanged
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: the blitz-net Handler in its lib.rs tests is not in a file the session reached and blitz-net is untouched
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider sits in the WPT runner, which the task leaves unchanged
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher in blitz-vibey-script's tests is untouched by the session
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider and its base URL sit in the WPT runner, which the task leaves unchanged
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op providers in blitz-traits are not in a file the session reached
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler in blitz-test-harness harness.rs is not in a file the session reached and the harness API is unchanged
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag.rs is not in a file the session reached and the session adds no event handler
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: the Probe and SizedProbe widgets in custom_widget_layout.rs are not in a file the session reached
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter in paint_tree_bench.rs is not in a file the session reached and no timing stand-in changes
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual timers sit in the WPT runner, which the task leaves unchanged
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: TimerTicks and timer_step sit in seven_guis stand.rs and timer.rs, which the session does not reach, the task lists TimerTicks as unchanged, and the read moves no timer step
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is outside every file the session reached
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacements sit in the WPT runner, which the task leaves unchanged
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the capturing publish closure in apps/browser is untouched, since the task leaves apps/browser helpers unchanged
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the no-op notify closure in apps/browser is untouched, since the task leaves apps/browser helpers unchanged
