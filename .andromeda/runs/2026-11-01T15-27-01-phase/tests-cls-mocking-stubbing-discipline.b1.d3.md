# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the ShellProvider fake sits in mutator.rs, a predicted conflict surface, so a resolution there can move its redraw counting
mocking-stubbing-discipline#1.2 · no · RecordingShell lives in interaction_state_teardown.rs, a file the chunk neither reaches nor names, and states no behaviour the scope names
mocking-stubbing-discipline#1.3 · yes · select_popup's CountingShell counts the redraws of open, move, scroll and close, the select list contract the scope names for select_popup, with the edited key-press row feeding those moves
mocking-stubbing-discipline#1.4 · no · CountedWakes is a blitz-shell event-loop double, a crate the chunk neither reaches nor names
mocking-stubbing-discipline#1.5 · no · RecordingNetProvider sits in link_rel_attribute.rs, a file the chunk neither reaches nor names
mocking-stubbing-discipline#1.6 · no · upstream's file-private ManualNetProvider sits in render_blocking_stylesheet.rs, not a reached file, is hand-written so S7 already holds, and needs no action from the chunk
mocking-stubbing-discipline#1.7 · yes · HeldNet's load counting through is_noop, with stand_settle reading NotSettled busy Loads through it, is a value a re-proven stand check reads
mocking-stubbing-discipline#1.8 · yes · LateNet's in-flight load window (200 ms, answered inside LOAD_WAIT) is what the re-proven stand check stand_load_wait reads on the synced engine
mocking-stubbing-discipline#1.9 · yes · LoadNet's in-flight load, read by the_load_page_click_leaves_a_load_in_flight and stand_load_page, is a value a re-proven stand check reads on the synced engine
mocking-stubbing-discipline#1.10 · no · settle.rs's Scripted Document and Held provider are the harness's own unit doubles, the settle contract is named as not needed, and settle.rs is not reached
mocking-stubbing-discipline#1.11 · no · WptNetProvider is WPT runner code, upstream-only and out of this chunk's scope
mocking-stubbing-discipline#1.12 · yes · MapFetcher's NotFound for unknown paths is what dom.rs reads, and dom.rs is a blitz-vibey-script test the scope names as run on the conflict result
mocking-stubbing-discipline#1.13 · no · DummyNavigationProvider is WPT runner code, out of scope
mocking-stubbing-discipline#1.14 · no · the no-op net, navigation and shell providers in blitz-traits are not named by the scope and state no behaviour it names
mocking-stubbing-discipline#1.15 · no · RecordingHandler is a blitz-test-harness dispatch recorder, a file the chunk neither reaches nor names
mocking-stubbing-discipline#1.16 · no · NoopEventHandler in scrollbar_drag.rs states only the handler passed, and the scope names no scrollbar bound it would witness
mocking-stubbing-discipline#1.17 · no · Probe and SizedProbe are custom-widget test stubs in custom_widget_layout.rs, a file the chunk neither reaches nor names
mocking-stubbing-discipline#1.18 · no · NullScenePainter stands in for a renderer in paint_tree_bench, an ignored benchmark the scope does not need
mocking-stubbing-discipline#1.19 · no · WPT virtual-time timers are WPT runner code, out of scope
mocking-stubbing-discipline#1.20 · yes · TimerTicks' advance moves the harness VirtualClock by exactly ms, the one-clock invariant the scope names through stand_virtual_clock and the timer stand checks
mocking-stubbing-discipline#1.21 · no · debug_timer's dummy timer is an instrumentation crate the chunk neither reaches nor names
mocking-stubbing-discipline#1.22 · no · the WPT JS harness replacement is WPT runner code, out of scope
mocking-stubbing-discipline#1.23 · no · the worker publish closure is in apps/browser, whose test helpers reach the chunk only through the workspace leg's total
mocking-stubbing-discipline#1.24 · no · the nucleo no-op notify is in apps/browser, out of this chunk's scope
mocking-stubbing-discipline#1.25 · no · the --dry-run GitHub stand-in belongs to the WPT diff script, out of scope, and the chunk changes no GitHub API call
