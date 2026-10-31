# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the ShellProvider redraw-count fakes in mutator.rs and document.rs test a redraw path the scope never names, and neither file is reached
mocking-stubbing-discipline#1.2 · no · RecordingShell's set_ime_enabled record is an IME double, and the chunk touches no IME path
mocking-stubbing-discipline#1.3 · no · CountedWakes is a blitz-shell event-loop double, and blitz-shell is not reached; no window is launched
mocking-stubbing-discipline#1.4 · no · RecordingNetProvider serves link_rel_attribute's URL recording, a network path the chunk does not reach
mocking-stubbing-discipline#1.5 · no · ManualNetProvider serves render_blocking_stylesheet's own delivery and stand_settle, which the chunk does not change
mocking-stubbing-discipline#1.6 · no · HeldNet serves stand_settle and the font-face check; the list loads nothing and the flight booker boots offline, as the scope's Not needed states, and the chunk adds no network stand-in
mocking-stubbing-discipline#1.7 · no · LateNet is stand_load_wait's in-file load stand-in, and the scope excludes the load and network checks
mocking-stubbing-discipline#1.8 · no · LoadNet is the load page's stand-in in examples/seven_guis, which the chunk does not touch; the flight booker keeps net_provider None and the scope excludes it
mocking-stubbing-discipline#1.9 · no · the Scripted Document and Held NetProvider are settle.rs's own unit-test doubles, and settle.rs is not reached; the chunk relies on settle's behaviour by its own code, not on these doubles
mocking-stubbing-discipline#1.10 · no · WptNetProvider serves the WPT runner, which the scope excludes as repository-guarded
mocking-stubbing-discipline#1.11 · no · MapFetcher is a blitz-vibey-script double, and that package is excluded as the chunk adds no script API
mocking-stubbing-discipline#1.12 · no · DummyNavigationProvider is the WPT runner's navigation double, which the scope excludes
mocking-stubbing-discipline#1.13 · no · the no-op net, navigation and shell providers in blitz-traits are not reached, and the chunk's checks boot offline through the pinned options with no call to them named
mocking-stubbing-discipline#1.14 · no · RecordingHandler records dispatched DOM event names for dispatch_recorded, and the scope names no event recording the list's choices must match
mocking-stubbing-discipline#1.15 · no · NoopEventHandler is passed to EventDriver in scrollbar_drag.rs, a scrollbar path the chunk does not reach
mocking-stubbing-discipline#1.16 · no · Probe and SizedProbe are custom-widget layout stubs in custom_widget_layout.rs, which the chunk does not reach
mocking-stubbing-discipline#1.17 · no · NullScenePainter stands in for a renderer in paint_tree_bench timings, and the scope gives paint_bench and its timings no duty
mocking-stubbing-discipline#1.18 · no · WPT timers on virtual time belong to the WPT runner, which the scope excludes
mocking-stubbing-discipline#1.19 · yes · the scope's reduced-motion fork reads the harness's VirtualClock, which advance moves by exactly ms and settle never moves, so the rule over that clock binds; timer_step and the Timer's ticks are excluded
mocking-stubbing-discipline#1.20 · no · debug_timer's dummy timer is an instrumentation double, and the chunk adds no timing instrumentation
mocking-stubbing-discipline#1.21 · no · the WPT JS harness replacement is WPT-runner code, which the scope excludes
mocking-stubbing-discipline#1.22 · no · the worker publish closure is apps/browser's url_suggestions double, and apps/browser is not changed
mocking-stubbing-discipline#1.23 · no · the no-op notify closure is apps/browser's url_suggestions double, and apps/browser is not changed
mocking-stubbing-discipline#1.24 · no · the --dry-run GitHub API stand-in belongs to the WPT diff script, which the scope excludes; ci.yml's free-disk step is re-read, not this script
mocking-stubbing-discipline#2.1 · no · standing S3 · a check holds no skip path, a convention every new check owes, and the chunk's new checks hold none; document.rs's hover-state tests are not reached
