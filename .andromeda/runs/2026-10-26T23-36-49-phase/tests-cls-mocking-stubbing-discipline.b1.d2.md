# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · blitz-dom's own mutator and document unit tests keep their ShellProvider fakes, and the chunk's lint, sweep and listing tests reach no shell double
mocking-stubbing-discipline#1.2 · no · RecordingShell belongs to interaction_state_teardown, an engine test the chunk does not touch
mocking-stubbing-discipline#1.3 · no · RecordingNetProvider belongs to link_rel_attribute, an engine test the chunk does not touch
mocking-stubbing-discipline#1.4 · no · upstream's file-private ManualNetProvider in render_blocking_stylesheet, an engine test the chunk does not touch
mocking-stubbing-discipline#1.5 · no · HeldNet serves the fixture pages' hold; the lint's session boots the offline stand through session_common::hold, which makes no request through it, and the sweep holds none
mocking-stubbing-discipline#1.6 · no · LateNet is stand_load_wait's own stand-in, neither changed nor read by the refactored tests or the lint check
mocking-stubbing-discipline#1.7 · yes · the load page's boot-time 0-findings reading, which the chunk re-measures, depends on LoadNet's one stylesheet answer given 200 ms after its fetch, and lint waits on no load
mocking-stubbing-discipline#1.8 · no · settle's unit-test Scripted document and Held net; lint runs no step and no settle
mocking-stubbing-discipline#1.9 · no · WptNetProvider serves the WPT runner, which the scope excludes
mocking-stubbing-discipline#1.10 · no · blitz-vibey-script's test-local MapFetcher sits in its DOM tests, which the scope excludes
mocking-stubbing-discipline#1.11 · no · the WPT runner's DummyNavigationProvider, which the scope excludes
mocking-stubbing-discipline#1.12 · no · the no-op providers the offline stand boots through; its pages make no request, so no reading depends on their behaviour, and the chunk changes no provider
mocking-stubbing-discipline#1.13 · no · dispatch_recorded's RecordingHandler records events for input helpers, and the lint dispatches none
mocking-stubbing-discipline#1.14 · no · NoopEventHandler in scrollbar_drag, an engine test the chunk does not touch
mocking-stubbing-discipline#1.15 · no · Probe and SizedProbe widgets in custom_widget_layout, an engine test the chunk does not touch
mocking-stubbing-discipline#1.16 · no · NullScenePainter in paint_tree_bench, a benchmark the scope excludes
mocking-stubbing-discipline#1.17 · no · the WPT runner's virtual timers, which the scope excludes
mocking-stubbing-discipline#1.18 · yes · TimerTicks is provided on every headless-stand boot the lint's session holds, and the timer's boot-time reading the chunk re-measures depends on its undelivered source holding elapsed at 0.0 s
mocking-stubbing-discipline#1.19 · no · debug_timer's dummy timer applies to phase timing, and the chunk adds no timing to the lint walk
mocking-stubbing-discipline#1.20 · no · the WPT JS harness replacement, which the scope excludes
mocking-stubbing-discipline#1.21 · no · apps/browser worker publication closure in its URL-suggestions suite, which the scope excludes
mocking-stubbing-discipline#1.22 · no · apps/browser no-op notify closure in its URL-suggestions suite, which the scope excludes
mocking-stubbing-discipline#1.23 · no · wpt_diff_to_pr.py's dry-run stand-in, a WPT script the chunk does not reach (its re-reads are ci-leg.sh and the target pin)
mocking-stubbing-discipline#2.1 · no · the no-skip rule governs blitz-dom's document.rs hover tests, which the chunk does not touch
mocking-stubbing-discipline#3.1 · no · the absence of mocking libraries stays true, since the chunk adds no dependency or CI library
