# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the shell fakes that count redraw requests sit in mutator.rs and document.rs tests, neither reached, and the chunk names no redraw count
mocking-stubbing-discipline#1.2 · no · RecordingShell's set_ime_enabled record sits in interaction_state_teardown.rs, not reached, and IME is not in the scope
mocking-stubbing-discipline#1.3 · no · CountedWakes is blitz-shell's event-loop stand-in, not reached, and no loop wake is in the scope
mocking-stubbing-discipline#1.4 · no · RecordingNetProvider is a link_rel_attribute.rs stand-in, and the list loads nothing
mocking-stubbing-discipline#1.5 · no · ManualNetProvider is upstream's render_blocking_stylesheet.rs stand-in, and the list loads nothing
mocking-stubbing-discipline#1.6 · no · HeldNet serves stand_settle, font-face and fixture loads, which the task's Not needed drops ("HeldNet ... the list loads nothing")
mocking-stubbing-discipline#1.7 · no · LateNet is stand_load_wait's stand-in, which the task's Not needed drops
mocking-stubbing-discipline#1.8 · no · LoadNet is the load page's stand-in in examples/seven_guis, which the task's Not needed drops; the flight booker's offline boot is not this entity's rule
mocking-stubbing-discipline#1.9 · no · the Scripted Document and Held NetProvider are settle.rs unit-test fakes in blitz-test-harness, not reached and not edited
mocking-stubbing-discipline#1.10 · no · WptNetProvider serves the WPT runner's files, which the task's Not needed drops
mocking-stubbing-discipline#1.11 · no · MapFetcher is a blitz-vibey-script test fake, which the task's Not needed drops
mocking-stubbing-discipline#1.12 · no · DummyNavigationProvider is the WPT runner's, which the task's Not needed drops
mocking-stubbing-discipline#1.13 · no · the no-op net, navigation and shell providers in blitz-traits are used only through the offline stand boot, and no scope verb carries their behaviour
mocking-stubbing-discipline#1.14 · no · RecordingHandler belongs to dispatch_recorded, which the scope does not name; the chunk's choice checks read the snapshot and apply_select_choice
mocking-stubbing-discipline#1.15 · no · NoopEventHandler is scrollbar_drag.rs's EventDriver stand-in, not reached and not named
mocking-stubbing-discipline#1.16 · no · Probe and SizedProbe are custom_widget_layout.rs widget stubs, not reached and not named
mocking-stubbing-discipline#1.17 · no · NullScenePainter is paint_tree_bench's timing stand-in, and the task gives paint_bench no duty
mocking-stubbing-discipline#1.18 · no · the WPT runner's virtual time is not reached, and the chunk does not run WPT
mocking-stubbing-discipline#1.19 · yes · the Timer's ticks and timer_step do not bind (the Timer holds no select), but the harness's VirtualClock that advance moves by exactly ms and that settle never moves is the virtual time the reduced-motion fork's checks read, as the scope names tick, advance_time and settle
mocking-stubbing-discipline#1.20 · no · debug_timer's dummy timer is not reached and not named
mocking-stubbing-discipline#1.21 · no · the WPT JS harness replacement is the WPT runner's, which the task's Not needed drops
mocking-stubbing-discipline#1.22 · no · the url_suggestions publish closure is apps/browser, which the task's Not needed drops
mocking-stubbing-discipline#1.23 · no · the url_suggestions no-op notify is apps/browser, which the task's Not needed drops
mocking-stubbing-discipline#1.24 · no · the GitHub API dry-run belongs to wpt_diff_to_pr.py, which the task's Not needed drops
mocking-stubbing-discipline#2.1 · no · the font-metric skip rule is over document.rs's hover_state_tests, a file the chunk does not reach or name, and the chunk's own stand checks hold no skip path under S3
