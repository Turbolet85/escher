# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the new windowed-shell redraw-route check must count redraw requests through a hand-written ShellProvider fake, as this entry states
mocking-stubbing-discipline#1.2 · no · RecordingShell is interaction_state_teardown's IME recorder and no chunk check reads its set_ime_enabled calls
mocking-stubbing-discipline#1.3 · no · RecordingNetProvider is link_rel_attribute's own URL recorder and the chunk reaches no link or stylesheet load
mocking-stubbing-discipline#1.4 · no · ManualNetProvider is upstream's file-private double in render_blocking_stylesheet, which no chunk check uses
mocking-stubbing-discipline#1.5 · no · HeldNet backs stand_settle, font_face_sniffed_source and stand_boot, none of which the chunk re-reads or adds, and the load row uses LoadNet instead
mocking-stubbing-discipline#1.6 · no · LateNet is stand_load_wait's in-file stand-in and not a component the chunk uses; the new load-fetch row acts over LoadNet and waits as stand_load_page does
mocking-stubbing-discipline#1.7 · yes · the keyboard-activation row on load-fetch acts over LoadNet, which answers LOADED_CSS after 200 ms and is counted as a load, so the row waits it out through the driver's load wait with no sleep or clock read of its own
mocking-stubbing-discipline#1.8 · no · the Scripted Document and the Held NetProvider are settle.rs unit-test doubles in blitz-test-harness, which no chunk check uses
mocking-stubbing-discipline#1.9 · no · WptNetProvider serves the local WPT directory and WPT conformance is not needed
mocking-stubbing-discipline#1.10 · no · MapFetcher is blitz-vibey-script's test-local double and no debt touches the script surface
mocking-stubbing-discipline#1.11 · no · DummyNavigationProvider is the WPT runner's and WPT conformance is not needed
mocking-stubbing-discipline#1.12 · no · the blitz-traits no-op providers stand in for no real provider, and no chunk check reads what a no-op returns
mocking-stubbing-discipline#1.13 · yes · the engine change-event proof for checkbox, radio and text input reads the dispatched DOM event names that dispatch_recorded's RecordingHandler records
mocking-stubbing-discipline#1.14 · no · NoopEventHandler is scrollbar_drag's own handler, and the change fork's proof reads recorded names through the RecordingHandler, not this handler
mocking-stubbing-discipline#1.15 · no · Probe and SizedProbe are custom_widget_layout's stub widgets, and the chunk states and reaches no Widget behaviour
mocking-stubbing-discipline#1.16 · no · NullScenePainter stands in only for paint_tree_bench timings, which the task's not-needed list drops
mocking-stubbing-discipline#1.17 · no · WPT virtual time, and WPT conformance is not needed
mocking-stubbing-discipline#1.18 · no · TimerTicks is the seven_guis Timer's stand-in, which the not-needed list drops as no debt moves time and the Cells and load rows drive no timer
mocking-stubbing-discipline#1.19 · no · debug_timer's zero-cost dummy changes no bounds value and the writing-mode body check names no timer
mocking-stubbing-discipline#1.20 · no · the WPT JS harness replacements are WPT-only and WPT conformance is not needed
mocking-stubbing-discipline#1.21 · no · the worker publish closure is apps/browser's and no debt reaches the browser app
mocking-stubbing-discipline#1.22 · no · the nucleo no-op notify closure is apps/browser's and no debt reaches the browser app
mocking-stubbing-discipline#1.23 · no · the --dry-run GitHub API stand-in belongs to the WPT diff script, which the chunk does not edit
mocking-stubbing-discipline#2.1 · no · standing S7 · a stand check asserts under the bundled DejaVu Sans with system fonts off and no skip path
mocking-stubbing-discipline#3.1 · no · the chunk adds no mocking library, dependency, install or workflow step, so the absence it records is untouched
