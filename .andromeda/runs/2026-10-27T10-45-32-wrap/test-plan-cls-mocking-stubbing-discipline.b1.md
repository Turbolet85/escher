# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · packages/blitz-dom/src/mutator.rs, a conflict surface, gained +8 lines at 368-371 and 520-523, moving the cited 1731-1740
mocking-stubbing-discipline#1.2 · no · interaction_state_teardown.rs is not reached, and RecordingShell still implements ShellProvider and records set_ime_enabled, which the session's IME change does not alter
mocking-stubbing-discipline#1.3 · no · link_rel_attribute.rs is not reached, and RecordingNetProvider's URL recording is unchanged (the file: narrowing sits in blitz-vibey-script's fetch)
mocking-stubbing-discipline#1.4 · no · render_blocking_stylesheet.rs is not reached, and upstream's file-private ManualNetProvider there is unchanged
mocking-stubbing-discipline#1.5 · no · fixtures/mod.rs is not reached (the Untouched list names HeldNet), so the held network stand-in is unchanged
mocking-stubbing-discipline#1.6 · no · stand_load_wait.rs is not reached and no stand file changed, so LateNet's self-answer stands
mocking-stubbing-discipline#1.7 · no · examples/seven_guis/src/stand.rs and stand_load_page.rs are not reached, so LoadNet stands
mocking-stubbing-discipline#1.8 · no · blitz-test-harness settle.rs is not reached and the crate is unchanged, so the Scripted Document and Held NetProvider stand
mocking-stubbing-discipline#1.9 · no · wpt/runner/src/net_provider.rs is not reached (only the WPT pin moved), and the provider still serves the local WPT directory
mocking-stubbing-discipline#1.10 · yes · stale · packages/blitz-vibey-script/tests/dom.rs, a merge surface, gained lines at 455-488 above the cited MapFetcher lines 744-754 and 779, moving them
mocking-stubbing-discipline#1.11 · no · wpt/runner/src/main.rs is not reached (only the WPT pin moved), so DummyNavigationProvider and its base URL stand
mocking-stubbing-discipline#1.12 · no · blitz-traits net.rs, navigation.rs and shell.rs are not reached, so their no-op provider implementations stand
mocking-stubbing-discipline#1.13 · no · blitz-test-harness harness.rs is not reached, so RecordingHandler's dispatch recording is unchanged
mocking-stubbing-discipline#1.14 · no · scrollbar_drag.rs is not reached, and the session names no change to EventDriver or NoopEventHandler
mocking-stubbing-discipline#1.15 · no · custom_widget_layout.rs is not reached, and the Probe and SizedProbe stubs still implement Widget
mocking-stubbing-discipline#1.16 · no · paint_tree_bench.rs is not reached, and NullScenePainter's timing stand-in is unchanged
mocking-stubbing-discipline#1.17 · no · wpt/runner/src/test_runners/mod.rs is not reached, and the session's timers change is a move inside blitz-vibey-script, not the runner's virtual time
mocking-stubbing-discipline#1.18 · no · the seven_guis timer and stand files and stand_timer, stand_settle and stand_act_timer are not reached, and the Untouched list rules TimerTicks out
mocking-stubbing-discipline#1.19 · no · packages/debug_timer/src/lib.rs is not reached, and the session names no change to debug_timer
mocking-stubbing-discipline#1.20 · no · wpt/runner/src/test_runners/harness_test.rs is not reached, and the WPT pin move changes the commit checked out, not the runner's replacement of the stock files
mocking-stubbing-discipline#1.21 · no · apps/browser/src/url_suggestions.rs is not reached (the browser change is tab.rs's new test module), so the capturing publish closure stands
mocking-stubbing-discipline#1.22 · no · url_suggestions.rs is not reached, so the synchronous driver's no-op notify closure stands
mocking-stubbing-discipline#1.23 · no · .github/scripts/wpt_diff_to_pr.py and its test are not reached, and the CI-scripts leg reads green and unchanged
mocking-stubbing-discipline#2.1 · no · packages/blitz-dom/src/document.rs is not reached, and the session names no change to hover_state_tests, their skips or assert_text_has_size
mocking-stubbing-discipline#3.1 · yes · stale · .github/workflows/*.yml, which the absence search covers, includes publish-browser.yml, a merge surface that gained 15 lines the search has not been re-read over
