@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: the ShellProvider redraw fakes sit in blitz-dom's mutator and document tests, which the chunk does not reach
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell's IME record is interaction_state_teardown's own double, outside the graph reader, bridge and coverage read
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell belongs to the select popup targets, which the chunk does not touch
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes is blitz-shell's event loop test double, which the chunk does not reach
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider is link_rel_attribute's own double, which the chunk does not reach
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider is upstream's render_blocking_stylesheet double, which the chunk does not reach
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet is not named by the scope, the coverage read neither adds nor changes the network stand-ins (not_needed n=11), and the default stand boot keeps net_provider None
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet and the load-wait behaviour are neither added nor changed by the coverage read (not_needed n=11)
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=no
why: LoadNet is private to boot_load, the default stand boot keeps net_provider None, and the coverage read neither adds nor changes LoadNet (not_needed n=11)
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted Document and Held NetProvider are settle.rs unit-test doubles the chunk does not reach
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's Handler double sits in its own cfg(test) module and the chunk does not touch blitz-net
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves the WPT runner, to which the chunk adds nothing (not_needed n=1)
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is blitz-vibey-script's test double, which the chunk does not reach (not_needed n=3)
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: WPT DummyNavigationProvider belongs to the WPT runner, which the chunk does not reach
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op provider implementations in blitz-traits are definitions that state no rule the chunk's code or tests must meet
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler is blitz-test-harness's dispatch_recorded record, while the chunk's exercised record reads the signal registry, the bridge's dispatch record and the graph reader
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler is scrollbar_drag's own EventDriver double, which the chunk does not reach
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe are custom_widget_layout's widgets, which the chunk does not reach
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is a paint_tree_bench timing stand-in (not_needed n=8), which the chunk does not reach
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time belongs to the WPT runner, which the chunk does not reach
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: the seven_guis headless boot provides TimerTicks and each acting verb of Session::run applies its pass, and the chunk's sessions and replay run through both
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is instrumentation the chunk does not use
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacement belongs to the WPT runner, which the chunk does not reach
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the worker publish closure in apps/browser url_suggestions is outside the chunk (not_needed n=2)
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the notify no-op in apps/browser url_suggestions is outside the chunk (not_needed n=2)
