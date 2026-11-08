@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's own shell fakes in its unit tests, a double the chunk's new code and tests do not use
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown.rs, a file the chunk does not touch or use
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell, in a file the chunk does not touch
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: blitz-shell's CountedWakes, an event-loop double the chunk does not reach
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute.rs, a file the chunk does not touch
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider is upstream's file-private double in render_blocking_stylesheet, which the chunk does not touch
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=yes
why: HeldNet in fixtures/mod.rs is the shared held-network helper the scope names, and stand_settle, which the timer entries cite, reads its Loads settle through it, so that check must stay green
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet is private to stand_load_wait, which the chunk neither edits nor uses, and no scope item names its sleep-timed answers
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: LoadNet is the net_provider that boot_load and stand::options carry, both named in scope, and the_load_page_click_leaves_a_load_in_flight and stand_load_page witness it, so those stand checks must stay green
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: Scripted Document and Held NetProvider are settle.rs unit-test doubles of the harness, which the chunk's tests do not use
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no standing=S1
why: blitz-net's test-local Handler sits in a #[cfg(test)] mod tests, the unit-test placement convention S1 already names
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves WPT files, and WPT is outside the chunk
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is a blitz-vibey-script test double, outside the chunk
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider is a WPT runner double, outside the chunk
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op net, navigation and shell providers are blitz-traits defaults whose behaviour the scope does not name
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler is a blitz-test-harness dispatch recorder that the chunk's graph reader and its tests do not call
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag.rs, a file the chunk does not touch
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe widgets in custom_widget_layout.rs, a file the chunk does not touch
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is a paint_tree_bench timing stand-in, and the ignored benchmarks are not needed
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: virtual time for WPT timers, outside the chunk
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: TimerTicks replaces the timer's delay in boot_timer, a named call whose signals move onto the registry, and stand_timer and stand_settle witness its ticks, so those checks must stay green while the clock itself does not move
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer, outside the chunk
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: WPT JS harness replacements, outside the chunk
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser worker publication, outside the chunk
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser notify closure, outside the chunk
