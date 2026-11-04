@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's redraw-counting ShellProvider fakes in the engine's own unit tests, which the probe never boots or reads
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown records IME calls, a shell reading the probe does not take
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's own CountingShell counts select-list redraws, a reading outside the UI graph's four links
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes is a blitz-shell test-local event-loop fake, and the windowed path is read from code only, not run
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute is a single-check network fake the probe does not reach
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider is upstream's file-private stand-in in render_blocking_stylesheet, not used by the probe
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet serves stand_settle and font checks, and the probe's served task boots through LoadNet, not HeldNet
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet is stand_load_wait's in-file stand-in with a wall-time answer, a check the probe does not use
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: the probe's served task boots through stand::boot_load, whose LoadNet answers on a 200 ms thread and counts as an in-flight load that pump and settle must account for
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: Scripted Document and Held NetProvider are settle's own unit-test doubles, while the probe runs the harness's real settle
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's unit-test Handler sits in blitz-net's own tests, outside the probe's reach
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves the WPT runner's files, and the chunk runs no WPT suite
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is blitz-vibey-script's script-fetch fake, and the probe is over the Dioxus runtime, not the script engine
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider is the WPT runner's navigation fake, which the probe does not reach
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=yes
why: stand::options carries net_provider None into the lean Counter or CRUD boots, so the probe takes these no-op net, navigation and shell providers and its pump and settle readings rest on them
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=yes
why: RecordingHandler is the handler dispatch_recorded installs to record event names, which the probe must not read as a Dioxus handler run
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler is scrollbar_drag's EventDriver stand-in, a check the probe does not use
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe are custom_widget_layout's stub widgets, which the chunk does not build or read
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter stands in for a renderer in paint_tree_bench timings, a benchmark the chunk does not touch
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT timers on virtual time belong to the WPT runner, which the chunk does not reach
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: TimerTicks is the harness's time source that every stand boot provides, and pump, settle and advance move the probe's time through it, with no sleep or wall-clock read allowed
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is a timing instrument the probe does not read
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacement belongs to the WPT runner, which the chunk does not run
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the capturing publish closure is apps/browser's worker fake, and the chunk reaches no browser crate
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the no-op notify closure is apps/browser's nucleo fake, and the chunk reaches no browser crate
