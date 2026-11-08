@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: hand-written ShellProvider fakes in blitz-dom's own mutator and document tests; the chunk touches none of them
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown is an unrelated blitz-tests shell fake the scope does not name
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell counts redraws for select lists, outside the graph, dispatch and coverage read
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes is a blitz-shell event-loop fake the chunk neither uses nor changes
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute is a single-test network fake, not reached by the chunk
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider is private to render_blocking_stylesheet and the scope does not name it
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet serves the settle, font and boot fixtures; the stand boots offline and the scope's hold builder does not name its network behaviour
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet is stand_load_wait's own sleeping stand-in, not one the coverage check or replay test uses
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: the coverage read reads every served screen, the load page included, through boot_load; LoadNet's in-flight answer and the offline default boot that the stand's session relies on bind
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: Scripted Document and Held NetProvider are blitz-test-harness settle unit-test doubles the chunk does not touch
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: the test-local Handler in blitz-net's cfg(test) module is outside the chunk's files and scope
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves the WPT runner, which the task marks not needed
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is a blitz-vibey-script test double, outside the chunk
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider belongs to the WPT runner, which the chunk does not reach
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op net, navigation and shell providers are default blitz-traits implementations the chunk does not change or name
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler records dispatched DOM event names in the harness; the chunk's record reads the bridge's dispatch record, not this, and the scope does not name it
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag is a single-test event double the chunk does not use
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe are custom-widget layout stubs, outside the chunk
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is a timings-only renderer stand-in in paint_tree_bench, which the task marks not needed
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: virtual time for WPT timers belongs to the WPT runner, outside the chunk
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: TimerTicks is the stand's timer stand-in that every headless boot provides, and settle and Session::run, the calls the scope names, apply its ticks, so the replay on a fresh session binds
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is an instrumentation swap the chunk neither uses nor changes
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT testharness replacements belong to the WPT runner, which the task marks not needed
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the capturing publish closure is an apps/browser worker double, outside the chunk
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the nucleo no-op notify closure is an apps/browser double, outside the chunk
