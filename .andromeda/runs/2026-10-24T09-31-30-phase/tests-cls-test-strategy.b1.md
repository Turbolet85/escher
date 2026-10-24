# Classified — tests · test-strategy.b1

## Answers
test-strategy#1.1 · yes · the harness's input synthesis, its re-exports of keyboard-types' Key, Modifiers and NamedKey, and its settle, screenshot and contrast readers are what the stand checks and upstream's key-pressing checks run through
test-strategy#2.1 · yes · the session host children and their drained stderr captures are what host_log, mcp_log and the per-target line counts of the sink read, and the chunk re-checks that sink against upstream's new print sites
test-strategy#2.2 · yes · the escher-session command-line checks, the counter.sh and flight.sh flows and record_checks.sh run inside the workspace test leg the chunk must keep green, and the recording check feeds the recorded-check replay
test-strategy#3.1 · yes · the committed Counter checks under tests/blitz-tests/checks/ and their replay by stand_replay and by seven_guis recording are a named need that turns red on a layout move until re-recorded
test-strategy#4.1 · yes · pixel tests and shot rasters (stand_act_screenshot, stand_screenshot_repeat, password_painted_masked) are a named exact-value need that a Taffy or Stylo move can shift
test-strategy#5.1 · yes · the stand's font-dependent checks assert unconditionally under the bundled DejaVu Sans with no skip path, under system-fonts in the workspace build, and Stylo's text keywords and inline layout split reach their measurements
test-strategy#6.1 · no · the WPT conformance procedure, its reftest scoring and its css/cssom-view reading are Not needed and the fork's conformance numbers are out of scope, the chunk reaching only wpt/runner's unit tests in the workspace leg
test-strategy#7.1 · yes · the blitz-tests target discipline (one target per file, tests/all.rs declared and never built, BlitzTestsTargetsTest pinning the manifest) and the #[path] unit-test modules such as bridge_tests are named needs that upstream's auto-discovered files move
