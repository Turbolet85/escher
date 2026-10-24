# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the shell fakes that count redraws sit in blitz-dom's unit tests, which the chunk does not reach, and no chunk row counts redraw requests
mocking-stubbing-discipline#1.2 · no · the RecordingShell IME record sits in interaction_state_teardown.rs, which the chunk does not reach, and no chunk row touches IME
mocking-stubbing-discipline#1.3 · no · the RecordingNetProvider sits in link_rel_attribute.rs, which the chunk does not reach, and the stand boots offline
mocking-stubbing-discipline#1.4 · no · the ManualNetProvider is upstream's file-private stand-in in render_blocking_stylesheet.rs, which the chunk does not reach
mocking-stubbing-discipline#1.5 · yes · HeldNet lives in tests/fixtures/mod.rs, the module that select_interaction and stand_act_select declare and the widened leg builds, so its held-request rule binds those rows
mocking-stubbing-discipline#1.6 · no · LateNet is stand_load_wait.rs's own in-file stand-in, which the chunk does not reach and no chunk row uses
mocking-stubbing-discipline#1.7 · no · the Scripted document and Held provider are settle.rs unit-test stand-ins, which the chunk does not reach and no chunk row drives
mocking-stubbing-discipline#1.8 · no · WptNetProvider serves the WPT runner's local files, and WPT is outside the chunk
mocking-stubbing-discipline#1.9 · no · the MapFetcher is a test-local stand-in in blitz-vibey-script's DOM tests, which the chunk does not reach
mocking-stubbing-discipline#1.10 · no · the WPT runner's DummyNavigationProvider and base URL are outside the chunk
mocking-stubbing-discipline#1.11 · yes · the stand boot is offline (no network, a Needs item) on the blitz-traits no-op net, navigation and shell providers, so their no-op rule binds every row booted through it
mocking-stubbing-discipline#1.12 · no · RecordingHandler is installed by dispatch_recorded, which no chunk row drives, since the rows use press, press_with, click, type_text, focused, query, attr and text_content
mocking-stubbing-discipline#1.13 · no · NoopEventHandler is scrollbar_drag.rs's own stand-in, which the chunk does not reach
mocking-stubbing-discipline#1.14 · no · Probe and SizedProbe are custom_widget_layout.rs's stub widgets, which the chunk does not reach
mocking-stubbing-discipline#1.15 · no · NullScenePainter is paint_tree_bench.rs's timing stand-in, and the chunk's rows read computed styles, not a paint
mocking-stubbing-discipline#1.16 · no · WPT's virtual timers belong to the WPT runner, which is outside the chunk
mocking-stubbing-discipline#1.17 · yes · every headless-stand boot provides TimerTicks, which settle neither advances nor waits on, and the typeahead row reads the harness VirtualClock this entity names
mocking-stubbing-discipline#1.18 · no · debug_timer's dummy timer is opt-in phase timing, which the chunk does not reach
mocking-stubbing-discipline#1.19 · no · the WPT runner's replaced JS harness files are outside the chunk
mocking-stubbing-discipline#1.20 · no · the browser's worker publication stand-in is outside the chunk, since the browser app tests are not needed
mocking-stubbing-discipline#1.21 · no · the browser's nucleo no-op notify closure is outside the chunk
mocking-stubbing-discipline#1.22 · no · the --dry-run GitHub stand-in belongs to wpt_diff_to_pr.py and its test, which the chunk does not reach
mocking-stubbing-discipline#2.1 · no · the font-metric skip rule is document.rs's hover_state_tests in blitz-dom, which the chunk does not reach, and the chunk's own no-skip rule comes from the Needs
mocking-stubbing-discipline#3.1 · yes · test_ci_workflows.py, a reached merge surface, and the a11y job in .github/workflows/ci.yml sit inside this absence's search, so the widened leg must add no mocking library
mocking-stubbing-discipline#3.2 · no · the mocking search covers the apps, which the chunk does not reach
