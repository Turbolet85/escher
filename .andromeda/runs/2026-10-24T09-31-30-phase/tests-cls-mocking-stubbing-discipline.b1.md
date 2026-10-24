# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the redraw-counting fake sits in mutator.rs, where the conflict and the disabled-state toggle unit tests land, and in document.rs, which the clock re-read reaches
mocking-stubbing-discipline#1.2 · no · an IME-call recorder in interaction_state_teardown.rs, a file the task does not name; the dropped-node change is hit testing, not IME
mocking-stubbing-discipline#1.3 · no · a link-rel URL recorder; the chunk changes no link or net path and the file is not named
mocking-stubbing-discipline#1.4 · no · a file-private network hold in render_blocking_stylesheet.rs, a file the task does not name
mocking-stubbing-discipline#1.5 · no · the shared HeldNet fixture, which the task leaves unedited unless a moved reading forces it; the chunk changes no load path
mocking-stubbing-discipline#1.6 · no · a driver load-wait check's self-answering stand-in; the chunk changes no driver behaviour and such checks bind only as must-stay-green
mocking-stubbing-discipline#1.7 · no · settle's scripted document and held net in blitz-test-harness's own unit tests; settle is not among the named surfaces
mocking-stubbing-discipline#1.8 · no · a WPT conformance runner file server; the chunk reaches wpt/runner only through its unit tests, and WPT conformance is out of scope
mocking-stubbing-discipline#1.9 · no · a script-fetch stand-in in blitz-vibey-script's dom tests; no named change reaches that crate
mocking-stubbing-discipline#1.10 · no · a WPT conformance runner navigation stand-in; the chunk reaches wpt/runner only through its unit tests, and conformance is out of scope
mocking-stubbing-discipline#1.11 · no · blitz-traits' no-op provider defaults; no named change reaches the provider traits or their defaults
mocking-stubbing-discipline#1.12 · yes · the harness's public event recorder, in the crate whose key re-exports move, which the key-pressing and verify_that checks compile against
mocking-stubbing-discipline#1.13 · no · a no-op event handler in scrollbar_drag.rs, a check the task does not name, and a no-op records nothing to honour
mocking-stubbing-discipline#1.14 · yes · the probe widgets' exact sizes are layout bounds that a Taffy or Stylo move can shift
mocking-stubbing-discipline#1.15 · no · NullScenePainter in paint_tree_bench, the ignored benchmark the task leaves out
mocking-stubbing-discipline#1.16 · no · a WPT conformance runner's virtual timers; the chunk reaches wpt/runner only through its unit tests, and conformance is out of scope
mocking-stubbing-discipline#1.17 · yes · the Timer's advance path runs on the VirtualClock the clock re-read covers, and the stand's timer checks run in both layout modes
mocking-stubbing-discipline#1.18 · no · debug_timer's feature-off dummy timer, in a crate the task does not name
mocking-stubbing-discipline#1.19 · no · a WPT conformance runner's harness replacements; the chunk reaches wpt/runner only through its unit tests, and conformance is out of scope
mocking-stubbing-discipline#1.20 · no · a capturing publish closure in apps/browser's suggestion tests, which the task says no change targets
mocking-stubbing-discipline#1.21 · no · a no-op notify in apps/browser's suggestion driver, the same untargeted suggestion test content
mocking-stubbing-discipline#1.22 · no · the dry-run GitHub stub of the WPT results-to-PR script, part of the upstream-only conformance workflow
mocking-stubbing-discipline#2.1 · yes · the font-dependent hover_state_tests in document.rs must not skip under system-fonts, and Stylo's text keywords and inline split reach their measurements
mocking-stubbing-discipline#3.1 · yes · the chunk edits Cargo.toml, Cargo.lock and the coverage leg's llvm-cov lines, so no mocking library may enter build configuration
mocking-stubbing-discipline#3.2 · yes · the chunk adds mimalloc to apps/browser, so no mocking library may enter the apps slice
