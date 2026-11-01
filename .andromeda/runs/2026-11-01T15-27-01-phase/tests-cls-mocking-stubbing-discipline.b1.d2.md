# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the ShellProvider redraw-counting fake sits in mutator.rs, a predicted conflict surface, so a resolution there must keep its redraw-count tests holding
mocking-stubbing-discipline#1.2 · no · RecordingShell in interaction_state_teardown.rs is outside the reached files and no named contract or check rules over it
mocking-stubbing-discipline#1.3 · yes · select_popup.rs is edited at the one key-press row, which runs through this CountingShell and its opening/moving/scrolling/closing redraw counts
mocking-stubbing-discipline#1.4 · no · CountedWakes in blitz-shell event.rs is outside the reached files and no named contract covers it
mocking-stubbing-discipline#1.5 · no · RecordingNetProvider in link_rel_attribute.rs is outside the reached files and no named check rules over it
mocking-stubbing-discipline#1.6 · no · upstream's file-private ManualNetProvider in render_blocking_stylesheet.rs is outside the reached files and nothing it rules over is named or changed
mocking-stubbing-discipline#1.7 · yes · HeldNet is the shared stand-in stand_settle reads through its counted Loads (is_noop not overridden), and the chunk re-proves every escher stand check on the synced engine
mocking-stubbing-discipline#1.8 · yes · LateNet's in-flight load answered inside LOAD_WAIT is the mechanism stand_load_wait reads, and that stand check is re-proved on the synced engine
mocking-stubbing-discipline#1.9 · yes · LoadNet's 200 ms in-flight load is the mechanism stand_load_page and the_load_page_click_leaves_a_load_in_flight read in examples/seven_guis/src/stand.rs, re-proved as an escher stand check
mocking-stubbing-discipline#1.10 · no · the settle unit tests' Scripted Document and Held NetProvider are harness-internal and the chunk drives settle only through the standing checks
mocking-stubbing-discipline#1.11 · no · WptNetProvider belongs to the WPT runner, which is upstream-only and out of scope
mocking-stubbing-discipline#1.12 · yes · blitz-vibey-script's dom.rs runs on the taken script engine, and MapFetcher's NotFound for unknown paths is what its script-fetch tests read
mocking-stubbing-discipline#1.13 · no · DummyNavigationProvider in the WPT runner is upstream-only and out of scope
mocking-stubbing-discipline#1.14 · no · the no-op net, navigation and shell providers in blitz-traits are outside the reached files and the chunk does not change or name their behaviour
mocking-stubbing-discipline#1.15 · no · RecordingHandler in blitz-test-harness harness.rs is harness-internal and the harness contracts are outside the chunk's work
mocking-stubbing-discipline#1.16 · no · NoopEventHandler in scrollbar_drag.rs is a driver detail of a test outside the reached files and no named contract covers it
mocking-stubbing-discipline#1.17 · no · the Probe/SizedProbe Widget stubs in custom_widget_layout.rs are outside the reached files and no named check rules over them
mocking-stubbing-discipline#1.18 · no · NullScenePainter is used only by the paint_tree_bench timing, an ignored benchmark outside the chunk's scope
mocking-stubbing-discipline#1.19 · no · WPT virtual time is upstream-only and out of scope
mocking-stubbing-discipline#1.20 · yes · TimerTicks states the clock move that advance applies to the session's VirtualClock by exactly ms, which is the clock invariant the chunk keeps (virtual_clock_engine, stand_virtual_clock, stand_timer, stand_act_timer)
mocking-stubbing-discipline#1.21 · no · the debug_timer dummy timer is outside the reached files and no named contract covers it
mocking-stubbing-discipline#1.22 · no · the WPT JS harness replacements are upstream-only and out of scope
mocking-stubbing-discipline#1.23 · no · the browser url_suggestions publish closure is a browser-crate test the scope does not change
mocking-stubbing-discipline#1.24 · no · the browser nucleo no-op notify closure is a browser-crate test the scope does not change
mocking-stubbing-discipline#1.25 · no · the wpt_diff_to_pr dry-run stand-in belongs to the repository-guarded WPT workflows, which are out of scope
