# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the shell fakes sit in blitz-dom's mutator and document tests, which the chunk reaches no file of and no re-read check runs
mocking-stubbing-discipline#1.2 · no · RecordingShell sits in interaction_state_teardown.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.3 · no · RecordingNetProvider sits in link_rel_attribute.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.4 · no · ManualNetProvider sits in render_blocking_stylesheet.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.5 · no · HeldNet serves stand_settle, font_face_sniffed_source, stand_boot and the never-answered arm of stand_load_wait, none of them re-read, and the stand boots stand_lint reads take net_provider None or LoadNet, never HeldNet
mocking-stubbing-discipline#1.6 · no · LateNet sits in stand_load_wait.rs, which the chunk neither edits nor re-reads
mocking-stubbing-discipline#1.7 · yes · the load page's boot, which stand_lint's re-read boot test pins at 0 findings, installs LoadNet, so its sheet answer after 200 ms is an input of that reading
mocking-stubbing-discipline#1.8 · no · the Scripted Document and the settle Held provider are settle.rs unit-test doubles, which the chunk does not reach
mocking-stubbing-discipline#1.9 · no · WptNetProvider sits in the WPT runner, which the chunk does not reach
mocking-stubbing-discipline#1.10 · no · MapFetcher sits in blitz-vibey-script's tests, which the chunk does not reach
mocking-stubbing-discipline#1.11 · no · DummyNavigationProvider sits in the WPT runner's main.rs, which the chunk does not reach
mocking-stubbing-discipline#1.12 · no · the no-op net is the default under the offline lean boots, and no re-pinned lint or snapshot reading takes an input from a fetch or a redraw request
mocking-stubbing-discipline#1.13 · no · RecordingHandler keeps the event names dispatch_recorded dispatches, and no re-read check reads that log, since stand_snapshot_state reads the chosen option's selected state
mocking-stubbing-discipline#1.14 · no · NoopEventHandler sits in scrollbar_drag.rs, which the chunk does not reach
mocking-stubbing-discipline#1.15 · no · Probe and SizedProbe sit in custom_widget_layout.rs, which the chunk does not reach
mocking-stubbing-discipline#1.16 · no · NullScenePainter sits in the paint_tree_bench.rs timing bench, which the chunk does not reach
mocking-stubbing-discipline#1.17 · no · virtual-time timers are the WPT runner's, which the chunk does not reach
mocking-stubbing-discipline#1.18 · yes · the timer's boot, which stand_lint's re-read boot test pins at 0 findings, runs with TimerTicks undelivered, so its elapsed text holds at 0.0 s, an input of that reading
mocking-stubbing-discipline#1.19 · no · debug_timer's dummy timer sits in the debug_timer crate, which the chunk does not reach
mocking-stubbing-discipline#1.20 · no · the WPT JS harness replacement sits in wpt/runner, which the chunk does not reach
mocking-stubbing-discipline#1.21 · no · the capturing publish closure sits in apps/browser/src/url_suggestions.rs, which the chunk does not reach
mocking-stubbing-discipline#1.22 · no · the no-op notify closure sits in apps/browser/src/url_suggestions.rs, which the chunk does not reach
mocking-stubbing-discipline#1.23 · no · the GitHub API dry-run stand-in sits in wpt_diff_to_pr.py and its test, neither reached by the chunk, and the re-read pin is test_capability_statement.py
mocking-stubbing-discipline#2.1 · no · the font-metric skip procedure governs blitz-dom's document.rs hover tests, which the chunk reaches no file of and adds no font-metric test to
mocking-stubbing-discipline#3.1 · no · the absence of mocking libraries in CI and build configuration holds as a re-check every change owes, since the chunk edits no workflow, manifest or script and adds no mock to the re-read pin
