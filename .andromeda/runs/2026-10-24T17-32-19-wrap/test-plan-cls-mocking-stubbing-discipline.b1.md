# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the hand-written ShellProvider fakes sit in packages/blitz-dom mutator.rs and document.rs, which the session does not reach
mocking-stubbing-discipline#1.2 · no · RecordingShell sits in interaction_state_teardown.rs, which the session does not reach
mocking-stubbing-discipline#1.3 · no · RecordingNetProvider sits in link_rel_attribute.rs, which the session does not reach
mocking-stubbing-discipline#1.4 · no · ManualNetProvider sits in render_blocking_stylesheet.rs, which the session does not reach
mocking-stubbing-discipline#1.5 · yes · stale · the session moved many_items and ITEMS into fixtures/mod.rs, a reached file that holds HeldNet at lines 87–130, so that cited range can have moved
mocking-stubbing-discipline#1.6 · no · LateNet sits in stand_load_wait.rs, which the session does not reach, and the report names no change to the HeldNet it uses
mocking-stubbing-discipline#1.7 · no · the settle unit-test stand-ins sit in packages/blitz-test-harness/src/settle.rs, which the session does not reach
mocking-stubbing-discipline#1.8 · no · WptNetProvider is WPT code in wpt/runner, which the report says nothing in the session reaches
mocking-stubbing-discipline#1.9 · no · MapFetcher sits in packages/blitz-vibey-script/tests/dom.rs, which the report says nothing in the session reaches
mocking-stubbing-discipline#1.10 · no · DummyNavigationProvider is WPT code in wpt/runner/src/main.rs, which the report says nothing in the session reaches
mocking-stubbing-discipline#1.11 · no · the no-op net, navigation and shell providers sit in packages/blitz-traits, which the session does not reach
mocking-stubbing-discipline#1.12 · no · RecordingHandler sits in packages/blitz-test-harness/src/harness.rs, which the session does not reach
mocking-stubbing-discipline#1.13 · no · NoopEventHandler sits in scrollbar_drag.rs, which the session does not reach
mocking-stubbing-discipline#1.14 · no · Probe and SizedProbe sit in custom_widget_layout.rs, which the session does not reach
mocking-stubbing-discipline#1.15 · no · NullScenePainter sits in paint_tree_bench.rs, which the session does not reach
mocking-stubbing-discipline#1.16 · no · the WPT virtual-time stand-in sits in wpt/runner, which the report says nothing in the session reaches
mocking-stubbing-discipline#1.17 · no · TimerTicks sits in the timer and stand sources the session does not reach, and the report's untouched line says the time stand-ins are unchanged with no stand-in added
mocking-stubbing-discipline#1.18 · no · debug_timer sits in packages/debug_timer, which the session does not reach
mocking-stubbing-discipline#1.19 · no · the WPT JS harness replacements sit in wpt/runner, which the report says nothing in the session reaches
mocking-stubbing-discipline#1.20 · no · the capturing publish closure sits in apps/browser, which the report says nothing in the session reaches
mocking-stubbing-discipline#1.21 · no · the no-op notify closure sits in apps/browser, which the report says nothing in the session reaches
mocking-stubbing-discipline#1.22 · no · wpt_diff_to_pr.py and its test are not reached (the reached CI files are ci-leg.sh and test_ci_workflows.py)
mocking-stubbing-discipline#2.1 · no · the hover_state_tests in packages/blitz-dom/src/document.rs are not reached, and the session changes no blitz-dom test
mocking-stubbing-discipline#3.1 · no · the search covers .github/scripts/*.py, which holds the reached test_ci_workflows.py, and the report's untouched line (no stand-in was added) holds the absence on that file
mocking-stubbing-discipline#3.2 · no · the apps search covers apps/browser, which the report says nothing in the session reaches
