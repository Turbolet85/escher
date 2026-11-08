@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's own mutator and document redraw-counting fakes, in an engine the chunk does not change and does not name
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown.rs, a file the chunk neither edits nor names
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell, in a file outside the chunk's files and scope
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes in blitz-shell's event loop, and blitz-shell is outside the chunk
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute.rs, a file the chunk neither edits nor names
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider in render_blocking_stylesheet.rs, a file the chunk neither edits nor names
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=yes
why: fixtures/mod.rs is named by the scope's shared stand helpers (item 6), and HeldNet is the held-network helper that the stand checks in scope read through
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet is private to stand_load_wait.rs, a file the chunk neither edits nor names
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: stand::boot_load and stand::options are named in scope (item 5), and LoadNet is the net provider boot_load installs through the options
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: Scripted Document and Held NetProvider are settle.rs's own unit-test doubles, and the chunk does not edit the harness's settle
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's test-local Handler, in an engine crate the chunk does not change
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider belongs to the WPT runner, which the chunk does not touch
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is blitz-vibey-script's test double, outside the chunk
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: WPT's DummyNavigationProvider belongs to the WPT runner, which the chunk does not touch
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op net, navigation and shell providers are blitz-traits defaults that the chunk uses unchanged, stand::options keeps its None net provider, and no scope item names these impls
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler is the harness double for dispatch_recorded, and the graph's handlers and click field are read from the vdom, with no scope item naming dispatch_recorded
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag.rs, a file the chunk does not touch
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe widgets in custom_widget_layout.rs, a file the chunk does not touch
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is a paint_tree_bench timing double, and the paint benchmarks are out of scope
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual-time timer threading, in the WPT runner which the chunk does not touch
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: the stand's signals move onto the registry through stand::boot_timer with TimerTicks, and timer_step is named in scope (item 5), so its one-tick delivery rule binds
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is timing instrumentation the chunk does not touch, and the chunk moves no clock
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: WPT JS harness replacement, in the WPT runner which the chunk does not touch
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser worker publication double, and no browser crate is touched
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser's no-op notify closure, and no browser crate is touched
