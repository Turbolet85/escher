@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's shell redraw fakes sit in its own mutator and document tests; the read touches no blitz-dom shell and the scope names no redraw count.
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown.rs records IME calls; a test file the chunk does not reach.
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell counts redraws for a popup's open, move, scroll and close; the read opens no popup and reads the owner-served options only through the graph.
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes belongs to blitz-shell's event loop test, which the chunk does not reach.
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute.rs is a test file the chunk does not touch.
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider in render_blocking_stylesheet.rs is upstream's file-private stand-in; not reached.
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet's held-request rule serves stand_settle and the font-face check; the new covered or off-screen fixture loads no network, so the rule rules over no unit the chunk writes.
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet's 200 ms self-answering fetch lives in stand_load_wait.rs, which the chunk does not touch.
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: the read is proven through stand::options, which the scope names and which keeps net_provider None so the offline stand stays offline; boot_load's LoadNet replaces it only through ..options.
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: Scripted Document and Held NetProvider are settle.rs unit doubles in blitz-test-harness, which the chunk does not change.
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's test-local Handler sits in its own cfg(test) module; blitz-net is not reached.
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves WPT files; WPT is not_needed by the chunk.
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is a blitz-vibey-script test double; no script engine is touched.
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider is in the WPT runner; not reached.
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op net, navigation and shell providers are stated as implementations only; the entity states no behaviour the read's boots must meet.
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler belongs to blitz-test-harness dispatch_recorded; the read dispatches no event and the chunk does not touch it.
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag.rs is a test file the chunk does not reach.
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe widgets belong to custom_widget_layout.rs; not reached.
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is a paint-timing stand-in in paint_tree_bench; benches are not_needed and the read paints nothing.
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time lives in the wpt runner; not reached.
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: stand::boot provides TimerTicks in context, the clock the read must leave unmoved since it runs no step and advances nothing; the boot is named in the scope.
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is blitz-dom instrumentation the chunk does not reach.
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacements belong to WPT, which is not_needed.
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: worker publication in apps/browser; the chunk does not reach apps/browser.
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the nucleo notify closure in apps/browser; the chunk does not reach apps/browser.
