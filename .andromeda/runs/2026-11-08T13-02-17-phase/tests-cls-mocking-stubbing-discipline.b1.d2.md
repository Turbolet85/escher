@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: hand-written ShellProvider redraw fakes in blitz-dom's mutator and document tests; the read reaches no blitz-dom redraw count
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown's IME test; not reached by the read
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell counts redraws when a select's list opens, moves and closes; the read opens no list and runs no step
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes in blitz-shell's event loop test; the read reaches no event loop
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute; the read makes no network request
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider in render_blocking_stylesheet; no stylesheet fetch in the read
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet in fixtures/mod.rs holds loads for stand_settle; the read's covered, off-screen or owner-served fixture needs no network answer and the scope names no load behaviour
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet in stand_load_wait, its own load-wait check; the scope names no load timing
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: the scope names stand::options and the stand boots the read is proven on; the rule that every boot other than boot_load keeps net_provider None binds those boots
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: Scripted Document and Held NetProvider in blitz-test-harness settle unit tests; the read runs no settle
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's Handler unit tests; blitz-net is not reached
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves the WPT runner; the WPT runner is not reached
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher in blitz-vibey-script; no script engine is touched
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider in the WPT runner; not reached
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: no-op net, navigation and shell providers in blitz-traits state existence only; the chunk states no rule over them
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler records dispatched DOM events; the read dispatches no event
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag; not reached
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe widgets in custom_widget_layout; not reached
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter in paint_tree_bench; the benchmark is run by hand and the read paints nothing
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time in the WPT runner; not reached
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: the scope names stand::boot, the call that provides TimerTicks to every headless-stand boot; the read's proof on those boots runs under the undelivered clock held at 0.0 s and advance moving only the session clock
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is instrumentation the chunk does not touch
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: WPT JS harness replacements; not reached
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: worker publish closure in apps/browser; the chunk does not reach apps/browser
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: no-op notify closure in apps/browser url_suggestions; not reached
