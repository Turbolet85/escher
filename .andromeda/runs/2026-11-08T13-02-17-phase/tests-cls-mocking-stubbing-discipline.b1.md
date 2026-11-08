@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's test-local redraw-counting ShellProvider fakes; the chunk touches no blitz-dom file and the read requests no redraw
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: interaction_state_teardown's RecordingShell records IME calls in a check the chunk does not reach
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell counts the list's open, move, scroll and close; the read opens no popup and the scope names no redraw count
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: blitz-shell's CountedWakes event-loop double; blitz-shell is not reached
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: link_rel_attribute's RecordingNetProvider, a double of a check outside the chunk
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: render_blocking_stylesheet's file-private ManualNetProvider, a double of a check outside the chunk
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet in fixtures/mod.rs is a network double; a covered, off-screen or owner-served fixture booted with the stand's options (net provider None) does not use it and no fixture page reads it
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet is stand_load_wait's in-file double, in a check the chunk neither edits nor reads
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: the scope names boot_load and stand::options as the boots the read is proven on; this entity states that boot_load carries LoadNet's load in flight and that every other boot, stand::options included, keeps net_provider None
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the settle unit tests' Scripted Document and Held NetProvider in blitz-test-harness; the chunk does not reach the settle module
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's test-local Handler in its own cfg(test) module; blitz-net is not reached
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves the WPT runner's local files; the chunk changes no conformance reading
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher in blitz-vibey-script's tests; no script engine is touched
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider and dummy.local are WPT runner details outside the chunk
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op net, navigation and shell providers in blitz-traits state no behaviour the chunk's code or proof must meet; the stand::options offline guarantee is carried by 1.9
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: dispatch_recorded's RecordingHandler records DOM event names and the read dispatches no event
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag, a check outside the chunk
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: custom_widget_layout's Probe widgets, a check outside the chunk
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter in paint_tree_bench, a benchmark run ignored or by hand; the read paints nothing
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: the WPT runner's virtual timers; no conformance reading changes
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=no
why: TimerTicks replaces the timer's delay; the read runs no step, calls no deliver, advance, settle or pump, and the scope does not name the timer's behaviour
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer when enable is off; the read takes no timing and the chunk adds no timed span
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT testharness replacement; no conformance change
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser's worker publication closure; the chunk does not reach apps/browser
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser's nucleo notify closure; the chunk does not reach apps/browser
