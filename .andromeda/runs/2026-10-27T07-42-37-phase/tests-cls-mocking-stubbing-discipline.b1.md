# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the ShellProvider fakes sit in the in-file tests of mutator.rs (a conflict surface) and document.rs (a merge surface), counting the redraw requests the kept changed-set resolution must still raise
mocking-stubbing-discipline#1.2 · yes · RecordingShell records set_ime_enabled, the shell call the take's IME commit 7fc9ee3af (IME on a focused input) reaches, and implements ShellProvider in blitz-traits' shell.rs, a merge surface
mocking-stubbing-discipline#1.3 · no · RecordingNetProvider sits in link_rel_attribute.rs, which the chunk does not reach, and no predicted surface or scope commit names the NetProvider or link behaviour it records
mocking-stubbing-discipline#1.4 · no · ManualNetProvider is upstream's file-private double in render_blocking_stylesheet.rs, which the chunk does not reach, and the scope names no stylesheet-blocking behaviour; the fast leg runs it as it runs any test
mocking-stubbing-discipline#1.5 · no · HeldNet is a shared fixture stand-in, and the task's Not needed names the shared fixture pages and stand-ins (HeldNet among them), the chunk adding none
mocking-stubbing-discipline#1.6 · no · LateNet sits in stand_load_wait.rs, which the chunk does not reach, and the task's Not needed names LateNet among the stand-ins it neither adds nor changes
mocking-stubbing-discipline#1.7 · no · LoadNet is private to examples/seven_guis/src/stand.rs, which the chunk does not reach, and the task's Not needed names LoadNet among the stand-ins it does not need
mocking-stubbing-discipline#1.8 · no · the settle doubles sit in blitz-test-harness's settle.rs, which the chunk does not reach, and the scope names neither settle nor the Document poll the doubles implement
mocking-stubbing-discipline#1.9 · no · WptNetProvider serves the WPT runner, which the task's Not needed puts out of scope as upstream-only; WPT_COMMIT pins the checkout and holds no rule of this provider
mocking-stubbing-discipline#1.10 · yes · the MapFetcher NotFound answers sit in tests/dom.rs, a merge surface, and the take's cee6b8731 narrows file: loads in the fetcher of fetch.rs, also a merge surface, whose ScriptFetcher trait this double implements
mocking-stubbing-discipline#1.11 · no · DummyNavigationProvider is WPT runner wiring in wpt/runner/src/main.rs, which the task's Not needed excludes, and no reached file holds the navigation provider
mocking-stubbing-discipline#1.12 · yes · the no-op ShellProvider implementation sits at blitz-traits' shell.rs, a merge surface, so a merge that moves the trait moves this impl with it
mocking-stubbing-discipline#1.13 · no · RecordingHandler sits in the harness's harness.rs, which the chunk does not reach, and dispatch_recorded is a harness input helper that the task's Not needed excludes
mocking-stubbing-discipline#1.14 · no · NoopEventHandler sits in scrollbar_drag.rs, which the chunk does not reach, and the scope names no scrollbar-drag behaviour or EventDriver call
mocking-stubbing-discipline#1.15 · no · the Probe and SizedProbe widgets sit in custom_widget_layout.rs, which the chunk does not reach, and the scope names no custom-widget behaviour
mocking-stubbing-discipline#1.16 · no · NullScenePainter serves the #[ignore] paint benchmark, which the task's Not needed excludes
mocking-stubbing-discipline#1.17 · no · WPT's virtual-time timers sit in the WPT runner, which the task's Not needed puts out of scope as upstream-only
mocking-stubbing-discipline#1.18 · no · TimerTicks is a shared stand-in the task's Not needed names, and the clock surface its advance rides is witnessed by virtual_clock_engine and stand_virtual_clock, which name no timer stand-in
mocking-stubbing-discipline#1.19 · no · debug_timer's dummy timer sits in packages/debug_timer, which the chunk does not reach, and the scope names no phase-timing behaviour
mocking-stubbing-discipline#1.20 · no · the WPT JS harness replacement sits in the WPT runner's harness_test.rs, which the task's Not needed excludes as upstream-only
mocking-stubbing-discipline#1.21 · no · the publish seam sits in apps/browser's url_suggestions.rs, which the chunk does not reach, and the scope names no browser worker behaviour
mocking-stubbing-discipline#1.22 · no · the no-op notify closure sits in apps/browser's url_suggestions.rs, which the chunk does not reach
mocking-stubbing-discipline#1.23 · no · the --dry-run GitHub API script is the WPT diff-to-PR workflow's, which the task's Not needed puts upstream-only, and no reached file holds it
mocking-stubbing-discipline#2.1 · yes · the no-skip rule for document.rs's font-metric hover tests lives in document.rs, a merge surface, which the merge must keep with upstream's new hovering test in that file
mocking-stubbing-discipline#3.1 · yes · the mocking-library absence is searched over Cargo.toml, a merge surface, and the take's merged manifest must keep it true
