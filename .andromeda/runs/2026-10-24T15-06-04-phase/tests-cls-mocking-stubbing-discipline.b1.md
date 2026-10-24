# Classified — tests · mocking-stubbing-discipline.b1

## Answers

mocking-stubbing-discipline#1.1 · no · the redraw-counting ShellProvider fakes sit in blitz-dom's mutator.rs and document.rs, neither reached, and no Need names a shell fake
mocking-stubbing-discipline#1.2 · no · RecordingShell sits in interaction_state_teardown.rs, which the chunk does not reach, and no Need names IME recording
mocking-stubbing-discipline#1.3 · no · the URL-recording net sits in link_rel_attribute.rs, which the chunk does not reach; the sweep's held stand-in is HeldNet (1.5), not this
mocking-stubbing-discipline#1.4 · no · upstream's file-private ManualNetProvider sits in render_blocking_stylesheet.rs, which the chunk does not reach
mocking-stubbing-discipline#1.5 · yes · fixtures/mod.rs is reached, and the Needs make HeldNet the one held network stand-in each new fixture check uses rather than a new mock, its Loads count read by stand_settle
mocking-stubbing-discipline#1.6 · no · LateNet sits in stand_load_wait.rs, which the chunk does not reach; its 200 ms sleep is the carve-out the wall-time Need names, and its never-answered arm reaches HeldNet only through 1.5
mocking-stubbing-discipline#1.7 · no · the settle.rs unit-test document and provider sit in blitz-test-harness, which the chunk does not reach, and no Need names them
mocking-stubbing-discipline#1.8 · no · WptNetProvider is WPT runner tooling, which the Not-needed list excludes
mocking-stubbing-discipline#1.9 · no · MapFetcher sits in blitz-vibey-script's tests, which the Not-needed list excludes
mocking-stubbing-discipline#1.10 · no · DummyNavigationProvider is the WPT runner's, excluded with the WPT runner
mocking-stubbing-discipline#1.11 · yes · the Needs boot every stand witness headless with no net, so the sweep's witnesses run on these no-op net, navigation and shell providers
mocking-stubbing-discipline#1.12 · no · dispatch_recorded's RecordingHandler sits in harness.rs, which the chunk does not reach, and no Need names event recording
mocking-stubbing-discipline#1.13 · no · NoopEventHandler sits in scrollbar_drag.rs, which the chunk does not reach
mocking-stubbing-discipline#1.14 · no · the Probe and SizedProbe widgets sit in custom_widget_layout.rs, which the chunk does not reach
mocking-stubbing-discipline#1.15 · no · NullScenePainter is paint_tree_bench.rs's, which the Not-needed list excludes
mocking-stubbing-discipline#1.16 · no · the WPT virtual timers are the WPT runner's, which the Not-needed list excludes
mocking-stubbing-discipline#1.17 · yes · the Needs hold time to move only by the driver's advance and the stand's timer_step, and the sweep's advance and timer witnesses run through TimerTicks and timer_step
mocking-stubbing-discipline#1.18 · no · debug_timer's dummy timer is opt-in phase timing, which the chunk does not reach or enable
mocking-stubbing-discipline#1.19 · no · the WPT JS harness replacement is WPT runner tooling, which the Not-needed list excludes
mocking-stubbing-discipline#1.20 · no · the capturing publish closure sits in apps/browser, which the Not-needed list excludes
mocking-stubbing-discipline#1.21 · no · the no-op notify closure sits in apps/browser's url_suggestions.rs, excluded with the apps' tests
mocking-stubbing-discipline#1.22 · no · the --dry-run GitHub stub sits in wpt_diff_to_pr.py and its test, which the Not-needed list excludes with the WPT diff script tests
mocking-stubbing-discipline#2.1 · yes · the chunk's new text fixture checks read font metrics and must assert their precondition rather than skip, the fonts rule the Needs set and this procedure states for document.rs
mocking-stubbing-discipline#3.1 · yes · the absence's search covers .github/workflows/*.yml and .github/scripts/*.py, which hold the reached test_ci_workflows.py, coverage_floor.py and test_coverage_floor.py, so no mocking library may enter them
mocking-stubbing-discipline#3.2 · no · the mocking absence is for the apps, which the chunk does not reach and the Not-needed list excludes
