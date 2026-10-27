# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · blitz-dom's own redraw-counting ShellProvider fakes in its mutator.rs and document.rs unit tests, files the chunk does not reach
mocking-stubbing-discipline#1.2 · no · RecordingShell's IME recording in interaction_state_teardown.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.3 · no · RecordingNetProvider in link_rel_attribute.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.4 · no · upstream's file-private ManualNetProvider in render_blocking_stylesheet.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.5 · no · HeldNet backs the fixture, settle and stand_boot checks, none on the lint's stand-session path, whose boots are offline except the load page's LoadNet
mocking-stubbing-discipline#1.6 · no · LateNet is stand_load_wait's in-file stand-in, and lint waits on no load, so its 200 ms answer reaches no lint reading
mocking-stubbing-discipline#1.7 · yes · the LoadNet 200 ms in-flight stylesheet answer, which sets the load page's boot-time 0-finding reading that stand_lint re-reads
mocking-stubbing-discipline#1.8 · no · Scripted Document and Held NetProvider are settle unit-test doubles in blitz-test-harness, and lint takes no settle step
mocking-stubbing-discipline#1.9 · no · WptNetProvider belongs to the WPT runner, which the scope excludes
mocking-stubbing-discipline#1.10 · no · MapFetcher is a blitz-vibey-script test double, excluded by the scope
mocking-stubbing-discipline#1.11 · no · DummyNavigationProvider is the WPT runner's, excluded by the scope
mocking-stubbing-discipline#1.12 · no · the blitz-traits no-op providers are called by neither the lint nor the two refactored tests, and the chunk changes none of them
mocking-stubbing-discipline#1.13 · no · RecordingHandler is blitz-test-harness's dispatch recorder, and neither the lint nor the refactored tests dispatch an event
mocking-stubbing-discipline#1.14 · no · NoopEventHandler in scrollbar_drag.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.15 · no · Probe and SizedProbe widgets in custom_widget_layout.rs, a file the chunk does not reach
mocking-stubbing-discipline#1.16 · no · NullScenePainter is in the paint-tree benchmark, which the scope excludes
mocking-stubbing-discipline#1.17 · no · WPT virtual timers belong to the WPT runner, which the scope excludes
mocking-stubbing-discipline#1.18 · yes · TimerTicks' undelivered source holding elapsed at 0.0 s, provided by every headless-stand boot, which sets the timer's boot-time 0-finding reading that stand_lint re-reads
mocking-stubbing-discipline#1.19 · no · debug_timer's dummy timer is opt-in phase timing that the lint and the two refactored tests do not use
mocking-stubbing-discipline#1.20 · no · the WPT JS harness replacements belong to the WPT runner, which the scope excludes
mocking-stubbing-discipline#1.21 · no · the capturing publish closure is apps/browser's url_suggestions test double, excluded by the scope
mocking-stubbing-discipline#1.22 · no · the no-op nucleo notify closure is apps/browser's, excluded by the scope
mocking-stubbing-discipline#1.23 · no · the wpt_diff_to_pr.py dry-run double is the WPT diff script's, which the chunk does not reach
mocking-stubbing-discipline#2.1 · no · the hover_state_tests font-metric skip rule covers blitz-dom's document.rs engine tests, which the scope excludes and the chunk does not reach
mocking-stubbing-discipline#3.1 · no · the search finds no mocking library in CI or build configuration, and the chunk adds no dependency or test double, so the absence holds unchanged
