# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the fakes sit in mutator.rs (a conflict surface) and document.rs (a merge surface), and the mutator's changed set and those in-file tests are kept by the Needs
mocking-stubbing-discipline#1.2 · yes · interaction_state_teardown.rs is unreached, but 7fc9ee3af (IME on a focused input) is a scope-named change and this shell records the set_ime_enabled calls that IME on a focused input is about
mocking-stubbing-discipline#1.3 · no · link_rel_attribute.rs is unreached and no scope-named commit or check reads link fetching through RecordingNetProvider
mocking-stubbing-discipline#1.4 · no · render_blocking_stylesheet.rs is unreached and the scope names no render-blocking behaviour
mocking-stubbing-discipline#1.5 · no · the task's Not needed list names HeldNet, the chunk adding no fixture page and no stand-in
mocking-stubbing-discipline#1.6 · no · the task's Not needed list names LateNet, the chunk adding no fixture page and no stand-in
mocking-stubbing-discipline#1.7 · no · the task's Not needed list names LoadNet, the chunk adding no fixture page and no stand-in
mocking-stubbing-discipline#1.8 · no · settle.rs is unreached and no Needs item names the settle loop these doubles exercise
mocking-stubbing-discipline#1.9 · no · net_provider.rs is unreached, and WPT conformance and its workflows are Not needed
mocking-stubbing-discipline#1.10 · yes · tests/dom.rs is a merge surface holding this MapFetcher, and cee6b8731 narrows file: loads in the fetcher while extending that same file
mocking-stubbing-discipline#1.11 · no · DummyNavigationProvider lives in the unreached wpt/runner/src/main.rs, and WPT conformance is Not needed
mocking-stubbing-discipline#1.12 · yes · the no-op shell provider sits in blitz-traits/src/shell.rs, a merge surface, and the Dummy defaults must keep holding
mocking-stubbing-discipline#1.13 · no · harness.rs is unreached, dispatch_recorded is the harness input API the Not needed list excludes, and the task names no check that reads the recorded names
mocking-stubbing-discipline#1.14 · no · scrollbar_drag.rs is unreached and the scope names no scrollbar-drag or EventDriver behaviour
mocking-stubbing-discipline#1.15 · no · custom_widget_layout.rs is unreached and the scope names no custom-widget layout path
mocking-stubbing-discipline#1.16 · no · paint_tree_bench.rs is the #[ignore] paint benchmark, which the Not needed list excludes
mocking-stubbing-discipline#1.17 · no · the WPT runner's virtual timers are WPT conformance, which the Not needed list excludes, and the clock surface is decided by its own named checks
mocking-stubbing-discipline#1.18 · no · the task's Not needed list names TimerTicks, and the clock surface is decided by virtual_clock_engine and stand_virtual_clock, not this stand-in
mocking-stubbing-discipline#1.19 · no · debug_timer's lib.rs is unreached and the scope names no phase-timing change
mocking-stubbing-discipline#1.20 · no · the WPT JS harness replacement is WPT conformance, which the Not needed list excludes
mocking-stubbing-discipline#1.21 · no · apps/browser's url_suggestions.rs is unreached and the scope names no apps/browser change
mocking-stubbing-discipline#1.22 · no · the notify closure in apps/browser's url_suggestions.rs is unreached and the scope names no apps/browser change
mocking-stubbing-discipline#1.23 · no · the WPT diff-to-PR script and its test are a WPT workflow surface, which the Not needed list excludes
mocking-stubbing-discipline#2.1 · yes · document.rs is a merge surface holding the two hover_state_tests and upstream's third, and the in-file tests of the conflicted blitz-dom files are kept, so no skip path may enter them
mocking-stubbing-discipline#3.1 · yes · the search reads Cargo.toml, a merge surface the take moves, so a mocking crate entering the build configuration would break this absence
